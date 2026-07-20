use worth_foundational::facade::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerBinaryPolicyDecision {
    metadata_identity: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    operation_name: String,
    diagnostics_profile: DiagnosticRichnessProfile,
    policy_lane: String,
    support_posture_digest: String,
    response_envelope_digest: String,
    transfer_authorization_digest: Option<String>,
    canonical_digest: String,
}

pub(crate) struct WorthServerBinaryPolicyDecisionParts {
    pub(crate) metadata_identity: String,
    pub(crate) tenant_id: String,
    pub(crate) workspace_digest: String,
    pub(crate) branch_digest: String,
    pub(crate) operation_name: String,
    pub(crate) diagnostics_profile: DiagnosticRichnessProfile,
    pub(crate) policy_lane: String,
    pub(crate) support_posture_digest: String,
    pub(crate) response_envelope_digest: String,
    pub(crate) transfer_authorization_digest: Option<String>,
}

impl WorthServerBinaryPolicyDecision {
    pub(crate) fn new(parts: WorthServerBinaryPolicyDecisionParts) -> Self {
        let WorthServerBinaryPolicyDecisionParts {
            metadata_identity,
            tenant_id,
            workspace_digest,
            branch_digest,
            operation_name,
            diagnostics_profile,
            policy_lane,
            support_posture_digest,
            response_envelope_digest,
            transfer_authorization_digest,
        } = parts;
        let metadata_identity = metadata_identity.trim().to_string();
        let tenant_id = tenant_id.trim().to_string();
        let workspace_digest = workspace_digest.trim().to_string();
        let branch_digest = branch_digest.trim().to_string();
        let operation_name = operation_name.trim().to_string();
        let policy_lane = policy_lane.trim().to_string();
        let support_posture_digest = support_posture_digest.trim().to_string();
        let response_envelope_digest = response_envelope_digest.trim().to_string();
        let canonical_digest = format!(
            "worth-server-file-policy-decision-v1|identity={metadata_identity}|tenant={tenant_id}|workspace={workspace_digest}|branch={branch_digest}|operation={operation_name}|lane={policy_lane}|support={support_posture_digest}|response={response_envelope_digest}|authorization={}|diagnostics={:?}",
            transfer_authorization_digest.as_deref().unwrap_or("none"),
            diagnostics_profile,
        );
        Self {
            metadata_identity,
            tenant_id,
            workspace_digest,
            branch_digest,
            operation_name,
            diagnostics_profile,
            policy_lane,
            support_posture_digest,
            response_envelope_digest,
            transfer_authorization_digest,
            canonical_digest,
        }
    }

    pub fn metadata_identity(&self) -> &str {
        &self.metadata_identity
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn policy_lane(&self) -> &str {
        &self.policy_lane
    }

    pub fn support_posture_digest(&self) -> &str {
        &self.support_posture_digest
    }

    pub fn response_envelope_digest(&self) -> &str {
        &self.response_envelope_digest
    }

    pub fn transfer_authorization_digest(&self) -> Option<&str> {
        self.transfer_authorization_digest.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
