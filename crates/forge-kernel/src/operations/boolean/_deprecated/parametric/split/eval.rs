//! Orchestration for the face-splitting phase — entry point only.
//!
//! DOMAIN: Coordinate BVH query, cut-proposal, per-solid splitting, reconciliation.
//! MODULES:
//!   - hint_norm     — ExpectedCutHint deduplication and localization
//!   - plane_table   — PlaneTable construction and vertex provenance assignment
//!   - cut_proposal  — BVH overlap detection and cut proposal generation
//!   - solid_split   — per-solid split loop

use std::collections::BTreeMap;

use forge_core::ToleranceProvider;
use forge_core::KernelError;
use forge_topo::handles::FaceId;
use forge_topo::transactions::TopologyState;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::geometry_state::GeometryState;
use crate::geom_facade::are_parallel_exact as planes_are_parallel;

use super::gate::compute_face_chord;
use super::reconcile::reconcile_boundary_vertices;
use super::schema::{
    ExpectedCutEndpointMap, ExpectedCutHint, ExpectedCutInterval,
    SplitPhaseResult,
};
use crate::shared_ops::intersection_registry::IntersectionRegistry;

use super::cut_proposal::{build_bvh_overlap_pairs, propose_cuts, supplement_cuts_exhaustive};
use super::hint_norm::normalize_hint_map;
use super::plane_table::build_plane_tables;
use super::solid_split::split_solid;

use crate::shared_ops::vertex::dedup::dedup_points_by_tolerance;
use super::schema::PlaneTable;

pub use crate::shared_ops::spatial::bvh::compute_face_aabbs;

/// Run the full split phase for both solids.
///
/// 1. Build the shared `PlaneTable`.
/// 2. Detect overlapping face pairs via BVH.
/// 3. Collect expected overlap-segment hints for the proof system.
/// 4. Propose and supplement cuts.
/// 5. Split both solids independently against the shared `PlaneTable`.
/// 6. Reconcile boundary vertices across the two split solids.
pub fn split_all_faces(
    target_topo: TopologyState,
    target_geom: GeometryState,
    tool_topo: TopologyState,
    tool_geom: GeometryState,
    ctx: &mut crate::core::ModelingContext,
) -> Result<SplitPhaseResult, KernelError> {
    let config = crate::core::ToleranceConfig::default();

    let (mut plane_table, target_face_planes, tool_face_planes) =
        build_plane_tables(&target_topo, &target_geom, &tool_topo, &tool_geom);

    let bvh_pairs = build_bvh_overlap_pairs(
        target_topo.arena(), &target_geom,
        tool_topo.arena(), &tool_geom,
        &config,
    )?;

    let (expected_shared_positions, target_expected_cut_endpoints, tool_expected_cut_endpoints) =
        collect_expected_overlap_hints(
            &bvh_pairs,
            target_topo.arena(), &target_geom, &target_face_planes,
            tool_topo.arena(), &tool_geom, &tool_face_planes,
            &plane_table, &config,
        )?;

    let (mut target_cuts, mut tool_cuts) = propose_cuts(
        &bvh_pairs, &target_face_planes, &tool_face_planes,
        &plane_table, target_topo.arena(), tool_topo.arena(),
    );

    let supplemented = supplement_cuts_exhaustive(
        target_topo.arena(), &target_geom, &target_face_planes,
        tool_topo.arena(), &tool_geom, &tool_face_planes,
        &plane_table, &config, &mut target_cuts, &mut tool_cuts,
    )?;

    eprintln!(
        "[split] BVH pairs: {}, supplemented: {}, target faces with cuts: {}, tool faces with cuts: {}",
        bvh_pairs.len(), supplemented, target_cuts.len(), tool_cuts.len()
    );

    let mut shared_registry = IntersectionRegistry::new();

    let (mut target_draft, mut target_geom_out, target_splits, mut target_dedup, target_original_vids) =
        split_solid(
            target_topo, target_geom, target_cuts, &target_face_planes,
            &mut plane_table, &config, &mut shared_registry,
            target_expected_cut_endpoints, ctx,
        )?;

    let (mut tool_draft, mut tool_geom_out, tool_splits, mut tool_dedup, tool_original_vids) =
        split_solid(
            tool_topo, tool_geom, tool_cuts, &tool_face_planes,
            &mut plane_table, &config, &mut shared_registry,
            tool_expected_cut_endpoints, ctx,
        )?;

    if std::env::var("FORGE_DEBUG_VALIDATE_PHASES").ok().as_deref() == Some("1") {
        log_validation("split target pre-reconcile", &target_draft);
        log_validation("split tool pre-reconcile", &tool_draft);
    }

    let base_reconcile_tol = config.get_residual()
        .max(ctx.get_gap_closure().get_max_gap())
        .max(target_geom_out.global_default())
        .max(tool_geom_out.global_default());
    let reconcile_search_tol =
        base_reconcile_tol.max(ctx.get_gap_closure().get_max_gap() * 256.0);
    if std::env::var("FORGE_DEBUG_VALIDATE_PHASES").ok().as_deref() == Some("1") {
        eprintln!(
            "[reconcile] search_tol={:.6e} (base={:.6e}, gap_max={:.6e})",
            reconcile_search_tol, base_reconcile_tol,
            ctx.get_gap_closure().get_max_gap()
        );
    }
    let weld_tol_sq = reconcile_search_tol * reconcile_search_tol;
    let expected_shared_tol = config.get_residual()
        .max(ctx.get_gap_closure().get_max_gap())
        .max(target_geom_out.global_default())
        .max(tool_geom_out.global_default())
        .max(config.get_min_edge_length() * 0.01);
    let expected_shared_tol_sq = expected_shared_tol * expected_shared_tol;

    let _reconciled = reconcile_boundary_vertices(
        &mut target_draft, &mut target_geom_out, &mut target_dedup,
        &mut tool_draft, &mut tool_geom_out, &mut tool_dedup,
        &shared_registry, weld_tol_sq, expected_shared_tol_sq,
        &expected_shared_positions, &target_original_vids, &tool_original_vids,
    )?;

    if std::env::var("FORGE_DEBUG_VALIDATE_PHASES").ok().as_deref() == Some("1") {
        log_validation("split target post-reconcile", &target_draft);
        log_validation("split tool post-reconcile", &tool_draft);
    }

    let target_res_topo = target_draft.commit()?;
    let tool_res_topo = tool_draft.commit()?;

    Ok(SplitPhaseResult {
        target_topology: target_res_topo,
        target_geometry: target_geom_out,
        tool_topology: tool_res_topo,
        tool_geometry: tool_geom_out,
        split_count: target_splits + tool_splits,
        target_provenance: target_dedup.provenance,
        tool_provenance: tool_dedup.provenance,
    })
}

// ── Expected overlap hint collection ────────────────────────────────────────

/// Collect expected chord-overlap endpoints for each non-parallel BVH pair.
///
/// These endpoints bound which positions are legitimate "shared boundary" candidates
/// during reconciliation, filtering out chord-tail vertices from one-sided splits.
fn collect_expected_overlap_hints(
    bvh_pairs: &[(FaceId, FaceId)],
    target_arena: &forge_topo::b_rep::TopologyArena,
    target_geom: &GeometryState,
    target_face_planes: &BTreeMap<FaceId, usize>,
    tool_arena: &forge_topo::b_rep::TopologyArena,
    tool_geom: &GeometryState,
    tool_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    config: &crate::core::ToleranceConfig,
) -> Result<(Vec<[f64; 3]>, ExpectedCutEndpointMap, ExpectedCutEndpointMap), KernelError> {
    let mut positions = Vec::new();
    let mut target_expected: ExpectedCutEndpointMap = BTreeMap::new();
    let mut tool_expected: ExpectedCutEndpointMap = BTreeMap::new();

    for &(face_a, face_b) in bvh_pairs {
        let Some(&pa) = target_face_planes.get(&face_a) else { continue; };
        let Some(&pb) = tool_face_planes.get(&face_b) else { continue; };
        let plane_a = plane_table.get(pa);
        let plane_b = plane_table.get(pb);

        if planes_are_parallel(plane_a, plane_b) { continue; }

        let chord_a = match compute_face_chord(target_arena, target_geom, face_a, plane_a, plane_b, config)? {
            Some(ch) => ch,
            None => continue,
        };
        let chord_b = match compute_face_chord(tool_arena, tool_geom, face_b, plane_b, plane_a, config)? {
            Some(ch) => ch,
            None => continue,
        };

        if let Some((p0, p1)) = forge_geom::algorithms::chord::chord_overlap_segment(
            chord_a, chord_b, config.get_min_edge_length(),
        ) {
            positions.push(p0);
            positions.push(p1);
            let target_hint = target_expected.entry((face_a, pb)).or_default();
            target_hint.endpoints.extend([p0, p1]);
            target_hint.intervals.push(ExpectedCutInterval { p0, p1 });
            let tool_hint = tool_expected.entry((face_b, pa)).or_default();
            tool_hint.endpoints.extend([p0, p1]);
            tool_hint.intervals.push(ExpectedCutInterval { p0, p1 });
        }
    }

    normalize_hint_map(&mut target_expected, config.get_min_edge_length());
    normalize_hint_map(&mut tool_expected, config.get_min_edge_length());

    Ok((
        dedup_points_by_tolerance(positions, config.get_min_edge_length()),
        target_expected,
        tool_expected,
    ))
}

// ── Validation helper ────────────────────────────────────────────────────────

fn log_validation(label: &str, draft: &forge_topo::transactions::MutableDraft) {
    match validate_topology(draft.arena(), ValidationLevel::Full) {
        Ok(()) => eprintln!("[phase-check] {} valid", label),
        Err(e) => eprintln!("[phase-check] {} invalid: {}", label, e),
    }
}
