use forge_relational::facade::history::BranchId;
#[cfg(test)]
use forge_relational::facade::snapshots::SnapshotHandle;
use forge_relational::facade::transactions::CommitResult;
#[cfg(test)]
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::TopologyMutation;
#[cfg(test)]
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};
#[cfg(test)]
use schema::facade::topology_authoring::TopologyCommittedMutationSet;
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, SeededTopologyCommit};
#[cfg(test)]
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub(crate) struct TopologyCommitCertificationInput {
    authority_mutations: Vec<TopologyMutation>,
    commits: Vec<CommitResult>,
    read_basis: DerivedTopologyReadBasis,
}

impl TopologyCommitCertificationInput {
    #[cfg(test)]
    pub(crate) fn empty_from_intent(
        snapshot: SnapshotHandle,
        branch_id: BranchId,
        intent: RawTopologyIntent,
    ) -> Self {
        let committed_mutation_set = TopologyCommittedMutationSet::from_raw_intent(
            intent.clone(),
            BTreeSet::<Aspect>::new(),
        );
        let persisted_truth = schema::facade::topology_authoring::PersistedTopologyTruth {
            committed_mutation_set,
            snapshot,
            branch_id,
        };
        Self {
            authority_mutations: intent.mutations,
            commits: Vec::new(),
            read_basis: DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth),
        }
    }

    #[cfg(test)]
    pub(crate) fn empty_on_main(snapshot: SnapshotHandle, mutation_origin: MutationOrigin) -> Self {
        Self::empty_from_intent(
            snapshot,
            BranchId("main".to_string()),
            RawTopologyIntent::new(Vec::new(), mutation_origin),
        )
    }

    pub(crate) fn from_seeded_commit(seed: SeededTopologyCommit) -> Self {
        let (_committed_mutation_set, _branch_id, commits, _snapshot, persisted_truth, read_basis) =
            seed.into_parts();
        Self {
            authority_mutations: persisted_truth
                .committed_mutation_set
                .raw_intent()
                .mutations,
            commits,
            read_basis,
        }
    }

    pub(crate) fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> &SnapshotHandle {
        self.read_basis.snapshot()
    }

    pub(crate) fn branch_id(&self) -> &BranchId {
        self.read_basis.branch_id()
    }

    pub(crate) fn authority_mutations(&self) -> &[TopologyMutation] {
        &self.authority_mutations
    }

    pub(crate) fn commits(&self) -> &[CommitResult] {
        &self.commits
    }
}
