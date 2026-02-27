//! Focused diagnostic for half-overlap Boolean to trace the pipeline.

use super::super::classify_schema::{FaceClassification, FaceOrigin};
use crate::operations::shared_steps::classify_faces::classify_faces;
use super::super::parametric::split::split_all_faces;
use super::super::test_helpers::{build_cube, face_centroid};
use crate::core::ModelingContext;

#[test]
fn diagnose_half_overlap_pipeline() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);

    let mut ctx = ModelingContext::default();
    let result = split_all_faces(topo_a, geom_a, topo_b, geom_b, &mut ctx).unwrap();
    let (t_topo, t_geom, l_topo, l_geom, _target_prov, _tool_prov) = result.into_parts();

    let mut ctx = ModelingContext::default();

    let target_classified = classify_faces(
        t_topo.arena(),
        &t_geom,
        l_topo.arena(),
        &l_geom,
        FaceOrigin::Target,
        &mut ctx,
    )
    .unwrap();

    let tool_classified = classify_faces(
        l_topo.arena(),
        &l_geom,
        t_topo.arena(),
        &t_geom,
        FaceOrigin::Tool,
        &mut ctx,
    )
    .unwrap();

    for cf in &target_classified {
        let fid = cf.face();
        let centroid = face_centroid(t_topo.arena(), &t_geom, fid);
        eprintln!(
            "  Face {:?}: {:?} centroid=({:.2}, {:.2}, {:.2})",
            fid,
            cf.classification(),
            centroid[0],
            centroid[1],
            centroid[2]
        );
    }

    for cf in &tool_classified {
        let fid = cf.face();
        let centroid = face_centroid(l_topo.arena(), &l_geom, fid);
        eprintln!(
            "  Face {:?}: {:?} centroid=({:.2}, {:.2}, {:.2})",
            fid,
            cf.classification(),
            centroid[0],
            centroid[1],
            centroid[2]
        );
    }

    let t_outside = target_classified
        .iter()
        .filter(|f| f.classification() == FaceClassification::Outside)
        .count();
    let t_inside = target_classified
        .iter()
        .filter(|f| f.classification() == FaceClassification::Inside)
        .count();
    let t_boundary = target_classified
        .iter()
        .filter(|f| f.classification() == FaceClassification::OnBoundary)
        .count();
    let l_outside = tool_classified
        .iter()
        .filter(|f| f.classification() == FaceClassification::Outside)
        .count();
    let l_inside = tool_classified
        .iter()
        .filter(|f| f.classification() == FaceClassification::Inside)
        .count();
    let l_boundary = tool_classified
        .iter()
        .filter(|f| f.classification() == FaceClassification::OnBoundary)
        .count();

    let union_target = t_outside + t_boundary;
    let union_tool = l_outside;
    eprintln!(
        "  Union would select: {} target + {} tool = {} total faces",
        union_target,
        union_tool,
        union_target + union_tool
    );
    eprintln!(
        "  Target: inside={}, outside={}, boundary={}",
        t_inside, t_outside, t_boundary
    );
    eprintln!(
        "  Tool:   inside={}, outside={}, boundary={}",
        l_inside, l_outside, l_boundary
    );
}
