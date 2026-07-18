use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum StructuralPredicate {
    Boundary,
    AgentContext,
    Inventory,
    Preservation,
    Feature,
    Dependency,
    LineCap,
    Naming,
    AdmittedResidue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "predicate", rename_all = "snake_case")]
pub enum DependencyBoundaryPredicate {
    ForbiddenFeatureEdge {
        source_package: String,
        feature: String,
        forbidden_dependency: String,
    },
}
