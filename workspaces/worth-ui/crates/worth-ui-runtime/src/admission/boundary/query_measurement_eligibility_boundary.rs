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
    WorthUiQueryBasisPosture, WorthUiQueryMeasurementFactReceiptError,
    WorthUiQueryPrerequisiteEvidence,
};

use super::UiAdmissionBoundary;

impl<'a> UiAdmissionBoundary<'a> {
    pub fn admit_query_measurement_eligibility_for_touch_from_projection_consumption(
        &self,
        touch: &crate::obligations::touch::UiGraphTouchDescriptor,
        consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Option<UiQueryMeasurementEligibility> {
        let base_target = crate::admission::UiAdmissionTarget::graph_node(
            touch.target().graph_node_identity(),
            crate::admission::UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        );
        let target = base_target
            .clone()
            .with_query_prerequisites_from_projection_consumption(consumption)
            .unwrap_or(base_target);
        let selected = self.select_obligations_for_target(touch, target);
        let measurement_admission = self.admit_measurement_requirement(&selected)?;
        self.admit_query_measurement_eligibility_from_projection_consumption(
            &selected,
            &measurement_admission,
            consumption,
        )
    }

    pub fn admit_query_measurement_eligibility_from_projection_consumption(
        &self,
        _selected: &crate::obligations::selection::UiSelectedObligationSet,
        measurement_admission: &UiMeasurementAdmission,
        consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Option<UiQueryMeasurementEligibility> {
        let declaration_identity = measurement_admission.declaration_identity()?.clone();
        let measurement_policy = measurement_policy_posture(self, measurement_admission)?;
        let dependencies = declared_query_measurement_dependencies(measurement_policy)?;
        let required_families = dependencies.fact_families().to_vec().into_boxed_slice();
        let target = measurement_admission.target().clone();
        let query_basis_digest = target
            .query_prerequisites()
            .map(|prerequisites| prerequisites.resolution_report().basis_digest().clone());
        let query_resolution_mode = target
            .query_prerequisites()
            .map(|prerequisites| prerequisites.resolution_report().resolution_mode().clone());
        let query_projection_contract_digest =
            target.query_prerequisites().and_then(|prerequisites| {
                prerequisites
                    .projection_contract_digest()
                    .map(|digest| digest.into())
            });
        let admission = |posture, projection_fact_receipt: Option<UiProjectionFactReceipt>| {
            UiQueryMeasurementEligibility::new(
                target.clone(),
                measurement_admission.graph_node_identity(),
                measurement_admission.declaration_identity().cloned(),
                measurement_admission.touch_identity_digest(),
                measurement_admission.selected_measurement_obligation_identity_digest(),
                measurement_admission.selected_support_authority_generation(),
                measurement_admission.boundary_support_authority_generation(),
                query_basis_digest.clone(),
                query_resolution_mode.clone(),
                query_projection_contract_digest.clone(),
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

        let query_receipt = match worth_ui_query_binding::WorthUiQueryBindingSubsystem::bootstrap()
            .prerequisites()
            .measurement_fact_receipt_from_projection_consumption(
                current_prerequisites.clone(),
                consumption,
            ) {
            Ok(query_receipt) => query_receipt,
            Err(WorthUiQueryMeasurementFactReceiptError::BasisDigestMismatch) => {
                let Some(posture) = stale_basis_posture_from_projection_consumption(
                    target.world().clone(),
                    current_prerequisites,
                    consumption,
                ) else {
                    return Some(admission(
                        UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                            world: target.world().clone(),
                            reason:
                                UiQueryMeasurementUnsupportedQueryReason::ProjectionConsumptionUnavailable,
                        },
                        None,
                    ));
                };
                return Some(admission(posture, None));
            }
            Err(
                WorthUiQueryMeasurementFactReceiptError::Observation(_)
                | WorthUiQueryMeasurementFactReceiptError::NonQueryOwnedProjectionSource
                | WorthUiQueryMeasurementFactReceiptError::ProjectionConsumptionNotAdmitted,
            ) => {
                return Some(admission(
                    UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                        world: target.world().clone(),
                        reason: UiQueryMeasurementUnsupportedQueryReason::ProjectionConsumptionUnavailable,
                    },
                    None,
                ));
            }
        };

        if !query_prerequisite_authority_matches(
            current_prerequisites,
            query_receipt.prerequisites(),
        ) {
            return Some(admission(
                stale_basis_posture(
                    target.world().clone(),
                    current_prerequisites,
                    query_receipt.prerequisites(),
                ),
                None,
            ));
        }

        if current_receipt_stale {
            return Some(admission(
                stale_basis_posture(
                    target.world().clone(),
                    current_prerequisites,
                    query_receipt.prerequisites(),
                ),
                None,
            ));
        }

        match admit_declared_measurement_projection_fact_receipt(
            declaration_identity,
            measurement_admission.selected_support_authority_generation(),
            dependencies,
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

fn query_prerequisite_authority_matches(
    current: &WorthUiQueryPrerequisiteEvidence,
    observed: &WorthUiQueryPrerequisiteEvidence,
) -> bool {
    current.basis_posture() == observed.basis_posture()
        && current.resolution_report().basis_digest() == observed.resolution_report().basis_digest()
        && current.resolution_report().resolution_mode()
            == observed.resolution_report().resolution_mode()
        && current.projection_consumption_lane() == observed.projection_consumption_lane()
        && current.inspection_lane() == observed.inspection_lane()
        && current.causal_explanation_lane() == observed.causal_explanation_lane()
        && current.projection_contract_digest() == observed.projection_contract_digest()
}

fn stale_basis_posture(
    world: crate::admission::UiAdmissionWorld,
    current: &WorthUiQueryPrerequisiteEvidence,
    observed: &WorthUiQueryPrerequisiteEvidence,
) -> UiQueryMeasurementEligibilityPosture {
    UiQueryMeasurementEligibilityPosture::StaleBasisGeneration {
        world,
        expected: prerequisite_basis_authority(current),
        observed: prerequisite_basis_authority(observed),
    }
}

fn stale_basis_posture_from_projection_consumption(
    world: crate::admission::UiAdmissionWorld,
    current: &WorthUiQueryPrerequisiteEvidence,
    consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
) -> Option<UiQueryMeasurementEligibilityPosture> {
    let completed = consumption.completed()?;
    let contract = completed.contract();
    let observed_basis_digest = contract.basis_digest()?;
    Some(UiQueryMeasurementEligibilityPosture::StaleBasisGeneration {
        world,
        expected: prerequisite_basis_authority(current),
        observed: UiQueryMeasurementBasisAuthority::ProjectionConsumption {
            basis_digest: observed_basis_digest.into(),
            projection_contract_digest: contract.contract_digest().into(),
        },
    })
}

fn prerequisite_basis_authority(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
) -> UiQueryMeasurementBasisAuthority {
    UiQueryMeasurementBasisAuthority::AdmittedPrerequisites {
        basis_digest: prerequisites.resolution_report().basis_digest().clone(),
        resolution_mode: prerequisites.resolution_report().resolution_mode().clone(),
        projection_contract_digest: prerequisites.projection_contract_digest().map(Into::into),
    }
}
