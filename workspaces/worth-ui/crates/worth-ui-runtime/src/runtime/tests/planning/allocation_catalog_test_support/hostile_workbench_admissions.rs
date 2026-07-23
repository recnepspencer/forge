use crate::evidence::measurement::projection::fact_test_support::{
    host_result_portal_anchor, host_result_scroll_container_viewport, host_result_viewport_extent,
    viewport_extent_policy,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput, UiMeasurementBasis};
use crate::graph::{UiGraphSnapshot, UiGraphWorldProfile};
use crate::obligations::selection::UiSelectedObligationSet;

pub(crate) fn admitted_hostile_workbench_planning_admissions(
    label: &str,
    view_binding_id: &str,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    super::admitted_planning_admissions_with_operators(
        label,
        &[
            "operator:scroll",
            "operator:portal-anchor",
            "operator:split",
        ],
        Some(UiGraphWorldProfile::settled_query_binding(
            crate::capability::ViewBindingId::new(view_binding_id)
                .expect("workbench Query binding identity"),
            fact.query_binding_identity(),
        )),
        |ordinal, identity, target, app, capability, generation| match ordinal {
            1 => {
                let viewport = host_result_scroll_container_viewport(995, capability, generation);
                let outer = host_result_viewport_extent(996, capability, generation);
                let policy = super::scroll_owner_policy();
                let query = crate::evidence::consume_settled_query_measurement_fact(
                    identity.clone(),
                    generation,
                    &policy,
                    crate::capability::ViewBindingId::new(view_binding_id)
                        .expect("workbench Query binding identity"),
                    fact,
                )
                .expect("workbench Query extent admits");
                admit_measurement_basis(
                    identity,
                    target,
                    app.graph_snapshot().world_profile().clone(),
                    generation,
                    &policy,
                    &[
                        MeasurementEvidenceInput::host_capability_report(capability),
                        MeasurementEvidenceInput::host_measurement_result(&outer),
                        MeasurementEvidenceInput::host_measurement_result(&viewport),
                        MeasurementEvidenceInput::settled_query_fact(&query),
                    ],
                )
            }
            2 => {
                let portal = host_result_portal_anchor(997, capability, generation);
                let policy = super::portal_anchor_policy();
                admit_measurement_basis(
                    identity,
                    target,
                    app.graph_snapshot().world_profile().clone(),
                    generation,
                    &policy,
                    &[
                        MeasurementEvidenceInput::host_capability_report(capability),
                        MeasurementEvidenceInput::host_measurement_result(&portal),
                    ],
                )
            }
            _ => {
                let viewport =
                    host_result_viewport_extent(998 + ordinal as u64, capability, generation);
                admit_measurement_basis(
                    identity,
                    target,
                    app.graph_snapshot().world_profile().clone(),
                    generation,
                    &viewport_extent_policy(),
                    &[
                        MeasurementEvidenceInput::host_capability_report(capability),
                        MeasurementEvidenceInput::host_measurement_result(&viewport),
                    ],
                )
            }
        },
    )
}
