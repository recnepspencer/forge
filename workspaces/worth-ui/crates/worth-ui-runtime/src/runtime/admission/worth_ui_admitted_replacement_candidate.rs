use crate::runtime::admission::{
    WorthUiActiveReplacementBasis, WorthUiCandidateAdmissionDenial, WorthUiCandidateAdmissionReport,
};
use crate::runtime::candidate::{WorthUiCandidateArtifactBundle, WorthUiReplacementCandidate};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedReplacementCandidate {
    candidate: WorthUiReplacementCandidate,
    active_basis: WorthUiActiveReplacementBasis,
    report: WorthUiCandidateAdmissionReport,
    admitted_query_contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
}

impl WorthUiAdmittedReplacementCandidate {
    pub(crate) fn new(
        candidate: WorthUiReplacementCandidate,
        active_basis: WorthUiActiveReplacementBasis,
        report: WorthUiCandidateAdmissionReport,
    ) -> Self {
        let admitted_query_contract_identity = candidate
            .lowering_basis()
            .query_support_receipt()
            .contract_identity();
        Self {
            candidate,
            active_basis,
            report,
            admitted_query_contract_identity,
        }
    }

    pub fn candidate(&self) -> &WorthUiReplacementCandidate {
        &self.candidate
    }

    pub fn active_basis(&self) -> WorthUiActiveReplacementBasis {
        self.active_basis
    }

    pub fn report(&self) -> WorthUiCandidateAdmissionReport {
        self.report
    }

    pub fn verify_receipts_unchanged(&self) -> Result<(), WorthUiCandidateAdmissionDenial> {
        let current_contract_identity = self
            .candidate
            .lowering_basis()
            .query_support_receipt()
            .contract_identity();
        if current_contract_identity == self.admitted_query_contract_identity {
            Ok(())
        } else {
            Err(
                WorthUiCandidateAdmissionDenial::QuerySupportContractChanged {
                    admitted_contract_identity: self.admitted_query_contract_identity,
                    current_contract_identity,
                },
            )
        }
    }

    pub(crate) fn artifact_bundle(&self) -> &WorthUiCandidateArtifactBundle {
        self.candidate.artifact_bundle()
    }

    #[cfg(test)]
    pub(crate) fn verify_test_query_contract(
        &self,
        contract_label: &str,
    ) -> Result<(), WorthUiCandidateAdmissionDenial> {
        let current_contract_identity = query_contract_identity_for_test(contract_label);
        if current_contract_identity == self.admitted_query_contract_identity {
            Ok(())
        } else {
            Err(
                WorthUiCandidateAdmissionDenial::QuerySupportContractChanged {
                    admitted_contract_identity: self.admitted_query_contract_identity,
                    current_contract_identity,
                },
            )
        }
    }

    #[cfg(test)]
    pub(crate) fn with_admitted_query_contract_for_test(
        mut self,
        contract_label: &str,
    ) -> Self {
        self.admitted_query_contract_identity = query_contract_identity_for_test(contract_label);
        self
    }
}

#[cfg(test)]
fn query_contract_identity_for_test(
    label: &str,
) -> worth_ui_query_binding::WorthUiQueryBindingContractIdentity {
    let definition = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
        label,
    )
    .expect("test Query contract label must be valid");
    worth_ui_query_binding::WorthUiQueryBindingContractIdentity::from_definitions([
        definition.digest(),
    ])
}
