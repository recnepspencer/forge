use std::collections::BTreeSet;

use forge_relational::facade::history::BranchId;
use forge_relational::facade::snapshots::SnapshotHandle;
use forge_relational::facade::transactions::CommitResult;
use schema::facade::platform::aspects::Aspect;
<<<<<<< HEAD
use schema::facade::platform::authority::{
    MutationOrigin, RawTopologyIntent, TopologyMutationBatch,
};
=======
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutation};
>>>>>>> origin/master
use schema::facade::topology_authoring::{
    DerivedTopologyReadBasis, PersistedTopologyTruth, SeededTopologyCommit,
    TopologyCommittedMutationSet,
};

#[derive(Debug, Clone)]
pub struct TopologyCommittedArtifact {
    branch_id: BranchId,
    commits: Vec<CommitResult>,
    persisted_truth: PersistedTopologyTruth,
    read_basis: DerivedTopologyReadBasis,
}

impl TopologyCommittedArtifact {
    pub fn from_parts(
        branch_id: BranchId,
        commits: Vec<CommitResult>,
        persisted_truth: PersistedTopologyTruth,
        read_basis: DerivedTopologyReadBasis,
    ) -> Self {
        Self {
            branch_id,
            commits,
            persisted_truth,
            read_basis,
        }
    }

    pub fn empty_from_intent(
        snapshot: SnapshotHandle,
        branch_id: BranchId,
        intent: RawTopologyIntent,
    ) -> Self {
        let committed_mutation_set =
            TopologyCommittedMutationSet::from_raw_intent(intent, BTreeSet::<Aspect>::new());
        let persisted_truth = PersistedTopologyTruth {
            committed_mutation_set,
            snapshot,
            branch_id: branch_id.clone(),
        };
        let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth);
        Self {
            branch_id,
            commits: Vec::new(),
            persisted_truth,
            read_basis,
        }
    }

    pub fn empty_on_main(snapshot: SnapshotHandle, mutation_origin: MutationOrigin) -> Self {
        Self::empty_from_intent(
            snapshot,
            BranchId("main".to_string()),
            RawTopologyIntent::new(Vec::new(), mutation_origin),
        )
    }

    pub fn from_seeded_commit(seed: SeededTopologyCommit) -> Self {
        let (_committed_mutation_set, branch_id, commits, _snapshot, persisted_truth, read_basis) =
            seed.into_parts();
        Self {
            branch_id,
            commits,
            persisted_truth,
            read_basis,
        }
    }

    pub fn persisted_truth(&self) -> &PersistedTopologyTruth {
        &self.persisted_truth
    }

    pub fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }

    pub fn mutations(&self) -> &[TopologyMutation] {
        &self.persisted_truth.committed_mutation_set.mutations
    }

    pub fn mutation_origin(&self) -> MutationOrigin {
        self.persisted_truth.committed_mutation_set.mutation_origin
    }

    pub fn raw_intent(&self) -> RawTopologyIntent {
        self.persisted_truth.committed_mutation_set.raw_intent()
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn commits(&self) -> &[CommitResult] {
        &self.commits
    }
}
