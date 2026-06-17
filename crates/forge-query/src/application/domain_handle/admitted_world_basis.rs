use super::admitted_handle::ForgeQueryAdmittedConfiguredDomainHandle;
use super::operating_context::{
    ForgeQueryDomainOperatingContext, ForgeQueryDomainOperatingRequirement,
};
use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedWorldBasis {
    domain_key: &'static str,
    display_name: &'static str,
    operating_context_identity_digest: String,
    handle_identity: ForgeQueryEvidenceIdentity,
    support_snapshot_digest: String,
    basis_lifecycle_support_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryAdmittedWorldBasis {
    pub(crate) fn new(
        domain_key: &'static str,
        display_name: &'static str,
        operating_context_identity_digest: String,
        handle_identity: ForgeQueryEvidenceIdentity,
        support_snapshot_digest: String,
        basis_lifecycle_support_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            domain_key,
            display_name,
            operating_context_identity_digest,
            handle_identity,
            support_snapshot_digest,
            basis_lifecycle_support_identity,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }

    pub fn display_name(&self) -> &'static str {
        self.display_name
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn handle_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.handle_identity
    }

    pub fn handle_identity_for_reporting(&self) -> &str {
        self.handle_identity.as_str()
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn basis_lifecycle_support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_lifecycle_support_identity
    }

    pub fn basis_lifecycle_support_for_reporting(&self) -> &str {
        self.basis_lifecycle_support_identity.as_str()
    }
}

pub(crate) fn compose_admitted_configured_domain_handle_identity_parts(
    domain_key: &str,
    display_name: &str,
    required_capability_families: &[ForgeQueryCapabilityFamily],
    required_config_sections: &[ForgeQueryConfigSectionFamily],
    required_operating_requirements: &[ForgeQueryDomainOperatingRequirement],
    operating_context_identity_digest: &str,
    validated_config_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    let required_capabilities = required_capability_families
        .iter()
        .map(ForgeQueryCapabilityFamily::as_str)
        .collect::<Vec<_>>()
        .join(",");
    let required_sections = required_config_sections
        .iter()
        .map(ForgeQueryConfigSectionFamily::as_str)
        .collect::<Vec<_>>()
        .join(",");
    let operating_requirements = required_operating_requirements
        .iter()
        .copied()
        .map(ForgeQueryDomainOperatingRequirement::as_str)
        .collect::<Vec<_>>()
        .join(",");

    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "admitted_configured_domain_handle_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("domain"), domain_key)
        .field_shape(ForgeQueryEvidenceTag::new("display"), display_name)
        .field_shape(
            ForgeQueryEvidenceTag::new("required_capabilities"),
            &required_capabilities,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("required_sections"),
            &required_sections,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("operating_requirements"),
            &operating_requirements,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("context"),
            operating_context_identity_digest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("validated_config"),
            validated_config_digest,
        )
        .seal()
}

pub(crate) fn compose_admitted_configured_domain_handle_identity<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryEvidenceIdentity {
    compose_admitted_configured_domain_handle_identity_parts(
        handle.domain_key(),
        handle.display_name(),
        handle.required_capability_families(),
        handle.required_config_sections(),
        handle.required_operating_requirements(),
        handle.operating_context_identity_digest(),
        handle.support_snapshot().validated_config_digest(),
    )
}
