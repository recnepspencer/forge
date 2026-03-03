//! Display implementation for `MergeError`.

use std::fmt;

use crate::errors::data::MergeError;

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeError::AmbiguousRadialSelection {
                edge_index,
                valence,
            } => {
                write!(
                    f,
                    "Ambiguous radial selection at edge {}: valence {} requires explicit RadialUseSelector",
                    edge_index, valence
                )
            }
            MergeError::SelectedUsesNotSheetLike { edge_index } => {
                write!(
                    f,
                    "Selected radial uses at edge {} are not sheet-like",
                    edge_index
                )
            }
            MergeError::ProtectedUseConflict {
                face_index,
                edge_index,
            } => match edge_index {
                Some(ei) => write!(
                    f,
                    "Protected face {} conflicts with merge at edge {}",
                    face_index, ei
                ),
                None => write!(
                    f,
                    "Protected face {} is in both selected and protected sets",
                    face_index
                ),
            },
            MergeError::WouldDisconnectSheet { face_index } => {
                write!(f, "Merging face {} would disconnect the sheet", face_index)
            }
            MergeError::BoundaryCertificationFailed { reason, witness } => match witness {
                Some(w) => write!(
                    f,
                    "Boundary certification failed at [{:.6}, {:.6}]: {}",
                    w[0], w[1], reason
                ),
                None => write!(f, "Boundary certification failed: {}", reason),
            },
            MergeError::PartialMergePlanRejected { step_index, reason } => match step_index {
                Some(n) => write!(f, "Merge plan rejected at step {}: {}", n, reason),
                None => write!(f, "Merge plan rejected during construction: {}", reason),
            },
            MergeError::PersistentResolutionMissing { role, query } => {
                write!(
                    f,
                    "Persistent resolution missing for {:?}: {:?}",
                    role, query
                )
            }
            MergeError::PersistentResolutionAmbiguous {
                role,
                candidate_count,
                query,
            } => {
                write!(
                    f,
                    "Persistent resolution ambiguous for {:?} ({} candidates): {:?}",
                    role, candidate_count, query
                )
            }
            MergeError::PersistentResolutionIncompatible {
                role,
                incompatibility,
                query,
            } => {
                write!(
                    f,
                    "Persistent resolution incompatible for {:?}: {:?} (query={:?})",
                    role, incompatibility, query
                )
            }
            MergeError::UnsupportedPersistentNmtOutput => {
                write!(
                    f,
                    "Persistent NMT output is not supported in this milestone"
                )
            }
        }
    }
}
