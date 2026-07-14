use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityStatus,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::{
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainGraphReadOperationDefinition,
    WorthQueryDomainIdentityDeclaration, WorthQueryDomainInvariantDefinition,
    WorthQueryDomainPackageAdmissionDenial, WorthQueryDomainPackageAdmissionDenialKind,
    WorthQueryDomainPackageIdentity, WorthQueryValidatedDomainPackage,
};
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainOperatingRequirement,
};
use crate::runtime::WorthQueryGraphObligationRegistration;

pub(crate) struct WorthQueryAdmittedDomainPackage<D: WorthQueryDomainEntryMarker> {
    pub(crate) marker: D,
    pub(crate) identity: WorthQueryDomainIdentityDeclaration<D>,
    pub(crate) package_identity: WorthQueryDomainPackageIdentity,
    pub(crate) admission_identity: WorthQueryEvidenceIdentity,
    pub(crate) required_capabilities: Vec<WorthQueryCapabilityFamily>,
    pub(crate) required_configuration: Vec<WorthQueryConfigSectionFamily>,
    pub(crate) operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    pub(crate) invariant_definitions: Vec<WorthQueryDomainInvariantDefinition>,
    pub(crate) graph_obligations: Vec<WorthQueryGraphObligationRegistration>,
    pub(crate) graph_read_operations: Vec<WorthQueryDomainGraphReadOperationDefinition>,
    pub(crate) declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
    pub(crate) contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

pub(crate) fn admit_domain_package<D: WorthQueryDomainEntryMarker>(
    package: WorthQueryValidatedDomainPackage<D>,
) -> Result<WorthQueryAdmittedDomainPackage<D>, WorthQueryDomainPackageAdmissionDenial> {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let support_matrix = facade.support_matrix();
    for family in &package.required_capabilities {
        let status = support_matrix
            .descriptor(*family)
            .map(|descriptor| descriptor.status())
            .unwrap_or(WorthQueryCapabilityStatus::Unsupported);
        match status {
            WorthQueryCapabilityStatus::Admitted => {}
            WorthQueryCapabilityStatus::DeferredDebt => {
                return Err(WorthQueryDomainPackageAdmissionDenial::new(
                    WorthQueryDomainPackageAdmissionDenialKind::DeferredCapability,
                    family.as_str(),
                ));
            }
            WorthQueryCapabilityStatus::Unsupported => {
                return Err(WorthQueryDomainPackageAdmissionDenial::new(
                    WorthQueryDomainPackageAdmissionDenialKind::UnsupportedCapability,
                    family.as_str(),
                ));
            }
        }
    }

    for section in &package.required_configuration {
        if !facade
            .validated_config()
            .resolve_section(*section)
            .enabled()
        {
            return Err(WorthQueryDomainPackageAdmissionDenial::new(
                WorthQueryDomainPackageAdmissionDenialKind::DisabledConfiguration,
                section.as_str(),
            ));
        }
    }

    let admission_identity =
        worth_query_evidence_identity(WorthQueryEvidenceScope::DomainPackageAdmission)
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("package"),
                package.package_identity.evidence_identity(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("support"),
                support_matrix.support_matrix_digest(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("configuration"),
                facade.validated_config().validated_digest(),
            )
            .seal();

    Ok(WorthQueryAdmittedDomainPackage {
        marker: package.marker,
        identity: package.identity,
        package_identity: package.package_identity,
        admission_identity,
        required_capabilities: package.required_capabilities,
        required_configuration: package.required_configuration,
        operating_requirements: package.operating_requirements,
        invariant_definitions: package.invariant_definitions,
        graph_obligations: package.graph_obligations,
        graph_read_operations: package.graph_read_operations,
        declaration_families: package.declaration_families,
        contribution_policy: package.contribution_policy,
    })
}
