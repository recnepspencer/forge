use worth_query::facade::{
    WORTHQueryCanonicalDeclarationArtifact, WORTHQueryDeclarationEnvelope,
    WORTHQueryDeclarationInput,
};

use crate::query_entry::HadwigerResearchDomainEntry;

use super::core_artifact::canonical_digest_token;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerQueryDeclarationReference {
    domain_key: &'static str,
    handle_identity_digest: String,
    declaration_family_key: &'static str,
    declaration_digest: String,
    canonicalization_version: String,
}

impl HadwigerQueryDeclarationReference {
    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn canonicalization_version(&self) -> &str {
        &self.canonicalization_version
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.domain_key,
            self.handle_identity_digest,
            self.declaration_family_key,
            self.declaration_digest,
            self.canonicalization_version
        )
    }
}

impl<I> From<WORTHQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>>
    for HadwigerQueryDeclarationReference
where
    I: WORTHQueryDeclarationInput<HadwigerResearchDomainEntry>,
{
    fn from(
        declaration: WORTHQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>,
    ) -> Self {
        Self {
            domain_key: "WORTH.hadwiger.research",
            handle_identity_digest: declaration.handle_identity_digest().to_string(),
            declaration_family_key: declaration.declaration_family_key(),
            declaration_digest: canonical_digest_token(declaration.declaration_digest()),
            canonicalization_version: declaration
                .canonicalization_version()
                .foundational()
                .as_str()
                .to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerQueryEnvelopeReference {
    domain_key: &'static str,
    declaration_family_key: &'static str,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
}

impl HadwigerQueryEnvelopeReference {
    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{:?}:{:?}:{}:{}",
            self.domain_key,
            self.declaration_family_key,
            self.handle_identity_digest,
            self.operating_context_identity_digest,
            self.declaration_digest,
            self.progression_digest,
            self.route_plan_digest,
            self.receipt_digest,
            self.envelope_digest
        )
    }
}

impl<I> From<WORTHQueryDeclarationEnvelope<HadwigerResearchDomainEntry, I>>
    for HadwigerQueryEnvelopeReference
where
    I: WORTHQueryDeclarationInput<HadwigerResearchDomainEntry>,
{
    fn from(envelope: WORTHQueryDeclarationEnvelope<HadwigerResearchDomainEntry, I>) -> Self {
        Self {
            domain_key: "WORTH.hadwiger.research",
            declaration_family_key: envelope.declaration_family_key(),
            handle_identity_digest: envelope.handle_identity_digest().to_string(),
            operating_context_identity_digest: envelope
                .operating_context_identity_digest()
                .to_string(),
            declaration_digest: envelope.declaration_digest().to_string(),
            progression_digest: envelope.progression_digest().map(ToOwned::to_owned),
            route_plan_digest: envelope.route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: canonical_digest_token(envelope.receipt_digest()),
            envelope_digest: canonical_digest_token(envelope.envelope_digest()),
        }
    }
}
