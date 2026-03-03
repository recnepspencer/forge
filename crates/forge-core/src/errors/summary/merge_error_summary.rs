//! Serializable summary for `MergeError`.

use serde::{Deserialize, Serialize};

use crate::errors::data::{
    MergeError, PersistentResolutionIncompatibility, PersistentResolutionRole,
};
use crate::tracing::ResolutionQuerySummary;

/// Serializable typed summary of `MergeError`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MergeErrorSummary {
    AmbiguousRadialSelection {
        edge_index: u32,
        valence: u32,
    },
    SelectedUsesNotSheetLike {
        edge_index: u32,
    },
    ProtectedUseConflict {
        face_index: u32,
        edge_index: Option<u32>,
    },
    WouldDisconnectSheet {
        face_index: u32,
    },
    BoundaryCertificationFailed {
        reason: String,
        witness: Option<[f64; 2]>,
    },
    PartialMergePlanRejected {
        step_index: Option<u32>,
        reason: String,
    },
    PersistentResolutionMissing {
        role: PersistentResolutionRole,
        query: ResolutionQuerySummary,
    },
    PersistentResolutionAmbiguous {
        role: PersistentResolutionRole,
        candidate_count: u32,
        query: ResolutionQuerySummary,
    },
    PersistentResolutionIncompatible {
        role: PersistentResolutionRole,
        incompatibility: PersistentResolutionIncompatibility,
        query: ResolutionQuerySummary,
    },
    UnsupportedPersistentNmtOutput,
}

impl From<&MergeError> for MergeErrorSummary {
    fn from(value: &MergeError) -> Self {
        match value {
            MergeError::AmbiguousRadialSelection {
                edge_index,
                valence,
            } => Self::AmbiguousRadialSelection {
                edge_index: *edge_index,
                valence: *valence,
            },
            MergeError::SelectedUsesNotSheetLike { edge_index } => Self::SelectedUsesNotSheetLike {
                edge_index: *edge_index,
            },
            MergeError::ProtectedUseConflict {
                face_index,
                edge_index,
            } => Self::ProtectedUseConflict {
                face_index: *face_index,
                edge_index: *edge_index,
            },
            MergeError::WouldDisconnectSheet { face_index } => Self::WouldDisconnectSheet {
                face_index: *face_index,
            },
            MergeError::BoundaryCertificationFailed { reason, witness } => {
                Self::BoundaryCertificationFailed {
                    reason: reason.clone(),
                    witness: *witness,
                }
            }
            MergeError::PartialMergePlanRejected { step_index, reason } => {
                Self::PartialMergePlanRejected {
                    step_index: *step_index,
                    reason: reason.clone(),
                }
            }
            MergeError::PersistentResolutionMissing { role, query } => {
                Self::PersistentResolutionMissing {
                    role: *role,
                    query: query.clone(),
                }
            }
            MergeError::PersistentResolutionAmbiguous {
                role,
                candidate_count,
                query,
            } => Self::PersistentResolutionAmbiguous {
                role: *role,
                candidate_count: *candidate_count,
                query: query.clone(),
            },
            MergeError::PersistentResolutionIncompatible {
                role,
                incompatibility,
                query,
            } => Self::PersistentResolutionIncompatible {
                role: *role,
                incompatibility: incompatibility.clone(),
                query: query.clone(),
            },
            MergeError::UnsupportedPersistentNmtOutput => Self::UnsupportedPersistentNmtOutput,
        }
    }
}
