use crate::{
    WorthServerBranchTarget, WorthServerQuerySupportPosture, WorthServerRequestContext,
    WorthServerResponseEnvelope, WorthServerWorkspaceTarget,
};

use super::{WorthServerDirectProvenance, WorthServerDirectRemaskPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectContextArtifact {
    workspace_target: WorthServerWorkspaceTarget,
    workspace_digest: String,
    branch_target: WorthServerBranchTarget,
    branch_digest: String,
    support_posture_digest: String,
    basis_digest: Option<String>,
    remask_posture: WorthServerDirectRemaskPosture,
    provenance: WorthServerDirectProvenance,
    canonical_digest: String,
}

impl WorthServerDirectContextArtifact {
    pub(crate) fn new(
        request_context: &WorthServerRequestContext,
        support_posture: &WorthServerQuerySupportPosture,
        response_envelope: &WorthServerResponseEnvelope,
        basis_digest: Option<&str>,
        remask_posture: WorthServerDirectRemaskPosture,
    ) -> Self {
        let workspace_target = request_context.workspace_target().clone();
        let workspace_digest = workspace_target.workspace_digest();
        let branch_target = request_context.branch_target().clone();
        let branch_digest = branch_target.branch_digest();
        let support_posture_digest = support_posture.canonical_label();
        let basis_digest = basis_digest.map(str::to_string);
        let provenance = WorthServerDirectProvenance::new(
            response_envelope.provenance(),
            response_envelope.diagnostics_profile(),
        );
        let canonical_digest = format!(
            "worth-server-direct-context-v1|workspace:{workspace_digest}|branch:{branch_digest}|support:{support_posture_digest}|basis:{}|remask:{}|provenance:{}",
            basis_digest.as_deref().unwrap_or("none"),
            remask_posture.remask_digest().unwrap_or("visible"),
            provenance.provenance_digest(),
        );
        Self {
            workspace_target,
            workspace_digest,
            branch_target,
            branch_digest,
            support_posture_digest,
            basis_digest,
            remask_posture,
            provenance,
            canonical_digest,
        }
    }

    pub fn workspace_target(&self) -> &WorthServerWorkspaceTarget {
        &self.workspace_target
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn branch_target(&self) -> &WorthServerBranchTarget {
        &self.branch_target
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn support_posture_digest(&self) -> &str {
        &self.support_posture_digest
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn remask_posture(&self) -> &WorthServerDirectRemaskPosture {
        &self.remask_posture
    }

    pub fn provenance(&self) -> &WorthServerDirectProvenance {
        &self.provenance
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
