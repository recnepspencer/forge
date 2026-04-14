//! Observable vertex placement — the atomic kernel write for all vertex creation.
//!
//! DOMAIN: Placing a vertex at a position in space with:
//!   - Position-based deduplication against already-placed vertices (coincidence detection)
//!   - Exact symbolic position storage when plane intersection data is available
//!   - `NearBoundary` decision recording when a proximity context exists
//!
//! CONSUMED BY: primitives (BSP mesh), booleans (intersection vertices),
//!              Euler operators, NURBS tessellation — anything that creates vertices.
//!
//! INVARIANTS:
//!   - The first vertex in a set never records a NearBoundary decision (no context yet).
//!   - A merged vertex reuses the existing `VertexId` — the arena is not written again.
//!   - `place_vertex_exact` falls back to float storage if exact computation fails.

use forge_core::tracing::DecisionSink;
use forge_core::KernelError;
use worth_geom::{intersect_three_planes_exact, Plane};
use forge_topo::b_rep::VertexData;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::provenance::LineageRecorder;
use forge_topo::transactions::MutableDraft;

use crate::geometry::facade::{ExactPosition, GeometryStore};

// ── Placement registry ───────────────────────────────────────────────────────

/// Tracks vertices already placed in an ongoing mesh-building operation.
///
/// Pass this by `&mut` through all `place_vertex*` calls for a single solid.
/// The registry is local to one construction session — reset per operation.
pub type PlacementRegistry = Vec<(VertexId, [f64; 3])>;

// ── Observable vertex placement ──────────────────────────────────────────────

/// Place a vertex at `pos`, deduplicating against `registry`.
///
/// If `pos` is within `tolerance` of an existing vertex, returns that vertex's
/// `VertexId` without writing to the arena. Otherwise inserts a new vertex.
///
/// Records a `NearBoundary` decision when a proximity context exists (i.e.
/// `registry` is non-empty — the first vertex has no neighbors to compare against).
///
/// # Returns
/// The `VertexId` for this position (new or merged).
pub fn place_vertex(
    draft: &mut MutableDraft,
    registry: &mut PlacementRegistry,
    pos: [f64; 3],
    tolerance: f64,
    sink: &mut dyn DecisionSink,
    recorder: &mut LineageRecorder,
) -> VertexId {
    let result = forge_spatial::find_coincident_vertex(registry, &pos, tolerance);

    let vid = if let Some((existing, _)) = result.coincident {
        // PROVENANCE CONTRACT: Deduped vertex retains its original lineage.
        // No event emitted — the vertex has lineage from first creation.
        //
        // FUTURE (Boolean vertex merging, Phase 3+): When dedup occurs across
        // different feature origins, emit EntityModified with
        // Lineage::merge(original, incoming) to capture shared provenance.
        existing
    } else {
        let placeholder = HalfEdgeId::new(u32::MAX, 0);
        let vid = draft.insert_vertex(VertexData::new(placeholder));
        recorder.stamp(draft.lineage_store_mut(), vid);
        registry.push((vid, pos));
        vid
    };

    // Record proximity decision when context exists.
    // The first vertex (empty registry) has no meaningful proximity — skip it.
    if result.nearest_distance.is_finite() {
        sink.record_near_boundary(vid.index(), result.nearest_distance, tolerance);
    }

    vid
}

/// Place a vertex at a symbolically exact position derived from three BSP planes.
///
/// Attempts to compute an exact `ExactPosition` from `planes[pa]`, `planes[pb]`,
/// `planes[pc]`. On success, stores the symbolic position in `geometry`. On
/// failure (degenerate planes), falls back to float storage.
///
/// Like `place_vertex`, deduplicates against `registry` and records a
/// `NearBoundary` decision when proximity context exists.
///
/// This is the variant used by BSP-to-mesh conversion where every vertex is
/// a plane-plane-plane intersection. Euler operators and Boolean vertex
/// insertion should use `place_vertex` directly.
pub fn place_vertex_exact(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    registry: &mut PlacementRegistry,
    pos: [f64; 3],
    plane_indices: [usize; 3],
    planes: &[Plane],
    tolerance: f64,
    sink: &mut dyn DecisionSink,
    recorder: &mut LineageRecorder,
) -> Result<VertexId, KernelError> {
    let result = forge_spatial::find_coincident_vertex(registry, &pos, tolerance);

    let vid = if let Some((existing, _)) = result.coincident {
        // PROVENANCE CONTRACT: Deduped vertex retains its original lineage.
        existing
    } else {
        let placeholder = HalfEdgeId::new(u32::MAX, 0);
        let vid = draft.insert_vertex(VertexData::new(placeholder));
        recorder.stamp(draft.lineage_store_mut(), vid);

        let [pa, pb, pc] = plane_indices;
        let stored_exact = if pa < planes.len() && pb < planes.len() && pc < planes.len() {
            match intersect_three_planes_exact(&planes[pa], &planes[pb], &planes[pc]) {
                Ok(exact_pos) => {
                    geometry.positions.set(
                        vid,
                        ExactPosition::from_symbolic(exact_pos, pos, [pa, pb, pc]),
                    );
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        };

        if !stored_exact {
            geometry.positions.set(vid, ExactPosition::from_f64(pos));
        }

        registry.push((vid, pos));
        vid
    };

    // Record proximity decision; skip first vertex (no context yet).
    if result.nearest_distance.is_finite() {
        sink.record_near_boundary(vid.index(), result.nearest_distance, tolerance);
    }

    Ok(vid)
}
