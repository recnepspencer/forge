use crate::identity::hash_parts;

use super::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadDomainInvariantSummary, ForgeQueryReadGraph,
    ForgeQueryReadOperatorFamily,
};

const DOMAIN_INVARIANT_PACK_HOOK_FAMILY: &str = "domain_invariant_pack_hook";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadInvariantPackViolation {
    invariant_family: String,
    message: String,
    violation_digest: String,
}

impl ForgeQueryReadInvariantPackViolation {
    pub fn new(invariant_family: impl Into<String>, message: impl Into<String>) -> Self {
        let invariant_family = invariant_family.into();
        let message = message.into();
        let violation_digest = hash_parts(&[
            "forge_query_read_invariant_pack_violation_v1".to_string(),
            format!("invariant:{invariant_family}"),
            format!("message:{message}"),
        ]);
        Self {
            invariant_family,
            message,
            violation_digest,
        }
    }

    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn violation_digest(&self) -> &str {
        &self.violation_digest
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ForgeQueryReadInvariantPackContext<'a> {
    read_graph: &'a ForgeQueryReadGraph,
}

impl<'a> ForgeQueryReadInvariantPackContext<'a> {
    pub(crate) fn new(read_graph: &'a ForgeQueryReadGraph) -> Self {
        Self { read_graph }
    }

    pub fn read_graph(&self) -> &'a ForgeQueryReadGraph {
        self.read_graph
    }

    pub fn operator_families(&self) -> Vec<ForgeQueryReadOperatorFamily> {
        self.read_graph.operator_families()
    }

    pub fn built_in_operator_coverage(&self) -> Vec<ForgeQueryReadBuiltInOperator> {
        self.read_graph.built_in_operators().to_vec()
    }

    pub fn read_domain_invariant_summary(&self) -> ForgeQueryReadDomainInvariantSummary {
        ForgeQueryReadDomainInvariantSummary::derive(self.read_graph)
    }
}

pub(crate) fn read_invariant_pack_hook_family() -> &'static str {
    DOMAIN_INVARIANT_PACK_HOOK_FAMILY
}
