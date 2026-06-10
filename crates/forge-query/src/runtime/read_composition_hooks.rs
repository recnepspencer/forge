use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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
        let violation_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ReadInvariantViolation,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("invariant_family"),
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
