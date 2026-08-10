use crate::authorized_projection::AuthorizedProjectionFieldPath;
use crate::identity::hash_parts;

use super::NarrowedPolicyQueryArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareOptimizerInput {
    source_narrowed_artifact_digest: String,
    authorized_projection_digest: String,
    visible_fields: Vec<AuthorizedProjectionFieldPath>,
    relationship_proof_digest: String,
    validation_report_digest: String,
    optimizer_input_digest: String,
}

impl PolicyAwareOptimizerInput {
    pub(crate) fn from_narrowed(artifact: &NarrowedPolicyQueryArtifact) -> Self {
        let source_narrowed_artifact_digest = artifact.digest().to_string();
        let authorized_projection_digest = artifact
            .authorized_projection()
            .identity()
            .as_str()
            .to_string();
        let visible_fields = artifact
            .authorized_projection()
            .visible_field_paths()
            .to_vec();
        let relationship_proof_digest = artifact
            .relationship_proof()
            .identity()
            .as_str()
            .to_string();
        let validation_report_digest = artifact.validation_report().digest().to_string();
        let mut parts = vec![
            format!("narrowed:{source_narrowed_artifact_digest}"),
            format!("authorized_projection:{authorized_projection_digest}"),
            format!("relationship_proof:{relationship_proof_digest}"),
            format!("validation:{validation_report_digest}"),
        ];
        parts.extend(
            visible_fields
                .iter()
                .map(|field| format!("visible:{}", field.terminal_projection_for_boundary())),
        );
        Self {
            source_narrowed_artifact_digest,
            authorized_projection_digest,
            visible_fields,
            relationship_proof_digest,
            validation_report_digest,
            optimizer_input_digest: hash_parts(&parts),
        }
    }

    pub fn source_narrowed_artifact_digest(&self) -> &str {
        &self.source_narrowed_artifact_digest
    }

    pub fn authorized_projection_digest(&self) -> &str {
        &self.authorized_projection_digest
    }

    pub fn visible_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.visible_fields
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn validation_report_digest(&self) -> &str {
        &self.validation_report_digest
    }

    pub fn optimizer_input_digest(&self) -> &str {
        &self.optimizer_input_digest
    }
}
