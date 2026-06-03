use forge_harness::facade::{
    certification_matrix, ExecutionProfile, ExecutionRequest, HarnessAdapter, ScenarioPlan,
};

use super::super::support::{
    committed_patch, field_aspect_registration, field_slice_snapshot, registration,
};
use super::continuity_authority::{
    ambiguous_continuity_authority, continuity_authority, continuity_authority_with_successor,
};
use crate::facade::{
    BridgeContinuityAuthorityBasis, BridgeHistoricalResolvedRecordIdentity, BridgeLineageContext,
    TruthBranchIdentity, TruthSnapshotIdentity,
};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;

#[test]
fn bridge_continuity_certification_matrix_reports_candidate_profile_parity() {
    let fixture = ScenarioPlan::new(
        "bridge-continuity-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(field_slice_snapshot(
                TruthSnapshotIdentity::new("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge continuity certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_harness_branch_divergence_changes_terminal_continuity_export() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );

    let main_fixture = ScenarioPlan::new(
        "bridge-continuity-main",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority_with_successor(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                    BridgeHistoricalResolvedRecordIdentity::new("entity:0:4:2"),
                ),
            )
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(field_slice_snapshot(
                TruthSnapshotIdentity::new("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let feature_fixture = ScenarioPlan::new(
        "bridge-continuity-feature",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("feature"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority_with_successor(
                    TruthBranchIdentity::new("feature"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                    BridgeHistoricalResolvedRecordIdentity::new("entity:0:5:2"),
                ),
            )
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(field_slice_snapshot(
                TruthSnapshotIdentity::new("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let mut main_runtime = adapter.create_runtime().expect("main harness runtime");
    adapter
        .prepare_runtime(&mut main_runtime, &profile)
        .expect("main harness prepare");
    adapter
        .load_fixture(&mut main_runtime, &main_fixture)
        .expect("main harness load fixture");
    let main_run = adapter
        .execute(&mut main_runtime, &main_fixture, &request, &profile)
        .expect("main harness execute");

    let mut feature_runtime = adapter.create_runtime().expect("feature harness runtime");
    adapter
        .prepare_runtime(&mut feature_runtime, &profile)
        .expect("feature harness prepare");
    adapter
        .load_fixture(&mut feature_runtime, &feature_fixture)
        .expect("feature harness load fixture");
    let feature_run = adapter
        .execute(&mut feature_runtime, &feature_fixture, &request, &profile)
        .expect("feature harness execute");

    assert_ne!(main_run.summary, feature_run.summary);
    assert_ne!(main_run.extensions, feature_run.extensions);
    assert!(main_run.extensions.contains_key("bridge_continuity_record"));
    assert!(feature_run
        .extensions
        .contains_key("bridge_continuity_record"));
}

#[test]
fn bridge_harness_continuity_exports_ambiguous_rejection_record() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );
    let fixture = ScenarioPlan::new(
        "bridge-continuity-ambiguous",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                ambiguous_continuity_authority(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(field_slice_snapshot(
                TruthSnapshotIdentity::new("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let mut runtime = adapter.create_runtime().expect("harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("harness load fixture");
    let run = adapter
        .execute(&mut runtime, &fixture, &request, &profile)
        .expect("harness execute");

    assert!(run.summary.is_object());
    assert!(run.extensions.contains_key("bridge_continuity_record"));
}
