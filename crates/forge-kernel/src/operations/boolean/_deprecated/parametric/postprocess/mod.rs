//! Post-processing of boolean results.
//!
//! DOMAIN: Simplification passes after boolean assembly:
//! - Merge coplanar faces (polygon extraction or legacy JoinFaces)
//! - Remove redundant collinear vertices
//! - Splice inner holes into outer boundaries
//!
//! DEPENDENCIES: forge_topo (Euler operators), GeometryState.

mod coplanar;
pub mod curved_merge;
pub mod hole_splice;
pub mod merge_eligibility;
pub mod polygon_extract;
mod vertex;

use crate::core::{KernelState, ModelingContext};
use forge_core::KernelError;

pub use coplanar::merge_coplanar_faces;
pub use hole_splice::splice_inner_holes;
pub use polygon_extract::extract_coplanar_regions;
pub use vertex::remove_redundant_vertices;

/// Merge coplanar faces using the O(N) polygon extraction approach.
///
/// Falls back to the legacy iterative JoinFaces if extraction fails.
pub fn merge_coplanar_faces_extracted(
    state: KernelState,
    ctx: &mut ModelingContext,
) -> Result<(KernelState, usize), KernelError> {
    let mut draft = crate::core::KernelDraft::new(state);

    match extract_coplanar_regions(&mut draft, ctx) {
        Ok(count) => {
            if count > 0 {
                Ok((draft.commit()?, count))
            } else {
                Ok((draft.rollback(), 0))
            }
        }
        Err(_) => merge_coplanar_faces(draft.rollback(), ctx),
    }
}

pub(crate) fn run_iterative_pass(
    mut state: KernelState,
    mut pass_fn: impl FnMut(KernelState) -> Result<(KernelState, usize), KernelError>,
) -> Result<(KernelState, usize), KernelError> {
    let mut total = 0;
    let mut changed = 1;
    while changed > 0 {
        let (new_state, count) = pass_fn(state)?;
        state = new_state;
        changed = count;
        total += count;
    }
    Ok((state, total))
}
