use crate::identity::hash_parts;

use super::super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewHandleBindingFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewBindingInspection {
    label: String,
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
        let basis_digest = hash_parts(&[
            "forge_query_preview_binding_basis_v1".to_string(),
            format!("label:{}", binding.label()),
            format!("family:{}", binding.family().as_str()),
            format!("basis:{}", basis_evidence.join("|")),
        ]);
        let effect_disposition = binding
            .effect_disposition()
            .map(|disposition| disposition.as_str().to_string());
        let admission_digest = hash_parts(&[
            "forge_query_preview_binding_admission_v1".to_string(),
            format!("label:{}", binding.label()),
            format!("handle:{}", binding.handle_name()),
            format!("family:{}", binding.family().as_str()),
            format!("source:{}", binding.source_lane()),
            format!("preview:{}", binding.preview_lane()),
            format!("policy:{}", binding.effect_policy().as_str()),
            format!(
                "effect-disposition:{}",
                effect_disposition.as_deref().unwrap_or("none")
            ),
            format!("basis:{basis_digest}"),
            format!(
                "effect-delivery-admitted:{}",
                binding.effect_delivery_admitted()
            ),
            format!(
                "pending-write-intent-admitted:{}",
                binding.pending_write_intent_admitted()
            ),
            format!(
                "authoritative-side-effect-admitted:{}",
                binding.authoritative_side_effect_admitted()
            ),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_preview_binding_inspection_v1".to_string(),
            admission_digest.clone(),
        ]);

        Self {
            label: binding.label().to_string(),
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
