use super::classification::{
    EdgeSplitOperatorClassification, EdgeSplitOperatorTruthAuthority, EdgeSplitRequiredQuerySurface,
};
use super::proof_obligation::EdgeSplitOperatorProofObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeSplitOperatorRow {
    operator_name: &'static str,
    classification: EdgeSplitOperatorClassification,
    truth_authority: EdgeSplitOperatorTruthAuthority,
    required_query_surface: EdgeSplitRequiredQuerySurface,
    topology_precedent: Option<&'static str>,
    proof_obligations: &'static [EdgeSplitOperatorProofObligation],
    support_warning: Option<&'static str>,
}

impl EdgeSplitOperatorRow {
    pub(crate) fn new(
        operator_name: &'static str,
        classification: EdgeSplitOperatorClassification,
        truth_authority: EdgeSplitOperatorTruthAuthority,
        required_query_surface: EdgeSplitRequiredQuerySurface,
        topology_precedent: Option<&'static str>,
        proof_obligations: &'static [EdgeSplitOperatorProofObligation],
        support_warning: Option<&'static str>,
    ) -> Self {
        Self {
            operator_name,
            classification,
            truth_authority,
            required_query_surface,
            topology_precedent,
            proof_obligations,
            support_warning,
        }
    }

    pub fn operator_name(&self) -> &'static str {
        self.operator_name
    }

    pub fn classification(&self) -> EdgeSplitOperatorClassification {
        self.classification
    }

    pub fn truth_authority(&self) -> EdgeSplitOperatorTruthAuthority {
        self.truth_authority
    }

    pub fn required_query_surface(&self) -> EdgeSplitRequiredQuerySurface {
        self.required_query_surface
    }

    pub fn topology_precedent(&self) -> Option<&'static str> {
        self.topology_precedent
    }

    pub fn proof_obligations(&self) -> &'static [EdgeSplitOperatorProofObligation] {
        self.proof_obligations
    }

    pub fn support_warning(&self) -> Option<&'static str> {
        self.support_warning
    }

    pub fn may_commit_topology_in_7_3(&self) -> bool {
        self.classification.may_commit_topology_in_7_3()
    }
}
