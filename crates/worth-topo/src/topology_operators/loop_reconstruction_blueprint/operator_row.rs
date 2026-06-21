use super::classification::{
    PlanarBooleanLoopOperatorClassification, PlanarBooleanLoopOperatorTruthAuthority,
    PlanarBooleanLoopRequiredQuerySurface,
};
use super::proof_obligation::PlanarBooleanLoopOperatorProofObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopOperatorRow {
    operator_name: &'static str,
    classification: PlanarBooleanLoopOperatorClassification,
    truth_authority: PlanarBooleanLoopOperatorTruthAuthority,
    required_query_surface: PlanarBooleanLoopRequiredQuerySurface,
    topology_precedent: Option<&'static str>,
    proof_obligations: &'static [PlanarBooleanLoopOperatorProofObligation],
    support_warning: Option<&'static str>,
}

impl PlanarBooleanLoopOperatorRow {
    pub(crate) fn new(
        operator_name: &'static str,
        classification: PlanarBooleanLoopOperatorClassification,
        truth_authority: PlanarBooleanLoopOperatorTruthAuthority,
        required_query_surface: PlanarBooleanLoopRequiredQuerySurface,
        topology_precedent: Option<&'static str>,
        proof_obligations: &'static [PlanarBooleanLoopOperatorProofObligation],
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

    pub fn classification(&self) -> PlanarBooleanLoopOperatorClassification {
        self.classification
    }

    pub fn truth_authority(&self) -> PlanarBooleanLoopOperatorTruthAuthority {
        self.truth_authority
    }

    pub fn required_query_surface(&self) -> PlanarBooleanLoopRequiredQuerySurface {
        self.required_query_surface
    }

    pub fn topology_precedent(&self) -> Option<&'static str> {
        self.topology_precedent
    }

    #[cfg(test)]
    pub(crate) fn proof_obligations(&self) -> &'static [PlanarBooleanLoopOperatorProofObligation] {
        self.proof_obligations
    }

    pub fn support_warning(&self) -> Option<&'static str> {
        self.support_warning
    }

    pub fn may_commit_topology_in_7_4(&self) -> bool {
        self.classification.may_commit_topology_in_7_4()
    }
}
