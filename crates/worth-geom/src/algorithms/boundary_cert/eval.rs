//! Two-phase boundary certification orchestration.
//!
//! DOMAIN: Stateless 2D boundary certification for merge eligibility.
//! The fast exact-predicate path runs before the exact arrangement fallback.
//! Projection, fast-path mechanics, and fallback classification are semantic
//! children so this file remains the decision pipeline's table of contents.

mod fallback;
mod fast_path;
mod projection;

use super::schema::{BoundaryRejectReason, ProjectedBoundary2D, WeakSimpleCertificate};
use fast_path::{detect_all_collinear, find_degenerate_segment, try_fast_path, FastPathResult};

pub use projection::{build_projection_frame, project_boundary_to_2d, project_point};

/// Certify whether a projected 2D boundary is weakly simple.
///
/// Two-phase algorithm (spec §4.7):
/// 1. **Fast path**: exact orient2d crossing check on all segment pairs.
///    Returns `Simple` if no degeneracy evidence.
/// 2. **Fallback**: classifies all segment interactions, then runs
///    weakly-simple recognition on the classified events.
///
/// Returns `Rejected` if exact predicates cannot be evaluated (predicate
/// failure is treated as a certification failure, never silently ignored).
pub fn certify_boundary(boundary: &ProjectedBoundary2D) -> WeakSimpleCertificate {
    let segments = boundary.get_segments();

    if segments.len() < 3 {
        return WeakSimpleCertificate::Rejected {
            reason: BoundaryRejectReason::DegenerateBoundary,
            witness: [0.0, 0.0],
        };
    }

    if let Some(degenerate_witness) = find_degenerate_segment(segments) {
        return WeakSimpleCertificate::Rejected {
            reason: BoundaryRejectReason::DegenerateBoundary,
            witness: degenerate_witness,
        };
    }

    if let Some(witness) = detect_all_collinear(segments) {
        return WeakSimpleCertificate::Rejected {
            reason: BoundaryRejectReason::DegenerateBoundary,
            witness,
        };
    }

    match try_fast_path(segments) {
        FastPathResult::Simple => WeakSimpleCertificate::Simple,
        FastPathResult::Rejected { reason, witness } => {
            WeakSimpleCertificate::Rejected { reason, witness }
        }
        FastPathResult::NeedsFallback => fallback::run_fallback_certifier(segments),
    }
}
