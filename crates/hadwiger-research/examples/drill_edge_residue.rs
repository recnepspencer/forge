//! Direct criticality resolution for residue edges that witness transfer
//! could not certify, using the contraction reformulation.
//!
//! For an edge `e = {u, v}` of the non-4-colorable seed `G`, any 4-coloring
//! of `G - e` must give `u` and `v` the same color (otherwise it would
//! 4-color `G` itself). Hence `G - e` is 4-colorable iff the contraction
//! `G / e` is 4-colorable, and the contraction is the strictly smaller and
//! more constrained instance:
//! - `SatModelVerified` on `G / e` proves `e` critical.
//! - UNSAT on `G / e` proves `e` removable: `G - e` is a strictly sparser
//!   5-chromatic graph on the same exact unit-distance embedding.
//!
//! `DRILL_EDGES` is a comma-separated list of `u-v` edge labels.

use std::time::Instant;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, import_frontier_graph_seed_checked,
    verify_k_colorability_checked, ColorabilityVerificationPosture, FrontierGraphSeedImport,
    GraphVersion, HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

fn main() {
    let edges = std::env::var("DRILL_EDGES").expect("DRILL_EDGES lists u-v edge labels");
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("Heule 510 imports");
    let graph = imported.graph_version();
    println!(
        "baseline vertices={} edges={} digest={}",
        graph.vertex_count(),
        graph.edge_count(),
        graph.artifact_digest().stable_token()
    );
    for spec in edges.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let Some((left, right)) = spec.split_once('-') else {
            println!("edge={spec} finding=MALFORMED-SPEC");
            continue;
        };
        let contracted = contract_edge(graph, left, right);
        let started = Instant::now();
        let checked = verify_k_colorability_checked(&handle, &contracted, 4)
            .expect("colorability checker returns a posture");
        let posture = checked.colorability_verification().posture();
        let finding = match posture {
            ColorabilityVerificationPosture::SatModelVerified => "CRITICAL",
            ColorabilityVerificationPosture::UnsatVerified
            | ColorabilityVerificationPosture::UnsupportedCertificateBudget => {
                "REMOVABLE-CANDIDATE"
            }
            ColorabilityVerificationPosture::Rejected => "UNDETERMINED",
        };
        println!(
            "edge={left}-{right} finding={finding} method=contraction-solve posture={posture:?} vertices={} edges={} seconds={:.1}",
            contracted.vertex_count(),
            contracted.edge_count(),
            started.elapsed().as_secs_f64()
        );
    }
}

fn contract_edge(graph: &GraphVersion, left: &str, right: &str) -> GraphVersion {
    let merged = format!("c{left}x{right}");
    let rename = |vertex: &str| -> String {
        if vertex == left || vertex == right {
            merged.clone()
        } else {
            vertex.to_string()
        }
    };
    let mut builder = GraphVersion::builder(
        graph.parent_artifacts()[0].clone(),
        format!("residue-contract-{left}-{right}"),
    );
    let mut seen_vertices = std::collections::BTreeSet::new();
    for vertex in graph.vertices() {
        let label = rename(vertex.vertex_label());
        if seen_vertices.insert(label.clone()) {
            builder = builder.with_vertex(label).expect("vertex shape admits");
        }
    }
    let mut seen_edges = std::collections::BTreeSet::new();
    for edge in graph.edges() {
        let (edge_left, edge_right) = edge.endpoints();
        let mapped_left = rename(edge_left);
        let mapped_right = rename(edge_right);
        if mapped_left == mapped_right {
            continue;
        }
        let key = if mapped_left <= mapped_right {
            (mapped_left.clone(), mapped_right.clone())
        } else {
            (mapped_right.clone(), mapped_left.clone())
        };
        if seen_edges.insert(key) {
            builder = builder
                .with_undirected_edge(mapped_left, mapped_right)
                .expect("edge shape admits");
        }
    }
    builder.finish().expect("contracted graph shape admits")
}
