use worth_query::facade::foundation::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationInput,
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

impl<I> From<WorthQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>>
    for HadwigerQueryDeclarationReference
where
    I: WorthQueryDeclarationInput<HadwigerResearchDomainEntry>,
{
    fn from(
        declaration: WorthQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>,
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
