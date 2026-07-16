use std::collections::BTreeMap;

use super::{
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainGraphObligationDefinition,
    WorthQueryDomainGraphReadOperationDefinition, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainInvariantDefinition, WorthQueryDomainPackage, WorthQueryDomainPackageIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainPackageValidationDenialKind {
    MarkerIdentityMismatch,
    MissingMarkerCapability,
    DuplicateInvariant,
    ConflictingInvariant,
    DuplicateGraphObligation,
    ConflictingGraphObligation,
    DuplicateGraphReadOperation,
    ConflictingGraphReadOperation,
    DuplicateDeclarationFamily,
    ConflictingDeclarationFamily,
    DuplicateContributionCategory,
    EmptyGraphReadRelationSet,
    InvalidInvariantPredicate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainPackageValidationDenial {
    kind: WorthQueryDomainPackageValidationDenialKind,
    detail: String,
}

impl WorthQueryDomainPackageValidationDenial {
    pub(crate) fn new(
        kind: WorthQueryDomainPackageValidationDenialKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryDomainPackageValidationDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for WorthQueryDomainPackageValidationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.detail)
    }
}

impl std::error::Error for WorthQueryDomainPackageValidationDenial {}
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingRequirement,
};

pub(crate) struct WorthQueryValidatedDomainPackage<D: WorthQueryDomainEntryMarker> {
    pub(crate) marker: D,
    pub(crate) identity: WorthQueryDomainIdentityDeclaration<D>,
    pub(crate) package_identity: WorthQueryDomainPackageIdentity,
    pub(crate) required_capabilities: Vec<WorthQueryCapabilityFamily>,
    pub(crate) required_configuration: Vec<WorthQueryConfigSectionFamily>,
    pub(crate) operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    pub(crate) invariant_definitions: Vec<WorthQueryDomainInvariantDefinition>,
    pub(crate) graph_obligations: Vec<WorthQueryDomainGraphObligationDefinition>,
    pub(crate) graph_read_operations: Vec<WorthQueryDomainGraphReadOperationDefinition>,
    pub(crate) declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
    pub(crate) contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryValidatedDomainPackage<D> {
    #[cfg(test)]
    pub fn identity(&self) -> &WorthQueryDomainPackageIdentity {
        &self.package_identity
    }
    #[cfg(test)]
    pub fn invariant_count(&self) -> usize {
        self.invariant_definitions.len()
    }
    #[cfg(test)]
    pub fn graph_obligation_count(&self) -> usize {
        self.graph_obligations.len()
    }
    #[cfg(test)]
    pub fn graph_read_operation_count(&self) -> usize {
        self.graph_read_operations.len()
    }
    #[cfg(test)]
    pub fn declaration_family_count(&self) -> usize {
        self.declaration_families.len()
    }
    #[cfg(test)]
    pub fn contribution_category_count(&self) -> usize {
        self.contribution_policy.len()
    }
}

pub(super) fn validate_domain_package<D: WorthQueryDomainEntryMarker>(
    package: WorthQueryDomainPackage<D>,
) -> Result<WorthQueryValidatedDomainPackage<D>, WorthQueryDomainPackageValidationDenial> {
    let marker_owner = package.marker.domain_key();
    let identity_owner = package.identity.canonical_owner();
    if marker_owner != identity_owner {
        return Err(WorthQueryDomainPackageValidationDenial::new(
            WorthQueryDomainPackageValidationDenialKind::MarkerIdentityMismatch,
            format!(
                "domain marker `{marker_owner}` does not match package identity `{identity_owner}`"
            ),
        ));
    }
    if let Some(missing) = package
        .marker
        .required_capability_families()
        .iter()
        .find(|family| !package.required_capabilities.contains(family))
    {
        return Err(WorthQueryDomainPackageValidationDenial::new(
            WorthQueryDomainPackageValidationDenialKind::MissingMarkerCapability,
            format!(
                "domain marker `{marker_owner}` requires package capability `{}`",
                missing.as_str()
            ),
        ));
    }
    let mut validated = WorthQueryValidatedDomainPackage {
        marker: package.marker,
        identity: package.identity,
        package_identity: WorthQueryDomainPackageIdentity::new(
            crate::evidence_identity::worth_query_evidence_identity(
                crate::evidence_identity::WorthQueryEvidenceScope::DomainPackageValidation,
            )
            .seal(),
        ),
        required_capabilities: canonicalize(package.required_capabilities),
        required_configuration: canonicalize(package.required_configuration),
        operating_requirements: canonicalize(package.operating_requirements),
        invariant_definitions: package.invariant_definitions,
        graph_obligations: package.graph_obligations,
        graph_read_operations: package.graph_read_operations,
        declaration_families: package.declaration_families,
        contribution_policy: package.contribution_policy,
    };

    canonicalize_and_validate_invariants(&mut validated.invariant_definitions)?;
    canonicalize_and_validate_obligations(&mut validated.graph_obligations)?;
    canonicalize_and_validate_operations(&mut validated.graph_read_operations)?;
    canonicalize_and_validate_families(&mut validated.declaration_families)?;
    canonicalize_and_validate_contributions(&mut validated.contribution_policy)?;
    validated.package_identity = super::canonical_identity::canonical_package_identity(&validated);
    Ok(validated)
}

fn canonicalize<T: Ord>(mut values: Vec<T>) -> Vec<T> {
    values.sort();
    values.dedup();
    values
}

fn canonicalize_and_validate_invariants(
    definitions: &mut Vec<WorthQueryDomainInvariantDefinition>,
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    if let Some(invalid) = definitions.iter().find(|definition| {
        matches!(
            definition.predicate(),
            super::WorthQueryDomainInvariantPredicate::RequiresOutgoingRelations {
                relevant_entity_kinds,
                required_relation_kinds,
                traversal_depth,
            } if relevant_entity_kinds.is_empty()
                || required_relation_kinds.is_empty()
                || *traversal_depth == 0
        )
    }) {
        return Err(WorthQueryDomainPackageValidationDenial::new(
            WorthQueryDomainPackageValidationDenialKind::InvalidInvariantPredicate,
            invalid.name().as_str(),
        ));
    }
    definitions.sort_by_key(WorthQueryDomainInvariantDefinition::canonical_part);
    validate_slots(
        definitions,
        WorthQueryDomainInvariantDefinition::slot_key,
        WorthQueryDomainInvariantDefinition::canonical_part,
        WorthQueryDomainPackageValidationDenialKind::DuplicateInvariant,
        WorthQueryDomainPackageValidationDenialKind::ConflictingInvariant,
        "invariant",
    )
}

fn canonicalize_and_validate_obligations(
    definitions: &mut Vec<WorthQueryDomainGraphObligationDefinition>,
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    definitions.sort_by_key(WorthQueryDomainGraphObligationDefinition::canonical_part);
    validate_slots(
        definitions,
        WorthQueryDomainGraphObligationDefinition::slot_key,
        WorthQueryDomainGraphObligationDefinition::canonical_part,
        WorthQueryDomainPackageValidationDenialKind::DuplicateGraphObligation,
        WorthQueryDomainPackageValidationDenialKind::ConflictingGraphObligation,
        "graph obligation",
    )
}

fn canonicalize_and_validate_operations(
    definitions: &mut Vec<WorthQueryDomainGraphReadOperationDefinition>,
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    if let Some(empty) = definitions
        .iter()
        .find(|definition| definition.accepted_relations().is_empty())
    {
        return Err(WorthQueryDomainPackageValidationDenial::new(
            WorthQueryDomainPackageValidationDenialKind::EmptyGraphReadRelationSet,
            empty.name().as_str(),
        ));
    }
    definitions.sort_by_key(WorthQueryDomainGraphReadOperationDefinition::canonical_part);
    validate_slots(
        definitions,
        WorthQueryDomainGraphReadOperationDefinition::slot_key,
        WorthQueryDomainGraphReadOperationDefinition::canonical_part,
        WorthQueryDomainPackageValidationDenialKind::DuplicateGraphReadOperation,
        WorthQueryDomainPackageValidationDenialKind::ConflictingGraphReadOperation,
        "graph read operation",
    )
}

fn canonicalize_and_validate_families(
    definitions: &mut Vec<WorthQueryDomainDeclarationFamilyDefinition>,
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    definitions.sort_by_key(WorthQueryDomainDeclarationFamilyDefinition::canonical_part);
    validate_slots(
        definitions,
        |definition| definition.slot_key().to_string(),
        WorthQueryDomainDeclarationFamilyDefinition::canonical_part,
        WorthQueryDomainPackageValidationDenialKind::DuplicateDeclarationFamily,
        WorthQueryDomainPackageValidationDenialKind::ConflictingDeclarationFamily,
        "declaration family",
    )
}

fn canonicalize_and_validate_contributions(
    categories: &mut Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    categories.sort_by_key(|category| category.as_str());
    if categories.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(WorthQueryDomainPackageValidationDenial::new(
            WorthQueryDomainPackageValidationDenialKind::DuplicateContributionCategory,
            "duplicate contribution category",
        ));
    }
    Ok(())
}

fn validate_slots<T>(
    values: &[T],
    slot: impl Fn(&T) -> String,
    canonical: impl Fn(&T) -> String,
    duplicate_kind: WorthQueryDomainPackageValidationDenialKind,
    conflict_kind: WorthQueryDomainPackageValidationDenialKind,
    label: &str,
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    let mut observed = BTreeMap::<String, String>::new();
    for value in values {
        let slot_key = slot(value);
        let canonical_part = canonical(value);
        if let Some(existing) = observed.insert(slot_key.clone(), canonical_part.clone()) {
            let kind = if existing == canonical_part {
                duplicate_kind
            } else {
                conflict_kind
            };
            return Err(WorthQueryDomainPackageValidationDenial::new(
                kind,
                format!("{label} slot `{slot_key}` is registered more than once"),
            ));
        }
    }
    Ok(())
}
