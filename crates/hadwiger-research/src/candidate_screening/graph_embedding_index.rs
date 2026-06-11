use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::domain_artifacts::GraphVersion;
use crate::mathematical_verification::{ExactGraphEmbedding, ExactPoint2, ExactRational};

use super::{CandidateScreeningError, CandidateScreeningInvariantFamily};

#[derive(Clone, Debug)]
pub(crate) struct ScreeningFiniteGraphIndex {
    vertices: Vec<String>,
    vertex_index: BTreeMap<String, usize>,
    edges: BTreeSet<(String, String)>,
    adjacency: Vec<Vec<bool>>,
}

impl ScreeningFiniteGraphIndex {
    pub(crate) fn from_graph_version(graph: &GraphVersion) -> Self {
        let vertices = graph
            .vertices()
            .iter()
            .map(|vertex| vertex.vertex_label().to_string())
            .collect::<Vec<_>>();
        let vertex_index = vertices
            .iter()
            .enumerate()
            .map(|(index, vertex)| (vertex.clone(), index))
            .collect::<BTreeMap<_, _>>();
        let mut edges = BTreeSet::new();
        let mut adjacency = vec![vec![false; vertices.len()]; vertices.len()];
        for edge in graph.edges() {
            let normalized = normalize_edge(edge.endpoints().0, edge.endpoints().1);
            let Some(&left) = vertex_index.get(&normalized.0) else {
                continue;
            };
            let Some(&right) = vertex_index.get(&normalized.1) else {
                continue;
            };
            edges.insert(normalized);
            adjacency[left][right] = true;
            adjacency[right][left] = true;
        }
        Self {
            vertices,
            vertex_index,
            edges,
            adjacency,
        }
    }

    pub(crate) fn vertices(&self) -> &[String] {
        &self.vertices
    }

    pub(crate) fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub(crate) fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub(crate) fn edges(&self) -> &BTreeSet<(String, String)> {
        &self.edges
    }

    pub(crate) fn has_vertex(&self, vertex: &str) -> bool {
        self.vertex_index.contains_key(vertex)
    }

    pub(crate) fn is_adjacent_label(&self, left: &str, right: &str) -> bool {
        self.edges.contains(&normalize_edge(left, right))
    }

    pub(crate) fn require_vertex(
        &self,
        vertex: &str,
        family: CandidateScreeningInvariantFamily,
    ) -> Result<usize, CandidateScreeningError> {
        self.vertex_index.get(vertex).copied().ok_or(
            CandidateScreeningError::CertificateReplayRejected {
                family,
                reason: "unknown_vertex",
            },
        )
    }

    pub(crate) fn degrees(&self) -> Vec<usize> {
        self.adjacency
            .iter()
            .map(|row| row.iter().filter(|entry| **entry).count())
            .collect()
    }

    pub(crate) fn neighborhood_within(
        &self,
        start: &str,
        radius: usize,
        family: CandidateScreeningInvariantFamily,
    ) -> Result<BTreeSet<String>, CandidateScreeningError> {
        let start_index = self.require_vertex(start, family)?;
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([(start_index, 0usize)]);
        while let Some((index, distance)) = queue.pop_front() {
            if distance > radius || !seen.insert(self.vertices[index].clone()) {
                continue;
            }
            if distance == radius {
                continue;
            }
            for neighbor in 0..self.vertex_count() {
                if self.adjacency[index][neighbor] {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
        Ok(seen)
    }

    pub(crate) fn preserves_edges(&self, target: &Self, mapping: &[(String, String)]) -> bool {
        let map = mapping.iter().cloned().collect::<BTreeMap<_, _>>();
        for (left, right) in &self.edges {
            let Some(mapped_left) = map.get(left) else {
                return false;
            };
            let Some(mapped_right) = map.get(right) else {
                return false;
            };
            if !target.is_adjacent_label(mapped_left, mapped_right) {
                return false;
            }
        }
        true
    }

    pub(crate) fn permutation_preserves_edges(&self, mapping: &[(String, String)]) -> bool {
        self.mapping_is_vertex_permutation(mapping) && self.preserves_edges(self, mapping)
    }

    pub(crate) fn mapping_is_injective_to_target(
        &self,
        target: &Self,
        mapping: &[(String, String)],
    ) -> bool {
        let mut domain = BTreeSet::new();
        let mut codomain = BTreeSet::new();
        for (left, right) in mapping {
            if !self.has_vertex(left) || !target.has_vertex(right) {
                return false;
            }
            if !domain.insert(left) || !codomain.insert(right) {
                return false;
            }
        }
        true
    }

    pub(crate) fn wl_fingerprint(&self, rounds: usize) -> String {
        let mut colors = self
            .degrees()
            .into_iter()
            .map(|degree| degree.to_string())
            .collect::<Vec<_>>();
        for _ in 0..rounds {
            let mut next = Vec::new();
            for index in 0..self.vertex_count() {
                let mut neighbor_colors = (0..self.vertex_count())
                    .filter(|neighbor| self.adjacency[index][*neighbor])
                    .map(|neighbor| colors[neighbor].clone())
                    .collect::<Vec<_>>();
                neighbor_colors.sort();
                next.push(format!("{}({})", colors[index], neighbor_colors.join(",")));
            }
            let mut palette = next.clone();
            palette.sort();
            palette.dedup();
            let ids = palette
                .into_iter()
                .enumerate()
                .map(|(id, color)| (color, id))
                .collect::<BTreeMap<_, _>>();
            colors = next
                .into_iter()
                .map(|color| ids.get(&color).copied().unwrap_or(0).to_string())
                .collect();
        }
        let mut summary = colors;
        summary.sort();
        format!(
            "v={};e={};degrees={:?};wl{}={}",
            self.vertex_count(),
            self.edge_count(),
            {
                let mut degrees = self.degrees();
                degrees.sort();
                degrees
            },
            rounds,
            summary.join(".")
        )
    }

    fn mapping_is_vertex_permutation(&self, mapping: &[(String, String)]) -> bool {
        if mapping.len() != self.vertex_count() {
            return false;
        }
        self.mapping_is_injective_to_target(self, mapping)
            && mapping.iter().all(|(_, right)| self.has_vertex(right))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ScreeningExactEmbeddingIndex<'a> {
    graph: ScreeningFiniteGraphIndex,
    embedding: &'a ExactGraphEmbedding,
}

impl<'a> ScreeningExactEmbeddingIndex<'a> {
    pub(crate) fn new(graph: &GraphVersion, embedding: &'a ExactGraphEmbedding) -> Self {
        Self {
            graph: ScreeningFiniteGraphIndex::from_graph_version(graph),
            embedding,
        }
    }

    pub(crate) fn graph(&self) -> &ScreeningFiniteGraphIndex {
        &self.graph
    }

    pub(crate) fn point(
        &self,
        vertex: &str,
        family: CandidateScreeningInvariantFamily,
    ) -> Result<&'a ExactPoint2, CandidateScreeningError> {
        self.graph.require_vertex(vertex, family)?;
        self.embedding.coordinate(vertex).ok_or(
            CandidateScreeningError::CertificateReplayRejected {
                family,
                reason: "missing_embedding_coordinate",
            },
        )
    }

    pub(crate) fn squared_distance(
        &self,
        left: &str,
        right: &str,
        family: CandidateScreeningInvariantFamily,
    ) -> Result<ExactRational, CandidateScreeningError> {
        Ok(self
            .point(left, family)?
            .squared_distance(self.point(right, family)?))
    }

    pub(crate) fn all_graph_edges_are_unit(
        &self,
        family: CandidateScreeningInvariantFamily,
    ) -> Result<bool, CandidateScreeningError> {
        for (left, right) in self.graph.edges() {
            if self.squared_distance(left, right, family)? != ExactRational::integer(1) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn stable_token(&self) -> String {
        let mut token = format!("embedding={}", self.embedding.embedding_id());
        for (vertex, point) in self.embedding.coordinates() {
            token.push_str(&format!(":{vertex}@{}", point.stable_token()));
        }
        token
    }
}

pub(crate) fn normalize_edge(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    }
}
