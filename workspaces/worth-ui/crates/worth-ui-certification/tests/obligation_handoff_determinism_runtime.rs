#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

use worth_ui::facade::admission::{UiAdmissionQueryBasis, UiAdmissionTarget};
use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::graph::UiGraphGeneration;
use worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence;
use worth_ui_runtime::facade::admission::UiSupportSnapshot;
use worth_ui_runtime::facade::obligations::UiSelectedObligationSet;

use self::obligation_dispatch_prerequisite_support::{
    query_prerequisites, query_touch, query_touch_app, selection_target,
};

#[derive(Debug, Eq, PartialEq)]
struct DeterministicAdmissionBasis {
    graph_generation: UiGraphGeneration,
    support_snapshot: UiSupportSnapshot,
    query_prerequisites: WorthUiQueryPrerequisiteEvidence,
}

#[test]
fn equivalent_touch_descriptors_converge_to_equivalent_admission_reports() {
    let app = query_touch_app();
    let left_touch = query_touch(&app);
    let right_touch = query_touch(&app);

    let left_target = selection_target(&left_touch);
    let right_prerequisites =
        query_prerequisites(&right_touch, UiAdmissionQueryBasis::GraphAligned);
    let right_target = selection_target(&right_touch).with_query_prerequisites(right_prerequisites);

    let left_basis = deterministic_basis(&app, &left_target);
    let right_basis = deterministic_basis(&app, &right_target);
    let left_selected = app
        .admission()
        .select_obligations_for_target(&left_touch, left_target.clone());
    let right_selected = app
        .admission()
        .select_obligations_for_target(&right_touch, right_target.clone());
    let left_report = app.admission().admit_selected_obligations(&left_selected);
    let right_report = app.admission().admit_selected_obligations(&right_selected);

    assert_eq!(left_touch, right_touch);
    assert_eq!(left_target, right_target);
    assert_eq!(left_basis, right_basis);
    assert_eq!(
        support_snapshot_digest_basis(&left_selected),
        support_snapshot_digest_basis(&right_selected)
    );
    assert_eq!(left_selected, right_selected);
    assert_eq!(left_report, right_report);
    assert_eq!(
        left_report.handoff().closeout_report(),
        right_report.handoff().closeout_report()
    );
}

fn deterministic_basis(
    app: &WorthUiApp,
    target: &UiAdmissionTarget,
) -> DeterministicAdmissionBasis {
    DeterministicAdmissionBasis {
        graph_generation: app.graph().generation(),
        support_snapshot: app.admission().support_snapshot(target),
        query_prerequisites: target
            .query_prerequisites()
            .cloned()
            .expect("determinism proof requires explicit prerequisite evidence"),
    }
}

fn support_snapshot_digest_basis(selected: &UiSelectedObligationSet) -> (UiSupportSnapshot, u64) {
    (
        selected.support_snapshot().clone(),
        selected.touch().identity_digest(),
    )
}
