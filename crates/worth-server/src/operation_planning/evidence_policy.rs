use crate::request_context::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationPlanEvidencePolicy {
    diagnostics_profile: DiagnosticRichnessProfile,
    materialization_lane: &'static str,
    evidence_identity: String,
}

impl WorthServerOperationPlanEvidencePolicy {
    pub(crate) fn from_diagnostics_profile(diagnostics_profile: DiagnosticRichnessProfile) -> Self {
        let materialization_lane = match diagnostics_profile {
            DiagnosticRichnessProfile::OperationalMinimal => "operational-minimal",
            DiagnosticRichnessProfile::Standard => "standard",
            DiagnosticRichnessProfile::Forensic => "forensic",
        };
        let evidence_identity = format!(
            "worth-server-operation-plan-evidence-policy-v1|diagnostics={materialization_lane}"
        );
        Self {
            diagnostics_profile,
            materialization_lane,
            evidence_identity,
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn materialization_lane(&self) -> &str {
        self.materialization_lane
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
}
