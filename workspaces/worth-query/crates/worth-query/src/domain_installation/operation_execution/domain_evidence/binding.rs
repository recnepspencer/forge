use super::super::{
    progression::evidence_completion::WorthQueryDirectDomainEvidenceAttachment,
    workflow_progression::evidence_validation::WorthQueryWorkflowDomainEvidenceAttachment,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceBinding {
    operation_identity: String,
    binding_identity: String,
    run_identity: Option<String>,
    stage_identity: Option<String>,
    basis_identity: String,
    execution_snapshot_identity: String,
    output_occurrence_identity: String,
    execution_occurrence_identity: String,
}

impl WorthQueryDomainEvidenceBinding {
    pub(super) fn from_direct(attachment: &WorthQueryDirectDomainEvidenceAttachment) -> Self {
        Self {
            operation_identity: attachment.operation_identity().to_owned(),
            binding_identity: attachment.binding_identity().to_owned(),
            run_identity: None,
            stage_identity: None,
            basis_identity: attachment.basis_identity().to_owned(),
            execution_snapshot_identity: attachment.execution_snapshot_identity().to_owned(),
            output_occurrence_identity: attachment.output_occurrence_identity().to_owned(),
            execution_occurrence_identity: direct_occurrence_identity(attachment),
        }
    }

    pub(super) fn from_workflow(attachment: &WorthQueryWorkflowDomainEvidenceAttachment) -> Self {
        Self {
            operation_identity: attachment.operation_identity().to_owned(),
            binding_identity: attachment.binding_identity().to_owned(),
            run_identity: Some(attachment.run_identity().to_owned()),
            stage_identity: Some(attachment.stage_identity().to_owned()),
            basis_identity: attachment.basis_identity().to_owned(),
            execution_snapshot_identity: attachment.execution_snapshot_identity().to_owned(),
            output_occurrence_identity: attachment.output_occurrence_identity().to_owned(),
            execution_occurrence_identity: workflow_occurrence_identity(attachment),
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn run_identity(&self) -> Option<&str> {
        self.run_identity.as_deref()
    }

    pub fn stage_identity(&self) -> Option<&str> {
        self.stage_identity.as_deref()
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn execution_snapshot_identity(&self) -> &str {
        &self.execution_snapshot_identity
    }

    pub fn output_occurrence_identity(&self) -> &str {
        &self.output_occurrence_identity
    }

    pub fn execution_occurrence_identity(&self) -> &str {
        &self.execution_occurrence_identity
    }
}

pub(crate) fn domain_evidence_binding_material(
    binding: &WorthQueryDomainEvidenceBinding,
) -> String {
    crate::domain_installation::operation_identity_basis::canonical_operation_material(vec![
        ("operation", binding.operation_identity().into()),
        ("binding", binding.binding_identity().into()),
        (
            "run",
            binding.run_identity().unwrap_or("not-required").into(),
        ),
        (
            "stage",
            binding.stage_identity().unwrap_or("not-required").into(),
        ),
        ("basis", binding.basis_identity().into()),
        ("snapshot", binding.execution_snapshot_identity().into()),
        ("output", binding.output_occurrence_identity().into()),
        (
            "execution_occurrence",
            binding.execution_occurrence_identity().into(),
        ),
    ])
}

fn direct_occurrence_identity(attachment: &WorthQueryDirectDomainEvidenceAttachment) -> String {
    hash_occurrence(vec![
        format!("operation:{}", attachment.operation_identity()),
        format!("binding:{}", attachment.binding_identity()),
        format!("basis:{}", attachment.basis_identity()),
        format!("snapshot:{}", attachment.execution_snapshot_identity()),
        "run:not-required".into(),
        "stage:not-required".into(),
        format!("output:{}", attachment.output_occurrence_identity()),
        format!(
            "provider-session:{}",
            attachment.provider_session_identity()
        ),
        format!(
            "provider-session-attempt:{}",
            attachment.provider_session_attempt_identity()
        ),
        graph_receipt_material(attachment.graph_receipt_identities()),
    ])
}

fn workflow_occurrence_identity(attachment: &WorthQueryWorkflowDomainEvidenceAttachment) -> String {
    hash_occurrence(vec![
        format!("operation:{}", attachment.operation_identity()),
        format!("binding:{}", attachment.binding_identity()),
        format!("basis:{}", attachment.basis_identity()),
        format!("snapshot:{}", attachment.execution_snapshot_identity()),
        format!("run:{}", attachment.run_identity()),
        format!("stage:{}", attachment.stage_identity()),
        format!("output:{}", attachment.output_occurrence_identity()),
        format!(
            "provider-session:{}",
            attachment.provider_session_identity()
        ),
        format!(
            "provider-session-attempt:{}",
            attachment.provider_session_attempt_identity()
        ),
        graph_receipt_material(attachment.graph_receipt_identities()),
    ])
}

fn graph_receipt_material(graph_receipt_identities: impl Iterator<Item = String>) -> String {
    let graph_receipts =
        crate::domain_installation::operation_identity_basis::canonical_indexed_operation_material(
            "ordinary.domain-evidence.graph-receipt",
            graph_receipt_identities,
        );
    format!("graph-receipts:{graph_receipts}")
}

fn hash_occurrence(mut material: Vec<String>) -> String {
    material.insert(
        0,
        "worth_query_ordinary_domain_evidence_occurrence_v1".into(),
    );
    crate::identity::hash_parts(&material)
}
