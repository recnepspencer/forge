use std::collections::BTreeSet;

use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::snapshots::SnapshotHandle;
use serde::{Deserialize, Serialize};

use crate::data::aspects::Aspect;
use crate::data::authority::{
    PrecisionBudgetFallbackRecord, PrecisionFallbackRecord, TopologyInterpretationRecordSet,
};
use crate::data::entities::EntityKind;
use crate::data::relations::RelationKind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreateKey(pub String);

impl CreateKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityReference {
    Existing(EntityId),
    Created(CreateKey),
}

impl From<EntityId> for EntityReference {
    fn from(value: EntityId) -> Self {
        Self::Existing(value)
    }
}

impl From<CreateKey> for EntityReference {
    fn from(value: CreateKey) -> Self {
        Self::Created(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MutationOrigin {
    Seed,
    LocalEdit,
    Replay,
    BranchLocalApplication,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyMutation {
    CreateEntity {
        create_key: CreateKey,
        kind: EntityKind,
    },
    CreateRelation {
        create_key: CreateKey,
        kind: RelationKind,
        source: EntityReference,
        target: EntityReference,
    },
    UpsertEntity {
        entity_id: EntityId,
        kind: EntityKind,
    },
    UpsertRelation {
        relation_id: RelationId,
        kind: RelationKind,
        source: EntityId,
        target: EntityId,
    },
    RemoveEntity {
        entity_id: EntityId,
    },
    RemoveRelation {
        relation_id: RelationId,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawTopologyIntent {
    pub mutations: Vec<TopologyMutation>,
    pub mutation_origin: MutationOrigin,
    pub precision_fallbacks: Vec<PrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<PrecisionBudgetFallbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopologyCommittedMutationSet {
    pub mutations: Vec<TopologyMutation>,
    pub touched_aspects: BTreeSet<Aspect>,
    pub mutation_origin: MutationOrigin,
    pub precision_fallbacks: Vec<PrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<PrecisionBudgetFallbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedTopologyTruth {
    pub committed_mutation_set: TopologyCommittedMutationSet,
    pub snapshot: SnapshotHandle,
    pub branch_id: BranchId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedTruthBasisIdentity {
    pub mutation_digest_hex: String,
    pub touched_aspect_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthoritativeTopologySnapshot {
    pub snapshot: SnapshotHandle,
    pub branch_id: BranchId,
    pub touched_aspects: BTreeSet<Aspect>,
    pub authoritative_mutation_origin: MutationOrigin,
    pub truth_basis_identity: DerivedTruthBasisIdentity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedTopologyReadBasis {
    pub authority: AuthoritativeTopologySnapshot,
    pub derivation_origin: MutationOrigin,
    pub precision_fallbacks: Vec<PrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<PrecisionBudgetFallbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopologyReadArtifact {
    pub snapshot: SnapshotHandle,
    pub precision_fallbacks: Vec<PrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<PrecisionBudgetFallbackRecord>,
    pub interpretations: TopologyInterpretationRecordSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertifiedTopologyInterpretation {
    pub read_basis: DerivedTopologyReadBasis,
    pub precision_fallbacks: Vec<PrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<PrecisionBudgetFallbackRecord>,
    pub interpretations: TopologyInterpretationRecordSet,
}

impl RawTopologyIntent {
    pub fn new(mutations: Vec<TopologyMutation>, mutation_origin: MutationOrigin) -> Self {
        Self {
            mutations,
            mutation_origin,
            precision_fallbacks: Vec::new(),
            precision_budget_fallbacks: Vec::new(),
        }
    }

    pub fn with_precision_fallback(mut self, fallback: impl Into<PrecisionFallbackRecord>) -> Self {
        self.precision_fallbacks.push(fallback.into());
        self
    }

    pub fn with_precision_budget_fallback(
        mut self,
        fallback: impl Into<PrecisionBudgetFallbackRecord>,
    ) -> Self {
        self.precision_budget_fallbacks.push(fallback.into());
        self
    }
}

impl TopologyCommittedMutationSet {
    pub fn from_raw_intent(intent: RawTopologyIntent, touched_aspects: BTreeSet<Aspect>) -> Self {
        Self {
            mutations: intent.mutations,
            touched_aspects,
            mutation_origin: intent.mutation_origin,
            precision_fallbacks: intent.precision_fallbacks,
            precision_budget_fallbacks: intent.precision_budget_fallbacks,
        }
    }

    pub fn raw_intent(&self) -> RawTopologyIntent {
        RawTopologyIntent {
            mutations: self.mutations.clone(),
            mutation_origin: self.mutation_origin,
            precision_fallbacks: self.precision_fallbacks.clone(),
            precision_budget_fallbacks: self.precision_budget_fallbacks.clone(),
        }
    }
}

impl DerivedTopologyReadBasis {
    pub fn from_persisted_truth(persisted_truth: &PersistedTopologyTruth) -> Self {
        Self {
            authority: AuthoritativeTopologySnapshot {
                snapshot: persisted_truth.snapshot.clone(),
                branch_id: persisted_truth.branch_id.clone(),
                touched_aspects: persisted_truth
                    .committed_mutation_set
                    .touched_aspects
                    .clone(),
                authoritative_mutation_origin: persisted_truth
                    .committed_mutation_set
                    .mutation_origin,
                truth_basis_identity: DerivedTruthBasisIdentity {
                    mutation_digest_hex: mutation_digest_hex(
                        &persisted_truth.committed_mutation_set,
                    ),
                    touched_aspect_count: persisted_truth
                        .committed_mutation_set
                        .touched_aspects
                        .len(),
                },
            },
            derivation_origin: persisted_truth.committed_mutation_set.mutation_origin,
            precision_fallbacks: persisted_truth
                .committed_mutation_set
                .precision_fallbacks
                .clone(),
            precision_budget_fallbacks: persisted_truth
                .committed_mutation_set
                .precision_budget_fallbacks
                .clone(),
        }
    }

    pub fn replay_of(&self) -> Self {
        let mut replay = self.clone();
        replay.derivation_origin = MutationOrigin::Replay;
        replay
    }

    pub fn snapshot(&self) -> &SnapshotHandle {
        &self.authority.snapshot
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.authority.branch_id
    }

    pub fn touched_aspects(&self) -> &BTreeSet<Aspect> {
        &self.authority.touched_aspects
    }

    pub fn authoritative_mutation_origin(&self) -> MutationOrigin {
        self.authority.authoritative_mutation_origin
    }

    pub fn derivation_origin(&self) -> MutationOrigin {
        self.derivation_origin
    }
}

impl TopologyReadArtifact {
    pub fn from_read_basis(read_basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            snapshot: read_basis.snapshot().clone(),
            precision_fallbacks: read_basis.precision_fallbacks.clone(),
            precision_budget_fallbacks: read_basis.precision_budget_fallbacks.clone(),
            interpretations: TopologyInterpretationRecordSet::default(),
        }
    }

    pub fn from_read_basis_and_interpretation(
        read_basis: &DerivedTopologyReadBasis,
        interpretations: TopologyInterpretationRecordSet,
    ) -> Self {
        Self {
            snapshot: read_basis.snapshot().clone(),
            precision_fallbacks: read_basis.precision_fallbacks.clone(),
            precision_budget_fallbacks: read_basis.precision_budget_fallbacks.clone(),
            interpretations,
        }
    }
}

impl CertifiedTopologyInterpretation {
    pub fn from_read_basis(read_basis: DerivedTopologyReadBasis) -> Self {
        Self {
            precision_fallbacks: read_basis.precision_fallbacks.clone(),
            precision_budget_fallbacks: read_basis.precision_budget_fallbacks.clone(),
            interpretations: TopologyInterpretationRecordSet::default(),
            read_basis,
        }
    }

    pub fn from_read_basis_and_interpretation(
        read_basis: DerivedTopologyReadBasis,
        interpretations: TopologyInterpretationRecordSet,
    ) -> Self {
        Self {
            precision_fallbacks: read_basis.precision_fallbacks.clone(),
            precision_budget_fallbacks: read_basis.precision_budget_fallbacks.clone(),
            interpretations,
            read_basis,
        }
    }
}

fn mutation_digest_hex(committed_mutation_set: &TopologyCommittedMutationSet) -> String {
    let mut state: u64 = 0xcbf29ce484222325;
    fn write_str(state: &mut u64, value: &str) {
        for byte in value.as_bytes() {
            *state ^= u64::from(*byte);
            *state = state.wrapping_mul(0x100000001b3);
        }
    }

    write_str(
        &mut state,
        &format!("{:?}", committed_mutation_set.mutation_origin),
    );
    for aspect in &committed_mutation_set.touched_aspects {
        write_str(&mut state, &format!("{aspect:?}"));
    }
    for mutation in &committed_mutation_set.mutations {
        write_str(&mut state, &format!("{mutation:?}"));
    }
    for fallback in &committed_mutation_set.precision_fallbacks {
        write_str(&mut state, &format!("{fallback:?}"));
    }
    for fallback in &committed_mutation_set.precision_budget_fallbacks {
        write_str(&mut state, &format!("{fallback:?}"));
    }

    format!("{state:016x}")
}
