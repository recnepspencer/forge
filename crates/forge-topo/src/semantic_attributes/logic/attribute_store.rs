//! Semantic attribute storage for topology entities.
//!
//! DOMAIN: Side-car metadata map for manufacturing data.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::semantic_attributes::data::semantic_tag::{EntityKey, SemanticTag, TagValue};

/// Side-car attribute storage for the topology arena.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeStore {
    tags: HashMap<EntityKey, SemanticTag>,
}

impl AttributeStore {
    /// Create an empty attribute store.
    pub fn new() -> Self {
        Self { tags: HashMap::new() }
    }

    /// Get all tags for an entity, if any exist.
    pub fn get_tags(&self, key: EntityKey) -> Option<&SemanticTag> {
        self.tags.get(&key)
    }

    /// Set a single tag on an entity (creates the tag set if needed).
    pub fn set_tag(&mut self, key: EntityKey, tag_name: String, value: TagValue) {
        self.tags.entry(key).or_default().insert(tag_name, value);
    }

    /// Remove a single tag from an entity. Returns the removed value.
    pub fn remove_tag(&mut self, key: EntityKey, tag_name: &str) -> Option<TagValue> {
        let entry = self.tags.get_mut(&key)?;
        let removed = entry.remove(tag_name);
        if entry.is_empty() {
            self.tags.remove(&key);
        }
        removed
    }

    /// Remove all tags for an entity.
    pub fn remove_all_tags(&mut self, key: EntityKey) -> Option<SemanticTag> {
        self.tags.remove(&key)
    }

    /// Find all entities that have a specific tag name.
    pub fn get_entities_by_tag(&self, tag_name: &str) -> Vec<EntityKey> {
        self.tags
            .iter()
            .filter(|(_, tags)| tags.contains_key(tag_name))
            .map(|(key, _)| *key)
            .collect()
    }

    /// Returns the number of entities with attributes.
    pub fn entity_count(&self) -> usize {
        self.tags.len()
    }

    /// Returns true if no entities have attributes.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handles::{FaceId, VertexId};

    #[test]
    fn set_and_get_tag() {
        let mut store = AttributeStore::new();
        let face = EntityKey::Face(FaceId::new(0, 0));
        store.set_tag(face, "material".to_string(), TagValue::Text("AL_6061".to_string()));
        let tags = store.get_tags(face);
        assert!(tags.is_some());
        assert_eq!(tags.unwrap().get("material"), Some(&TagValue::Text("AL_6061".to_string())));
    }

    #[test]
    fn remove_tag_cleans_up_empty_entry() {
        let mut store = AttributeStore::new();
        let face = EntityKey::Face(FaceId::new(0, 0));
        store.set_tag(face, "material".to_string(), TagValue::Text("steel".to_string()));
        store.remove_tag(face, "material");
        assert!(store.get_tags(face).is_none());
        assert!(store.is_empty());
    }

    #[test]
    fn get_entities_by_tag_filters_correctly() {
        let mut store = AttributeStore::new();
        let f0 = EntityKey::Face(FaceId::new(0, 0));
        let f1 = EntityKey::Face(FaceId::new(1, 0));
        let v0 = EntityKey::Vertex(VertexId::new(0, 0));
        store.set_tag(f0, "material".to_string(), TagValue::Text("AL_6061".to_string()));
        store.set_tag(f1, "material".to_string(), TagValue::Text("steel".to_string()));
        store.set_tag(v0, "datum".to_string(), TagValue::Flag(true));
        assert_eq!(store.get_entities_by_tag("material").len(), 2);
        assert_eq!(store.get_entities_by_tag("datum").len(), 1);
    }

    #[test]
    fn multiple_tags_on_same_entity() {
        let mut store = AttributeStore::new();
        let face = EntityKey::Face(FaceId::new(5, 2));
        store.set_tag(face, "material".to_string(), TagValue::Text("AL_6061".to_string()));
        store.set_tag(face, "tolerance_class".to_string(), TagValue::Text("fine".to_string()));
        store.set_tag(face, "roughness_ra".to_string(), TagValue::Number(1.6));
        assert_eq!(store.get_tags(face).unwrap().len(), 3);
    }

    #[test]
    fn remove_all_tags() {
        let mut store = AttributeStore::new();
        let face = EntityKey::Face(FaceId::new(0, 0));
        store.set_tag(face, "a".to_string(), TagValue::Flag(true));
        store.set_tag(face, "b".to_string(), TagValue::Flag(false));
        let removed = store.remove_all_tags(face);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().len(), 2);
        assert!(store.is_empty());
    }
}
