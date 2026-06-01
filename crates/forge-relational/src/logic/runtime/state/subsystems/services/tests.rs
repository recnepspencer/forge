use super::RuntimeServices;
use crate::history::data::{BranchId, CommitId};
use crate::identity::data::{PartitionId, VersionId};
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::simulation::data::{CompiledExecutionArtifact, TopologyFreezeMode};

#[test]
fn runtime_services_fork_restarts_sequence_but_preserves_artifacts() {
    let mut services = <RuntimeServices as RuntimeSubsystem>::new(&());
    let first_tx = services.next_transaction_id();
    let first_savepoint = services.next_savepoint_id();
    let first_runtime_id = services.runtime_instance_id();
    let artifact_id = services.store_compiled_artifact(CompiledExecutionArtifact {
        artifact_id: 0,
        source_commit_id: CommitId(7),
        source_version_id: VersionId(9),
        source_branch_id: BranchId("main".to_string()),
        partition_ids: vec![PartitionId::main()],
        topology_freeze_mode: TopologyFreezeMode::FreezeAtCommit,
        compiled_record_count: 3,
    });

    let mut forked = RuntimeSubsystem::fork(&services);

    assert_eq!(first_tx.0, 1);
    assert_eq!(first_savepoint.0, 1);
    assert_ne!(forked.runtime_instance_id(), first_runtime_id);
    assert_eq!(forked.next_transaction_id().0, 1);
    assert_eq!(forked.next_savepoint_id().0, 1);
    assert!(forked.compiled_artifact(artifact_id).is_some());
    assert_eq!(forked.next_compiled_artifact_id(), artifact_id + 1);
}
