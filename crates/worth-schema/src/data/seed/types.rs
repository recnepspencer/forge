use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::snapshots::SnapshotHandle;
use forge_relational::facade::transactions::CommitResult;
use serde::{Deserialize, Serialize};

use crate::data::authority::gateway::VerifiedTopologyCommit;
use crate::data::authority::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruth,
    TopologyCommittedMutationSet, TopologyReadArtifact,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimalTopologySeed {
    pub snapshot: SnapshotHandle,
    pub model: EntityId,
    pub body: EntityId,
    pub lump: EntityId,
    pub region: EntityId,
    pub shell: EntityId,
    pub face: EntityId,
    pub outer_loop: EntityId,
    pub wire: EntityId,
    pub half_edge: EntityId,
    pub edge: EntityId,
    pub vertex: EntityId,
    pub persistent_name_ids: Vec<EntityId>,
    persisted_truth: PersistedTopologyTruth,
    read_basis: DerivedTopologyReadBasis,
    read_artifact: TopologyReadArtifact,
    certified_interpretation: CertifiedTopologyInterpretation,
}

#[derive(Debug, Clone)]
pub struct SeededTopologyCommit {
    committed_mutation_set: TopologyCommittedMutationSet,
    branch_id: BranchId,
    commits: Vec<CommitResult>,
    snapshot: SnapshotHandle,
    persisted_truth: PersistedTopologyTruth,
    read_basis: DerivedTopologyReadBasis,
}

impl MinimalTopologySeed {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_parts(
        snapshot: SnapshotHandle,
        model: EntityId,
        body: EntityId,
        lump: EntityId,
        region: EntityId,
        shell: EntityId,
        face: EntityId,
        outer_loop: EntityId,
        wire: EntityId,
        half_edge: EntityId,
        edge: EntityId,
        vertex: EntityId,
        persistent_name_ids: Vec<EntityId>,
        persisted_truth: PersistedTopologyTruth,
        read_basis: DerivedTopologyReadBasis,
        read_artifact: TopologyReadArtifact,
        certified_interpretation: CertifiedTopologyInterpretation,
    ) -> Self {
        Self {
            snapshot,
            model,
            body,
            lump,
            region,
            shell,
            face,
            outer_loop,
            wire,
            half_edge,
            edge,
            vertex,
            persistent_name_ids,
            persisted_truth,
            read_basis,
            read_artifact,
            certified_interpretation,
        }
    }

    pub fn persisted_truth(&self) -> &PersistedTopologyTruth {
        &self.persisted_truth
    }

    pub fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }

    pub fn read_artifact(&self) -> &TopologyReadArtifact {
        &self.read_artifact
    }

    pub fn certified_interpretation(&self) -> &CertifiedTopologyInterpretation {
        &self.certified_interpretation
    }
}

impl SeededTopologyCommit {
    pub(crate) fn from_verified_commit(verified: VerifiedTopologyCommit) -> Self {
        Self {
            committed_mutation_set: verified.committed_mutation_set,
            branch_id: verified.branch_id,
            snapshot: verified.persisted_truth.snapshot.clone(),
            commits: verified.commits,
            persisted_truth: verified.persisted_truth,
            read_basis: verified.read_basis,
        }
    }

    pub fn committed_mutation_set(&self) -> &TopologyCommittedMutationSet {
        &self.committed_mutation_set
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn commits(&self) -> &[CommitResult] {
        &self.commits
    }

    pub fn snapshot(&self) -> &SnapshotHandle {
        &self.snapshot
    }

    pub fn persisted_truth(&self) -> &PersistedTopologyTruth {
        &self.persisted_truth
    }

    pub fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }

    pub fn into_parts(
        self,
    ) -> (
        TopologyCommittedMutationSet,
        BranchId,
        Vec<CommitResult>,
        SnapshotHandle,
        PersistedTopologyTruth,
        DerivedTopologyReadBasis,
    ) {
        (
            self.committed_mutation_set,
            self.branch_id,
            self.commits,
            self.snapshot,
            self.persisted_truth,
            self.read_basis,
        )
    }
}
