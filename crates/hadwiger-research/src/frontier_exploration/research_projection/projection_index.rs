use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::GraphVersion;

use super::projection_types::{
    FrontierDegreeBucket, FrontierPressureHaloMotif, FrontierPressureSatellite,
    FrontierPressureVertex,
};

pub(super) struct ProjectionIndex {
    pub(super) vertices: Vec<String>,
    pub(super) adjacency: BTreeMap<String, BTreeSet<String>>,
    triangle_counts: BTreeMap<String, usize>,
}

impl ProjectionIndex {
    pub(super) fn from_graph(graph: &GraphVersion) -> Self {
        let vertices = graph
            .vertices()
            .iter()
            .map(|vertex| vertex.vertex_label().to_string())
            .collect::<Vec<_>>();
        let mut adjacency = vertices
            .iter()
            .map(|vertex| (vertex.clone(), BTreeSet::new()))
            .collect::<BTreeMap<_, _>>();
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
        let triangle_counts = triangle_counts(&vertices, &adjacency);
        Self {
            vertices,
            adjacency,
            triangle_counts,
        }
    }

    pub(super) fn degree(&self, vertex: &str) -> usize {
        self.adjacency.get(vertex).map_or(0, BTreeSet::len)
    }

    fn common_neighbors(&self, left: &str, right: &str) -> Vec<String> {
        let Some(left_neighbors) = self.adjacency.get(left) else {
            return Vec::new();
        };
        let Some(right_neighbors) = self.adjacency.get(right) else {
            return Vec::new();
        };
        left_neighbors
            .intersection(right_neighbors)
            .cloned()
            .collect()
    }

    fn adjacent(&self, left: &str, right: &str) -> bool {
        self.adjacency
            .get(left)
            .is_some_and(|neighbors| neighbors.contains(right))
    }
}

pub(super) fn degree_histogram(index: &ProjectionIndex) -> Vec<FrontierDegreeBucket> {
    let mut counts = BTreeMap::new();
    for vertex in &index.vertices {
        *counts.entry(index.degree(vertex)).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .map(|(degree, vertex_count)| FrontierDegreeBucket::new(degree, vertex_count))
        .collect()
}

pub(super) fn pressure_vertices(index: &ProjectionIndex) -> Vec<FrontierPressureVertex> {
    let mut rows = index
        .vertices
        .iter()
        .map(|vertex| {
            let degree = index.degree(vertex);
            let triangle_count = *index.triangle_counts.get(vertex).unwrap_or(&0);
            FrontierPressureVertex::new(
                vertex.clone(),
                degree,
                triangle_count,
                degree * 100 + triangle_count,
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .pressure_score()
            .cmp(&left.pressure_score())
            .then_with(|| left.vertex_label().cmp(right.vertex_label()))
    });
    rows
}

pub(super) fn pressure_halo_motifs(index: &ProjectionIndex) -> Vec<FrontierPressureHaloMotif> {
    let max_degree = index
        .vertices
        .iter()
        .map(|vertex| index.degree(vertex))
        .max()
        .unwrap_or(0);
    let satellite_threshold = (max_degree * 2).div_ceil(3);
    let mut motifs = Vec::new();
    for hub in &index.vertices {
        if index.degree(hub) != max_degree || max_degree < 12 {
            continue;
        }
        let mut satellites = Vec::new();
        for candidate in &index.vertices {
            if candidate == hub || index.adjacent(hub, candidate) {
                continue;
            }
            let degree = index.degree(candidate);
            let common_spokes = index.common_neighbors(hub, candidate);
            if degree >= satellite_threshold && common_spokes.len() >= 2 {
                satellites.push(FrontierPressureSatellite::new(
                    candidate.clone(),
                    degree,
                    common_spokes,
                ));
            }
        }
        satellites.sort_by(|left, right| {
            right
                .degree()
                .cmp(&left.degree())
                .then_with(|| left.vertex_label().cmp(right.vertex_label()))
        });
        if satellites.len() >= 3 {
            satellites.truncate(12);
            let pressure_score = index.degree(hub)
                * satellites
                    .iter()
                    .map(FrontierPressureSatellite::degree)
                    .sum::<usize>();
            motifs.push(FrontierPressureHaloMotif::new(
                hub.clone(),
                satellites,
                pressure_score,
                format!("pressure-halo:{hub}:degree-{max_degree}"),
            ));
        }
    }
    motifs.sort_by(|left, right| {
        right
            .pressure_score()
            .cmp(&left.pressure_score())
            .then_with(|| left.hub_vertex().cmp(right.hub_vertex()))
    });
    motifs
}

fn triangle_counts(
    vertices: &[String],
    adjacency: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, usize> {
    let mut counts = vertices
        .iter()
        .map(|vertex| (vertex.clone(), 0))
        .collect::<BTreeMap<_, _>>();
    for (left_index, left) in vertices.iter().enumerate() {
        for (right_index, right) in vertices.iter().enumerate().skip(left_index + 1) {
            if !has_edge(adjacency, left, right) {
                continue;
            }
            for third in vertices.iter().skip(right_index + 1) {
                if has_edge(adjacency, left, third) && has_edge(adjacency, right, third) {
                    *counts.entry(left.clone()).or_insert(0) += 1;
                    *counts.entry(right.clone()).or_insert(0) += 1;
                    *counts.entry(third.clone()).or_insert(0) += 1;
                }
            }
        }
    }
    counts
}

fn has_edge(adjacency: &BTreeMap<String, BTreeSet<String>>, left: &str, right: &str) -> bool {
    adjacency
        .get(left)
        .is_some_and(|neighbors| neighbors.contains(right))
}
