//! Budgeted criticality drill over the exact Heule-510 frontier seed.
//!
//! Hypothesis under test: the seed graph is not edge-critical for
//! non-4-colorability, and the frontier pressure projection predicts where
//! removable (non-critical) edges concentrate. Every mutation is replayed
//! through the real varisat lane; a `SatModelVerified` posture proves the
//! deleted element was critical, while an UNSAT posture on the mutated graph
//! marks a candidate for a strictly smaller 5-chromatic unit-distance graph.
//!
//! Environment knobs:
//! - `DRILL_MODE`: `vertices`, `edges`, or `both` (default `both`)
//! - `DRILL_BUDGET`: maximum number of SAT solves (default 24)
//! - `DRILL_ORDER`: `low-pressure-first` (default) or `high-pressure-first`

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, import_frontier_graph_seed_checked,
    verify_k_colorability_checked, ColorabilityVerificationPosture, FrontierGraphSeedImport,
    GraphVersion, HadwigerCanonicalArtifact, HadwigerResearchHandle,
    HadwigerResearchOperatingContext,
};

const WL_REFINEMENT_ROUNDS: usize = 6;

fn main() {
    let mode = std::env::var("DRILL_MODE").unwrap_or_else(|_| "both".to_string());
    let budget = std::env::var("DRILL_BUDGET")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(24);
    let low_pressure_first = std::env::var("DRILL_ORDER").as_deref() != Ok("high-pressure-first");

    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("Heule 510 imports");
    let graph = imported.graph_version();
    let index = LocalIndex::from_graph(graph);
    println!(
        "baseline vertices={} edges={} digest={} wl_vertex_classes={} budget={} mode={}",
        graph.vertex_count(),
        graph.edge_count(),
        graph.artifact_digest().stable_token(),
        index.class_count(),
        budget,
        mode
    );

    let mut remaining = budget;
    if mode == "vertices" || mode == "both" {
        remaining = drill_vertices(&handle, graph, &index, remaining, low_pressure_first);
    }
    if mode == "edges" || mode == "both" {
        drill_edges(&handle, graph, &index, remaining, low_pressure_first);
    }
}

fn drill_vertices(
    handle: &HadwigerResearchHandle,
    graph: &GraphVersion,
    index: &LocalIndex,
    budget: usize,
    low_pressure_first: bool,
) -> usize {
    let mut representatives: BTreeMap<u64, (usize, String)> = BTreeMap::new();
    for vertex in index.vertices() {
        let class = index.vertex_class(vertex);
        let pressure = index.vertex_pressure(vertex);
        let entry = representatives
            .entry(class)
            .or_insert_with(|| (pressure, vertex.to_string()));
        if pressure < entry.0 {
            *entry = (pressure, vertex.to_string());
        }
    }
    let mut ranked: Vec<(usize, u64, String)> = representatives
        .into_iter()
        .map(|(class, (pressure, vertex))| (pressure, class, vertex))
        .collect();
    ranked.sort();
    if !low_pressure_first {
        ranked.reverse();
    }
    let mut remaining = budget;
    for (pressure, class, vertex) in ranked {
        if remaining == 0 {
            break;
        }
        remaining -= 1;
        let mutated = rebuild_without_vertex(graph, &vertex);
        report(
            handle,
            &mutated,
            format!("vertex-deletion:{vertex}"),
            class,
            pressure,
            index.vertex_class_size(class),
        );
    }
    remaining
}

fn drill_edges(
    handle: &HadwigerResearchHandle,
    graph: &GraphVersion,
    index: &LocalIndex,
    budget: usize,
    low_pressure_first: bool,
) {
    let mut representatives: BTreeMap<(u64, u64, usize), EdgeClassPick> = BTreeMap::new();
    for (left, right) in index.normalized_edges() {
        let class = index.edge_class(left, right);
        let pressure = index.vertex_pressure(left) + index.vertex_pressure(right);
        let entry = representatives
            .entry(class)
            .or_insert_with(|| EdgeClassPick {
                pressure,
                left: left.to_string(),
                right: right.to_string(),
                class_size: 0,
            });
        entry.class_size += 1;
        if pressure < entry.pressure {
            entry.pressure = pressure;
            entry.left = left.to_string();
            entry.right = right.to_string();
        }
    }
    let mut ranked: Vec<(usize, (u64, u64, usize), EdgeClassPick)> = representatives
        .into_iter()
        .map(|(class, pick)| (pick.pressure, class, pick))
        .collect();
    ranked.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    if !low_pressure_first {
        ranked.reverse();
    }
    let mut remaining = budget;
    for (pressure, class, pick) in ranked {
        if remaining == 0 {
            break;
        }
        remaining -= 1;
        let mutated = rebuild_without_edge(graph, &pick.left, &pick.right);
        report(
            handle,
            &mutated,
            format!("edge-deletion:{}-{}", pick.left, pick.right),
            class.0 ^ class.1.rotate_left(17) ^ (class.2 as u64),
            pressure,
            pick.class_size,
        );
    }
}

struct EdgeClassPick {
    pressure: usize,
    left: String,
    right: String,
    class_size: usize,
}

fn report(
    handle: &HadwigerResearchHandle,
    mutated: &GraphVersion,
    label: String,
    class: u64,
    pressure: usize,
    class_size: usize,
) {
    let started = Instant::now();
    let checked = verify_k_colorability_checked(handle, mutated, 4)
        .expect("colorability checker returns a posture");
    let posture = checked.colorability_verification().posture();
    let finding = match posture {
        ColorabilityVerificationPosture::SatModelVerified => "CRITICAL",
        ColorabilityVerificationPosture::UnsatVerified
        | ColorabilityVerificationPosture::UnsupportedCertificateBudget => "REMOVABLE-CANDIDATE",
        _ => "UNDETERMINED",
    };
    println!(
        "mutation={} class={:016x} class_size={} pressure={} posture={:?} finding={} vertices={} edges={} seconds={:.1} digest={}",
        label,
        class,
        class_size,
        pressure,
        posture,
        finding,
        mutated.vertex_count(),
        mutated.edge_count(),
        started.elapsed().as_secs_f64(),
        mutated.artifact_digest().stable_token()
    );
}

fn rebuild_without_vertex(graph: &GraphVersion, target: &str) -> GraphVersion {
    rebuild(
        graph,
        &format!("drill-delete-vertex-{target}"),
        |vertex| vertex != target,
        |_, _| true,
    )
}

fn rebuild_without_edge(graph: &GraphVersion, left: &str, right: &str) -> GraphVersion {
    rebuild(
        graph,
        &format!("drill-delete-edge-{left}-{right}"),
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

struct LocalIndex {
    vertices: Vec<String>,
    adjacency: BTreeMap<String, BTreeSet<String>>,
    triangle_counts: BTreeMap<String, usize>,
    wl_classes: BTreeMap<String, u64>,
    class_sizes: BTreeMap<u64, usize>,
}

impl LocalIndex {
    fn from_graph(graph: &GraphVersion) -> Self {
        let vertices: Vec<String> = graph
            .vertices()
            .iter()
            .map(|vertex| vertex.vertex_label().to_string())
            .collect();
        let mut adjacency: BTreeMap<String, BTreeSet<String>> = vertices
            .iter()
            .map(|vertex| (vertex.clone(), BTreeSet::new()))
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
        let triangle_counts = triangle_counts(&adjacency);
        let wl_classes = weisfeiler_leman_classes(&vertices, &adjacency);
        let mut class_sizes = BTreeMap::new();
        for class in wl_classes.values() {
            *class_sizes.entry(*class).or_insert(0) += 1;
        }
        Self {
            vertices,
            adjacency,
            triangle_counts,
            wl_classes,
            class_sizes,
        }
    }

    fn vertices(&self) -> impl Iterator<Item = &str> {
        self.vertices.iter().map(String::as_str)
    }

    fn normalized_edges(&self) -> impl Iterator<Item = (&str, &str)> {
        self.adjacency.iter().flat_map(|(left, neighbors)| {
            neighbors
                .iter()
                .filter(move |right| left < *right)
                .map(move |right| (left.as_str(), right.as_str()))
        })
    }

    fn vertex_pressure(&self, vertex: &str) -> usize {
        let degree = self.adjacency.get(vertex).map_or(0, BTreeSet::len);
        degree * 100 + self.triangle_counts.get(vertex).copied().unwrap_or(0)
    }

    fn vertex_class(&self, vertex: &str) -> u64 {
        self.wl_classes.get(vertex).copied().unwrap_or(0)
    }

    fn vertex_class_size(&self, class: u64) -> usize {
        self.class_sizes.get(&class).copied().unwrap_or(0)
    }

    fn edge_class(&self, left: &str, right: &str) -> (u64, u64, usize) {
        let left_class = self.vertex_class(left);
        let right_class = self.vertex_class(right);
        let common = self
            .adjacency
            .get(left)
            .and_then(|left_neighbors| {
                self.adjacency
                    .get(right)
                    .map(|right_neighbors| left_neighbors.intersection(right_neighbors).count())
            })
            .unwrap_or(0);
        (
            left_class.min(right_class),
            left_class.max(right_class),
            common,
        )
    }

    fn class_count(&self) -> usize {
        self.class_sizes.len()
    }
}

fn weisfeiler_leman_classes(
    vertices: &[String],
    adjacency: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, u64> {
    let mut colors: BTreeMap<String, u64> = vertices
        .iter()
        .map(|vertex| {
            (
                vertex.clone(),
                adjacency.get(vertex).map_or(0, BTreeSet::len) as u64,
            )
        })
        .collect();
    for _ in 0..WL_REFINEMENT_ROUNDS {
        let mut signatures: BTreeMap<String, (u64, Vec<u64>)> = BTreeMap::new();
        for vertex in vertices {
            let mut neighbor_colors: Vec<u64> = adjacency
                .get(vertex)
                .map(|neighbors| {
                    neighbors
                        .iter()
                        .map(|neighbor| colors.get(neighbor).copied().unwrap_or(0))
                        .collect()
                })
                .unwrap_or_default();
            neighbor_colors.sort_unstable();
            signatures.insert(
                vertex.clone(),
                (colors.get(vertex).copied().unwrap_or(0), neighbor_colors),
            );
        }
        let mut canonical: BTreeMap<(u64, Vec<u64>), u64> = BTreeMap::new();
        for signature in signatures.values() {
            let next = canonical.len() as u64;
            canonical.entry(signature.clone()).or_insert(next);
        }
        for vertex in vertices {
            colors.insert(vertex.clone(), canonical[&signatures[vertex]]);
        }
    }
    colors
}

fn triangle_counts(adjacency: &BTreeMap<String, BTreeSet<String>>) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> =
        adjacency.keys().map(|vertex| (vertex.clone(), 0)).collect();
    for (left, neighbors) in adjacency {
        for right in neighbors {
            if left >= right {
                continue;
            }
            let common = adjacency
                .get(right)
                .map(|right_neighbors| neighbors.intersection(right_neighbors).count())
                .unwrap_or(0);
            *counts.entry(left.clone()).or_insert(0) += common;
            *counts.entry(right.clone()).or_insert(0) += common;
        }
    }
    for count in counts.values_mut() {
        *count /= 2;
    }
    counts
}
