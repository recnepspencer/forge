//! Assembly operations for the disjoint/contained zero-split path.
//!
//! DOMAIN: Copy complete solids into fresh arenas for disjoint/contained/touching results.

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::replay::ReplayLog;
use forge_topo::lineage::{LineageEvent, EntityKind, Lineage, OpSignature};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::classify::find_coplanar_face_pairs;
use crate::operations::boolean::eval::VertexMatchKey;
use crate::operations::boolean::schema::{BooleanResult, BooleanOp};
use super::super::copy::{copy_faces, VertexDedup};
use super::super::stitch::stitch_twins;
use super::super::cleanup::cleanup_degenerate_topology;
use super::eval::{are_solids_coincident, compute_disjoint_scale};

/// Handle Booleans where tool is contained inside target (or vice versa).
///
/// Face counts in the result must reflect which solid contributed the faces:
/// target faces vs tool faces, not just "primary" vs "secondary".
pub(super) fn execute_contained_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    tool_inside_target: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let target_fc = target_topo.arena().face_count();
    let tool_fc = tool_topo.arena().face_count();

    match (operation, tool_inside_target) {
        (BooleanOp::Union, true) => {
            let mut r = assemble_complete_shells(
                target_topo, target_geom, None, false, ctx
            )?;
            r.set_face_counts(target_fc, 0);
            Ok(r)
        }
        (BooleanOp::Union, false) => {
            let mut r = assemble_complete_shells(
                tool_topo, tool_geom, None, false, ctx
            )?;
            r.set_face_counts(0, tool_fc);
            Ok(r)
        }
        (BooleanOp::Intersection, true) => {
            let mut r = assemble_complete_shells(
                tool_topo, tool_geom, None, false, ctx
            )?;
            r.set_face_counts(0, tool_fc);
            Ok(r)
        }
        (BooleanOp::Intersection, false) => {
            let mut r = assemble_complete_shells(
                target_topo, target_geom, None, false, ctx
            )?;
            r.set_face_counts(target_fc, 0);
            Ok(r)
        }
        (BooleanOp::Subtraction, true) => {
            if are_solids_coincident(target_topo, target_geom, tool_topo, tool_geom)? {
                let empty_topo = TopologyState::empty();
                let empty_geom = GeometryStore::new();
                return Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), Vec::new()));
            }
            let mut r = assemble_complete_shells(
                target_topo, target_geom,
                Some((tool_topo, tool_geom)),
                true,
                ctx,
            )?;
            r.set_face_counts(target_fc, tool_fc);
            Ok(r)
        }
        (BooleanOp::Subtraction, false) => {
            let empty_topo = TopologyState::empty();
            let empty_geom = GeometryStore::new();
            Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), Vec::new()))
        }
    }
}

/// Handle Booleans where the two solids are disjoint.
pub(super) fn execute_disjoint_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    match operation {
        BooleanOp::Union => {
            assemble_complete_shells(
                target_topo, target_geom,
                Some((tool_topo, tool_geom)),
                false,
                ctx,
            )
        }
        BooleanOp::Intersection => {
            let empty_topo = TopologyState::empty();
            let empty_geom = GeometryStore::new();
            Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), Vec::new()))
        }
        BooleanOp::Subtraction => {
            assemble_complete_shells(
                target_topo, target_geom,
                None, false, ctx
            )
        }
    }
}

/// Handle Booleans where the two solids are touching (flush coplanar contact).
///
/// For Union: identifies coplanar face pairs (same geometric plane, opposite
/// normals) between the two solids and drops them — they form an internal
/// boundary that must be removed by regularization. Remaining faces are
/// assembled into a single manifold with shared vertex merging.
///
/// For other operations: delegates to existing handlers.
pub(super) fn execute_touching_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    if operation != BooleanOp::Union {
        return execute_disjoint_boolean(
            target_topo, target_geom, tool_topo, tool_geom, operation, ctx,
        );
    }

    let (excluded_target, excluded_tool) = find_coplanar_face_pairs(
        target_topo, target_geom, tool_topo, tool_geom,
    );

    eprintln!("[TOUCHING_UNION] excluded_target={} excluded_tool={}",
        excluded_target.len(), excluded_tool.len());

    let target_faces: Vec<FaceId> = target_topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .filter(|fid| !excluded_target.contains(&fid.index()))
        .collect();
    let tool_faces: Vec<FaceId> = tool_topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .filter(|fid| !excluded_tool.contains(&fid.index()))
        .collect();

    let target_count = target_faces.len();
    let tool_count = tool_faces.len();

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();

    let characteristic_scale = compute_disjoint_scale(
        target_topo.arena(), target_geom,
        Some((tool_topo.arena(), tool_geom)),
    );

    let mut global_vertex_map: std::collections::BTreeMap<VertexMatchKey, VertexId> =
        std::collections::BTreeMap::new();
    let mut spatial_index = super::super::copy::SpatialVertexIndex::new(characteristic_scale);
    let mut all_he_ids: Vec<HalfEdgeId> = Vec::new();

    let mut target_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut target_dedup,
        &mut all_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        target_topo.arena(), target_geom, &target_faces,
        false,
        None,
    )?;

    let mut tool_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut tool_dedup,
        &mut all_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        tool_topo.arena(), tool_geom, &tool_faces,
        false,
        None,
    )?;

    cleanup_degenerate_topology(&mut draft, &result_geom)?;

    stitch_twins(&mut draft, &all_he_ids, &result_geom, spatial_index.weld_tolerance_sq(), ctx)?;

    let topo = draft.commit()?;
    let lineage_events: Vec<LineageEvent> = topo.arena().iter_faces()
        .map(|(fid, _)| LineageEvent::EntityCreated {
            entity_kind: EntityKind::Face,
            lineage: Lineage::root(fid.index() as u64, OpSignature::new("touching_boolean_union")),
        })
        .collect();
    Ok(BooleanResult::new(
        topo, result_geom, target_count, tool_count,
        crate::operations::boolean::schema::BooleanIntrospection::default(),
        ReplayLog::new(), lineage_events,
    ))
}

/// Copy one or two complete solids into a fresh arena.
///
/// Each shell gets its own vertex dedup and twin stitching pass,
/// so they remain independent manifolds. No vertex merging across
/// shells — touching/shared vertices stay separate.
fn assemble_complete_shells(
    primary_topo: &TopologyState,
    primary_geom: &GeometryStore,
    secondary: Option<(&TopologyState, &GeometryStore)>,
    reverse_secondary: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();

    let primary_faces: Vec<FaceId> = primary_topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();
    let primary_count = primary_faces.len();

    let mut primary_he: Vec<HalfEdgeId> = Vec::new();
    let mut primary_dedup = VertexDedup::new();
    let mut primary_vertex_map: std::collections::BTreeMap<crate::operations::boolean::eval::VertexMatchKey, forge_topo::handles::VertexId> = std::collections::BTreeMap::new();
    let mut primary_spatial: super::super::copy::SpatialVertexIndex = super::super::copy::SpatialVertexIndex::new(
        compute_disjoint_scale(primary_topo.arena(), primary_geom, secondary.map(|(t, g)| (t.arena(), g))),
    );

    copy_faces(
        &mut draft, &mut result_geom, &mut primary_dedup,
        &mut primary_he,
        &mut primary_vertex_map,
        &mut primary_spatial,
        primary_topo.arena(), primary_geom, &primary_faces,
        false,
        None,
    )?;

    stitch_twins(&mut draft, &primary_he, &result_geom, primary_spatial.weld_tolerance_sq(), ctx)?;

    let mut secondary_count = 0usize;
    if let Some((sec_topo, sec_geom)) = secondary {
        let sec_faces: Vec<FaceId> = sec_topo.arena().iter_faces()
            .map(|(fid, _)| fid)
            .collect();
        secondary_count = sec_faces.len();

        let mut sec_he: Vec<HalfEdgeId> = Vec::new();
        let mut sec_dedup = VertexDedup::new();
        let mut sec_vertex_map: std::collections::BTreeMap<crate::operations::boolean::eval::VertexMatchKey, forge_topo::handles::VertexId> = std::collections::BTreeMap::new();
        let mut sec_spatial: super::super::copy::SpatialVertexIndex = super::super::copy::SpatialVertexIndex::new(
            compute_disjoint_scale(sec_topo.arena(), sec_geom, None),
        );

        copy_faces(
            &mut draft, &mut result_geom, &mut sec_dedup,
            &mut sec_he,
            &mut sec_vertex_map,
            &mut sec_spatial,
            sec_topo.arena(), sec_geom, &sec_faces,
            reverse_secondary,
            None,
        )?;

        stitch_twins(&mut draft, &sec_he, &result_geom, sec_spatial.weld_tolerance_sq(), ctx)?;
    }

    let topo = draft.commit()?;
    let lineage_events: Vec<LineageEvent> = topo.arena().iter_faces()
        .map(|(fid, _)| LineageEvent::EntityCreated {
            entity_kind: EntityKind::Face,
            lineage: Lineage::root(fid.index() as u64, OpSignature::new("assemble_complete_shells")),
        })
        .collect();
    Ok(BooleanResult::new(topo, result_geom, primary_count, secondary_count, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), lineage_events))
}
