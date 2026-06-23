use worth_ui::facade::{
    WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadFamilyStatus,
    WorthUiCapabilityReloadStatus, WorthUiHeaderFrameRebindStatus, WorthUiRebindPhaseLane,
    WorthUiRebindPhaseSelectionStatus, WorthUiRuntimeFactId,
};

mod capability_reload_family_batch_support;

use capability_reload_family_batch_support::{
    appearance_id, assert_exact_changed_facts, batch_request, density_id, mixed_appearance_source,
    mixed_density_source, runtime_workbench,
};

#[test]
fn multi_family_capability_reload_activates_as_one_atomic_snapshot_replacement() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let prepared = workbench
        .runtime()
        .prepare_capability_reload(batch_request());

    let evidence = prepared.evidence();
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
    );
    assert_eq!(evidence.source_parse_count(), 3);
    assert_eq!(evidence.family_rows().len(), 3);
    assert_eq!(evidence.edited_delta_width(), 8);
    assert_eq!(evidence.registry_lookup_count(), 8);
    assert_eq!(evidence.artifact_tree_scan_count(), 0);
    assert_eq!(
        evidence
            .capability_changed_facts()
            .active_snapshot_digest_before(),
        before_snapshot
    );
    assert_eq!(
        Some(
            evidence
                .capability_changed_facts()
                .active_snapshot_digest_after()
        ),
        evidence.candidate_snapshot_digest()
    );
    assert_eq!(
        evidence
            .family_rows()
            .iter()
            .map(|row| (
                row.family(),
                row.status(),
                row.counters().edited_delta_width()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                WorthUiCapabilityReloadFamilyKind::ThemeTokens,
                WorthUiCapabilityReloadFamilyStatus::AdmittedChanged,
                2,
            ),
            (
                WorthUiCapabilityReloadFamilyKind::Commands,
                WorthUiCapabilityReloadFamilyStatus::AdmittedChanged,
                2,
            ),
            (
                WorthUiCapabilityReloadFamilyKind::CommandProjections,
                WorthUiCapabilityReloadFamilyStatus::AdmittedChanged,
                4,
            ),
        ]
    );
    assert_exact_changed_facts(evidence);

    let activated = workbench
        .activate_capability_reload(prepared)
        .expect("admitted multi-family batch activates at one frame boundary");

    assert_eq!(activated.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_ne!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_eq!(
        activated.active_snapshot_digest_after(),
        workbench.runtime().inspect_active().snapshot_digest()
    );
}

#[test]
fn mixed_appearance_and_density_reload_activates_as_one_candidate_snapshot() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();

    let (evidence, phase_execution) = workbench
        .apply_appearance_and_density_capability_reload(
            &mixed_appearance_source(),
            &mixed_density_source(),
        )
        .expect("mixed appearance and density reload should prepare and activate");
    let phase_execution =
        phase_execution.expect("mixed appearance+density reload should emit phase execution");

    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.family_rows().len(), 2);
    assert_eq!(evidence.canonicalization_count(), 2);
    assert_eq!(evidence.registry_lookup_count(), 2);
    assert_eq!(evidence.changed_descriptor_count(), 1);
    assert_eq!(evidence.changed_facts().len(), 1);
    assert_eq!(
        evidence.family_rebuild_breadth_for(WorthUiCapabilityReloadFamilyKind::Appearance),
        workbench
            .app()
            .capabilities()
            .appearance_tokens()
            .entries()
            .len()
    );
    assert_eq!(
        evidence.family_rebuild_breadth_for(WorthUiCapabilityReloadFamilyKind::Density),
        workbench
            .app()
            .capabilities()
            .density_tokens()
            .entries()
            .len()
    );
    assert_eq!(
        evidence
            .family_rows()
            .iter()
            .map(|row| (
                row.family(),
                row.status(),
                row.counters().changed_descriptor_count()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                WorthUiCapabilityReloadFamilyKind::Appearance,
                WorthUiCapabilityReloadFamilyStatus::AdmittedChanged,
                1,
            ),
            (
                WorthUiCapabilityReloadFamilyKind::Density,
                WorthUiCapabilityReloadFamilyStatus::EquivalentNoOp,
                0,
            ),
        ]
    );
    assert!(evidence
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::appearance_token(&appearance_id(
            "validation.appearance.header.menu_min_width",
        ))));
    assert!(!evidence
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::density_token(&density_id(
            "validation.density.header.container_padding",
        ))));
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert!(phase_execution.rows().iter().any(|row| {
        row.lane() == WorthUiRebindPhaseLane::HeaderFrame
            && row.status() == WorthUiRebindPhaseSelectionStatus::RebuildScheduled
    }));
    assert_ne!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
}
