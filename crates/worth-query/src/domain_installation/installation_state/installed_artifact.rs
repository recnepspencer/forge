use std::any::TypeId;

use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntrySupportSnapshot, WorthQueryDomainOperatingRequirement,
};
use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::runtime::{
    WorthQueryGraphReadOperationRegistration, WorthQueryInstalledDomainSubstrateProvenance,
};

use super::{
    WorthQueryAdmittedDomainPackage, WorthQueryDomainDeclarationFamilyDefinition,
    WorthQueryDomainGraphObligationDefinition, WorthQueryDomainInvariantDefinition,
    WorthQueryDomainPackageIdentity, WorthQueryDomainSemanticVersion,
    WorthQueryPendingPackageCandidate,
};

#[derive(Clone)]
pub(crate) struct WorthQueryInstalledDomainArtifact {
    pub(crate) marker_type: TypeId,
    pub(crate) marker_domain_key: &'static str,
    pub(crate) marker_display_name: &'static str,
    pub(crate) domain_owner: String,
    pub(crate) semantic_version: WorthQueryDomainSemanticVersion,
    pub(crate) package_identity: WorthQueryDomainPackageIdentity,
    pub(crate) admission_identity: WorthQueryEvidenceIdentity,
    pub(crate) support_snapshot: WorthQueryDomainEntrySupportSnapshot,
    pub(crate) required_capabilities: Vec<WorthQueryCapabilityFamily>,
    pub(crate) required_configuration: Vec<WorthQueryConfigSectionFamily>,
    pub(crate) operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    pub(crate) substrate_provenance: WorthQueryInstalledDomainSubstrateProvenance,
    pub(crate) invariant_definitions: Vec<WorthQueryDomainInvariantDefinition>,
    pub(crate) graph_obligation_definitions: Vec<WorthQueryDomainGraphObligationDefinition>,
    pub(crate) graph_read_operations: Vec<WorthQueryGraphReadOperationRegistration>,
    pub(crate) declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
    pub(crate) contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

pub(super) fn assemble_installed_domain_artifact<D: WorthQueryDomainEntryMarker>(
    package: WorthQueryAdmittedDomainPackage<D>,
    candidate: &WorthQueryPendingPackageCandidate,
    graph_read_operations: Vec<WorthQueryGraphReadOperationRegistration>,
) -> WorthQueryInstalledDomainArtifact {
    WorthQueryInstalledDomainArtifact {
        marker_type: candidate.marker_type,
        marker_domain_key: package.marker.domain_key(),
        marker_display_name: package.marker.display_name(),
        domain_owner: candidate.domain_owner.clone(),
        semantic_version: candidate.semantic_version,
        package_identity: package.package_identity,
        admission_identity: package.admission_identity,
        support_snapshot: package.support_snapshot,
        required_capabilities: package.required_capabilities,
        required_configuration: package.required_configuration,
        operating_requirements: package.operating_requirements,
        substrate_provenance: candidate.substrate_provenance.clone(),
        invariant_definitions: package.invariant_definitions,
        graph_obligation_definitions: package.graph_obligations,
        graph_read_operations,
        declaration_families: package.declaration_families,
        contribution_policy: package.contribution_policy,
    }
}
