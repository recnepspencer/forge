use forge_relational::facade::identity::EntityId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsSourceRow {
    traversal_kind: &'static str,
    anchor_entity_id: EntityId,
    reached_entity_count: usize,
    row_digest: String,
}

impl TraversalViewsSourceRow {
    #[cfg(test)]
    pub(super) fn new(
        traversal_kind: &'static str,
        anchor_entity_id: EntityId,
        reached_entity_count: usize,
    ) -> Self {
        let row_digest = super::super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-source-row:v1".to_string(),
            format!("kind:{traversal_kind}"),
            format!("anchor:{anchor_entity_id:?}"),
            format!("reached:{reached_entity_count}"),
        ]);
        Self {
            traversal_kind,
            anchor_entity_id,
            reached_entity_count,
            row_digest,
        }
    }

    pub const fn traversal_kind(&self) -> &'static str {
        self.traversal_kind
    }

    pub const fn anchor_entity_id(&self) -> EntityId {
        self.anchor_entity_id
    }

    pub const fn reached_entity_count(&self) -> usize {
        self.reached_entity_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
