//! Lineage tracking and operation signatures for provenance.
//!
//! Every topological entity carries provenance from birth (Doctrine D1).
//! This enables:
//! - **Persistent naming** (Phase 9): selectors find entities by ancestry
//! - **Replay**: every operation sequence is fully reproducible
//! - **Debugging**: trace any entity back to the operation that created it


use serde::{Deserialize, Serialize};

use forge_core::EntityRef;

/// Unique signature for a topology operation, used for lineage and replay.
///
/// Two operations with the same name but different parameters produce
/// different lineage hashes (the parameters are hashed in).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpSignature {
    /// Human-readable operation name (e.g., "split_edge", "join_faces").
    name: String,
    /// Unique invocation counter (assigned by the draft).
    invocation_id: u64,
}

impl OpSignature {
    /// Create a new operation signature with a placeholder invocation ID.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            invocation_id: 0,
        }
    }

    /// Create a signature with a specific invocation ID.
    pub fn with_id(name: &str, id: u64) -> Self {
        Self {
            name: name.to_string(),
            invocation_id: id,
        }
    }

    /// The operation name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// The invocation counter.
    pub fn get_invocation_id(&self) -> u64 {
        self.invocation_id
    }

    /// Set the invocation counter (used by the operator runner).
    pub fn set_invocation_id(&mut self, id: u64) {
        self.invocation_id = id;
    }
}

impl std::fmt::Display for OpSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", self.name, self.invocation_id)
    }
}

/// Provenance record attached to every topological entity.
///
/// Tracks which feature and operation created this entity, and a
/// deterministic hash of the parent lineage chain. This is the
/// foundation for persistent naming in Phase 9 selectors.
///
/// # Example
/// When `split_edge` creates two new edges from a parent edge:
/// - Both children carry the parent's `ancestry_hash` combined with
///   the split operation's ID
/// - A selector can later query "edges descended from Edge-7 via split_edge"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lineage {
    /// Which feature created this entity (will be FeatureId in Phase 9).
    origin_feature: u64,
    /// Which Euler operation created this entity.
    creation_op: OpSignature,
    /// Deterministic hash of the parent lineage chain.
    ancestry_hash: u128,
}

impl Lineage {
    /// Create a root lineage (for entities created from scratch, not derived).
    pub fn root(feature_id: u64, op: OpSignature) -> Self {
        let ancestry_hash = Self::compute_hash(0, &op);
        Self {
            origin_feature: feature_id,
            creation_op: op,
            ancestry_hash,
        }
    }

    /// Derive a child lineage from a parent + operation.
    pub fn derive(parent: &Lineage, op: OpSignature) -> Self {
        let ancestry_hash = Self::compute_hash(parent.ancestry_hash, &op);
        Self {
            origin_feature: parent.origin_feature,
            creation_op: op,
            ancestry_hash,
        }
    }

    /// Derive a child lineage from an optional parent.
    ///
    /// If parent is `None`, creates a root lineage at feature 0.
    pub fn derive_from(parent: &Option<Lineage>, op: OpSignature) -> Lineage {
        match parent {
            Some(p) => Lineage::derive(p, op),
            None => Lineage::root(0, op),
        }
    }

    /// The originating feature ID.
    pub fn get_origin_feature(&self) -> u64 {
        self.origin_feature
    }

    /// The operation that created this entity.
    pub fn get_creation_op(&self) -> &OpSignature {
        &self.creation_op
    }

    /// The deterministic ancestry hash.
    pub fn get_ancestry_hash(&self) -> u128 {
        self.ancestry_hash
    }

    /// Compute a deterministic ancestry hash using FNV-style mixing.
    fn compute_hash(parent_hash: u128, op: &OpSignature) -> u128 {
        let op_hash = {
            let mut h: u128 = 0xcbf29ce484222325;
            for byte in op.get_name().bytes() {
                h = h.wrapping_mul(0x100000001b3);
                h ^= byte as u128;
            }
            h ^= op.get_invocation_id() as u128;
            h
        };
        parent_hash.wrapping_mul(0x100000001b3) ^ op_hash
    }

    /// Merge two lineages using Merkle DAG hash mixing.
    ///
    /// Combines the ancestry hashes of both parents via FNV mixing,
    /// preserving traceability from both lineage chains. This is
    /// superior to dominant-parent selection because downstream
    /// selectors can detect entities descended from the union of
    /// two features (e.g., in boolean vertex merging).
    pub fn merge(a: &Option<Lineage>, b: &Option<Lineage>, sig: &OpSignature) -> Lineage {
        match (a, b) {
            (Some(la), Some(lb)) => {
                let combined = Self::fnv_mix_128(la.ancestry_hash, lb.ancestry_hash);
                let ancestry_hash = Self::compute_hash(combined, sig);
                Lineage {
                    origin_feature: la.origin_feature.min(lb.origin_feature),
                    creation_op: sig.clone(),
                    ancestry_hash,
                }
            }
            (Some(la), None) => Lineage::derive(la, sig.clone()),
            (None, Some(lb)) => Lineage::derive(lb, sig.clone()),
            (None, None) => Lineage::root(0, sig.clone()),
        }
    }

    /// FNV-1a style mixing of two 128-bit hashes.
    fn fnv_mix_128(a: u128, b: u128) -> u128 {
        let mixed = a.wrapping_mul(0x100000001b3) ^ b;
        mixed.wrapping_mul(0x100000001b3) ^ (a.rotate_left(17))
    }
}

/// Events logged during topology mutations for the replay system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineageEvent {
    /// A new entity was created
    EntityCreated {
        /// The specific entity (kind + index)
        entity: EntityRef,
        /// The lineage assigned to it
        lineage: Lineage,
    },
    /// An entity was deleted
    EntityDeleted {
        /// The specific entity (kind + index)
        entity: EntityRef,
        /// The lineage of the deleted entity (preserved for replay)
        lineage: Lineage,
    },
    /// An entity was modified (e.g., connectivity changed)
    EntityModified {
        /// The specific entity (kind + index)
        entity: EntityRef,
        old_lineage: Lineage,
        new_lineage: Lineage,
    },
}

impl LineageEvent {
    /// The entity this event refers to.
    pub fn get_entity(&self) -> &EntityRef {
        match self {
            LineageEvent::EntityCreated { entity, .. }
            | LineageEvent::EntityDeleted { entity, .. }
            | LineageEvent::EntityModified { entity, .. } => entity,
        }
    }

    /// The entity kind ("Face", "HalfEdge", etc.).
    pub fn get_entity_kind_str(&self) -> &str {
        self.get_entity().get_kind()
    }
}

/// The kinds of topological entities we track lineage for.
///
/// This is the type-safe enum used internally by `forge-topo`.
/// For crate-neutral references that cross layer boundaries,
/// use `EntityRef` from `forge-core` and the conversion bridge below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityKind {
    Face,
    HalfEdge,
    Vertex,
    Solid,
}

impl EntityKind {
    /// The canonical string name for this kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityKind::Face => "Face",
            EntityKind::HalfEdge => "HalfEdge",
            EntityKind::Vertex => "Vertex",
            EntityKind::Solid => "Solid",
        }
    }

    /// Parse a string into an EntityKind.
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "Face" => Some(EntityKind::Face),
            "HalfEdge" => Some(EntityKind::HalfEdge),
            "Vertex" => Some(EntityKind::Vertex),
            "Solid" => Some(EntityKind::Solid),
            _ => None,
        }
    }

    /// Create a crate-neutral `EntityRef` from this kind and an arena index.
    pub fn to_entity_ref(self, index: u32) -> EntityRef {
        EntityRef::new(self.as_str(), index)
    }
}

impl std::fmt::Display for EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lineage_derivation_is_deterministic() {
        let op = OpSignature::with_id("make_vertex_face", 1);
        let root = Lineage::root(0, op.clone());

        let op2 = OpSignature::with_id("split_edge", 2);
        let child_a = Lineage::derive(&root, op2.clone());
        let child_b = Lineage::derive(&root, op2);

        assert_eq!(child_a.get_ancestry_hash(), child_b.get_ancestry_hash());
    }

    #[test]
    fn different_ops_produce_different_hashes() {
        let root = Lineage::root(0, OpSignature::with_id("create", 1));

        let child_a = Lineage::derive(&root, OpSignature::with_id("split_edge", 2));
        let child_b = Lineage::derive(&root, OpSignature::with_id("join_faces", 2));

        assert_ne!(child_a.get_ancestry_hash(), child_b.get_ancestry_hash());
    }

    #[test]
    fn op_signature_display() {
        let op = OpSignature::with_id("split_edge", 42);
        assert_eq!(format!("{}", op), "split_edge#42");
    }
}
