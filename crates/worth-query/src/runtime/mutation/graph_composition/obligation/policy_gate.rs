use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::policy_basis::{AdmittedPolicyTenantContext, PolicyAdmissionDisposition};
use crate::policy_execution_seam::PolicyAwareExecutionMode;

use super::{
    WorthQueryGraphObligationOperatingWorldDescriptor, WorthQueryGraphObligationSelection,
    WorthQueryGraphObligationSupportStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphMutationPolicyGateVerdict {
    Allow,
    Advise,
    Deny,
}

impl WorthQueryGraphMutationPolicyGateVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Advise => "advise",
            Self::Deny => "deny",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphMutationPolicyGateEvidence {
    policy_tenant_admission_digest: String,
    policy_digest: String,
    tenant_truth_basis_digest: String,
    tenant_schema_basis_digest: String,
    branch_access_digest: String,
    admission_disposition: PolicyAdmissionDisposition,
    execution_mode: PolicyAwareExecutionMode,
    operating_world_digest: String,
    touch_descriptor_digest: String,
    selection_digest: String,
    matched_obligation_count: usize,
    registration_full_scan_count: usize,
    verdict: WorthQueryGraphMutationPolicyGateVerdict,
    evidence_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphMutationPolicyGateEvidence {
    pub(crate) fn from_admitted_context_and_selection(
        context: &AdmittedPolicyTenantContext,
        operating_world: &WorthQueryGraphObligationOperatingWorldDescriptor,
        selection: &WorthQueryGraphObligationSelection,
    ) -> Self {
        let bundle = context.bundle();
        let verdict = policy_gate_verdict(context.policy_basis().disposition(), selection);
        let evidence_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationMaterializedDispatch,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "graph-mutation-policy-gate",
        )
        .field_value(
            WorthQueryEvidenceTag::new("policy_tenant_admission"),
            bundle.digest().as_str(),
        )
        .field_value(WorthQueryEvidenceTag::new("policy"), bundle.policy_digest())
        .field_value(
            WorthQueryEvidenceTag::new("tenant_truth"),
            bundle.tenant_truth_basis_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("tenant_schema"),
            bundle.tenant_schema_basis_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("branch"),
            bundle.branch_access_digest(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("disposition"),
            bundle.admission_disposition().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("execution_mode"),
            PolicyAwareExecutionMode::GraphMutation.as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("operating_world"),
            operating_world.descriptor_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("touch_descriptor"),
            selection.touch_descriptor_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("selection"),
            selection.selection_digest(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("matched_obligation_count"),
            selection.matched_obligation_count(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("registration_full_scan_count"),
            selection.counters().registration_full_scan_count(),
        )
        .field_shape(WorthQueryEvidenceTag::new("verdict"), verdict.as_str())
        .seal();
        Self {
            policy_tenant_admission_digest: bundle.digest().as_str().to_string(),
            policy_digest: bundle.policy_digest().to_string(),
            tenant_truth_basis_digest: bundle.tenant_truth_basis_digest().to_string(),
            tenant_schema_basis_digest: bundle.tenant_schema_basis_digest().to_string(),
            branch_access_digest: bundle.branch_access_digest().to_string(),
            admission_disposition: bundle.admission_disposition(),
            execution_mode: PolicyAwareExecutionMode::GraphMutation,
            operating_world_digest: operating_world.descriptor_digest().to_string(),
            touch_descriptor_digest: selection.touch_descriptor_digest().to_string(),
            selection_digest: selection.selection_digest().to_string(),
            matched_obligation_count: selection.matched_obligation_count(),
            registration_full_scan_count: selection.counters().registration_full_scan_count(),
            verdict,
            evidence_digest,
        }
    }

    pub fn policy_tenant_admission_digest(&self) -> &str {
        &self.policy_tenant_admission_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_truth_basis_digest(&self) -> &str {
        &self.tenant_truth_basis_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn branch_access_digest(&self) -> &str {
        &self.branch_access_digest
    }

    pub fn admission_disposition(&self) -> PolicyAdmissionDisposition {
        self.admission_disposition
    }

    pub fn execution_mode(&self) -> PolicyAwareExecutionMode {
        self.execution_mode
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn selection_digest(&self) -> &str {
        &self.selection_digest
    }

    pub fn matched_obligation_count(&self) -> usize {
        self.matched_obligation_count
    }

    pub fn registration_full_scan_count(&self) -> usize {
        self.registration_full_scan_count
    }

    pub fn verdict(&self) -> WorthQueryGraphMutationPolicyGateVerdict {
        self.verdict
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_str()
    }
}

fn policy_gate_verdict(
    disposition: PolicyAdmissionDisposition,
    selection: &WorthQueryGraphObligationSelection,
) -> WorthQueryGraphMutationPolicyGateVerdict {
    if selection
        .matched_registrations()
        .iter()
        .any(|registration| {
            registration.support_posture().status()
                == WorthQueryGraphObligationSupportStatus::Unsupported
        })
    {
        return WorthQueryGraphMutationPolicyGateVerdict::Deny;
    }
    if disposition == PolicyAdmissionDisposition::AdmittedWithNonDisclosingUse {
        return WorthQueryGraphMutationPolicyGateVerdict::Deny;
    }
    if disposition == PolicyAdmissionDisposition::AdmittedNarrowed
        || selection
            .matched_registrations()
            .iter()
            .any(|registration| {
                registration.support_posture().status()
                    == WorthQueryGraphObligationSupportStatus::DiagnosticOnly
            })
    {
        return WorthQueryGraphMutationPolicyGateVerdict::Advise;
    }
    WorthQueryGraphMutationPolicyGateVerdict::Allow
}
