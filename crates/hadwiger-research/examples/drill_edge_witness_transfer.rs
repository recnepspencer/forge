//! Edge-criticality mapping via witness transfer over the exact Heule-510
//! frontier seed.
//!
//! Phase 1 solves all 510 vertex deletions once and decodes each retained
//! SAT model into a checked 4-coloring of `G - v` (the vertex-criticality
//! witnesses). Phase 2 then certifies edge criticality without solving: for
//! an edge `{u, v}`, if the retained coloring of `G - u` gives `v` a color
//! no other neighbor of `u` wears, assigning that color to `u` constructs a
//! 4-coloring of `G - uv` that is exhaustively re-verified against every
//! remaining edge. Edges no endpoint witness can certify form the rigid
//! residue: the only places a removable edge (a strictly sparser 5-chromatic
//! unit-distance graph) can hide, and the priority queue for direct UNSAT
//! probes.
//!
//! Environment knobs:
//! - `TRANSFER_SOLVE_RESIDUE`: when set to a number N, direct-solve up to N
//!   residue edges (low pressure first) through the varisat lane.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, import_frontier_graph_seed_checked,
    verify_k_colorability_checked, ColorabilityVerificationPosture, FrontierGraphSeedImport,
    GraphVersion, HadwigerCanonicalArtifact, HadwigerResearchHandle,
    HadwigerResearchOperatingContext,
};

fn main() {
    let residue_solve_budget = std::env::var("TRANSFER_SOLVE_RESIDUE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("Heule 510 imports");
    let graph = imported.graph_version();
    let adjacency = adjacency_of(graph);
    println!(
        "baseline vertices={} edges={} digest={}",
        graph.vertex_count(),
        graph.edge_count(),
        graph.artifact_digest().stable_token()
    );

    let witnesses = collect_vertex_deletion_witnesses(&handle, graph, &adjacency);
    println!("witnesses_collected={}", witnesses.len());

    let mut certified = 0_usize;
    let mut residue = Vec::new();
    for (left, right) in normalized_edges(&adjacency) {
        let transferred = transfer_certificate(&adjacency, &witnesses, &left, &right)
            .or_else(|| transfer_certificate(&adjacency, &witnesses, &right, &left));
        match transferred {
            Some(donor) => {
                certified += 1;
                println!(
                    "edge={left}-{right} finding=CRITICAL method=witness-transfer donor={donor}"
                );
            }
            None => residue.push((left, right)),
        }
    }
    println!(
        "transfer_summary certified={} residue={} total={}",
        certified,
        residue.len(),
        certified + residue.len()
    );
    for (left, right) in &residue {
        println!("residue_edge={left}-{right}");
    }

    let mut remaining = residue_solve_budget;
    for (left, right) in &residue {
        if remaining == 0 {
            break;
        }
        remaining -= 1;
        let mutated = rebuild_without_edge(graph, left, right);
        let started = Instant::now();
        let checked = verify_k_colorability_checked(&handle, &mutated, 4)
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
            "edge={left}-{right} finding={finding} method=direct-solve posture={posture:?} seconds={:.1}",
            started.elapsed().as_secs_f64()
        );
    }
}

type Coloring = BTreeMap<String, u32>;

fn collect_vertex_deletion_witnesses(
    handle: &HadwigerResearchHandle,
    graph: &GraphVersion,
    adjacency: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, Coloring> {
    let mut witnesses = BTreeMap::new();
    for (index, vertex) in adjacency.keys().enumerate() {
        let mutated = rebuild_without_vertex(graph, vertex);
        let checked = verify_k_colorability_checked(handle, &mutated, 4)
            .expect("colorability checker returns a posture");
        if checked.colorability_verification().posture()
            != ColorabilityVerificationPosture::SatModelVerified
        {
            println!(
                "witness_gap vertex={} posture={:?}",
                vertex,
                checked.colorability_verification().posture()
            );
            continue;
        }
        let coloring = decode_model(
            checked.encoding().variable_map(),
            checked.solver_run().model(),
        );
        witnesses.insert(vertex.clone(), coloring);
        if (index + 1) % 50 == 0 {
            println!("witness_progress collected={}", index + 1);
        }
    }
    witnesses
}

fn decode_model(variable_map: &[(String, u32, i32)], model: &[i32]) -> Coloring {
    let assigned: BTreeSet<i32> = model
        .iter()
        .copied()
        .filter(|literal| *literal > 0)
        .collect();
    let mut coloring = BTreeMap::new();
    for (vertex, color, variable) in variable_map {
        if assigned.contains(variable) {
            coloring.entry(vertex.clone()).or_insert(*color);
        }
    }
    coloring
}

fn transfer_certificate(
    adjacency: &BTreeMap<String, BTreeSet<String>>,
    witnesses: &BTreeMap<String, Coloring>,
    anchor: &str,
    partner: &str,
) -> Option<String> {
    let witness = witnesses.get(anchor)?;
    let partner_color = *witness.get(partner)?;
    let neighbors = adjacency.get(anchor)?;
    let conflict = neighbors
        .iter()
        .filter(|neighbor| neighbor.as_str() != partner)
        .any(|neighbor| witness.get(neighbor) == Some(&partner_color));
    if conflict {
        return None;
    }
    let mut candidate = witness.clone();
    candidate.insert(anchor.to_string(), partner_color);
    if coloring_respects_all_edges_except(adjacency, &candidate, anchor, partner) {
        Some(format!("vertex-deletion-witness:{anchor}"))
    } else {
        None
    }
}

fn coloring_respects_all_edges_except(
    adjacency: &BTreeMap<String, BTreeSet<String>>,
    coloring: &Coloring,
    skip_left: &str,
    skip_right: &str,
) -> bool {
    for (left, neighbors) in adjacency {
        for right in neighbors {
            if left >= right {
                continue;
            }
            let skipped = (left == skip_left && right == skip_right)
                || (left == skip_right && right == skip_left);
            if skipped {
                continue;
            }
            match (coloring.get(left), coloring.get(right)) {
                (Some(left_color), Some(right_color)) if left_color != right_color => {}
                _ => return false,
            }
        }
    }
    true
}

fn adjacency_of(graph: &GraphVersion) -> BTreeMap<String, BTreeSet<String>> {
    let mut adjacency: BTreeMap<String, BTreeSet<String>> = graph
        .vertices()
        .iter()
        .map(|vertex| (vertex.vertex_label().to_string(), BTreeSet::new()))
        .collect();
    for edge in graph.edges() {
        let (left, right) = edge.endpoints();
        adjacency
            .entry(left.to_string())
            .or_default()
            .insert(right.to_string());
        adjacency
            .entry(right.to_string())
            .or_default()
            .insert(left.to_string());
    }
    adjacency
}

fn normalized_edges(adjacency: &BTreeMap<String, BTreeSet<String>>) -> Vec<(String, String)> {
    let mut edges = Vec::new();
    for (left, neighbors) in adjacency {
        for right in neighbors {
            if left < right {
                edges.push((left.clone(), right.clone()));
            }
        }
    }
    edges
}

fn rebuild_without_vertex(graph: &GraphVersion, target: &str) -> GraphVersion {
    rebuild(
        graph,
        &format!("transfer-delete-vertex-{target}"),
        |vertex| vertex != target,
        |_, _| true,
    )
}

fn rebuild_without_edge(graph: &GraphVersion, left: &str, right: &str) -> GraphVersion {
    rebuild(
        graph,
        &format!("transfer-delete-edge-{left}-{right}"),
        |_| true,
        |edge_left, edge_right| {
            let is_target = (edge_left == left && edge_right == right)
                || (edge_left == right && edge_right == left);
            !is_target
        },
    )
}

fn rebuild(
    graph: &GraphVersion,
    version_id: &str,
    keep_vertex: impl Fn(&str) -> bool,
    keep_edge: impl Fn(&str, &str) -> bool,
) -> GraphVersion {
    let mut builder = GraphVersion::builder(graph.parent_artifacts()[0].clone(), version_id);
    for vertex in graph.vertices() {
        if keep_vertex(vertex.vertex_label()) {
            builder = builder
                .with_vertex(vertex.vertex_label())
                .expect("vertex shape admits");
        }
    }
    for edge in graph.edges() {
        let (left, right) = edge.endpoints();
        if keep_vertex(left) && keep_vertex(right) && keep_edge(left, right) {
            builder = builder
                .with_undirected_edge(left, right)
                .expect("edge shape admits");
        }
    }
    builder.finish().expect("mutated graph shape admits")
}
