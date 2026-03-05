//! Central registry of invariant contract profiles.
//!
//! Every `TopoOperator` references a profile from this file.
//! This is the single place to understand what any family of operators may break.

use crate::validators::invariant_id::{InvariantContract, InvariantRelation};
use crate::validators::invariant_group::InvariantGroup;

/// Operators that only create or destroy containment entities (Body, Lump, Region, Shell).
/// 
/// These operators never touch half-edges, edges, faces, vertices, loops, or disks.
/// They are purely structural scaffolding, so they relate to no wiring invariants.
pub const CONTAINER_LIFECYCLE: InvariantContract = InvariantContract {
    relation: |_| InvariantRelation::Unrelated,
};

/// Operators that fully rewire topology (MakeEdgeFace, SplitEdge, JoinFaces, etc.).
/// 
/// These are the most common operators. They assume the right to break pointer
/// coherence, loops, radial edges, and disks. They do not break shell closure
/// (they maintain watertightness) and they do not violate the Euler formula
/// (they maintain the balanced sum).
pub const FULL_TOPO_WIRING: InvariantContract = InvariantContract {
    relation: |id| match id.group() {
        InvariantGroup::PointerCoherence => InvariantRelation::MayBreak,
        InvariantGroup::LoopIntegrity    => InvariantRelation::MayBreak,
        InvariantGroup::Ownership        => InvariantRelation::MayBreak,
        InvariantGroup::RadialEdge       => InvariantRelation::MayBreak,
        InvariantGroup::VertexDisk       => InvariantRelation::MayBreak,
        InvariantGroup::CacheCoherence   => InvariantRelation::MayBreak,

        // Topo wiring operators typically preserve these:
        InvariantGroup::ShellClosure     => InvariantRelation::Unrelated,
        InvariantGroup::EulerFormula     => InvariantRelation::Unrelated,
    },
};

/// Operators that only manipulate radial cycles (non-manifold sewing / unsewing).
/// 
/// These operators alter how faces meet at edges, but do not change the loops
/// on those faces, the vertices, or the ownership hierarchy.
pub const RADIAL_SPLICE: InvariantContract = InvariantContract {
    relation: |id| match id.group() {
        InvariantGroup::PointerCoherence => InvariantRelation::MayBreak,
        InvariantGroup::RadialEdge       => InvariantRelation::MayBreak,
        InvariantGroup::CacheCoherence   => InvariantRelation::MayBreak,

        // Radial splices preserve all face-level and shell-level topology:
        InvariantGroup::LoopIntegrity    => InvariantRelation::Unrelated,
        InvariantGroup::Ownership        => InvariantRelation::Unrelated,
        InvariantGroup::VertexDisk       => InvariantRelation::Unrelated,
        InvariantGroup::ShellClosure     => InvariantRelation::Unrelated,
        InvariantGroup::EulerFormula     => InvariantRelation::Unrelated,
    },
};

/// Creation of isolated vertices (MakeIsolatedVertex).
/// 
/// These vertices have no edges yet, so they only affect the VertexDisk invariants
/// (they create a new empty disk) and potentially the cache coherence.
pub const ISOLATED_VERTEX: InvariantContract = InvariantContract {
    relation: |id| match id.group() {
        InvariantGroup::VertexDisk       => InvariantRelation::MayBreak,
        InvariantGroup::CacheCoherence   => InvariantRelation::MayBreak,

        // An isolated vertex has no edges, loops, or pointer structure:
        InvariantGroup::PointerCoherence => InvariantRelation::Unrelated,
        InvariantGroup::RadialEdge       => InvariantRelation::Unrelated,
        InvariantGroup::LoopIntegrity    => InvariantRelation::Unrelated,
        InvariantGroup::Ownership        => InvariantRelation::Unrelated,
        InvariantGroup::ShellClosure     => InvariantRelation::Unrelated,
        InvariantGroup::EulerFormula     => InvariantRelation::Unrelated,
    },
};
