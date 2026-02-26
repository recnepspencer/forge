//! Lineage tracking and operation signatures for provenance.
//!
//! Every topological entity carries provenance from birth (Doctrine D1).
//! This enables:
//! - **Persistent naming** (Phase 9): selectors find entities by ancestry
//! - **Replay**: every operation sequence is fully reproducible
//! - **Debugging**: trace any entity back to the operation that created it


use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};

use forge_core::EntityRef;
use forge_core::EntityKind;

use crate::handles::{BodyId, EdgeId, FaceId, HalfEdgeId, LumpId, RegionId, ShellId, VertexId};

/// Shape of parent linkage recorded in lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ParentLinkageMode {
    #[default]
    None,
    Single,
    Compound,
}

/// Topology-local generational entity reference for lineage events.
///
/// This preserves ABA-safe snapshot identity for lineage/re-identification
/// infrastructure while `EntityRef` remains available as a crate-neutral,
/// index-only reference for broader tracing/replay consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineageEntityRef {
    kind: EntityKind,
    index: u32,
    generation: u32,
}

impl LineageEntityRef {
    pub fn new(kind: EntityKind, index: u32, generation: u32) -> Self {
        Self { kind, index, generation }
    }

    pub fn kind(self) -> EntityKind {
        self.kind
    }

    pub fn index(self) -> u32 {
        self.index
    }

    pub fn generation(self) -> u32 {
        self.generation
    }

    pub fn as_entity_ref(self) -> EntityRef {
        EntityRef::new(self.kind, self.index)
    }
}

macro_rules! impl_lineage_entity_ref_from_handle {
    ($ty:ty, $kind:expr) => {
        impl From<$ty> for LineageEntityRef {
            fn from(id: $ty) -> Self {
                Self::new($kind, id.index(), id.generation())
            }
        }
    };
}

impl_lineage_entity_ref_from_handle!(FaceId, EntityKind::Face);
impl_lineage_entity_ref_from_handle!(HalfEdgeId, EntityKind::HalfEdge);
impl_lineage_entity_ref_from_handle!(VertexId, EntityKind::Vertex);
impl_lineage_entity_ref_from_handle!(EdgeId, EntityKind::Edge);
impl_lineage_entity_ref_from_handle!(ShellId, EntityKind::Shell);
impl_lineage_entity_ref_from_handle!(BodyId, EntityKind::Body);
impl_lineage_entity_ref_from_handle!(LumpId, EntityKind::Lump);
impl_lineage_entity_ref_from_handle!(RegionId, EntityKind::Region);

/// Unique signature for a topology operation, used for lineage and replay.
///
/// Lineage hashes are based on the operation name and invocation ID.
/// Parameters are NOT currently included in the hash. This is sufficient
/// for the current replay system where invocation IDs are unique per draft.
/// Phase 9 persistent naming may require adding a `param_hash` field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpSignature {
    /// Human-readable operation name (e.g., "split_edge", "join_faces").
    name: String,
    /// Unique invocation counter (assigned by the draft).
    invocation_id: u64,
}

impl OpSignature {
    /// Create a new operation signature with a zeroed invocation ID.
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
    /// Which features created this entity. Single-element for root/derive,
    /// multi-element for merge (compound provenance from both parents).
    origin_features: SmallVec<[u64; 2]>,
    /// Which Euler operation created this entity.
    creation_op: OpSignature,
    /// Deterministic hash of the parent lineage chain.
    ancestry_hash: u128,
    /// Immediate parent ancestry hashes, sorted ascending and deduplicated.
    #[serde(default)]
    parent_ancestry_hashes: SmallVec<[u128; 2]>,
    /// Shape of the stored parent linkage.
    #[serde(default)]
    parent_linkage_mode: ParentLinkageMode,
}

impl Lineage {
    /// Create a root lineage (for entities created from scratch, not derived).
    pub fn root(feature_id: u64, op: OpSignature) -> Self {
        let ancestry_hash = Self::compute_hash(0, &op);
        Self {
            origin_features: smallvec![feature_id],
            creation_op: op,
            ancestry_hash,
            parent_ancestry_hashes: SmallVec::new(),
            parent_linkage_mode: ParentLinkageMode::None,
        }
    }

    /// Derive a child lineage from a parent + operation.
    pub fn derive(parent: &Lineage, op: OpSignature) -> Self {
        let ancestry_hash = Self::compute_hash(parent.ancestry_hash, &op);
        Self {
            origin_features: parent.origin_features.clone(),
            creation_op: op,
            ancestry_hash,
            parent_ancestry_hashes: smallvec![parent.ancestry_hash],
            parent_linkage_mode: ParentLinkageMode::Single,
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

    /// The originating feature IDs (compound provenance for merged entities).
    pub fn get_origin_features(&self) -> &[u64] {
        &self.origin_features
    }

    /// The operation that created this entity.
    pub fn get_creation_op(&self) -> &OpSignature {
        &self.creation_op
    }

    /// The deterministic ancestry hash.
    pub fn get_ancestry_hash(&self) -> u128 {
        self.ancestry_hash
    }

    /// Immediate parent ancestry hashes, sorted ascending and deduplicated.
    pub fn get_parent_ancestry_hashes(&self) -> &[u128] {
        &self.parent_ancestry_hashes
    }

    /// Shape of the stored parent ancestry linkage.
    pub fn get_parent_linkage_mode(&self) -> ParentLinkageMode {
        self.parent_linkage_mode
    }

    /// Compute a deterministic ancestry hash using FNV-style mixing.
    fn compute_hash(parent_hash: u128, op: &OpSignature) -> u128 {
        let op_hash = {
            let mut h: u128 = 0x6c62272e07bb014262b821756295c58d;
            for byte in op.get_name().bytes() {
                h = h.wrapping_mul(0x1000000000000000000013b);
                h ^= byte as u128;
            }
            // Delimit name bytes from invocation bytes, then hash the invocation
            // bytes structurally to avoid xor-alias collisions like ("a",1) == ("b",2).
            h = h.wrapping_mul(0x1000000000000000000013b);
            h ^= 0xff;
            for byte in op.get_invocation_id().to_le_bytes() {
                h = h.wrapping_mul(0x1000000000000000000013b);
                h ^= byte as u128;
            }
            h
        };
        parent_hash.wrapping_mul(0x1000000000000000000013b) ^ op_hash
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
                let mut features = la.origin_features.clone();
                for &f in &lb.origin_features {
                    if !features.contains(&f) {
                        features.push(f);
                    }
                }
                features.sort_unstable();
                let mut parent_ancestry_hashes = smallvec![la.ancestry_hash, lb.ancestry_hash];
                parent_ancestry_hashes.sort_unstable();
                parent_ancestry_hashes.dedup();
                let parent_linkage_mode = match parent_ancestry_hashes.len() {
                    0 => ParentLinkageMode::None,
                    1 => ParentLinkageMode::Single,
                    _ => ParentLinkageMode::Compound,
                };
                Lineage {
                    origin_features: features,
                    creation_op: sig.clone(),
                    ancestry_hash,
                    parent_ancestry_hashes,
                    parent_linkage_mode,
                }
            }
            (Some(la), None) => Lineage::derive(la, sig.clone()),
            (None, Some(lb)) => Lineage::derive(lb, sig.clone()),
            (None, None) => Lineage::root(0, sig.clone()),
        }
    }

    /// FNV-1a style mixing of two 128-bit hashes.
    fn fnv_mix_128(a: u128, b: u128) -> u128 {
        let mixed = a.wrapping_mul(0x1000000000000000000013b) ^ b;
        mixed.wrapping_mul(0x1000000000000000000013b) ^ (a.rotate_left(17))
    }
}

/// Events logged during topology mutations for the replay system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineageEvent {
    /// A new entity was created
    EntityCreated {
        /// The specific entity (kind + index)
        entity: EntityRef,
        /// Generational snapshot identity, if captured by the recording path.
        #[serde(default)]
        entity_snapshot: Option<LineageEntityRef>,
        /// The lineage assigned to it
        lineage: Lineage,
    },
    /// An entity was deleted
    EntityDeleted {
        /// The specific entity (kind + index)
        entity: EntityRef,
        /// Generational snapshot identity, if captured by the recording path.
        #[serde(default)]
        entity_snapshot: Option<LineageEntityRef>,
        /// The lineage of the deleted entity (preserved for replay)
        lineage: Lineage,
    },
    /// An entity was modified (e.g., connectivity changed)
    EntityModified {
        /// The specific entity (kind + index)
        entity: EntityRef,
        /// Generational snapshot identity, if captured by the recording path.
        #[serde(default)]
        entity_snapshot: Option<LineageEntityRef>,
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

    /// The entity kind.
    pub fn get_entity_kind(&self) -> forge_core::EntityKind {
        self.get_entity().kind()
    }

    /// Generational snapshot identity for this event, if recorded.
    ///
    /// `None` indicates legacy/index-only lineage history or synthetic lineage
    /// events that did not capture generation.
    pub fn get_entity_snapshot(&self) -> Option<LineageEntityRef> {
        match self {
            LineageEvent::EntityCreated { entity_snapshot, .. }
            | LineageEvent::EntityDeleted { entity_snapshot, .. }
            | LineageEvent::EntityModified { entity_snapshot, .. } => *entity_snapshot,
        }
    }
}

// EntityKind is now defined in forge-core::tracing::schema.
// forge-topo re-exports it via `use forge_core::EntityKind;`

#[cfg(test)]
mod tests {
    use super::*;

    fn json_safe_lineage() -> Lineage {
        Lineage {
            origin_features: smallvec![7],
            creation_op: OpSignature::with_id("make_face", 1),
            ancestry_hash: 42,
            parent_ancestry_hashes: SmallVec::new(),
            parent_linkage_mode: ParentLinkageMode::None,
        }
    }
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
    fn op_signature_hashing_does_not_alias_name_and_invocation_xor_patterns() {
        let a = Lineage::root(0, OpSignature::with_id("a", 1));
        let b = Lineage::root(0, OpSignature::with_id("b", 2));
        assert_ne!(a.get_ancestry_hash(), b.get_ancestry_hash());
    }

    #[test]
    fn op_signature_display() {
        let op = OpSignature::with_id("split_edge", 42);
        assert_eq!(format!("{}", op), "split_edge#42");
    }

    #[test]
    fn derive_records_single_parent_hash_linkage() {
        let root = Lineage::root(7, OpSignature::with_id("make", 1));
        let child = Lineage::derive(&root, OpSignature::with_id("split_edge", 2));

        assert_eq!(child.get_parent_linkage_mode(), ParentLinkageMode::Single);
        assert_eq!(child.get_parent_ancestry_hashes(), &[root.get_ancestry_hash()]);
    }

    #[test]
    fn merge_records_compound_parent_hash_linkage_sorted_and_deduped() {
        let a = Lineage::root(1, OpSignature::with_id("a", 1));
        let b = Lineage::root(2, OpSignature::with_id("b", 2));
        let merged = Lineage::merge(&Some(a.clone()), &Some(b.clone()), &OpSignature::with_id("merge", 3));

        let mut expected = vec![a.get_ancestry_hash(), b.get_ancestry_hash()];
        expected.sort_unstable();
        assert_eq!(merged.get_parent_linkage_mode(), ParentLinkageMode::Compound);
        assert_eq!(merged.get_parent_ancestry_hashes(), expected.as_slice());
    }

    #[test]
    fn merge_same_parent_collapses_to_single_parent_linkage_mode() {
        let a = Lineage::root(1, OpSignature::with_id("a", 1));
        let merged = Lineage::merge(&Some(a.clone()), &Some(a.clone()), &OpSignature::with_id("merge", 3));

        assert_eq!(merged.get_parent_linkage_mode(), ParentLinkageMode::Single);
        assert_eq!(merged.get_parent_ancestry_hashes(), &[a.get_ancestry_hash()]);
    }

    #[test]
    fn lineage_event_legacy_deserialize_without_entity_snapshot_preserves_none() {
        let ev = LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::Face, 42),
            entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 42, 9)),
            lineage: json_safe_lineage(),
        };
        let mut payload = serde_json::to_value(&ev).unwrap();
        if let serde_json::Value::Object(wrapper) = &mut payload {
            if let Some(serde_json::Value::Object(inner)) = wrapper.get_mut("EntityCreated") {
                inner.remove("entity_snapshot");
            }
        }

        let ev: LineageEvent = serde_json::from_value(payload).unwrap();
        match ev {
            LineageEvent::EntityCreated { entity_snapshot, .. } => {
                assert_eq!(entity_snapshot, None);
            }
            _ => panic!("expected EntityCreated"),
        }
    }

    #[test]
    fn lineage_event_round_trip_preserves_generational_snapshot_identity() {
        let ev = LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::Face, 42),
            entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 42, 9)),
            lineage: json_safe_lineage(),
        };
        let encoded = serde_json::to_value(&ev).unwrap();
        let decoded: LineageEvent = serde_json::from_value(encoded).unwrap();
        assert_eq!(
            decoded.get_entity_snapshot(),
            Some(LineageEntityRef::new(EntityKind::Face, 42, 9))
        );
    }
}
