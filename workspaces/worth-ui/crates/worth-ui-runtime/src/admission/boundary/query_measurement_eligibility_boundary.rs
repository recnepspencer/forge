use crate::admission::{
    UiMeasurementAdmission, UiQueryMeasurementBasisAuthority, UiQueryMeasurementEligibility,
    UiQueryMeasurementEligibilityPosture, UiQueryMeasurementUnsupportedQueryReason,
};
use crate::declaration::{
    declared_query_measurement_dependencies, UiDeclarationSupportRowSchemaKind,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::{
    admit_declared_measurement_projection_fact_receipt, UiProjectionFactReceipt,
    UiProjectionFactReceiptDenial,
};
use worth_ui_query_binding::{
    WorthUiQueryAuthorityHandle, WorthUiQueryBasisPosture, WorthUiQueryMeasurementFactReceiptError,
    WorthUiQueryPrerequisiteBoundary, WorthUiQueryPrerequisiteEvidence,
};

use super::UiAdmissionBoundary;

impl<'a> UiAdmissionBoundary<'a> {
    pub fn admit_query_measurement_eligibility_for_touch_from_query_authority(
        &self,
        touch: &crate::obligations::touch::UiGraphTouchDescriptor,
        query_authority: WorthUiQueryAuthorityHandle,
    ) -> Option<UiQueryMeasurementEligibility> {
        let base_target = crate::admission::UiAdmissionTarget::graph_node(
            touch.target().graph_node_identity(),
            crate::admission::UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        );
        let target = base_target
            .clone()
            .with_query_prerequisites_from_query_authority(&query_authority)
            .unwrap_or(base_target);
        let selected = self.select_obligations_for_target(touch, target);
        let measurement_admission = self.admit_measurement_requirement(&selected)?;
        self.admit_query_measurement_eligibility_from_query_authority(
            &selected,
            &measurement_admission,
            query_authority,
        )
    }

    pub fn admit_query_measurement_eligibility_from_query_authority(
        &self,
        _selected: &crate::obligations::selection::UiSelectedObligationSet,
        measurement_admission: &UiMeasurementAdmission,
        query_authority: WorthUiQueryAuthorityHandle,
    ) -> Option<UiQueryMeasurementEligibility> {
        let declaration_identity = measurement_admission.declaration_identity()?.clone();
        let measurement_policy = measurement_policy_posture(self, measurement_admission)?;
        let dependencies = declared_query_measurement_dependencies(measurement_policy)?;
        let required_families = dependencies.fact_families().to_vec().into_boxed_slice();
        let target = measurement_admission.target().clone();
        let admission = |posture, projection_fact_receipt: Option<UiProjectionFactReceipt>| {
            UiQueryMeasurementEligibility::new(
                target.clone(),
                measurement_admission.graph_node_identity(),
                measurement_admission.declaration_identity().cloned(),
                measurement_admission.touch_identity_digest(),
                measurement_admission.selected_measurement_obligation_identity_digest(),
                measurement_admission.selected_support_authority_generation(),
                measurement_admission.boundary_support_authority_generation(),
                required_families.clone(),
                projection_fact_receipt,
                posture,
            )
        };

        let Some(current_prerequisites) = target.query_prerequisites() else {
            return Some(admission(
                UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                    world: target.world().clone(),
                    reason: UiQueryMeasurementUnsupportedQueryReason::MissingQueryPrerequisites,
                },
                None,
            ));
        };

        let current_receipt_stale = matches!(
            current_prerequisites.basis_posture(),
            WorthUiQueryBasisPosture::StaleReceipt
        );
        match current_prerequisites.basis_posture() {
            WorthUiQueryBasisPosture::WrongWorldProjection => {
                return Some(admission(
                    UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                        world: target.world().clone(),
                        reason: UiQueryMeasurementUnsupportedQueryReason::WrongWorldProjection,
                    },
                    None,
                ));
            }
            WorthUiQueryBasisPosture::RebindRequired => {
                return Some(admission(
                    UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                        world: target.world().clone(),
                        reason: UiQueryMeasurementUnsupportedQueryReason::RebindRequired,
                    },
                    None,
                ));
            }
            WorthUiQueryBasisPosture::AmbiguousSources => {
                return Some(admission(
                    UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                        world: target.world().clone(),
                        reason: UiQueryMeasurementUnsupportedQueryReason::AmbiguousSources,
                    },
                    None,
                ));
            }
            WorthUiQueryBasisPosture::StaleReceipt => {}
            WorthUiQueryBasisPosture::GraphAligned => {}
        }

        let observed_authority = observed_basis_authority(&query_authority);
        let query_receipt = match WorthUiQueryPrerequisiteBoundary::new()
            .measurement_fact_receipt_from_query_authority(
                current_prerequisites.clone(),
                query_authority,
            ) {
            Ok(receipt) => receipt,
            Err(WorthUiQueryMeasurementFactReceiptError::BasisDigestMismatch) => {
                let posture = stale_basis_posture_from_query_authority(
                    target.world().clone(),
                    current_prerequisites,
                    observed_authority,
                );
                return Some(admission(posture, None));
            }
            Err(_) => {
                return Some(admission(
                    UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                        world: target.world().clone(),
                        reason: UiQueryMeasurementUnsupportedQueryReason::ProjectionConsumptionUnavailable,
                    },
                    None,
                ));
            }
        };

        if !query_receipt.binds_prerequisites(current_prerequisites) {
            return Some(admission(
                stale_basis_posture_from_query_authority(
                    target.world().clone(),
                    current_prerequisites,
                    observed_authority.clone(),
                ),
                None,
            ));
        }

        if current_receipt_stale {
            return Some(admission(
                stale_basis_posture_from_query_authority(
                    target.world().clone(),
                    current_prerequisites,
                    observed_authority,
                ),
                None,
            ));
        }

        match admit_declared_measurement_projection_fact_receipt(
            declaration_identity,
            measurement_admission.selected_support_authority_generation(),
            dependencies,
            current_prerequisites.resolution_mode(),
            query_receipt,
        ) {
            Ok(receipt) => Some(admission(
                UiQueryMeasurementEligibilityPosture::Eligible {
                    world: target.world().clone(),
                    available_families: receipt
                        .consumed_fact_families()
                        .to_vec()
                        .into_boxed_slice(),
                    available_fact_family_set_digest: receipt.consumed_fact_family_set_digest(),
                },
                Some(receipt),
            )),
            Err(UiProjectionFactReceiptDenial::MissingRequiredFactFamilies {
                required,
                consumed,
            }) => Some(admission(
                UiQueryMeasurementEligibilityPosture::UnavailableFactFamilies {
                    world: target.world().clone(),
                    available_families: consumed.clone(),
                    missing_families: required
                        .iter()
                        .copied()
                        .filter(|family| !consumed.contains(family))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                },
                None,
            )),
            Err(UiProjectionFactReceiptDenial::NoQueryMeasurementDependencies) => None,
            Err(UiProjectionFactReceiptDenial::QueryFactReceipt(_)) => Some(admission(
                UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                    world: target.world().clone(),
                    reason:
                        UiQueryMeasurementUnsupportedQueryReason::ProjectionConsumptionUnavailable,
                },
                None,
            )),
        }
    }
}

fn measurement_policy_posture<'a>(
    boundary: &'a UiAdmissionBoundary<'_>,
    measurement_admission: &UiMeasurementAdmission,
) -> Option<&'a UiDeclaredMeasurementPolicyPosture> {
    let declaration_identity = measurement_admission.declaration_identity()?;
    let artifact = boundary.support_artifact(declaration_identity)?;
    let snapshot = artifact.support_snapshot().ok()?;
    let row = snapshot.row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)?;
    row.declared_measurement_policy_posture()
}

fn stale_basis_posture_from_query_authority(
    world: crate::admission::UiAdmissionWorld,
    current: &WorthUiQueryPrerequisiteEvidence,
    observed: UiQueryMeasurementBasisAuthority,
) -> UiQueryMeasurementEligibilityPosture {
    UiQueryMeasurementEligibilityPosture::StaleBasisGeneration {
        world,
        expected: prerequisite_basis_authority(current),
        observed,
    }
}

fn observed_basis_authority(
    authority: &WorthUiQueryAuthorityHandle,
) -> UiQueryMeasurementBasisAuthority {
    UiQueryMeasurementBasisAuthority::ProjectionConsumption {
        authority: authority.clone(),
    }
}

fn prerequisite_basis_authority(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
) -> UiQueryMeasurementBasisAuthority {
    UiQueryMeasurementBasisAuthority::AdmittedPrerequisites {
        prerequisites: prerequisites.clone(),
    }
}
