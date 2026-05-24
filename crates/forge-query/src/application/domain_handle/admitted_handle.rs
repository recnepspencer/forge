use super::operating_context::ForgeQueryDomainOperatingContext;
use crate::application::{
    forge_query_canonical_declaration, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalizationError, ForgeQueryDeclarationCanonicalizationVersion,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainEntrySupportSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedConfiguredDomainHandle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    required_capability_families: Vec<ForgeQueryCapabilityFamily>,
    required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
    handle_identity_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
        required_capability_families: Vec<ForgeQueryCapabilityFamily>,
        required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
        handle_identity_digest: String,
    ) -> Self {
        Self {
            marker,
            operating_context,
            support_snapshot,
            required_capability_families,
            required_config_sections,
            handle_identity_digest,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.marker.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.marker.display_name()
    }

    pub fn operating_context(&self) -> &C {
        &self.operating_context
    }

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn required_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &[ForgeQueryConfigSectionFamily] {
        &self.required_config_sections
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn declare<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryCanonicalDeclarationArtifact<D, I>,
        ForgeQueryDeclarationCanonicalizationError,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_canonical_declaration(
            self,
            input,
            ForgeQueryDeclarationCanonicalizationVersion::default(),
        )
    }

    pub fn declare_with_version<I>(
        &self,
        input: I,
        version: ForgeQueryDeclarationCanonicalizationVersion,
    ) -> Result<
        ForgeQueryCanonicalDeclarationArtifact<D, I>,
        ForgeQueryDeclarationCanonicalizationError,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_canonical_declaration(self, input, version)
    }
}
