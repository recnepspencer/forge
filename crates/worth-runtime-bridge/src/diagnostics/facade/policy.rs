use super::*;

impl BridgeDiagnosticsFacade {
    pub fn explain_policy_provenance_report(
        &self,
        report: &crate::policy::BridgePolicyProvenanceReport,
    ) -> crate::diagnostics::BridgePolicyExplanation {
        crate::diagnostics::BridgePolicyExplanation::from_report(report)
    }

    pub fn explain_policy_rejection(
        &self,
        rejection: &crate::policy::BridgePolicyRejection,
    ) -> crate::diagnostics::BridgePolicyRejectionExplanation {
        crate::diagnostics::BridgePolicyRejectionExplanation::from_rejection(rejection)
    }
}
