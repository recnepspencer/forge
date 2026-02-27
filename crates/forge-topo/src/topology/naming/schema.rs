//! Persistent naming data shapes.
//!
//! DOMAIN: Stable entity references that survive parametric rebuild.
//!
//! A `PersistentName` captures the `ancestry_hash` from an entity's
//! `Lineage` at time-of-naming plus the entity kind and an optional
//! ordinal disambiguator for split entities.
//!
//! A `Selector` is a composable query that resolves to a set of
//! `EntityKey`s against a live topology arena.
//!
//! DEPENDENCIES: `forge_core::EntityKind`, `topology::attributes::EntityKey`

use serde::{Deserialize, Serialize};

use forge_core::EntityKind;

// ── PersistentName ────────────────────────────────────────────────────────────

/// A stable entity reference that survives parametric rebuild.
///
/// Captures the `ancestry_hash` from a `Lineage` at time-of-naming,
/// the entity kind, and an ordinal to distinguish siblings produced by
/// splitting one parent entity into multiple children.
///
/// The ordinal is 0 for entities that were not split, and 1..N for siblings
/// produced by a split (assigned in deterministic order by ancestry_hash).
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistentName {
    /// Ancestry hash captured at time-of-naming.
    ancestry_hash: u128,
    /// The entity kind.
    kind: EntityKind,
    /// Ordinal disambiguator for split entities (0 = single / no split).
    ordinal: u32,
}

impl PersistentName {
    /// Create a persistent name from an ancestry hash and entity kind.
    ///
    /// `ordinal` is 0 for entities not produced by a split.
    pub fn new(ancestry_hash: u128, kind: EntityKind, ordinal: u32) -> Self {
        Self {
            ancestry_hash,
            kind,
            ordinal,
        }
    }

    /// The ancestry hash embedded in this name.
    pub fn get_ancestry_hash(&self) -> u128 {
        self.ancestry_hash
    }

    /// The entity kind this name refers to.
    pub fn get_kind(&self) -> EntityKind {
        self.kind
    }

    /// Ordinal disambiguator (0 means unambiguous / no split).
    pub fn get_ordinal(&self) -> u32 {
        self.ordinal
    }

    /// Returns `true` if this name was produced by a split operation.
    pub fn is_split_child(&self) -> bool {
        self.ordinal > 0
    }
}

// ── Selector ──────────────────────────────────────────────────────────────────

/// A composable query for finding named entities after rebuild.
///
/// Selectors are evaluated lazily against a live topology arena.
/// Boolean composition (And / Or) allows multi-criteria filtering
/// without re-iterating the arena for each clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Selector {
    /// Match by exact ancestry hash (most precise).
    ByAncestry {
        /// The ancestry hash to match.
        hash: u128,
        /// Only return entities of this kind.
        kind: EntityKind,
    },

    /// Match all entities created by a specific feature ID.
    ByFeature {
        /// The feature ID from the `FeatureTree`.
        feature_id: u64,
        /// Only return entities of this kind.
        kind: EntityKind,
    },

    /// Match all entities created by a specific operation name.
    ByOperation {
        /// The operation name (e.g. `"make_vertex_face"`, `"split_edge"`).
        op_name: String,
        /// Only return entities of this kind.
        kind: EntityKind,
    },

    /// Both sub-selectors must match (intersection).
    And(Box<Selector>, Box<Selector>),

    /// Either sub-selector may match (union).
    Or(Box<Selector>, Box<Selector>),
}
