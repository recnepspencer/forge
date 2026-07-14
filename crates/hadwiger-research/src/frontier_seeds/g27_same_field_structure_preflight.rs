use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::greedy_independent_witness;
use super::g27_same_field_structure_preflight_support::{
    biconnected_stats, connected_components, degree_stats, elimination_width,
    exact_small_component_weight, open_twin_stats, simplicial_vertex_count, EliminationMode,
    LocalGraph,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_instance;

const TARGET_WEIGHT: i128 = 512_933;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27StructurePreflightStatus {
    ContinueNativeStructure,
    RetireNativeStructure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27StructurePreflightReport {
    status: G27StructurePreflightStatus,
    compatible_w_vertex_count: usize,
    component_count: usize,
    dominant_component_size: usize,
    dominant_edge_count: usize,
    exact_small_component_weight: i128,
    dominant_required_weight: i128,
    dominant_incumbent_weight: i128,
    min_degree: usize,
    median_degree: usize,
    max_degree: usize,
    degeneracy: usize,
    articulation_count: usize,
    biconnected_component_count: usize,
    largest_biconnected_component_size: usize,
    simplicial_vertex_count: usize,
    open_twin_class_count: usize,
    largest_open_twin_class_size: usize,
    min_degree_elimination_width: usize,
    min_degree_fill_edge_count: usize,
    min_fill_elimination_width: usize,
    min_fill_fill_edge_count: usize,
}

impl G27StructurePreflightReport {
    pub fn status(&self) -> G27StructurePreflightStatus {
        self.status
    }

    pub fn instance_summary(&self) -> (usize, usize, usize, usize, i128, i128, i128) {
        (
            self.compatible_w_vertex_count,
            self.component_count,
            self.dominant_component_size,
            self.dominant_edge_count,
            self.exact_small_component_weight,
            self.dominant_required_weight,
            self.dominant_incumbent_weight,
        )
    }

    pub fn degree_summary(&self) -> (usize, usize, usize, usize) {
        (
            self.min_degree,
            self.median_degree,
            self.max_degree,
            self.degeneracy,
        )
    }

    pub fn decomposition_summary(&self) -> (usize, usize, usize, usize, usize, usize) {
        (
            self.articulation_count,
            self.biconnected_component_count,
            self.largest_biconnected_component_size,
            self.simplicial_vertex_count,
            self.open_twin_class_count,
            self.largest_open_twin_class_size,
        )
    }

    pub fn width_summary(&self) -> (usize, usize, usize, usize) {
        (
            self.min_degree_elimination_width,
            self.min_degree_fill_edge_count,
            self.min_fill_elimination_width,
            self.min_fill_fill_edge_count,
        )
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

pub fn preflight_g27_same_field_structure_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27StructurePreflightReport, G27GeometricFractionalError> {
    let instance = threshold_mwis_instance(handle)?;
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let small_weight =
        exact_small_component_weight(&instance.adjacency, &instance.weights, &components);
    let dominant = components[0].clone();
    let graph = LocalGraph::new(&instance.adjacency, dominant);
    let (_, incumbent_vertices) =
        greedy_independent_witness(&instance.adjacency, &instance.weights, &graph.vertices);
    let incumbent_weight = incumbent_vertices
        .iter()
        .map(|vertex| instance.weights[*vertex])
        .sum();
    let degree_stats = degree_stats(&graph);
    let block_stats = biconnected_stats(&graph);
    let twin_stats = open_twin_stats(&graph);
    let min_degree_order = elimination_width(&graph, EliminationMode::MinDegree);
    let min_fill_order = elimination_width(&graph, EliminationMode::MinFill);
    let status = if block_stats.articulation_count == 0
        && block_stats.largest_block_size >= 470
        && min_degree_order.width > 80
        && min_fill_order.width > 80
        && twin_stats.reducible_vertex_count < 25
    {
        G27StructurePreflightStatus::RetireNativeStructure
    } else {
        G27StructurePreflightStatus::ContinueNativeStructure
    };
    Ok(G27StructurePreflightReport {
        status,
        compatible_w_vertex_count: instance.candidates.len(),
        component_count: components.len(),
        dominant_component_size: graph.len(),
        dominant_edge_count: graph.edge_count(),
        exact_small_component_weight: small_weight,
        dominant_required_weight: TARGET_WEIGHT - small_weight,
        dominant_incumbent_weight: incumbent_weight,
        min_degree: degree_stats.min,
        median_degree: degree_stats.median,
        max_degree: degree_stats.max,
        degeneracy: degree_stats.degeneracy,
        articulation_count: block_stats.articulation_count,
        biconnected_component_count: block_stats.block_count,
        largest_biconnected_component_size: block_stats.largest_block_size,
        simplicial_vertex_count: simplicial_vertex_count(&graph),
        open_twin_class_count: twin_stats.class_count,
        largest_open_twin_class_size: twin_stats.largest_class_size,
        min_degree_elimination_width: min_degree_order.width,
        min_degree_fill_edge_count: min_degree_order.fill_edges,
        min_fill_elimination_width: min_fill_order.width,
        min_fill_fill_edge_count: min_fill_order.fill_edges,
    })
}
