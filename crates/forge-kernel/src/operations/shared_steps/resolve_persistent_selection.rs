//! Resolve persistent entity selections to concrete topology handles.
//!
//! DOMAIN: Translates selection specs (by-name, by-annotation, by-lineage)
//! into resolved `FaceId` / `HalfEdgeId` sets so that downstream steps
//! operate on verified live handles.
//!
//! DEPENDENCIES: forge-topo (handles, naming), forge-core (KernelError)

use forge_core::KernelError;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;

/// A resolved face selection ready for downstream pipeline steps.
#[derive(Debug, Clone)]
pub struct ResolvedFaceSelection {
    /// Live face handles, sorted deterministically (BTreeSet order).
    pub faces: Vec<FaceId>,
    /// Human-readable label for tracing (e.g. "target", "tool_edge_ring").
    pub label: String,
}

/// Resolve a set of face indices (from a JSON command or persistent name) to
/// live `FaceId` handles verified against the arena.
///
/// Returns `KernelError::InvalidInput` for any index that is not alive in the
/// arena, so that downstream steps never silently skip stale handles.
pub fn resolve_face_indices(
    indices: &[u32],
    topo: &TopologyState,
    label: &str,
) -> Result<ResolvedFaceSelection, KernelError> {
    let mut faces = Vec::with_capacity(indices.len());

    for &idx in indices {
        // Scan arena for a live face whose index matches.
        // `iter_faces` returns only live entries (generation-checked).
        let found = topo.arena().iter_faces().find(|(fid, _)| fid.index() == idx);
        match found {
            Some((fid, _)) => faces.push(fid),
            None => {
                return Err(KernelError::InvalidInput {
                    message: format!(
                        "resolve_persistent_selection: face index {} not alive in arena (label='{}')",
                        idx, label
                    ),
                    context: None,
                });
            }
        }
    }

    // Deterministic ordering — by index ascending.
    faces.sort_by_key(|f| f.index());
    faces.dedup_by_key(|f| f.index());

    Ok(ResolvedFaceSelection {
        faces,
        label: label.to_string(),
    })
}
