use std::collections::BTreeSet;

use forge_relational::facade::history::BranchId;
use forge_relational::facade::snapshots::SnapshotHandle;
use forge_relational::facade::transactions::CommitResult;
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutationBatch};
use schema::facade::topology_authoring::{
    CanonicalTopologyMutationBatch, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    SeededTopologyCommit,
};

#[derive(Debug, Clone)]
pub struct TopologyCommittedArtifact {
    pub canonical_batch: CanonicalTopologyMutationBatch,
    pub branch_id: BranchId,
    pub commits: Vec<CommitResult>,
    pub persisted_truth: PersistedTopologyTruthBatch,
    pub read_basis: DerivedTopologyReadBasis,
}

impl TopologyCommittedArtifact {
    pub fn from_parts(
        canonical_batch: CanonicalTopologyMutationBatch,
        branch_id: BranchId,
        commits: Vec<CommitResult>,
        persisted_truth: PersistedTopologyTruthBatch,
        read_basis: DerivedTopologyReadBasis,
    ) -> Self {
        Self {
            canonical_batch,
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
        let batch = TopologyMutationBatch::from_raw_intent(intent, BTreeSet::<Aspect>::new());
        let canonical_batch = CanonicalTopologyMutationBatch {
            batch: batch.clone(),
        };
        let persisted_truth = PersistedTopologyTruthBatch {
            batch,
            snapshot,
            branch_id: branch_id.clone(),
            mutation_origin: canonical_batch.batch.mutation_origin,
        };
        let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth);
        Self {
            canonical_batch,
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
        let (canonical_batch, branch_id, commits, _snapshot, persisted_truth, read_basis) =
            seed.into_parts();
        Self {
            canonical_batch,
            branch_id,
            commits,
            persisted_truth,
            read_basis,
        }
    }

    pub fn persisted_truth(&self) -> &PersistedTopologyTruthBatch {
        &self.persisted_truth
    }

    pub fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }

    pub fn canonical_batch(&self) -> &CanonicalTopologyMutationBatch {
        &self.canonical_batch
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn commits(&self) -> &[CommitResult] {
        &self.commits
    }
}
