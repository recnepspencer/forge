use std::collections::{BTreeMap, BTreeSet};

use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_mwis_exact::exact_mwis;

pub(super) struct LocalGraph {
    pub(super) vertices: Vec<usize>,
    pub(super) adjacency: Vec<Vec<bool>>,
}

impl LocalGraph {
    pub(super) fn new(global_adjacency: &[BitWords], vertices: Vec<usize>) -> Self {
        let mut adjacency = vec![vec![false; vertices.len()]; vertices.len()];
        for (left_local, left_global) in vertices.iter().enumerate() {
            for (right_local, right_global) in vertices.iter().enumerate().skip(left_local + 1) {
                if has_bit(&global_adjacency[*left_global], *right_global) {
                    adjacency[left_local][right_local] = true;
                    adjacency[right_local][left_local] = true;
                }
            }
        }
        Self {
            vertices,
            adjacency,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.vertices.len()
    }

    pub(super) fn edge_count(&self) -> usize {
        let mut count = 0;
        for left in 0..self.len() {
            for right in left + 1..self.len() {
                count += usize::from(self.adjacency[left][right]);
            }
        }
        count
    }

    pub(super) fn neighbors(&self, vertex: usize) -> Vec<usize> {
        (0..self.len())
            .filter(|other| self.adjacency[vertex][*other])
            .collect()
    }
}

pub(super) struct DegreeStats {
    pub(super) min: usize,
    pub(super) median: usize,
    pub(super) max: usize,
    pub(super) degeneracy: usize,
}

pub(super) fn degree_stats(graph: &LocalGraph) -> DegreeStats {
    let mut degrees = (0..graph.len())
        .map(|vertex| graph.neighbors(vertex).len())
        .collect::<Vec<_>>();
    degrees.sort_unstable();
    DegreeStats {
        min: degrees[0],
        median: degrees[degrees.len() / 2],
        max: degrees[degrees.len() - 1],
        degeneracy: degeneracy(graph),
    }
}

fn degeneracy(graph: &LocalGraph) -> usize {
    let mut alive = vec![true; graph.len()];
    let mut degree = (0..graph.len())
        .map(|vertex| graph.neighbors(vertex).len())
        .collect::<Vec<_>>();
    let mut result = 0;
    for _ in 0..graph.len() {
        let vertex = (0..graph.len())
            .filter(|candidate| alive[*candidate])
            .min_by_key(|candidate| (degree[*candidate], *candidate))
            .unwrap();
        result = result.max(degree[vertex]);
        alive[vertex] = false;
        for neighbor in graph.neighbors(vertex) {
            if alive[neighbor] {
                degree[neighbor] -= 1;
            }
        }
    }
    result
}

pub(super) struct BlockStats {
    pub(super) articulation_count: usize,
    pub(super) block_count: usize,
    pub(super) largest_block_size: usize,
}

pub(super) fn biconnected_stats(graph: &LocalGraph) -> BlockStats {
    let mut dfs = BiconnectedDfs::new(graph);
    dfs.visit(0, None);
    BlockStats {
        articulation_count: dfs.articulation.iter().filter(|flag| **flag).count(),
        block_count: dfs.block_sizes.len(),
        largest_block_size: dfs.block_sizes.into_iter().max().unwrap_or(0),
    }
}

struct BiconnectedDfs<'a> {
    graph: &'a LocalGraph,
    time: usize,
    discovery: Vec<usize>,
    low: Vec<usize>,
    articulation: Vec<bool>,
    edge_stack: Vec<(usize, usize)>,
    block_sizes: Vec<usize>,
}

impl<'a> BiconnectedDfs<'a> {
    fn new(graph: &'a LocalGraph) -> Self {
        Self {
            graph,
            time: 0,
            discovery: vec![0; graph.len()],
            low: vec![0; graph.len()],
            articulation: vec![false; graph.len()],
            edge_stack: Vec::new(),
            block_sizes: Vec::new(),
        }
    }

    fn visit(&mut self, vertex: usize, parent: Option<usize>) {
        self.time += 1;
        self.discovery[vertex] = self.time;
        self.low[vertex] = self.time;
        let mut child_count = 0;
        for neighbor in self.graph.neighbors(vertex) {
            if self.discovery[neighbor] == 0 {
                child_count += 1;
                self.edge_stack.push((vertex, neighbor));
                self.visit(neighbor, Some(vertex));
                self.low[vertex] = self.low[vertex].min(self.low[neighbor]);
                if (parent.is_none() && child_count > 1)
                    || (parent.is_some() && self.low[neighbor] >= self.discovery[vertex])
                {
                    self.articulation[vertex] = true;
                    self.flush_block(vertex, neighbor);
                }
            } else if Some(neighbor) != parent && self.discovery[neighbor] < self.discovery[vertex]
            {
                self.low[vertex] = self.low[vertex].min(self.discovery[neighbor]);
                self.edge_stack.push((vertex, neighbor));
            }
        }
        if parent.is_none() && !self.edge_stack.is_empty() {
            let size = self
                .edge_stack
                .drain(..)
                .flat_map(|(left, right)| [left, right])
                .collect::<BTreeSet<_>>()
                .len();
            self.block_sizes.push(size);
        }
    }

    fn flush_block(&mut self, stop_left: usize, stop_right: usize) {
        let mut block = BTreeSet::new();
        while let Some((left, right)) = self.edge_stack.pop() {
            block.insert(left);
            block.insert(right);
            if left == stop_left && right == stop_right {
                break;
            }
        }
        self.block_sizes.push(block.len());
    }
}

pub(super) fn simplicial_vertex_count(graph: &LocalGraph) -> usize {
    (0..graph.len())
        .filter(|vertex| {
            let neighbors = graph.neighbors(*vertex);
            neighbors.iter().enumerate().all(|(index, left)| {
                neighbors[index + 1..]
                    .iter()
                    .all(|right| graph.adjacency[*left][*right])
            })
        })
        .count()
}

pub(super) struct TwinStats {
    pub(super) class_count: usize,
    pub(super) largest_class_size: usize,
    pub(super) reducible_vertex_count: usize,
}

pub(super) fn open_twin_stats(graph: &LocalGraph) -> TwinStats {
    let mut classes: BTreeMap<Vec<usize>, Vec<usize>> = BTreeMap::new();
    for vertex in 0..graph.len() {
        classes
            .entry(graph.neighbors(vertex))
            .or_default()
            .push(vertex);
    }
    let non_singletons = classes
        .values()
        .filter(|class| class.len() > 1)
        .collect::<Vec<_>>();
    TwinStats {
        class_count: non_singletons.len(),
        largest_class_size: non_singletons
            .iter()
            .map(|class| class.len())
            .max()
            .unwrap_or(1),
        reducible_vertex_count: non_singletons.iter().map(|class| class.len() - 1).sum(),
    }
}

#[derive(Clone, Copy)]
pub(super) enum EliminationMode {
    MinDegree,
    MinFill,
}

pub(super) struct EliminationReport {
    pub(super) width: usize,
    pub(super) fill_edges: usize,
}

pub(super) fn elimination_width(graph: &LocalGraph, mode: EliminationMode) -> EliminationReport {
    let mut adjacency = graph.adjacency.clone();
    let mut alive = vec![true; graph.len()];
    let mut width = 0;
    let mut fill_edges = 0;
    for _ in 0..graph.len() {
        let vertex = (0..graph.len())
            .filter(|candidate| alive[*candidate])
            .min_by_key(|candidate| elimination_score(&adjacency, &alive, *candidate, mode))
            .unwrap();
        let neighbors = alive_neighbors(&adjacency, &alive, vertex);
        width = width.max(neighbors.len());
        for (index, left) in neighbors.iter().enumerate() {
            for right in &neighbors[index + 1..] {
                if !adjacency[*left][*right] {
                    adjacency[*left][*right] = true;
                    adjacency[*right][*left] = true;
                    fill_edges += 1;
                }
            }
        }
        alive[vertex] = false;
    }
    EliminationReport { width, fill_edges }
}

fn elimination_score(
    adjacency: &[Vec<bool>],
    alive: &[bool],
    vertex: usize,
    mode: EliminationMode,
) -> (usize, usize, usize) {
    let neighbors = alive_neighbors(adjacency, alive, vertex);
    let fill = match mode {
        EliminationMode::MinDegree => 0,
        EliminationMode::MinFill => fill_count(adjacency, &neighbors),
    };
    (fill, neighbors.len(), vertex)
}

fn alive_neighbors(adjacency: &[Vec<bool>], alive: &[bool], vertex: usize) -> Vec<usize> {
    (0..alive.len())
        .filter(|other| alive[*other] && adjacency[vertex][*other])
        .collect()
}

fn fill_count(adjacency: &[Vec<bool>], neighbors: &[usize]) -> usize {
    let mut count = 0;
    for (index, left) in neighbors.iter().enumerate() {
        for right in &neighbors[index + 1..] {
            count += usize::from(!adjacency[*left][*right]);
        }
    }
    count
}

pub(super) fn exact_small_component_weight(
    adjacency: &[BitWords],
    weights: &[i128],
    components: &[Vec<usize>],
) -> i128 {
    components
        .iter()
        .skip(1)
        .map(|component| exact_mwis(adjacency, weights, component).0)
        .sum()
}

pub(super) fn connected_components(
    adjacency: &[BitWords],
    candidates: &[usize],
) -> Vec<Vec<usize>> {
    let mut remaining = candidates.to_vec();
    remaining.sort_unstable();
    let mut components = Vec::new();
    while let Some(start) = remaining.pop() {
        let mut stack = vec![start];
        let mut component = Vec::new();
        while let Some(vertex) = stack.pop() {
            component.push(vertex);
            let mut index = 0;
            while index < remaining.len() {
                if has_bit(&adjacency[vertex], remaining[index]) {
                    stack.push(remaining.swap_remove(index));
                } else {
                    index += 1;
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort_by(|left, right| right.len().cmp(&left.len()).then_with(|| left.cmp(right)));
    components
}
