use crate::application::WorthQueryDomainEntryMarker;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::{WorthQueryDomainPackageIdentity, WorthQueryValidatedDomainPackage};

pub(super) fn canonical_package_identity<D: WorthQueryDomainEntryMarker>(
    package: &WorthQueryValidatedDomainPackage<D>,
) -> WorthQueryDomainPackageIdentity {
    let capability_parts = package
        .required_capabilities
        .iter()
        .map(|family| family.as_str())
        .collect::<Vec<_>>();
    let configuration_parts = package
        .required_configuration
        .iter()
        .map(|section| section.as_str())
        .collect::<Vec<_>>();
    let operating_parts = package
        .operating_requirements
        .iter()
        .map(|requirement| requirement.as_str())
        .collect::<Vec<_>>();
    let invariant_parts = package
        .invariant_definitions
        .iter()
        .map(|definition| definition.canonical_part())
        .collect::<Vec<_>>();
    let obligation_parts = package
        .graph_obligations
        .iter()
        .map(|registration| registration.registration_digest())
        .collect::<Vec<_>>();
    let operation_parts = package
        .graph_read_operations
        .iter()
        .map(|definition| definition.canonical_part())
        .collect::<Vec<_>>();
    let family_parts = package
        .declaration_families
        .iter()
        .map(|definition| definition.canonical_part())
        .collect::<Vec<_>>();
    let contribution_parts = package
        .contribution_policy
        .iter()
        .map(|category| category.as_str())
        .collect::<Vec<_>>();

    WorthQueryDomainPackageIdentity::new(
        worth_query_evidence_identity(WorthQueryEvidenceScope::DomainPackageIdentity)
            .field_usize(WorthQueryEvidenceTag::new("schema_version"), 1)
            .field_value(
                WorthQueryEvidenceTag::new("domain_identity"),
                package.identity.canonical_part(),
            )
            .field_value_sequence(WorthQueryEvidenceTag::new("capability"), capability_parts)
            .field_value_sequence(
                WorthQueryEvidenceTag::new("configuration"),
                configuration_parts,
            )
            .field_value_sequence(WorthQueryEvidenceTag::new("operating"), operating_parts)
            .field_value_sequence(
                WorthQueryEvidenceTag::new("invariant"),
                invariant_parts.iter().map(String::as_str),
            )
            .field_value_sequence(WorthQueryEvidenceTag::new("obligation"), obligation_parts)
            .field_value_sequence(
                WorthQueryEvidenceTag::new("operation"),
                operation_parts.iter().map(String::as_str),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("declaration_family"),
                family_parts.iter().map(String::as_str),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("contribution"),
                contribution_parts,
            )
            .seal(),
    )
}
