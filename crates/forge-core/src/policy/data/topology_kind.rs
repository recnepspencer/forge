//! Topology kind and body-state classification.
//!
//! DOMAIN: Shared contract types for validation policy resolution.
//! Used by `forge-kernel` (config) and `forge-topo` (dispatch) to agree
//! on what kind of topology is being validated.

use serde::{Deserialize, Serialize};

/// What dimension of topology this body represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TopologyKind {
    /// 0D — vertices only (construction geometry, degenerate bodies).
    Point,
    /// 1D — edges + vertices, no faces.
    Wire,
    /// 2D — faces + edges + vertices, may or may not be closed.
    Sheet,
    /// 3D — closed 2-manifold boundary enclosing volume.
    Solid,
}

/// Whether the boundary is topologically closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Closure {
    Open,
    Closed,
}

/// Manifold state of the topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Manifoldness {
    Manifold,
    NonManifold,
}

/// How much trust we place in this topology's correctness.
///
/// This is a body-level state, not just draft metadata.
/// Transitions:
/// - `Uncertified → Certified` on successful PostCommit validation.
/// - `Certified → Uncertified` on any mutation via `execute()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificationStage {
    /// Fully validated — safe to export, fillet, boolean.
    Certified,
    /// Exists but not yet validated — imported, mid-operation, or healing.
    Uncertified,
}

/// Body-level topology context for validation policy resolution.
///
/// Passed to `GroupPolicyRuntime::resolve()` so policy decisions
/// are kind-aware. Constructed at draft creation time from body metadata.
#[derive(Debug, Clone, Copy)]
pub struct TopologyContext {
    pub kind: TopologyKind,
    pub closure: Closure,
    pub manifoldness: Manifoldness,
    pub stage: CertificationStage,
}

impl TopologyContext {
    /// Default for new solids (most common construction path).
    pub const SOLID: Self = Self {
        kind: TopologyKind::Solid,
        closure: Closure::Closed,
        manifoldness: Manifoldness::Manifold,
        stage: CertificationStage::Uncertified,
    };

    /// Uncertified NMT intermediate (boolean/import staging).
    pub const NMT_INTERMEDIATE: Self = Self {
        kind: TopologyKind::Solid,
        closure: Closure::Open,
        manifoldness: Manifoldness::NonManifold,
        stage: CertificationStage::Uncertified,
    };

    /// Wire body (edges + vertices only).
    pub const WIRE: Self = Self {
        kind: TopologyKind::Wire,
        closure: Closure::Open,
        manifoldness: Manifoldness::Manifold,
        stage: CertificationStage::Uncertified,
    };

    /// Open sheet body.
    pub const SHEET_OPEN: Self = Self {
        kind: TopologyKind::Sheet,
        closure: Closure::Open,
        manifoldness: Manifoldness::Manifold,
        stage: CertificationStage::Uncertified,
    };
}
