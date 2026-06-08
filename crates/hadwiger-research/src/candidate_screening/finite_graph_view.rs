use std::collections::BTreeMap;

use crate::domain_artifacts::GraphVersion;

use super::CandidateScreeningError;

#[derive(Clone, Debug)]
pub(crate) struct FiniteGraphView {
    vertices: Vec<String>,
    adjacency: Vec<Vec<bool>>,
}

impl FiniteGraphView {
    pub(crate) fn from_graph_version(graph: &GraphVersion) -> Self {
        let vertices = graph
            .vertices()
            .iter()
            .map(|vertex| vertex.vertex_label().to_string())
            .collect::<Vec<_>>();
        let index = vertices
            .iter()
            .enumerate()
            .map(|(index, label)| (label.clone(), index))
            .collect::<BTreeMap<_, _>>();
        let mut adjacency = vec![vec![false; vertices.len()]; vertices.len()];
        for edge in graph.edges() {
            let (left, right) = edge.endpoints();
            let Some(&left_index) = index.get(left) else {
                continue;
            };
            let Some(&right_index) = index.get(right) else {
                continue;
            };
            adjacency[left_index][right_index] = true;
            adjacency[right_index][left_index] = true;
        }
        Self {
            vertices,
            adjacency,
        }
    }

    pub(crate) fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub(crate) fn vertices(&self) -> &[String] {
        &self.vertices
    }

    pub(crate) fn edge_count(&self) -> usize {
        self.adjacency
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(column_index, adjacent)| row_index < *column_index && **adjacent)
                    .count()
            })
            .sum()
    }

    pub(crate) fn maximum_degree(&self) -> usize {
        self.degrees().into_iter().max().unwrap_or(0)
    }

    pub(crate) fn degrees(&self) -> Vec<usize> {
        self.adjacency
            .iter()
            .map(|row| row.iter().filter(|adjacent| **adjacent).count())
            .collect()
    }

    pub(crate) fn require_subset_budget(
        &self,
        exact_limit: usize,
    ) -> Result<(), CandidateScreeningError> {
        if self.vertex_count() <= exact_limit {
            Ok(())
        } else {
            Err(CandidateScreeningError::GraphScreeningBudgetExceeded {
                vertex_count: self.vertex_count(),
                exact_limit,
            })
        }
    }

    pub(crate) fn clique_number(&self) -> usize {
        self.maximum_subset_size(true)
    }

    pub(crate) fn maximum_clique_witness(&self) -> Vec<usize> {
        let mut best = Vec::new();
        for subset in self.subsets() {
            if subset.len() > best.len() && self.subset_pair_relation_holds(&subset, true) {
                best = subset;
            }
        }
        best
    }

    pub(crate) fn independence_number(&self) -> usize {
        self.maximum_subset_size(false)
    }

    pub(crate) fn max_hall_ratio_witness(&self) -> (usize, usize) {
        let mut best = (0, 1);
        for subset in self.subsets() {
            let alpha = self.independence_number_for_subset(&subset).max(1);
            if subset.len() * best.1 > best.0 * alpha {
                best = (subset.len(), alpha);
            }
        }
        best
    }

    pub(crate) fn k_core_size(&self, k: usize) -> usize {
        let mut active = vec![true; self.vertex_count()];
        loop {
            let removed = (0..self.vertex_count())
                .filter(|index| active[*index] && self.active_degree(*index, &active) < k)
                .collect::<Vec<_>>();
            if removed.is_empty() {
                break;
            }
            for index in removed {
                active[index] = false;
            }
        }
        active.into_iter().filter(|value| *value).count()
    }

    pub(crate) fn is_k_colorable(&self, color_count: usize) -> bool {
        let mut colors = vec![None; self.vertex_count()];
        self.color_vertex(0, color_count, &mut colors)
    }

    pub(crate) fn is_bipartite(&self) -> bool {
        let mut colors = vec![None; self.vertex_count()];
        for start in 0..self.vertex_count() {
            if colors[start].is_some() {
                continue;
            }
            colors[start] = Some(false);
            let mut stack = vec![start];
            while let Some(vertex) = stack.pop() {
                let next_color = !colors[vertex].unwrap_or(false);
                for neighbor in 0..self.vertex_count() {
                    if !self.adjacency[vertex][neighbor] {
                        continue;
                    }
                    match colors[neighbor] {
                        Some(color) if color == colors[vertex].unwrap_or(false) => return false,
                        Some(_) => {}
                        None => {
                            colors[neighbor] = Some(next_color);
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
        true
    }

    pub(crate) fn is_regular(&self) -> Option<usize> {
        let degrees = self.degrees();
        let first = degrees.first().copied().unwrap_or(0);
        degrees
            .iter()
            .all(|degree| *degree == first)
            .then_some(first)
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.edge_count() == self.vertex_count().saturating_sub(1) * self.vertex_count() / 2
    }

    pub(crate) fn is_adjacent(&self, left: usize, right: usize) -> bool {
        self.adjacency[left][right]
    }

    pub(crate) fn independent_sets(&self) -> Vec<Vec<usize>> {
        self.subsets()
            .into_iter()
            .filter(|subset| self.subset_pair_relation_holds(subset, false))
            .collect()
    }

    pub(crate) fn has_smaller_non_k_colorable_subgraph(&self, color_count: usize) -> bool {
        for vertex in 0..self.vertex_count() {
            if !self.without_vertex(vertex).is_k_colorable(color_count) {
                return true;
            }
        }
        for left in 0..self.vertex_count() {
            for right in (left + 1)..self.vertex_count() {
                if self.adjacency[left][right]
                    && !self.without_edge(left, right).is_k_colorable(color_count)
                {
                    return true;
                }
            }
        }
        false
    }

    fn maximum_subset_size(&self, require_edges: bool) -> usize {
        let mut best = 0;
        for subset in self.subsets() {
            if subset.len() > best && self.subset_pair_relation_holds(&subset, require_edges) {
                best = subset.len();
            }
        }
        best
    }

    fn independence_number_for_subset(&self, subset: &[usize]) -> usize {
        self.subsets_from(subset)
            .into_iter()
            .filter(|candidate| self.subset_pair_relation_holds(candidate, false))
            .map(|candidate| candidate.len())
            .max()
            .unwrap_or(0)
    }

    fn color_vertex(
        &self,
        vertex_index: usize,
        color_count: usize,
        colors: &mut [Option<usize>],
    ) -> bool {
        if vertex_index == self.vertex_count() {
            return true;
        }
        for color in 0..color_count {
            if self.can_use_color(vertex_index, color, colors) {
                colors[vertex_index] = Some(color);
                if self.color_vertex(vertex_index + 1, color_count, colors) {
                    return true;
                }
                colors[vertex_index] = None;
            }
        }
        false
    }

    fn can_use_color(&self, vertex_index: usize, color: usize, colors: &[Option<usize>]) -> bool {
        self.adjacency[vertex_index]
            .iter()
            .enumerate()
            .all(|(neighbor, adjacent)| !adjacent || colors[neighbor] != Some(color))
    }

    fn active_degree(&self, index: usize, active: &[bool]) -> usize {
        self.adjacency[index]
            .iter()
            .enumerate()
            .filter(|(candidate, adjacent)| active[*candidate] && **adjacent)
            .count()
    }

    fn subset_pair_relation_holds(&self, subset: &[usize], require_edges: bool) -> bool {
        for left in 0..subset.len() {
            for right in (left + 1)..subset.len() {
                if self.adjacency[subset[left]][subset[right]] != require_edges {
                    return false;
                }
            }
        }
        true
    }

    fn subsets(&self) -> Vec<Vec<usize>> {
        let indices = (0..self.vertex_count()).collect::<Vec<_>>();
        self.subsets_from(&indices)
    }

    fn subsets_from(&self, indices: &[usize]) -> Vec<Vec<usize>> {
        let mut subsets = Vec::new();
        for mask in 1usize..(1usize << indices.len()) {
            let mut subset = Vec::new();
            for (bit, index) in indices.iter().enumerate() {
                if (mask & (1usize << bit)) != 0 {
                    subset.push(*index);
                }
            }
            subsets.push(subset);
        }
        subsets
    }

    fn without_vertex(&self, removed_vertex: usize) -> Self {
        let kept = (0..self.vertex_count())
            .filter(|index| *index != removed_vertex)
            .collect::<Vec<_>>();
        self.induced_by(&kept)
    }

    fn without_edge(&self, left: usize, right: usize) -> Self {
        let mut next = self.clone();
        next.adjacency[left][right] = false;
        next.adjacency[right][left] = false;
        next
    }

    fn induced_by(&self, kept: &[usize]) -> Self {
        let vertices = kept
            .iter()
            .map(|index| self.vertices[*index].clone())
            .collect::<Vec<_>>();
        let mut adjacency = vec![vec![false; kept.len()]; kept.len()];
        for (new_left, old_left) in kept.iter().enumerate() {
            for (new_right, old_right) in kept.iter().enumerate() {
                adjacency[new_left][new_right] = self.adjacency[*old_left][*old_right];
            }
        }
        Self {
            vertices,
            adjacency,
        }
    }
}
