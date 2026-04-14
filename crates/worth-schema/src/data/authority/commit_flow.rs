use std::collections::BTreeSet;

use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::snapshots::SnapshotHandle;
use serde::{Deserialize, Serialize};

use crate::data::aspects::WorthAspect;
use crate::data::authority::{
    WorthPrecisionBudgetFallbackRecord, WorthPrecisionFallbackRecord,
    WorthTopologyInterpretationRecordSet,
};
use crate::data::entities::WorthEntityKind;
use crate::data::relations::WorthRelationKind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorthCreateKey(pub String);

impl WorthCreateKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthEntityReference {
    Existing(EntityId),
    Created(WorthCreateKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthMutationOrigin {
    Seed,
    LocalEdit,
    Replay,
    BranchLocalApplication,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthTopologyMutation {
    CreateEntity {
        create_key: WorthCreateKey,
        kind: WorthEntityKind,
    },
    CreateRelation {
        create_key: WorthCreateKey,
        kind: WorthRelationKind,
        source: WorthEntityReference,
        target: WorthEntityReference,
    },
    UpsertEntity {
        entity_id: EntityId,
        kind: WorthEntityKind,
    },
    UpsertRelation {
        relation_id: RelationId,
        kind: WorthRelationKind,
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
pub struct WorthTopologyMutationBatch {
    pub mutations: Vec<WorthTopologyMutation>,
    pub touched_aspects: BTreeSet<WorthAspect>,
    pub mutation_origin: WorthMutationOrigin,
    pub precision_fallbacks: Vec<WorthPrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<WorthPrecisionBudgetFallbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawWorthTopologyIntent {
    pub mutations: Vec<WorthTopologyMutation>,
    pub mutation_origin: WorthMutationOrigin,
    pub precision_fallbacks: Vec<WorthPrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<WorthPrecisionBudgetFallbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalTopologyMutationBatch {
    pub batch: WorthTopologyMutationBatch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedTopologyTruthBatch {
    pub batch: WorthTopologyMutationBatch,
    pub snapshot: SnapshotHandle,
    pub branch_id: BranchId,
    pub mutation_origin: WorthMutationOrigin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedTopologyReadBasis {
    pub snapshot: SnapshotHandle,
    pub branch_id: BranchId,
    pub touched_aspects: BTreeSet<WorthAspect>,
    pub mutation_origin: WorthMutationOrigin,
    pub precision_fallbacks: Vec<WorthPrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<WorthPrecisionBudgetFallbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthTopologyReadArtifact {
    pub snapshot: SnapshotHandle,
    pub precision_fallbacks: Vec<WorthPrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<WorthPrecisionBudgetFallbackRecord>,
    pub interpretations: WorthTopologyInterpretationRecordSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertifiedTopologyInterpretation {
    pub read_basis: DerivedTopologyReadBasis,
    pub precision_fallbacks: Vec<WorthPrecisionFallbackRecord>,
    pub precision_budget_fallbacks: Vec<WorthPrecisionBudgetFallbackRecord>,
    pub interpretations: WorthTopologyInterpretationRecordSet,
}

impl RawWorthTopologyIntent {
    pub fn new(mutations: Vec<WorthTopologyMutation>, mutation_origin: WorthMutationOrigin) -> Self {
        Self {
            mutations,
            mutation_origin,
            precision_fallbacks: Vec::new(),
            precision_budget_fallbacks: Vec::new(),
        }
    }

    pub fn with_precision_fallback(
        mut self,
        fallback: impl Into<WorthPrecisionFallbackRecord>,
    ) -> Self {
        self.precision_fallbacks.push(fallback.into());
        self
    }

    pub fn with_precision_budget_fallback(
        mut self,
        fallback: impl Into<WorthPrecisionBudgetFallbackRecord>,
    ) -> Self {
        self.precision_budget_fallbacks.push(fallback.into());
        self
    }
}

impl WorthTopologyMutationBatch {
    pub fn from_raw_intent(
        intent: RawWorthTopologyIntent,
        touched_aspects: BTreeSet<WorthAspect>,
    ) -> Self {
        Self {
            mutations: intent.mutations,
            touched_aspects,
            mutation_origin: intent.mutation_origin,
            precision_fallbacks: intent.precision_fallbacks,
            precision_budget_fallbacks: intent.precision_budget_fallbacks,
        }
    }
}

impl DerivedTopologyReadBasis {
    pub fn from_persisted_truth(batch: &PersistedTopologyTruthBatch) -> Self {
        Self {
            snapshot: batch.snapshot.clone(),
            branch_id: batch.branch_id.clone(),
            touched_aspects: batch.batch.touched_aspects.clone(),
            mutation_origin: batch.mutation_origin,
            precision_fallbacks: batch.batch.precision_fallbacks.clone(),
            precision_budget_fallbacks: batch.batch.precision_budget_fallbacks.clone(),
        }
    }

    pub fn replay_of(&self) -> Self {
        let mut replay = self.clone();
        replay.mutation_origin = WorthMutationOrigin::Replay;
        replay
    }
}

impl WorthTopologyReadArtifact {
    pub fn from_read_basis(read_basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            snapshot: read_basis.snapshot.clone(),
            precision_fallbacks: read_basis.precision_fallbacks.clone(),
            precision_budget_fallbacks: read_basis.precision_budget_fallbacks.clone(),
            interpretations: WorthTopologyInterpretationRecordSet::default(),
        }
    }

    pub fn from_read_basis_and_interpretation(
        read_basis: &DerivedTopologyReadBasis,
        interpretations: WorthTopologyInterpretationRecordSet,
    ) -> Self {
        Self {
            snapshot: read_basis.snapshot.clone(),
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
            interpretations: WorthTopologyInterpretationRecordSet::default(),
            read_basis,
        }
    }

    pub fn from_read_basis_and_interpretation(
        read_basis: DerivedTopologyReadBasis,
        interpretations: WorthTopologyInterpretationRecordSet,
    ) -> Self {
        Self {
            precision_fallbacks: read_basis.precision_fallbacks.clone(),
            precision_budget_fallbacks: read_basis.precision_budget_fallbacks.clone(),
            interpretations,
            read_basis,
        }
    }
}
