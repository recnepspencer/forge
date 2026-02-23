//! Live lineage store mapping active entities to their lineage records.
//!
//! DOMAIN: Entity provenance tracking (Phase 3 causality).
//! Maps each active `EntityRef` to its current `Lineage`, enabling
//! per-entity ancestry queries ("Show me the history of HalfEdge#117").
//!
//! INVARIANTS:
//! - Every active entity in the arena should have a lineage entry.
//! - Deleted entities are removed from the store but preserved in the event log.
//!
//! DEPENDENCIES: `lineage` (Lineage, OpSignature, LineageEvent),
//! `forge-core` (EntityRef)

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use forge_core::EntityRef;

use super::lineage::{Lineage, LineageEvent, OpSignature};
use forge_core::KernelError;

/// Live mapping from entity handles to their current lineage.
///
/// Maintained during `MutableDraft` mutations. When an Euler operator
/// creates an entity, the caller inserts a root lineage here. When
/// an entity is deleted, its lineage is removed (but preserved in the
/// `LineageEvent` log for replay).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineageStore {
    /// Active entity → lineage mapping, ordered for deterministic iteration.
    entries: BTreeMap<EntityRef, Lineage>,
    /// Chronological event log (mirrors MutableDraft::lineage_log).
    events: Vec<LineageEvent>,
}

impl LineageStore {
    /// Create an empty lineage store.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    /// Record the creation of a new entity with the given lineage.
    ///
    /// Inserts into the live map and appends an `EntityCreated` event.
    pub fn record_creation(&mut self, entity: EntityRef, lineage: Lineage) {
        self.events.push(LineageEvent::EntityCreated {
            entity: entity.clone(),
            lineage: lineage.clone(),
        });
        self.entries.insert(entity, lineage);
    }

    /// Record the deletion of an entity.
    ///
    /// Removes from the live map and appends an `EntityDeleted` event
    /// preserving the lineage for replay.
    pub fn record_deletion(&mut self, entity: EntityRef) -> Result<(), KernelError> {
        let lineage = self.entries.remove(&entity)
            .ok_or_else(|| KernelError::InternalError {
                message: format!(
                    "LineageStore: attempted to delete untracked entity {:?} — arena/history desync",
                    entity
                ),
                context: None,
            })?;
        self.events.push(LineageEvent::EntityDeleted {
            entity,
            lineage,
        });
        Ok(())
    }

    /// Record a modification of an entity's lineage (e.g., edge rewiring).
    ///
    /// Updates the live map and appends an `EntityModified` event.
    pub fn record_mutation(
        &mut self,
        entity: EntityRef,
        new_lineage: Lineage,
    ) -> Result<(), KernelError> {
        let old_lineage = self.entries.get(&entity)
            .cloned()
            .ok_or_else(|| KernelError::InternalError {
                message: format!(
                    "LineageStore: attempted to mutate untracked entity {:?} — arena/history desync",
                    entity
                ),
                context: None,
            })?;
        self.events.push(LineageEvent::EntityModified {
            entity: entity.clone(),
            old_lineage,
            new_lineage: new_lineage.clone(),
        });
        self.entries.insert(entity, new_lineage);
        Ok(())
    }

    /// Look up the current lineage of an entity.
    pub fn get_lineage(&self, entity: &EntityRef) -> Option<&Lineage> {
        self.entries.get(entity)
    }

    /// Query the full event history for a specific entity.
    ///
    /// Returns all `LineageEvent`s that reference this entity,
    /// in chronological order.
    pub fn query_entity_history(&self, entity: &EntityRef) -> Vec<&LineageEvent> {
        self.events.iter()
            .filter(|e| e.get_entity() == entity)
            .collect()
    }

    /// The full chronological event log.
    pub fn events(&self) -> &[LineageEvent] {
        &self.events
    }

    /// Number of active entities tracked.
    pub fn active_count(&self) -> usize {
        self.entries.len()
    }

    /// All active entity references (deterministic order).
    pub fn active_entities(&self) -> impl Iterator<Item = &EntityRef> {
        self.entries.keys()
    }

    /// Drain all events from the store (used when committing a draft).
    pub fn drain_events(&mut self) -> Vec<LineageEvent> {
        std::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::EntityKind;

    #[test]
    fn creation_and_query() {
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Face, 42);
        let lineage = Lineage::root(42, OpSignature::new("make_face"));

        store.record_creation(entity.clone(), lineage.clone());

        assert_eq!(store.active_count(), 1);
        assert!(store.get_lineage(&entity).is_some());

        let history = store.query_entity_history(&entity);
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn deletion_removes_from_live_map() {
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::HalfEdge, 117);
        let lineage = Lineage::root(117, OpSignature::new("split_edge"));

        store.record_creation(entity.clone(), lineage);
        assert_eq!(store.active_count(), 1);

        store.record_deletion(entity.clone()).unwrap();
        assert_eq!(store.active_count(), 0);
        assert!(store.get_lineage(&entity).is_none());

        let history = store.query_entity_history(&entity);
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn mutation_updates_lineage() {
        let mut store = LineageStore::new();
        let entity = EntityRef::new(EntityKind::Face, 10);
        let original = Lineage::root(10, OpSignature::new("create"));
        let updated = Lineage::root(10, OpSignature::new("split"));

        store.record_creation(entity.clone(), original);
        store.record_mutation(entity.clone(), updated.clone()).unwrap();

        let current = store.get_lineage(&entity).unwrap();
        assert_eq!(current.get_creation_op().get_name(), "split");

        let history = store.query_entity_history(&entity);
        assert_eq!(history.len(), 2);
    }
}
