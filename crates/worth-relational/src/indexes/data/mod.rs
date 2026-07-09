use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use worth_foundational::facade::AspectFieldLocator;

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::{EntityId, RelationId, VersionId};
use crate::schema::data::SchemaVersionId;
use crate::storage::data::AuthoritativeFieldComparisonKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DerivedIndexId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DerivedIndexGenerationId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedIndexKind {
    EntityField {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
    },
    RelationField {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexDefinition {
    pub index_id: DerivedIndexId,
    pub name: String,
    pub kind: DerivedIndexKind,
    pub branch_scoped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedIndexEntries {
    EntityField(BTreeMap<AuthoritativeFieldComparisonKey, Vec<EntityId>>),
    RelationField(BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedIndexPublicationStatus {
    Published,
    BuildFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexApplicability {
    pub branch_id: BranchId,
    pub version_id: VersionId,
    pub schema_version: SchemaVersionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexGeneration {
    pub generation_id: DerivedIndexGenerationId,
    pub index_id: DerivedIndexId,
    pub source_commit_id: CommitId,
    pub source_branch_id: BranchId,
    pub applicability: DerivedIndexApplicability,
    pub status: DerivedIndexPublicationStatus,
    pub entries: DerivedIndexEntries,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DerivedIndexArtifacts {
    generations: Vec<DerivedIndexGeneration>,
}

impl DerivedIndexArtifacts {
    pub fn new(generations: Vec<DerivedIndexGeneration>) -> Self {
        let mut artifacts = Self::default();
        artifacts.extend_canonical(&generations);
        artifacts
    }

    pub fn is_empty(&self) -> bool {
        self.generations.is_empty()
    }

    pub fn generations(&self) -> &[DerivedIndexGeneration] {
        &self.generations
    }

    pub fn generation_ids(&self) -> Vec<u64> {
        self.generations
            .iter()
            .map(|generation| generation.generation_id.0)
            .collect()
    }

    pub fn extend_canonical(&mut self, generations: &[DerivedIndexGeneration]) {
        for generation in generations {
            if let Some(existing) = self
                .generations
                .iter_mut()
                .find(|candidate| candidate.generation_id == generation.generation_id)
            {
                *existing = generation.clone();
            } else {
                self.generations.push(generation.clone());
            }
        }
        self.generations
            .sort_by_key(|generation| (generation.index_id.0, generation.generation_id.0));
    }

    #[cfg(test)]
    pub(crate) fn generations_mut_for_test(&mut self) -> &mut Vec<DerivedIndexGeneration> {
        &mut self.generations
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexBuildRequest {
    pub source_commit_id: CommitId,
    pub branch_id: BranchId,
    pub index_ids: Vec<DerivedIndexId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexBuildOutcome {
    pub source_commit_id: CommitId,
    pub generations: Vec<DerivedIndexGeneration>,
    pub failed_indexes: Vec<DerivedIndexId>,
}
