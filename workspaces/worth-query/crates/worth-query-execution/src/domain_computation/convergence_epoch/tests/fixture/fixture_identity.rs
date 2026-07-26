use worth_query_installation::facade::{
    WorthQueryArtifactComparatorFamily, WorthQueryArtifactFamily,
};

pub(super) const OWNER: &str = "worth.convergence.fixture";
pub(super) const OPERATION_SLOT: &str = "iterate:1";
pub(super) const WORKFLOW_OPERATION_SLOT: &str = "iterate-workflow:1";
pub(crate) const WORKFLOW_STAGE: &str = "iterate-stage";
pub(super) const GRAPH_ROLE: &str = "model";

pub(super) struct CandidateFamily;

impl WorthQueryArtifactFamily for CandidateFamily {
    const SEMANTIC_FAMILY: &'static str = "worth.convergence.candidate";
}

pub(super) struct ComparatorFamily;

impl WorthQueryArtifactComparatorFamily for ComparatorFamily {
    const SEMANTIC_FAMILY: &'static str = "worth.convergence.comparator";
}
