//! Merge operation error types and supporting enums.

use serde::{Deserialize, Serialize};

use crate::tracing::ResolutionQuerySummary;

/// Structured failure reasons for `MergeSheetRegion` and related NMT merge operations.
///
/// Wrapped by `KernelError::MergeFailure`. Never downgraded to `InternalError`.
/// Fields use raw indices (`u32`) for serialisability and trace output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeError {
    /// A high-valence edge neighborhood is ambiguous and requires an explicit
    /// `RadialUseSelector` to disambiguate which pair of face-uses to merge.
    AmbiguousRadialSelection { edge_index: u32, valence: u32 },

    /// The selected radial uses at the given edge do not form a sheet-like
    /// (locally planar, merging) pair.
    SelectedUsesNotSheetLike { edge_index: u32 },

    /// An explicitly protected radial use conflicts with the merge selection.
    ///
    /// `face_index`: the protected face that conflicts.
    /// `edge_index`: the edge where the conflict was detected, or `None` if
    /// detected at input validation (selected ∩ protected overlap).
    ProtectedUseConflict {
        face_index: u32,
        edge_index: Option<u32>,
    },

    /// Merging the selected face group would disconnect the sheet topology.
    WouldDisconnectSheet { face_index: u32 },

    /// The Epic A boundary certifier rejected the merge boundary.
    BoundaryCertificationFailed {
        reason: String,
        /// 2D witness point (projected face plane space) where the rejection occurred.
        witness: Option<[f64; 2]>,
    },

    /// The merge plan could not be fully executed.
    ///
    /// `step_index: None`    — rejected during plan construction, before execution began.
    /// `step_index: Some(n)` — rejected at execution step `n` (0-indexed).
    PartialMergePlanRejected {
        step_index: Option<u32>,
        reason: String,
    },

    /// Persistent reference resolution returned zero matches for a required merge role.
    PersistentResolutionMissing {
        role: PersistentResolutionRole,
        query: ResolutionQuerySummary,
    },

    /// Persistent reference resolution returned multiple matches and merge cannot choose.
    PersistentResolutionAmbiguous {
        role: PersistentResolutionRole,
        candidate_count: u32,
        query: ResolutionQuerySummary,
    },

    /// Persistent reference resolution could not run due to typed incompatibility.
    PersistentResolutionIncompatible {
        role: PersistentResolutionRole,
        incompatibility: PersistentResolutionIncompatibility,
        query: ResolutionQuerySummary,
    },

    /// Persistent NMT output is unavailable in this milestone.
    UnsupportedPersistentNmtOutput,
}

/// Role of a persistent reference inside region-merge intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistentResolutionRole {
    SurvivingFace,
    SelectedFace,
    ProtectedFace,
}

/// Typed incompatibility reported by persistent resolution adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistentResolutionIncompatibility {
    UnsupportedEntityKind {
        requested: crate::EntityKind,
    },
    MissingLineageStore,
    SubstrateUnavailable,
    UnsupportedEntityOrigin {
        origin: PersistentResolutionOriginKind,
    },
    UnsupportedLineageFallback,
    SchemaVersionMismatch {
        expected: u32,
        actual: u32,
    },
    Other {
        code: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistentResolutionOriginKind {
    TopoOperator,
    GeometricIntersection,
    ConstraintSolver,
    Unknown,
}
