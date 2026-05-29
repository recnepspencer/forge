use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use worth_spatial::facade::{
    primitive_birth_contract_matches_counts, PrimitiveConstructionBirthContractCounts,
    SpatialConstructionBirthPlan,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionMutationSurface {
    ComposeGraph,
}

impl TopologyConstructionMutationSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ComposeGraph => "compose_graph",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionLoweringPlan {
    source_birth_digest: String,
    topology_birth_class: String,
    mutation_surface: TopologyConstructionMutationSurface,
    expected_vertex_births: usize,
    expected_edge_births: usize,
    expected_loop_births: usize,
    expected_wire_births: usize,
    expected_face_births: usize,
    expected_shell_births: usize,
    expected_body_births: usize,
    lowering_digest: String,
}

impl TopologyConstructionLoweringPlan {
    fn new(plan: &SpatialConstructionBirthPlan) -> Self {
        let mutation_surface = TopologyConstructionMutationSurface::ComposeGraph;
        let parts = [
            plan.birth_digest().to_string(),
            plan.topology_birth_class().to_string(),
            mutation_surface.as_str().to_string(),
            plan.supported_vertex_count().to_string(),
            plan.supported_edge_count().to_string(),
            plan.supported_loop_count().to_string(),
            plan.supported_wire_count().to_string(),
            plan.supported_face_count().to_string(),
            plan.supported_shell_count().to_string(),
            plan.supported_body_count().to_string(),
        ];
        Self {
            source_birth_digest: plan.birth_digest().to_string(),
            topology_birth_class: plan.topology_birth_class().to_string(),
            mutation_surface,
            expected_vertex_births: plan.supported_vertex_count(),
            expected_edge_births: plan.supported_edge_count(),
            expected_loop_births: plan.supported_loop_count(),
            expected_wire_births: plan.supported_wire_count(),
            expected_face_births: plan.supported_face_count(),
            expected_shell_births: plan.supported_shell_count(),
            expected_body_births: plan.supported_body_count(),
            lowering_digest: digest_parts(&parts),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_tests(
        source_birth_digest: &str,
        topology_birth_class: &str,
        expected_vertex_births: usize,
        expected_edge_births: usize,
        expected_loop_births: usize,
        expected_wire_births: usize,
        expected_face_births: usize,
        expected_shell_births: usize,
        expected_body_births: usize,
    ) -> Self {
        let mutation_surface = TopologyConstructionMutationSurface::ComposeGraph;
        let parts = [
            source_birth_digest.to_string(),
            topology_birth_class.to_string(),
            mutation_surface.as_str().to_string(),
            expected_vertex_births.to_string(),
            expected_edge_births.to_string(),
            expected_loop_births.to_string(),
            expected_wire_births.to_string(),
            expected_face_births.to_string(),
            expected_shell_births.to_string(),
            expected_body_births.to_string(),
        ];
        Self {
            source_birth_digest: source_birth_digest.to_string(),
            topology_birth_class: topology_birth_class.to_string(),
            mutation_surface,
            expected_vertex_births,
            expected_edge_births,
            expected_loop_births,
            expected_wire_births,
            expected_face_births,
            expected_shell_births,
            expected_body_births,
            lowering_digest: digest_parts(&parts),
        }
    }

    pub fn source_birth_digest(&self) -> &str {
        &self.source_birth_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn mutation_surface(&self) -> TopologyConstructionMutationSurface {
        self.mutation_surface
    }

    pub fn expected_vertex_births(&self) -> usize {
        self.expected_vertex_births
    }

    pub fn expected_edge_births(&self) -> usize {
        self.expected_edge_births
    }

    pub fn expected_loop_births(&self) -> usize {
        self.expected_loop_births
    }

    pub fn expected_wire_births(&self) -> usize {
        self.expected_wire_births
    }

    pub fn expected_face_births(&self) -> usize {
        self.expected_face_births
    }

    pub fn expected_shell_births(&self) -> usize {
        self.expected_shell_births
    }

    pub fn expected_body_births(&self) -> usize {
        self.expected_body_births
    }

    pub fn lowering_digest(&self) -> &str {
        &self.lowering_digest
    }
}

#[derive(Debug)]
pub enum TopologyConstructionLoweringError {
    UnsupportedBirthClass(&'static str),
}

impl std::fmt::Display for TopologyConstructionLoweringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedBirthClass(reason) => {
                write!(f, "unsupported topology birth class: {reason}")
            }
        }
    }
}

impl std::error::Error for TopologyConstructionLoweringError {}

pub fn lower_primitive_construction_birth_plan(
    plan: &SpatialConstructionBirthPlan,
) -> Result<TopologyConstructionLoweringPlan, TopologyConstructionLoweringError> {
    let admitted = primitive_birth_contract_matches_counts(
        plan.family(),
        PrimitiveConstructionBirthContractCounts::from_plan(plan),
    );
    if !admitted {
        return Err(TopologyConstructionLoweringError::UnsupportedBirthClass(
            "only admitted primitive construction birth plans may lower through topology in this phase",
        ));
    }
    Ok(TopologyConstructionLoweringPlan::new(plan))
}

fn digest_parts(parts: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::{TopologyConstructionLoweringPlan, TopologyConstructionMutationSurface};

    #[test]
    fn topo_lowering_plan_preserves_compose_graph_family_counts() {
        let simplex_lowering = TopologyConstructionLoweringPlan::new_for_tests(
            "simplex-birth",
            "closed_simplex_body",
            4,
            6,
            4,
            0,
            4,
            1,
            1,
        );
        let wire_lowering = TopologyConstructionLoweringPlan::new_for_tests(
            "wire-birth",
            "planar_wire_body",
            5,
            5,
            1,
            1,
            0,
            0,
            1,
        );

        assert_eq!(
            simplex_lowering.mutation_surface(),
            TopologyConstructionMutationSurface::ComposeGraph
        );
        assert_eq!(simplex_lowering.expected_loop_births(), 4);
        assert_eq!(wire_lowering.expected_wire_births(), 1);
    }
}




