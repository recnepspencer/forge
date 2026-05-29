use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::construction::lowering::{
    TopologyConstructionLoweringPlan, TopologyConstructionMutationSurface,
};

const REQUIRED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 1] =
    [ForgeQueryRuntimeFacadeFamily::Write];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionExecutionPlan {
    source_lowering_digest: String,
    mutation_surface: TopologyConstructionMutationSurface,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    execution_digest: String,
}

impl TopologyConstructionExecutionPlan {
    fn new(source_lowering_digest: String) -> Self {
        let mutation_surface = TopologyConstructionMutationSurface::ComposeGraph;
        let required_query_families = REQUIRED_QUERY_FAMILIES.to_vec();
        let mut parts = vec![
            source_lowering_digest.clone(),
            mutation_surface.as_str().to_string(),
        ];
        parts.extend(
            required_query_families
                .iter()
                .map(|family| format!("required-query-family:{family:?}")),
        );
        Self {
            source_lowering_digest,
            mutation_surface,
            required_query_families,
            execution_digest: digest_parts(&parts),
        }
    }

    pub fn source_lowering_digest(&self) -> &str {
        &self.source_lowering_digest
    }

    pub fn mutation_surface(&self) -> TopologyConstructionMutationSurface {
        self.mutation_surface
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    #[cfg(test)]
    pub(crate) fn new_for_tests(source_lowering_digest: &str) -> Self {
        Self::new(source_lowering_digest.to_string())
    }
}

#[derive(Debug)]
pub enum TopologyConstructionExecutionError {
    UnsupportedMutationSurface(&'static str),
}

impl std::fmt::Display for TopologyConstructionExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMutationSurface(reason) => {
                write!(f, "unsupported construction execution surface: {reason}")
            }
        }
    }
}

impl std::error::Error for TopologyConstructionExecutionError {}

pub fn prepare_primitive_construction_execution(
    plan: &TopologyConstructionLoweringPlan,
) -> Result<TopologyConstructionExecutionPlan, TopologyConstructionExecutionError> {
    if plan.mutation_surface() != TopologyConstructionMutationSurface::ComposeGraph {
        return Err(
            TopologyConstructionExecutionError::UnsupportedMutationSurface(
                "phase 3 closed-solid lowering requires compose_graph execution",
            ),
        );
    }
    Ok(TopologyConstructionExecutionPlan::new(
        plan.lowering_digest().to_string(),
    ))
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
    use super::TopologyConstructionExecutionPlan;
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

    #[test]
    fn compose_graph_execution_plan_requires_write_family() {
        let execution = TopologyConstructionExecutionPlan::new_for_tests("pyramid-lowering");

        assert_eq!(
            execution.required_query_families(),
            &[ForgeQueryRuntimeFacadeFamily::Write]
        );
        assert!(!execution.execution_digest().is_empty());
    }
}
