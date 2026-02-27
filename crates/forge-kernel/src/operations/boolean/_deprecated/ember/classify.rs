//! EMBER-specific coplanar face classification.
//!
//! DOMAIN: After the split phase, detect coplanar face pairs using
//! exact rational plane comparison (`coplanar_eq`) plus AABB overlap.
//! Force-exclude both faces from the Boolean selection, preventing the
//! asymmetric classification that causes `MissingTwin` on stitch.
//!
//! KEY DIFFERENCE FROM LEGACY: The legacy `find_coplanar_face_pairs`
//! uses `faces_overlap_3d` (2D polygon projection) to filter out
//! coplanar faces that don't spatially overlap. This is fragile on
//! split fragments. EMBER uses AABB overlap in 3D — cheaper and
//! sufficient after split fragments are aligned.
//!
//! INVARIANTS:
//!   - Symmetric: if face A is excluded, its coplanar partner B is too.
//!   - Only spatially overlapping coplanar faces are excluded.

use std::collections::BTreeSet;

use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;

use crate::geometry_state::GeometryState;
use crate::operations::boolean::{ClassifiedFace, FaceClassification};

/// Override classified face lists to force-exclude coplanar face pairs.
///
/// Scans both post-split topologies for coplanar face pairs using exact
/// rational plane comparison + AABB overlap. Any face with a coplanar,
/// overlapping counterpart in the other solid is reclassified to
/// `OnBoundary`, preventing asymmetric classification.
pub fn apply_ember_coplanar_overrides(
    target_classified: &mut Vec<ClassifiedFace>,
    tool_classified: &mut Vec<ClassifiedFace>,
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
) {
    let (excluded_target, excluded_tool) =
        find_coplanar_pairs_exact(target_topo, target_geom, tool_topo, tool_geom);

    if std::env::var("FORGE_DEBUG_COPLANAR_OVERRIDES")
        .ok()
        .as_deref()
        == Some("1")
    {
        for fid in [14u32, 15u32] {
            let tool_hit = excluded_tool.contains(&fid);
            if tool_hit {
                let face_id = tool_topo
                    .arena()
                    .iter_faces()
                    .find_map(|(face_id, _)| (face_id.index() == fid).then_some(face_id));
                let lineage = tool_topo
                    .arena()
                    .get_face(face_id.unwrap())
                    .ok()
                    .and_then(|f| f.lineage())
                    .map(|lin| {
                        format!(
                            "{}#{}",
                            lin.get_creation_op().get_name(),
                            lin.get_creation_op().get_invocation_id()
                        )
                    })
                    .unwrap_or_else(|| "no-lineage".to_string());
                eprintln!("[coplanar-override] tool F#{} excluded {}", fid, lineage);
            } else {
                eprintln!("[coplanar-override] tool F#{} not excluded", fid);
            }
        }
    }

    for face in target_classified.iter_mut() {
        if excluded_target.contains(&face.face().index()) {
            face.set_classification(FaceClassification::OnBoundary);
        }
    }

    for face in tool_classified.iter_mut() {
        if excluded_tool.contains(&face.face().index()) {
            face.set_classification(FaceClassification::OnBoundary);
        }
    }
}

/// Face AABB for overlap testing.
struct FaceAabb {
    min: [f64; 3],
    max: [f64; 3],
}

/// Compute the AABB of a face from its vertex positions.
fn compute_face_aabb(
    arena: &TopologyArena,
    geom: &GeometryState,
    face: FaceId,
) -> Option<FaceAabb> {
    let face_data = arena.get_face(face).ok()?;
    let loop_data = arena.get_loop(face_data.outer_loop()).ok()?;
    let first_he = loop_data.half_edge();
    let mut he = first_he;
    let mut min = [f64::MAX; 3];
    let mut max = [f64::MIN; 3];
    let mut found_any = false;

    loop {
        let he_data = arena.get_half_edge(he).ok()?;
        let origin = he_data.origin();
        if let Some(pos) = geom.get_vertex_position(origin) {
            for i in 0..3 {
                min[i] = min[i].min(pos[i]);
                max[i] = max[i].max(pos[i]);
            }
            found_any = true;
        }
        he = he_data.next();
        if he == first_he {
            break;
        }
    }

    if found_any {
        Some(FaceAabb { min, max })
    } else {
        None
    }
}

/// Whether two AABBs overlap (with small epsilon for touching faces).
fn aabbs_overlap(a: &FaceAabb, b: &FaceAabb) -> bool {
    let eps = 1e-10;
    (0..3).all(|i| a.min[i] <= b.max[i] + eps && b.min[i] <= a.max[i] + eps)
}

/// Whether two plane normals point in opposite directions.
///
/// For shared boundary faces between two solids, one normal faces outward
/// from each solid — so they point in opposite directions at the boundary.
/// Parallel exterior faces have same-direction normals.
fn normals_anti_parallel(a: &crate::geom_facade::Plane, b: &crate::geom_facade::Plane) -> bool {
    let dot = a.normal()[0] * b.normal()[0]
        + a.normal()[1] * b.normal()[1]
        + a.normal()[2] * b.normal()[2];
    dot < 0.0
}

/// Find coplanar face pairs using exact plane comparison + AABB overlap.
fn find_coplanar_pairs_exact(
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
) -> (BTreeSet<u32>, BTreeSet<u32>) {
    let mut excluded_target = BTreeSet::new();
    let mut excluded_tool = BTreeSet::new();

    let tool_data: Vec<_> = tool_topo
        .arena()
        .iter_faces()
        .filter_map(|(fid, _)| {
            let plane = tool_geom.get_face_plane(fid)?;
            let aabb = compute_face_aabb(tool_topo.arena(), tool_geom, fid)?;
            Some((fid, plane.clone(), aabb))
        })
        .collect();

    for (target_fid, _) in target_topo.arena().iter_faces() {
        if let Some(target_plane) = target_geom.get_face_plane(target_fid) {
            let target_aabb = match compute_face_aabb(target_topo.arena(), target_geom, target_fid)
            {
                Some(a) => a,
                None => continue,
            };

            let matched = tool_data.iter().find(|(tool_fid, tool_plane, tool_aabb)| {
                let not_excluded = !excluded_tool.contains(&tool_fid.index());
                let is_coplanar =
                    crate::geom_facade::coplanar_eq(target_plane, tool_plane);
                let anti_parallel = normals_anti_parallel(target_plane, tool_plane);
                let overlaps = aabbs_overlap(&target_aabb, tool_aabb);
                not_excluded && is_coplanar && anti_parallel && overlaps
            });

            if let Some((tool_fid, _, _)) = matched {
                excluded_target.insert(target_fid.index());
                excluded_tool.insert(tool_fid.index());
            }
        }
    }

    (excluded_target, excluded_tool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::boolean::test_helpers::build_cube;

    #[test]
    fn detect_shared_face_between_touching_cubes() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([2.0, 0.0, 0.0], 1.0);

        let (excl_a, excl_b) = find_coplanar_pairs_exact(&topo_a, &geom_a, &topo_b, &geom_b);

        assert!(
            excl_a.len() >= 1,
            "Target should have at least 1 excluded face (shared x=1 face)"
        );
        assert!(
            excl_b.len() >= 1,
            "Tool should have at least 1 excluded face (shared x=1 face)"
        );
        assert_eq!(excl_a.len(), excl_b.len(), "Exclusion must be symmetric");
    }

    #[test]
    fn no_shared_faces_for_disjoint_cubes() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([10.0, 0.0, 0.0], 1.0);

        let (excl_a, excl_b) = find_coplanar_pairs_exact(&topo_a, &geom_a, &topo_b, &geom_b);

        assert_eq!(
            excl_a.len(),
            0,
            "Disjoint cubes should have no overlapping coplanars"
        );
        assert_eq!(
            excl_b.len(),
            0,
            "Disjoint cubes should have no overlapping coplanars"
        );
    }

    #[test]
    fn exclusion_is_symmetric() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([2.0, 0.0, 0.0], 1.0);

        let (excl_ab_a, excl_ab_b) = find_coplanar_pairs_exact(&topo_a, &geom_a, &topo_b, &geom_b);
        let (excl_ba_b, excl_ba_a) = find_coplanar_pairs_exact(&topo_b, &geom_b, &topo_a, &geom_a);

        assert_eq!(
            excl_ab_a.len(),
            excl_ba_a.len(),
            "Symmetry: target exclusion count"
        );
        assert_eq!(
            excl_ab_b.len(),
            excl_ba_b.len(),
            "Symmetry: tool exclusion count"
        );
    }
}
