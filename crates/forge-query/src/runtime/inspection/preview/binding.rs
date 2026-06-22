use super::super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewHandleBindingFamily,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::session_label::ForgeQuerySessionLabel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewBindingInspection {
    label: ForgeQuerySessionLabel,
    handle_name: String,
    family: ForgeQueryPreviewHandleBindingFamily,
    source_lane: ForgeQueryAuthorityLane,
    preview_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    effect_disposition: Option<String>,
    basis_evidence: Vec<String>,
    basis_digest: String,
    effect_delivery_admitted: bool,
    pending_write_intent_admitted: bool,
    authoritative_side_effect_admitted: bool,
    admission_digest: String,
    inspection_digest: String,
}

impl ForgeQueryPreviewBindingInspection {
    pub(in crate::runtime) fn from_binding(
        binding: &ForgeQueryPreviewHandleBindingEvidence,
    ) -> Self {
        let basis_evidence = binding.basis_evidence().to_vec();
        let basis_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewBindingInspectionArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("artifact_kind"), "basis")
        .field_value(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            binding.label_identity().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("binding_family"),
            binding.family().as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("basis_evidence"),
            basis_evidence.iter().map(String::as_str),
        )
        .seal()
        .as_str()
        .to_string();
        let effect_disposition = binding
            .effect_disposition()
            .map(|disposition| disposition.as_str().to_string());
        let admission_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewBindingInspectionArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("artifact_kind"), "admission")
        .field_value(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            binding.label_identity().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("handle_name"),
            binding.handle_name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("binding_family"),
            binding.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            binding.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("preview_lane"),
            binding.preview_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            binding.effect_policy().as_str(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("effect_disposition"),
            effect_disposition.as_deref(),
        )
        .field_value(ForgeQueryEvidenceTag::new("basis_digest"), &basis_digest)
        .field_bool(
            ForgeQueryEvidenceTag::new("effect_delivery_admitted"),
            binding.effect_delivery_admitted(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("pending_write_intent_admitted"),
            binding.pending_write_intent_admitted(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("authoritative_side_effect_admitted"),
            binding.authoritative_side_effect_admitted(),
        )
        .seal()
        .as_str()
        .to_string();
        let inspection_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewBindingInspectionArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("artifact_kind"), "inspection")
        .field_value(
            ForgeQueryEvidenceTag::new("admission_digest"),
            &admission_digest,
        )
        .seal()
        .as_str()
        .to_string();

        Self {
            label: binding.session_label().clone(),
            handle_name: binding.handle_name().to_string(),
            family: binding.family(),
            source_lane: binding.source_lane(),
            preview_lane: binding.preview_lane(),
            effect_policy: binding.effect_policy(),
            effect_disposition,
            basis_evidence,
            basis_digest,
            effect_delivery_admitted: binding.effect_delivery_admitted(),
            pending_write_intent_admitted: binding.pending_write_intent_admitted(),
            authoritative_side_effect_admitted: binding.authoritative_side_effect_admitted(),
            admission_digest,
            inspection_digest,
        }
    }

    pub fn label(&self) -> &str {
        self.label.display()
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
        &self.label
    }
    pub fn handle_name(&self) -> &str {
        &self.handle_name
    }
    pub fn family(&self) -> ForgeQueryPreviewHandleBindingFamily {
        self.family
    }
    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }
    pub fn preview_lane(&self) -> ForgeQueryAuthorityLane {
        self.preview_lane
    }
    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
    pub fn effect_disposition(&self) -> Option<&str> {
        self.effect_disposition.as_deref()
    }
    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }
    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
    pub fn effect_delivery_admitted(&self) -> bool {
        self.effect_delivery_admitted
    }
    pub fn pending_write_intent_admitted(&self) -> bool {
        self.pending_write_intent_admitted
    }
    pub fn authoritative_side_effect_admitted(&self) -> bool {
        self.authoritative_side_effect_admitted
    }
    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
