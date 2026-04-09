use forge_harness::facade::ScenarioPlan;
use forge_harness::facade::{ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord};

use crate::facade::{BridgeSourceCapability, BridgeSourceCapabilitySet, BridgeTruthViewSelector};
use crate::harness::adapter::BridgeHarnessAdapter;
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
            .with_source_declaration(historical_source_declaration("source:analysis-history"))
            .with_source_adapter_capabilities(BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ]))
            .with_committed_patch(committed_patch_on_branch(
                "analysis",
                "commit-a",
                "patch-a",
                "snapshot-a",
                "name",
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("source")
    .declare_observation("source")
    .compile()
}

pub(super) fn historical_source_declaration(declaration_id: &str) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(declaration_id),
        BridgeTruthViewSelector::historical_commit(
            crate::facade::TruthBranchIdentity::new("analysis"),
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayCompatibleRead,
        ]),
    )
}

pub(super) fn materialize_target() -> String {
    "source-materialize:source:analysis-history".to_string()
}

pub(super) fn replay_target() -> String {
    "source-replay:source:analysis-history".to_string()
}

pub(super) fn materialize_batch_target() -> String {
    "source-materialize-batch:source:analysis-history".to_string()
}

pub(super) fn hostile_target() -> String {
    "source-reject-unregistered:source:hostile-missing".to_string()
}

pub(super) fn reject_open_snapshot_target() -> String {
    "source-reject-open-snapshot:source:analysis-history".to_string()
}

pub(super) fn reject_snapshot_drift_target() -> String {
    "source-reject-snapshot-drift:source:analysis-history".to_string()
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
    target: String,
) -> RunRecord<String> {
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
