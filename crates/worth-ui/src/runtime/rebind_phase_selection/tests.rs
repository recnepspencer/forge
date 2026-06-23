use crate::runtime::projection_rebind::projection_rebind_test_support::{
    capability_activated, capability_denied, capability_equivalent, header_frame_plan,
    page_host_plan, projection_rebind_app, runtime_for_source_authored_page_host,
    validation_activated,
};
use crate::runtime::{
    WorthUiClassifiedRuntimeChange, WorthUiRebindPhaseLane, WorthUiRebindPhaseSelectionStatus,
    WorthUiRuntimeChangeFamilyRow,
};

#[test]
fn header_only_capability_delta_skips_page_host_lane() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let evidence = capability_activated(&mut runtime, "theme.header.text", "#ffffff");
    let admitted = runtime.admit_capability_runtime_change(&evidence).unwrap();

    let batch = runtime
        .plan_rebind_phase_selection(
            &admitted,
            runtime
                .admit_projection_plan(header_frame_plan(&app))
                .unwrap(),
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(batch.counters().phase_row_count(), 2);
    assert_eq!(batch.counters().inspected_projection_count(), 2);
    assert_eq!(batch.counters().dependency_intersection_count(), 1);
    assert_eq!(batch.counters().skipped_phase_count(), 1);
    assert_eq!(batch.counters().rebuild_attempt_count(), 1);
    assert_eq!(batch.counters().preserved_projection_count(), 1);
    assert_eq!(batch.counters().rebuilt_projection_count(), 1);
    assert_eq!(
        batch.rows(),
        &[
            crate::runtime::WorthUiRebindPhaseSelectionRow::new(
                WorthUiRebindPhaseLane::HeaderFrame,
                WorthUiRebindPhaseSelectionStatus::RebuildScheduled,
                1,
            ),
            crate::runtime::WorthUiRebindPhaseSelectionRow::new(
                WorthUiRebindPhaseLane::PageHost,
                WorthUiRebindPhaseSelectionStatus::PreservedWithoutIntersection,
                0,
            ),
        ]
    );
}

#[test]
fn authored_source_delta_skips_header_lane() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let evidence = validation_activated(&mut runtime);
    let admitted = runtime.admit_validation_runtime_change(&evidence).unwrap();

    let batch = runtime
        .plan_rebind_phase_selection(
            &admitted,
            runtime
                .admit_projection_plan(header_frame_plan(&app))
                .unwrap(),
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(batch.counters().dependency_intersection_count(), 1);
    assert_eq!(batch.counters().skipped_phase_count(), 1);
    assert_eq!(batch.counters().rebuild_attempt_count(), 1);
    assert_eq!(batch.counters().preserved_projection_count(), 1);
    assert_eq!(batch.counters().rebuilt_projection_count(), 1);
    assert_eq!(
        batch.rows()[0],
        crate::runtime::WorthUiRebindPhaseSelectionRow::new(
            WorthUiRebindPhaseLane::HeaderFrame,
            WorthUiRebindPhaseSelectionStatus::PreservedWithoutIntersection,
            0,
        )
    );
    assert_eq!(
        batch.rows()[1],
        crate::runtime::WorthUiRebindPhaseSelectionRow::new(
            WorthUiRebindPhaseLane::PageHost,
            WorthUiRebindPhaseSelectionStatus::RebuildScheduled,
            1,
        )
    );
}

#[test]
fn equivalent_capability_delta_preserves_all_lanes_without_rebuild() {
    let app = projection_rebind_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let evidence = capability_equivalent(&runtime);
    let admitted = runtime.admit_capability_runtime_change(&evidence).unwrap();

    let batch = runtime
        .plan_rebind_phase_selection(
            &admitted,
            runtime
                .admit_projection_plan(header_frame_plan(&app))
                .unwrap(),
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(batch.counters().phase_row_count(), 2);
    assert_eq!(batch.counters().dependency_intersection_count(), 0);
    assert_eq!(batch.counters().skipped_phase_count(), 2);
    assert_eq!(batch.counters().rebuild_attempt_count(), 0);
    assert_eq!(batch.counters().preserved_projection_count(), 2);
    assert_eq!(batch.counters().rebuilt_projection_count(), 0);
    assert_eq!(
        batch.rows(),
        &[
            crate::runtime::WorthUiRebindPhaseSelectionRow::new(
                WorthUiRebindPhaseLane::HeaderFrame,
                WorthUiRebindPhaseSelectionStatus::PreservedEquivalentReload,
                0,
            ),
            crate::runtime::WorthUiRebindPhaseSelectionRow::new(
                WorthUiRebindPhaseLane::PageHost,
                WorthUiRebindPhaseSelectionStatus::PreservedEquivalentReload,
                0,
            ),
        ]
    );
}

#[test]
fn denied_capability_delta_preserves_all_lanes_without_rebuild() {
    let app = projection_rebind_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let evidence = capability_denied(&runtime);
    let admitted = runtime.admit_capability_runtime_change(&evidence).unwrap();

    let batch = runtime
        .plan_rebind_phase_selection(
            &admitted,
            runtime
                .admit_projection_plan(header_frame_plan(&app))
                .unwrap(),
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(batch.counters().phase_row_count(), 2);
    assert_eq!(batch.counters().dependency_intersection_count(), 0);
    assert_eq!(batch.counters().skipped_phase_count(), 2);
    assert_eq!(batch.counters().rebuild_attempt_count(), 0);
    assert_eq!(batch.counters().preserved_projection_count(), 2);
    assert_eq!(batch.counters().rebuilt_projection_count(), 0);
    assert_eq!(
        batch.rows(),
        &[
            crate::runtime::WorthUiRebindPhaseSelectionRow::new(
                WorthUiRebindPhaseLane::HeaderFrame,
                WorthUiRebindPhaseSelectionStatus::PreservedDeniedReload,
                0,
            ),
            crate::runtime::WorthUiRebindPhaseSelectionRow::new(
                WorthUiRebindPhaseLane::PageHost,
                WorthUiRebindPhaseSelectionStatus::PreservedDeniedReload,
                0,
            ),
        ]
    );
}

#[test]
fn mixed_family_rows_rebuild_only_consuming_lanes_and_replay_deterministically() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let capability = capability_activated(&mut runtime, "theme.header.text", "#ffffff");
    let validation = validation_activated(&mut runtime);
    let classified = WorthUiClassifiedRuntimeChange::from_rows(vec![
        WorthUiRuntimeChangeFamilyRow::from_validation_evidence(&validation),
        WorthUiRuntimeChangeFamilyRow::from_capability_evidence(&capability),
    ])
    .unwrap();
    let runtime_witness = classified.runtime_instance();
    let admitted = crate::runtime::WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified.clone(),
        runtime_witness,
    )
    .unwrap();
    let replay_admitted = crate::runtime::WorthUiAdmittedRuntimeChangeEvidence::admit(
        WorthUiClassifiedRuntimeChange::from_rows(
            classified.family_rows().iter().cloned().rev().collect(),
        )
        .unwrap(),
        runtime_witness,
    )
    .unwrap();

    let batch = runtime
        .plan_rebind_phase_selection(
            &admitted,
            runtime
                .admit_projection_plan(header_frame_plan(&app))
                .unwrap(),
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .unwrap();
    let replay_batch = runtime
        .plan_rebind_phase_selection(
            &replay_admitted,
            runtime
                .admit_projection_plan(header_frame_plan(&app))
                .unwrap(),
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(batch.counters().dependency_intersection_count(), 2);
    assert_eq!(batch.counters().skipped_phase_count(), 0);
    assert_eq!(batch.counters().rebuild_attempt_count(), 2);
    assert_eq!(batch.counters().preserved_projection_count(), 0);
    assert_eq!(batch.counters().rebuilt_projection_count(), 2);
    assert_eq!(
        batch.rows()[0].status(),
        WorthUiRebindPhaseSelectionStatus::RebuildScheduled
    );
    assert_eq!(
        batch.rows()[1].status(),
        WorthUiRebindPhaseSelectionStatus::RebuildScheduled
    );
    assert_eq!(batch.replay_digest(), replay_batch.replay_digest());
    assert_eq!(batch.rows(), replay_batch.rows());
}
