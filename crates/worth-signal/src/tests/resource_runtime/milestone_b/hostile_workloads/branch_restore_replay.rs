use super::super::super::support::resource_branch_replay_workload;
use super::super::super::{ReplayEventKind, ResourceBoundaryKind, ResourceRequestId};

#[test]
fn resource_async_branch_restore_replay_equivalence_converges_for_equivalent_hostile_suffixes() {
    // Phase 9 branch-local async restore/replay torture coverage:
    // - 18: async branch restore and replay equivalence
    // - reinforces 15 and 17 under branch-local hostile async suffixes
    let outcome = resource_branch_replay_workload(ResourceRequestId::new(50_001));
    let feature = &outcome.feature;
    let sibling = &outcome.sibling;

    for (name, branch) in [("feature", feature), ("sibling", sibling)] {
        assert_ne!(
            branch.replay_after_snapshot_drift.replay_digest(),
            branch.replay_before_restore.replay_digest(),
            "{name} branch drift must perturb replay truth before restore"
        );
        assert_eq!(
            branch.head_snapshot_after_restore, branch.head_snapshot_before_restore,
            "{name} restore must preserve the branch head snapshot checkpoint"
        );
        assert!(
            branch.replay_history_after_restore.frames.len()
                >= branch.replay_history_before_restore.frames.len(),
            "{name} restore may append restore evidence, but it must not erase prior branch replay history"
        );
        assert!(
            branch
                .replay_history_after_restore
                .frames
                .iter()
                .all(|frame| frame.branch_id == branch.branch_id),
            "{name} replay history must stay branch-local after restore"
        );
        assert_eq!(
            branch
                .replay_history_after_restore
                .frames
                .iter()
                .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
                .count(),
            branch
                .replay_history_before_restore
                .frames
                .iter()
                .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
                .count(),
            "{name} restore must not invent or erase committed async replay history"
        );
        assert_eq!(
            branch.replay_after_restore.descriptor_digest(),
            branch.replay_before_restore.descriptor_digest(),
            "{name} restore must preserve descriptor truth"
        );
        assert_eq!(
            branch.replay_after_restore.lifecycle_digest(),
            branch.replay_before_restore.lifecycle_digest(),
            "{name} restore must preserve lifecycle truth"
        );
        assert_eq!(
            branch.replay_after_restore.denied_completion_digest(),
            branch.replay_before_restore.denied_completion_digest(),
            "{name} restore must preserve denial history"
        );
        assert_eq!(
            branch.replay_after_restore.in_flight_digest(),
            branch.replay_before_restore.in_flight_digest(),
            "{name} restore must reconstruct the same in-flight story"
        );
        assert_eq!(
            branch.replay_after_restore.retry_lineage_digest(),
            branch.replay_before_restore.retry_lineage_digest(),
            "{name} restore must preserve retry lineage truth"
        );
        assert_eq!(
            branch.replay_after_restore.replay_digest(),
            branch.replay_before_restore.replay_digest(),
            "{name} equivalent restored suffix must converge exactly"
        );
        assert_eq!(
            branch.restore_report.performance().boundary(),
            ResourceBoundaryKind::BranchRestore,
            "{name} restore must report branch-restore boundary truth"
        );
        assert_eq!(
            branch.restore_report.restored_in_flight_width(),
            branch.replay_after_restore.in_flight_width(),
            "{name} restore report must match replayed in-flight width"
        );
        assert_eq!(
            branch
                .diagnostics_after_restore
                .replay_reconstruction()
                .replay_digest(),
            branch.replay_after_restore.replay_digest(),
            "{name} diagnostics replay provenance must agree with replay reconstruction"
        );
    }

    assert_eq!(
        feature.replay_after_restore.descriptor_digest(),
        sibling.replay_after_restore.descriptor_digest(),
        "equivalent branch restores must converge on identical descriptor truth"
    );
    assert_eq!(
        feature.replay_after_restore.lifecycle_digest(),
        sibling.replay_after_restore.lifecycle_digest(),
        "equivalent branch restores must converge on identical lifecycle truth"
    );
    assert_eq!(
        feature.replay_after_restore.denied_completion_digest(),
        sibling.replay_after_restore.denied_completion_digest(),
        "equivalent branch restores must converge on identical denial truth"
    );
    assert_eq!(
        feature.replay_after_restore.in_flight_digest(),
        sibling.replay_after_restore.in_flight_digest(),
        "equivalent branch restores must converge on identical inflight truth"
    );
    assert_eq!(
        feature.replay_after_restore.replay_digest(),
        sibling.replay_after_restore.replay_digest(),
        "equivalent branch restores must converge on identical replay truth"
    );
    assert_eq!(
        feature.diagnostics_after_restore.provenance_digest(),
        sibling.diagnostics_after_restore.provenance_digest(),
        "equivalent restored suffixes must preserve branch-local diagnostics explanations"
    );
    assert_eq!(
        feature
            .replay_history_after_restore
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::SnapshotRestored)
            .count(),
        sibling
            .replay_history_after_restore
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::SnapshotRestored)
            .count(),
        "equivalent restored suffixes must preserve identical restore replay causality"
    );
}
