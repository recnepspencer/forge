use std::any::TypeId;
use std::collections::{BTreeSet, HashMap};

use worth_relational::facade::runtime::CustomInvariantRegistration;

use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntrySupportSnapshot, WorthQueryDomainOperatingRequirement,
};
use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::runtime::{
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRegistrationCatalog,
    WorthQueryGraphReadOperationRegistration, WorthQueryGraphReadOperationRegistry,
    WorthQueryGraphReadRegistryAdmissionError,
};

use super::{
    invariant_rule::compile_invariant_definition, WorthQueryAdmittedDomainPackage,
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainInstallationDenial,
    WorthQueryDomainInstallationDenialKind, WorthQueryDomainInvariantDefinition,
    WorthQueryDomainPackageIdentity, WorthQueryDomainSemanticVersion,
};

pub(crate) struct WorthQueryCompiledDomainPackage {
    pub(crate) custom_invariants: Vec<CustomInvariantRegistration>,
    pub(crate) graph_obligations: Vec<WorthQueryGraphObligationRegistration>,
}

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
    pub(crate) invariant_definitions: Vec<WorthQueryDomainInvariantDefinition>,
    pub(crate) graph_obligations: Vec<WorthQueryGraphObligationRegistration>,
    pub(crate) graph_read_operations: Vec<WorthQueryGraphReadOperationRegistration>,
    pub(crate) declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
    pub(crate) contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

#[derive(Default)]
pub(crate) struct WorthQueryPendingDomainInstallations {
    artifacts: Vec<WorthQueryInstalledDomainArtifact>,
    marker_types: HashMap<TypeId, String>,
    package_identities: BTreeSet<String>,
    domain_owners: BTreeSet<String>,
    invariant_slots: BTreeSet<String>,
    declaration_family_slots: BTreeSet<String>,
}

impl WorthQueryPendingDomainInstallations {
    pub(crate) fn install<D: 'static>(
        &mut self,
        package: WorthQueryAdmittedDomainPackage<D>,
    ) -> Result<WorthQueryCompiledDomainPackage, WorthQueryDomainInstallationDenial>
    where
        D: WorthQueryDomainEntryMarker,
    {
        let marker_type = TypeId::of::<D>();
        let package_identity = package.package_identity.as_str().to_string();
        let domain_owner = package.identity.canonical_owner();
        self.reject_identity_conflicts(marker_type, &package_identity, &domain_owner)?;

        let invariant_slots = package
            .invariant_definitions
            .iter()
            .map(|definition| format!("{domain_owner}:{}", definition.slot_key()))
            .collect::<Vec<_>>();
        self.reject_new_slots(
            &invariant_slots,
            &self.invariant_slots,
            WorthQueryDomainInstallationDenialKind::ConflictingInvariant,
        )?;
        let declaration_slots = package
            .declaration_families
            .iter()
            .map(|definition| format!("{domain_owner}:{}", definition.slot_key()))
            .collect::<Vec<_>>();
        self.reject_new_slots(
            &declaration_slots,
            &self.declaration_family_slots,
            WorthQueryDomainInstallationDenialKind::ConflictingDeclarationFamily,
        )?;

        let graph_read_operations = package
            .graph_read_operations
            .iter()
            .map(|definition| definition.lower_with_owner(&domain_owner))
            .collect::<Vec<_>>();
        self.validate_operation_union(&graph_read_operations)?;
        self.validate_obligation_union(&package.graph_obligations)?;

        let custom_invariants = package
            .invariant_definitions
            .iter()
            .map(|definition| compile_invariant_definition(&domain_owner, definition))
            .collect::<Result<Vec<_>, _>>()?;
        let graph_obligations = package.graph_obligations.clone();
        let artifact = WorthQueryInstalledDomainArtifact {
            marker_type,
            marker_domain_key: package.marker.domain_key(),
            marker_display_name: package.marker.display_name(),
            domain_owner: domain_owner.clone(),
            semantic_version: package.identity.semantic_version(),
            package_identity: package.package_identity,
            admission_identity: package.admission_identity,
            support_snapshot: package.support_snapshot,
            required_capabilities: package.required_capabilities,
            required_configuration: package.required_configuration,
            operating_requirements: package.operating_requirements,
            invariant_definitions: package.invariant_definitions,
            graph_obligations: package.graph_obligations,
            graph_read_operations,
            declaration_families: package.declaration_families,
            contribution_policy: package.contribution_policy,
        };

        self.marker_types
            .insert(marker_type, package_identity.clone());
        self.package_identities.insert(package_identity);
        self.domain_owners.insert(domain_owner);
        self.invariant_slots.extend(invariant_slots);
        self.declaration_family_slots.extend(declaration_slots);
        self.artifacts.push(artifact);
        Ok(WorthQueryCompiledDomainPackage {
            custom_invariants,
            graph_obligations,
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    pub(crate) fn into_artifacts(mut self) -> Vec<WorthQueryInstalledDomainArtifact> {
        self.artifacts.sort_by(|left, right| {
            left.package_identity
                .as_str()
                .cmp(right.package_identity.as_str())
        });
        self.artifacts
    }

    fn reject_identity_conflicts(
        &self,
        marker_type: TypeId,
        package_identity: &str,
        domain_owner: &str,
    ) -> Result<(), WorthQueryDomainInstallationDenial> {
        if self.marker_types.contains_key(&marker_type) {
            return Err(WorthQueryDomainInstallationDenial::new(
                WorthQueryDomainInstallationDenialKind::DuplicateMarkerType,
                domain_owner,
            ));
        }
        if self.package_identities.contains(package_identity) {
            return Err(WorthQueryDomainInstallationDenial::new(
                WorthQueryDomainInstallationDenialKind::DuplicatePackageIdentity,
                package_identity,
            ));
        }
        if self.domain_owners.contains(domain_owner) {
            return Err(WorthQueryDomainInstallationDenial::new(
                WorthQueryDomainInstallationDenialKind::ConflictingDomainOwner,
                domain_owner,
            ));
        }
        Ok(())
    }

    fn reject_new_slots(
        &self,
        new_slots: &[String],
        existing_slots: &BTreeSet<String>,
        kind: WorthQueryDomainInstallationDenialKind,
    ) -> Result<(), WorthQueryDomainInstallationDenial> {
        if let Some(conflict) = new_slots.iter().find(|slot| existing_slots.contains(*slot)) {
            return Err(WorthQueryDomainInstallationDenial::new(kind, conflict));
        }
        Ok(())
    }

    fn validate_operation_union(
        &self,
        additions: &[WorthQueryGraphReadOperationRegistration],
    ) -> Result<(), WorthQueryDomainInstallationDenial> {
        let registrations = self
            .artifacts
            .iter()
            .flat_map(|artifact| artifact.graph_read_operations.iter().cloned())
            .chain(additions.iter().cloned())
            .collect::<Vec<_>>();
        WorthQueryGraphReadOperationRegistry::admit(registrations)
            .map(|_| ())
            .map_err(operation_admission_denial)
    }

    fn validate_obligation_union(
        &self,
        additions: &[WorthQueryGraphObligationRegistration],
    ) -> Result<(), WorthQueryDomainInstallationDenial> {
        let registrations = self
            .artifacts
            .iter()
            .flat_map(|artifact| artifact.graph_obligations.iter().cloned())
            .chain(additions.iter().cloned())
            .collect::<Vec<_>>();
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(registrations)
            .map(|_| ())
            .map_err(|error| {
                WorthQueryDomainInstallationDenial::new(
                    WorthQueryDomainInstallationDenialKind::ConflictingGraphObligation,
                    error.to_string(),
                )
            })
    }
}

fn operation_admission_denial(
    error: WorthQueryGraphReadRegistryAdmissionError,
) -> WorthQueryDomainInstallationDenial {
    let kind = match error {
        WorthQueryGraphReadRegistryAdmissionError::AmbiguousDomainReferenceAdmission => {
            WorthQueryDomainInstallationDenialKind::AmbiguousGraphReadRelationScope
        }
        _ => WorthQueryDomainInstallationDenialKind::ConflictingGraphReadOperation,
    };
    WorthQueryDomainInstallationDenial::new(kind, error.as_str())
}
