//! Invariant group classification for validation scheduling.
//!
//! DOMAIN: Shared contract types for the invariant validation system.
//! `InvariantGroup` defines named subsets of structural invariants.
//! Individual `InvariantId`s and the `invariant_ids()` mapping live in
//! `forge-topo` — this crate only carries the scheduling contract.

use serde::{Deserialize, Serialize};

use super::topology_kind::{CertificationStage, Closure, TopologyContext, TopologyKind};

// ── InvariantGroup ──────────────────────────────────────────────────────

/// Named subsets of structural invariants.
///
/// Groups are the scheduling unit: policies skip, defer, or gate entire
/// groups rather than individual invariants. This keeps policy resolution
/// O(1) via bitmasks even as the invariant count grows to 80+.
///
/// # Bit positions
///
/// `#[repr(u8)]` assigns stable bit positions for bitmask operations.
/// Future groups are reserved (documented in comments) and must be
/// appended — never renumber existing variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum InvariantGroup {
    /// Pointer coherence — radial/next/prev reciprocity, no dangling refs, generational freshness.
    PointerCoherence = 0,
    /// Loop integrity — face-has-loop, min cardinality, no duplicate coedges, continuity.
    LoopIntegrity = 1,
    /// Ownership — single loop owner, no orphans, acyclic containment, inner/outer consistency.
    Ownership = 2,
    /// Radial edge — cycle uniqueness, neighbor consistency, no broken splices.
    RadialEdge = 3,
    /// Shell closure — face adjacency, no broken boundaries, laminar edges.
    ShellClosure = 4,
    /// Vertex disk — disk entries alive, partition correct, closure, no cross-disk coedges.
    VertexDisk = 5,
    /// Euler formula — per-component Euler characteristic.
    EulerFormula = 6,
    /// Cache coherence — side-car and index coherence.
    CacheCoherence = 7,
    /// Geometry-dependent — zero-length edges, zero-area faces, shell volume, orientation.
    Geometry = 10,
    // ── Future (reserved bit positions) ─────────────────────────────
    // WireIntegrity       = 8,   // junction valence, chain closure (Wire only)
    // ParametricBinding   = 9,   // pcurve/3D curve consistency
    // Determinism         = 11,  // canonical ordering, hash stability
    // PersistentNaming    = 12,  // name survival through split/merge
    // IntersectionGraph   = 13,  // imprint graph connectivity (boolean)
    // NumericalPipeline   = 14,  // predicate divergence, interval bounds
}

impl InvariantGroup {
    /// Number of currently defined groups.
    pub const COUNT: usize = 9;

    /// All currently defined groups.
    pub const ALL: &[Self] = &[
        Self::PointerCoherence,
        Self::LoopIntegrity,
        Self::Ownership,
        Self::RadialEdge,
        Self::ShellClosure,
        Self::VertexDisk,
        Self::EulerFormula,
        Self::CacheCoherence,
        Self::Geometry,
    ];

    /// Bitmask for this group (for `GroupPolicy` bitset operations).
    pub const fn mask(&self) -> u32 {
        1u32 << (*self as u8)
    }

    /// Scheduling tier for this group.
    pub const fn tier(&self) -> InvariantTier {
        match self {
            Self::PointerCoherence
            | Self::LoopIntegrity
            | Self::Ownership
            | Self::RadialEdge
            | Self::VertexDisk => InvariantTier::Topology,

            Self::ShellClosure | Self::EulerFormula | Self::Geometry
                => InvariantTier::Semantic,

            Self::CacheCoherence => InvariantTier::Cache,
        }
    }
}

// ── InvariantTier ───────────────────────────────────────────────────────

/// Scheduling tier for validation groups.
///
/// Tiers control **when** a group runs if it is applicable.
/// Applicability (whether it applies at all) is determined by
/// `TopologyKind` and `skip_mask`.
///
/// - **Topology**: Structural graph integrity — per-op by default.
///   (PointerCoherence, LoopIntegrity, Ownership, RadialEdge, VertexDisk)
/// - **Semantic**: Domain correctness — deferred to commit by default.
///   (ShellClosure, EulerFormula)
/// - **Cache**: Index coherence — per-op by default.
///   (CacheCoherence)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvariantTier {
    Topology,
    Semantic,
    Cache,
}

// ── ValidatorCost ───────────────────────────────────────────────────────

/// Algorithmic cost classification for a validator.
///
/// Drives cost-tier filtering: cheap validators always run per-op,
/// expensive validators only run at commit time or in debug override.
///
/// Moved to `forge-core` so `GroupPolicyConfig` (in `forge-kernel`)
/// can reference it without depending on `forge-topo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ValidatorCost {
    /// O(n) single pass — always safe to run per-op.
    Cheap,
    /// O(n log n) or requires secondary data structures.
    Medium,
    /// O(n²) or global analysis (Euler, shell closure).
    Expensive,
}

// ── Bitmask Default Tables ──────────────────────────────────────────────

/// Default applicability: which groups apply to which topology kind.
/// Indexed by `[TopologyKind as usize]`. Value = bitmask of applicable groups.
///
/// Adding a new `InvariantGroup` variant? Update the relevant masks.
/// Adding a new `TopologyKind`? Add a row.
pub const APPLICABLE_BY_KIND: [u32; 4] = [
    // Point (0D): only CacheCoherence
    InvariantGroup::CacheCoherence.mask(),

    // Wire (1D): Pointer + Ownership + Cache
    // No faces → no loops, radial, shell closure, vertex disk (face-based), euler
    InvariantGroup::PointerCoherence.mask()
        | InvariantGroup::Ownership.mask()
        | InvariantGroup::CacheCoherence.mask(),

    // Sheet (2D): everything except ShellClosure (added for closed sheets via CLOSED_SHEET_EXTRA)
    InvariantGroup::PointerCoherence.mask()
        | InvariantGroup::LoopIntegrity.mask()
        | InvariantGroup::Ownership.mask()
        | InvariantGroup::RadialEdge.mask()
        | InvariantGroup::VertexDisk.mask()
        | InvariantGroup::EulerFormula.mask()
        | InvariantGroup::CacheCoherence.mask()
        | InvariantGroup::Geometry.mask(),

    // Solid (3D): everything
    InvariantGroup::PointerCoherence.mask()
        | InvariantGroup::LoopIntegrity.mask()
        | InvariantGroup::Ownership.mask()
        | InvariantGroup::RadialEdge.mask()
        | InvariantGroup::ShellClosure.mask()
        | InvariantGroup::VertexDisk.mask()
        | InvariantGroup::EulerFormula.mask()
        | InvariantGroup::CacheCoherence.mask()
        | InvariantGroup::Geometry.mask(),
];

/// Additional groups for closed sheets (ShellClosure becomes applicable).
pub const CLOSED_SHEET_EXTRA: u32 = InvariantGroup::ShellClosure.mask();

/// Groups deferred to PostCommit when body is Uncertified (import/boolean staging).
pub const DEFER_UNCERTIFIED: u32 =
    InvariantGroup::ShellClosure.mask() | InvariantGroup::EulerFormula.mask();

/// Groups deferred to PostCommit by Semantic tier default.
/// Geometry checks are deferred because they require vertex positions
/// (available only at commit time via forge-spatial).
pub const DEFER_SEMANTIC_TIER: u32 =
    InvariantGroup::ShellClosure.mask()
    | InvariantGroup::EulerFormula.mask()
    | InvariantGroup::Geometry.mask();

/// Compute the full applicable mask for a given topology context.
pub fn applicable_mask_for(ctx: &TopologyContext) -> u32 {
    let mut mask = APPLICABLE_BY_KIND[ctx.kind as usize];
    if ctx.kind == TopologyKind::Sheet && ctx.closure == Closure::Closed {
        mask |= CLOSED_SHEET_EXTRA;
    }
    mask
}

/// Compute the deferred mask for a given topology context.
pub fn deferred_mask_for(ctx: &TopologyContext) -> u32 {
    let mut mask = DEFER_SEMANTIC_TIER;
    if ctx.stage == CertificationStage::Uncertified {
        mask |= DEFER_UNCERTIFIED;
    }
    mask
}
