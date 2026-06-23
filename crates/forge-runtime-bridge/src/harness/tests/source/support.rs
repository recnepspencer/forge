use forge_harness::facade::ScenarioPlan;
use forge_harness::facade::{ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord};

use crate::facade::{BridgeSourceCapability, BridgeSourceCapabilitySet, BridgeTruthViewSelector};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::source::{SourceDeclaration, SourceDeclarationIdentity};

use super::super::support::{committed_patch_on_branch, registration, snapshot};

pub(super) fn source_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_source_declaration(historical_source_declaration(
                SourceDeclarationIdentity::admit_bridge_owned("source:analysis-history"),
            ))
            .with_source_adapter_capabilities(BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayContinuityRead,
            ]))
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("source")
    .declare_observation("source")
    .compile()
}

pub(super) fn historical_source_declaration(
    declaration_identity: SourceDeclarationIdentity,
) -> SourceDeclaration {
    SourceDeclaration::new(
        declaration_identity,
        BridgeTruthViewSelector::historical_commit(
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayContinuityRead,
        ]),
    )
}

pub(super) fn materialize_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::source_materialize(SourceDeclarationIdentity::admit_bridge_owned(
        "source:analysis-history",
    ))
}

pub(super) fn replay_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::source_replay(SourceDeclarationIdentity::admit_bridge_owned(
        "source:analysis-history",
    ))
}

pub(super) fn materialize_batch_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::source_materialize_batch(SourceDeclarationIdentity::admit_bridge_owned(
        "source:analysis-history",
    ))
}

pub(super) fn hostile_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::source_reject_unregistered(
        SourceDeclarationIdentity::admit_bridge_owned("source:hostile-missing"),
    )
}

pub(super) fn reject_open_snapshot_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::source_reject_open_snapshot(
        SourceDeclarationIdentity::admit_bridge_owned("source:analysis-history"),
    )
}

pub(super) fn reject_snapshot_drift_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::source_reject_snapshot_drift(
        SourceDeclarationIdentity::admit_bridge_owned("source:analysis-history"),
    )
}

pub(super) fn direct_host_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(name)
}

pub(super) fn wrapped_host_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-wrapped-host"))
        .with_metadata("source_adapter_shape", "wrapped")
}

pub(super) fn sources_first_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-sources-first"))
        .with_metadata("source_builder_load_order", "sources_first")
}

pub(super) fn rejecting_adapter_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-reject-open-snapshot"))
        .with_metadata("source_adapter_behavior", "reject_open_snapshot")
}

pub(super) fn drifting_adapter_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-drift-snapshot-identity"))
        .with_metadata("source_adapter_behavior", "drift_snapshot_identity")
}

pub(super) fn execute_source_run(
    profile: ExecutionProfile,
    request_name: &str,
    target: BridgeHarnessTargetId,
) -> RunRecord<BridgeHarnessTargetId> {
    let adapter = BridgeHarnessAdapter;
    let fixture = source_fixture("bridge-source-matrix");
    let mut runtime = adapter.create_runtime().expect("source harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("source harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("source harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target),
            &profile,
        )
        .expect("source harness execution")
}
