use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::{
    WorthQueryReadBuiltInOperator, WorthQueryReadDomainInvariantSummary, WorthQueryReadGraph,
    WorthQueryReadOperatorFamily,
};

const DOMAIN_INVARIANT_PACK_HOOK_FAMILY: &str = "domain_invariant_pack_hook";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadInvariantPackViolation {
    invariant_family: String,
    message: String,
    violation_digest: String,
}

impl WorthQueryReadInvariantPackViolation {
    pub fn new(invariant_family: impl Into<String>, message: impl Into<String>) -> Self {
        let invariant_family = invariant_family.into();
        let message = message.into();
        let violation_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::ReadInvariantViolation)
                .field_shape(
                    WorthQueryEvidenceTag::new("invariant_family"),
                    invariant_family.as_str(),
                )
                .seal()
                .as_str()
                .to_string();
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
pub struct WorthQueryReadInvariantPackContext<'a> {
    read_graph: &'a WorthQueryReadGraph,
}

impl<'a> WorthQueryReadInvariantPackContext<'a> {
    pub(crate) fn new(read_graph: &'a WorthQueryReadGraph) -> Self {
        Self { read_graph }
    }

    pub fn read_graph(&self) -> &'a WorthQueryReadGraph {
        self.read_graph
    }

    pub fn operator_families(&self) -> Vec<WorthQueryReadOperatorFamily> {
        self.read_graph.operator_families()
    }

    pub fn built_in_operator_coverage(&self) -> Vec<WorthQueryReadBuiltInOperator> {
        self.read_graph.built_in_operators().to_vec()
    }

    pub fn read_domain_invariant_summary(&self) -> WorthQueryReadDomainInvariantSummary {
        WorthQueryReadDomainInvariantSummary::derive(self.read_graph)
    }
}

pub(crate) fn read_invariant_pack_hook_family() -> &'static str {
    DOMAIN_INVARIANT_PACK_HOOK_FAMILY
}
