use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::identity::data::RelationId;

#[derive(Debug, Clone)]
pub(crate) enum AdjacencySet {
    Inline(Vec<RelationId>),
    Compressed(Vec<RelationId>),
}

impl AdjacencySet {
    pub(crate) fn new(policy: &AdjacencyPolicy) -> Self {
        match policy.backend {
            AdjacencyBackend::InlineSmallDegreeAdjacency => {
                Self::Inline(Vec::with_capacity(policy.small_degree_inline_capacity))
            }
            AdjacencyBackend::CompressedFanoutAdjacency => Self::Compressed(Vec::new()),
        }
    }

    pub(crate) fn clear(&mut self) {
        match self {
            Self::Inline(relations) | Self::Compressed(relations) => relations.clear(),
        }
    }

    pub(crate) fn insert(&mut self, relation_id: RelationId) {
        match self {
            Self::Inline(relations) | Self::Compressed(relations) => {
                if let Err(index) = relations.binary_search(&relation_id) {
                    relations.insert(index, relation_id);
                }
            }
        }
    }

    pub(crate) fn remove(&mut self, relation_id: &RelationId) {
        match self {
            Self::Inline(relations) | Self::Compressed(relations) => {
                if let Ok(index) = relations.binary_search(relation_id) {
                    relations.remove(index);
                }
            }
        }
    }

    pub(crate) fn ids(&self) -> Vec<RelationId> {
        match self {
            Self::Inline(relations) | Self::Compressed(relations) => relations.clone(),
        }
    }

    pub(crate) fn extend_into(&self, target: &mut std::collections::BTreeSet<RelationId>) {
        match self {
            Self::Inline(relations) | Self::Compressed(relations) => {
                target.extend(relations.iter().copied())
            }
        }
    }
}
