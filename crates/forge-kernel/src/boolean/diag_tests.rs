//! Focused diagnostic for half-overlap Boolean to trace the pipeline.

use super::test_helpers::{build_cube, face_centroid};
use super::split::split_all_faces;
use super::classify::classify_faces;
use super::schema::{FaceOrigin, FaceClassification};
use crate::core::ToleranceConfig;

#[test]
fn diagnose_half_overlap_pipeline() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);

    eprintln!("BEFORE SPLIT: target_faces={}, tool_faces={}",
        topo_a.arena().face_count(), topo_b.arena().face_count());

    let result = split_all_faces(topo_a, geom_a, topo_b, geom_b).unwrap();
    let (t_topo, t_geom, l_topo, l_geom, _target_prov, _tool_prov) = result.into_parts();

    eprintln!("AFTER SPLIT: target_faces={}, tool_faces={}",
        t_topo.arena().face_count(), l_topo.arena().face_count());

    let config = ToleranceConfig::default();

    let (target_classified, _target_log) = classify_faces(
        t_topo.arena(), &t_geom,
        l_topo.arena(), &l_geom,
        FaceOrigin::Target,
        &config,
    ).unwrap();

    let (tool_classified, _tool_log) = classify_faces(
        l_topo.arena(), &l_geom,
        t_topo.arena(), &t_geom,
        FaceOrigin::Tool,
        &config,
    ).unwrap();

    eprintln!("\n=== TARGET CLASSIFICATION ===");
    for cf in &target_classified {
        let fid = cf.face();
        let centroid = face_centroid(t_topo.arena(), &t_geom, fid);
        eprintln!("  Face {:?}: {:?} centroid=({:.2}, {:.2}, {:.2})",
            fid, cf.classification(), centroid[0], centroid[1], centroid[2]);
    }

    eprintln!("\n=== TOOL CLASSIFICATION ===");
    for cf in &tool_classified {
        let fid = cf.face();
        let centroid = face_centroid(l_topo.arena(), &l_geom, fid);
        eprintln!("  Face {:?}: {:?} centroid=({:.2}, {:.2}, {:.2})",
            fid, cf.classification(), centroid[0], centroid[1], centroid[2]);
    }

    let t_outside = target_classified.iter().filter(|f| f.classification() == FaceClassification::Outside).count();
    let t_inside = target_classified.iter().filter(|f| f.classification() == FaceClassification::Inside).count();
    let t_boundary = target_classified.iter().filter(|f| f.classification() == FaceClassification::OnBoundary).count();
    let l_outside = tool_classified.iter().filter(|f| f.classification() == FaceClassification::Outside).count();
    let l_inside = tool_classified.iter().filter(|f| f.classification() == FaceClassification::Inside).count();
    let l_boundary = tool_classified.iter().filter(|f| f.classification() == FaceClassification::OnBoundary).count();

    eprintln!("\nSUMMARY:");
    eprintln!("  Target: {} outside, {} inside, {} boundary = {} total",
        t_outside, t_inside, t_boundary, target_classified.len());
    eprintln!("  Tool:   {} outside, {} inside, {} boundary = {} total",
        l_outside, l_inside, l_boundary, tool_classified.len());

    let union_target = t_outside + t_boundary;
    let union_tool = l_outside;
    eprintln!("  Union would select: {} target + {} tool = {} total faces",
        union_target, union_tool, union_target + union_tool);
}
