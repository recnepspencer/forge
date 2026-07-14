#[cfg(test)]
use super::admitted_handle::WorthQueryInstalledDomainDeclarationContext;
#[cfg(test)]
use super::operating_context::{
    WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement,
};
#[cfg(test)]
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedWorldBasis {
    domain_key: &'static str,
    display_name: &'static str,
    operating_context_identity: WorthQueryEvidenceIdentity,
    handle_identity: WorthQueryEvidenceIdentity,
    support_snapshot_digest: String,
    basis_lifecycle_support_identity: WorthQueryEvidenceIdentity,
    installed_authority: crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness,
}

impl WorthQueryAdmittedWorldBasis {
    pub(crate) fn new(
        domain_key: &'static str,
        display_name: &'static str,
        operating_context_identity: WorthQueryEvidenceIdentity,
        handle_identity: WorthQueryEvidenceIdentity,
        support_snapshot_digest: String,
        basis_lifecycle_support_identity: WorthQueryEvidenceIdentity,
        installed_authority: crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness,
    ) -> Self {
        Self {
            domain_key,
            display_name,
            operating_context_identity,
            handle_identity,
            support_snapshot_digest,
            basis_lifecycle_support_identity,
            installed_authority,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }

    pub fn display_name(&self) -> &'static str {
        self.display_name
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.operating_context_identity.as_str()
    }

    pub fn operating_context_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.operating_context_identity
    }

    pub fn handle_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.handle_identity
    }

    pub fn handle_identity_for_reporting(&self) -> &str {
        self.handle_identity.as_str()
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn basis_lifecycle_support_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_lifecycle_support_identity
    }

    pub fn basis_lifecycle_support_for_reporting(&self) -> &str {
        self.basis_lifecycle_support_identity.as_str()
    }

    pub fn installed_authority(
        &self,
    ) -> &crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness {
        &self.installed_authority
    }
}

#[cfg(test)]
pub(crate) fn compose_admitted_configured_domain_handle_identity_parts(
    domain_key: &str,
    display_name: &str,
    required_capability_families: &[WorthQueryCapabilityFamily],
    required_config_sections: &[WorthQueryConfigSectionFamily],
    required_operating_requirements: &[WorthQueryDomainOperatingRequirement],
    operating_context_identity_digest: &str,
    validated_config_digest: &str,
) -> WorthQueryEvidenceIdentity {
    let required_capabilities = required_capability_families
        .iter()
        .map(WorthQueryCapabilityFamily::as_str)
        .collect::<Vec<_>>()
        .join(",");
    let required_sections = required_config_sections
        .iter()
        .map(WorthQueryConfigSectionFamily::as_str)
        .collect::<Vec<_>>()
        .join(",");
    let operating_requirements = required_operating_requirements
        .iter()
        .copied()
        .map(WorthQueryDomainOperatingRequirement::as_str)
        .collect::<Vec<_>>()
        .join(",");

    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "admitted_configured_domain_handle_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("domain"), domain_key)
        .field_shape(WorthQueryEvidenceTag::new("display"), display_name)
        .field_shape(
            WorthQueryEvidenceTag::new("required_capabilities"),
            &required_capabilities,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("required_sections"),
            &required_sections,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("operating_requirements"),
            &operating_requirements,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("context"),
            operating_context_identity_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("validated_config"),
            validated_config_digest,
        )
        .seal()
}

#[cfg(test)]
pub(crate) fn compose_admitted_configured_domain_handle_identity<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
) -> WorthQueryEvidenceIdentity {
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

pub(crate) fn compose_basis_lifecycle_support_identity(
    matrix_digest: &str,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "basis_lifecycle_support_matrix_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("support_matrix"), matrix_digest)
        .seal()
}
