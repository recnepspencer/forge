use crate::identity::hash_parts;
use crate::runtime::ForgeQueryIntentDeclaration;

use crate::intent_admission::{
    intent_family_for_entrypoint, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentViolationDecision,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRawIntentAdmissionRequest {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    declaration: ForgeQueryIntentDeclaration,
    request_digest: String,
}

impl ForgeQueryRawIntentAdmissionRequest {
    pub fn authoritative_runtime_entrypoint(
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
            declaration,
        )
    }

    pub fn effect_runtime_entrypoint(
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent,
            declaration,
        )
    }

    #[cfg(test)]
    pub(crate) fn deferred_neighbor(
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(entrypoint, declaration)
    }

    fn new(
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let family = intent_family_for_entrypoint(entrypoint);
        let request_digest = hash_parts(&[
            "forge_query_raw_intent_admission_request_v1".to_string(),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!("intent:{}", declaration.name()),
            format!("input:{}", declaration.input_digest()),
            format!("source:{}", declaration.source_lane().as_str()),
        ]);
        Ok(Self {
            family,
            entrypoint,
            declaration,
            request_digest,
        })
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        &self.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
}
