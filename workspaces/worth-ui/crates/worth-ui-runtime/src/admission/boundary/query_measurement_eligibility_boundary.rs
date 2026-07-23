use crate::admission::{
    UiMeasurementAdmission, UiQueryMeasurementEligibility, UiQueryMeasurementEligibilityPosture,
    UiQueryMeasurementSourceIdentity, UiQueryMeasurementUnsupportedQueryReason,
};
use crate::declaration::{
    declared_query_measurement_dependencies, UiDeclarationSupportRowSchemaKind,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::{consume_settled_query_measurement_fact, UiSettledQueryFactReceiptDenial};

use super::UiAdmissionBoundary;

impl<'a> UiAdmissionBoundary<'a> {
    pub fn admit_query_measurement_eligibility_for_touch_from_settled_fact(
        &self,
        touch: &crate::obligations::touch::UiGraphTouchDescriptor,
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Option<UiQueryMeasurementEligibility> {
        let target = crate::admission::UiAdmissionTarget::graph_node(
            touch.target().graph_node_identity(),
            crate::admission::UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        );
        let selected = self.select_obligations_for_target(touch, target);
        let measurement_admission = self.admit_measurement_requirement(&selected)?;
        self.admit_query_measurement_eligibility_from_settled_fact(
            &selected,
            &measurement_admission,
            view_binding_id,
            fact,
        )
    }

    pub fn admit_query_measurement_eligibility_from_settled_fact(
        &self,
        _selected: &crate::obligations::selection::UiSelectedObligationSet,
        measurement_admission: &UiMeasurementAdmission,
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Option<UiQueryMeasurementEligibility> {
        let declaration_identity = measurement_admission.declaration_identity()?.clone();
        let measurement_policy = measurement_policy_posture(self, measurement_admission)?;
        let dependencies = declared_query_measurement_dependencies(measurement_policy)?;
        let required_families = dependencies.fact_families().to_vec().into_boxed_slice();
        let target = measurement_admission.target().clone();
        let admission = |posture, projection_fact_receipt| {
            UiQueryMeasurementEligibility::new(
                crate::admission::UiQueryMeasurementEligibilityInput {
                    target: target.clone(),
                    graph_node_identity: measurement_admission.graph_node_identity(),
                    declaration_identity: measurement_admission.declaration_identity().cloned(),
                    touch_identity_digest: measurement_admission.touch_identity_digest(),
                    selected_measurement_obligation_identity_digest: measurement_admission
                        .selected_measurement_obligation_identity_digest(),
                    selected_support_authority_generation: measurement_admission
                        .selected_support_authority_generation(),
                    boundary_support_authority_generation: measurement_admission
                        .boundary_support_authority_generation(),
                    required_families: required_families.clone(),
                    projection_fact_receipt,
                    posture,
                },
            )
        };

        let crate::graph::UiGraphWorldProfile::SettledQueryBinding {
            view_binding_id: expected_view_binding_id,
            query_binding_identity: expected_query_binding_identity,
        } = target.world().graph_world_profile()
        else {
            return Some(admission(
                UiQueryMeasurementEligibilityPosture::UnsupportedQueryPosture {
                    world: target.world().clone(),
                    reason: UiQueryMeasurementUnsupportedQueryReason::MissingSettledQueryFact,
                },
                None,
            ));
        };

        let observed =
            UiQueryMeasurementSourceIdentity::from_settled_fact(view_binding_id.clone(), fact);
        if expected_view_binding_id != &view_binding_id
            || expected_query_binding_identity.as_ref() != fact.query_binding_identity()
        {
            return Some(admission(
                UiQueryMeasurementEligibilityPosture::StaleSettlement {
                    world: target.world().clone(),
                    expected_view_binding_id: expected_view_binding_id.clone(),
                    expected_query_binding_identity: expected_query_binding_identity.clone(),
                    observed,
                },
                None,
            ));
        }

        match consume_settled_query_measurement_fact(
            declaration_identity,
            measurement_admission.selected_support_authority_generation(),
            measurement_policy,
            view_binding_id,
            fact,
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
            Err(UiSettledQueryFactReceiptDenial::MissingRequiredFactFamilies {
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
            Err(UiSettledQueryFactReceiptDenial::NoQueryMeasurementDependencies) => None,
            Err(UiSettledQueryFactReceiptDenial::SettledFactObservation(_)) => Some(admission(
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
