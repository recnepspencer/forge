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
    InvalidPortablePackage,
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
    pub(crate) portable_package:
        worth_query_installation::facade::WorthQueryValidatedPortableDomainPackage,
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
    let required_capabilities = canonicalize(package.required_capabilities);
    let required_configuration = canonicalize(package.required_configuration);
    let operating_requirements = canonicalize(package.operating_requirements);
    let mut invariant_definitions = package.invariant_definitions;
    let mut graph_obligations = package.graph_obligations;
    let mut graph_read_operations = package.graph_read_operations;
    let mut declaration_families = package.declaration_families;
    let mut contribution_policy = package.contribution_policy;

    validate_invariant_predicates(&invariant_definitions)?;
    validate_operation_relations(&graph_read_operations)?;

    let portable_package = super::portable_validation::validate_portable_package(
        super::portable_validation::WorthQueryPortablePackageDeclaration {
            identity: &package.identity,
            required_capabilities: &required_capabilities,
            required_configuration: &required_configuration,
            operating_requirements: &operating_requirements,
            invariant_definitions: &invariant_definitions,
            graph_obligations: &graph_obligations,
            graph_read_operations: &graph_read_operations,
            declaration_families: &declaration_families,
            contribution_policy: &contribution_policy,
        },
    )
    .map_err(map_portable_validation_denial)?;

    invariant_definitions.sort_by_key(WorthQueryDomainInvariantDefinition::canonical_part);
    graph_obligations.sort_by_key(WorthQueryDomainGraphObligationDefinition::canonical_part);
    graph_read_operations.sort_by_key(WorthQueryDomainGraphReadOperationDefinition::canonical_part);
    declaration_families.sort_by_key(WorthQueryDomainDeclarationFamilyDefinition::canonical_part);
    contribution_policy.sort_by_key(|category| category.as_str());
    let package_identity = WorthQueryDomainPackageIdentity::new(
        crate::evidence_identity::worth_query_evidence_identity(
            crate::evidence_identity::WorthQueryEvidenceScope::DomainPackageIdentity,
        )
        .field_value(
            crate::evidence_identity::WorthQueryEvidenceTag::new("portable_package"),
            portable_package.identity().as_str(),
        )
        .seal(),
    );
    Ok(WorthQueryValidatedDomainPackage {
        marker: package.marker,
        identity: package.identity,
        package_identity,
        required_capabilities,
        required_configuration,
        operating_requirements,
        invariant_definitions,
        graph_obligations,
        graph_read_operations,
        declaration_families,
        contribution_policy,
        portable_package,
    })
}

fn canonicalize<T: Ord>(mut values: Vec<T>) -> Vec<T> {
    values.sort();
    values.dedup();
    values
}

fn validate_invariant_predicates(
    definitions: &[WorthQueryDomainInvariantDefinition],
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
    Ok(())
}

fn validate_operation_relations(
    definitions: &[WorthQueryDomainGraphReadOperationDefinition],
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
    Ok(())
}

fn map_portable_validation_denial(
    denial: worth_query_installation::facade::WorthQueryPortablePackageValidationDenial,
) -> WorthQueryDomainPackageValidationDenial {
    use worth_query_installation::facade::{
        WorthQueryPortableDefinitionKind as DefinitionKind,
        WorthQueryPortablePackageValidationDenialKind as DenialKind,
    };

    let kind = match (denial.kind(), denial.definition_kind()) {
        (DenialKind::DuplicateContributionCategory, _) => {
            WorthQueryDomainPackageValidationDenialKind::DuplicateContributionCategory
        }
        (DenialKind::DuplicateDefinition, Some(DefinitionKind::Invariant)) => {
            WorthQueryDomainPackageValidationDenialKind::DuplicateInvariant
        }
        (DenialKind::ConflictingDefinition, Some(DefinitionKind::Invariant)) => {
            WorthQueryDomainPackageValidationDenialKind::ConflictingInvariant
        }
        (DenialKind::DuplicateDefinition, Some(DefinitionKind::GraphObligation)) => {
            WorthQueryDomainPackageValidationDenialKind::DuplicateGraphObligation
        }
        (DenialKind::ConflictingDefinition, Some(DefinitionKind::GraphObligation)) => {
            WorthQueryDomainPackageValidationDenialKind::ConflictingGraphObligation
        }
        (DenialKind::DuplicateDefinition, Some(DefinitionKind::GraphReadOperation)) => {
            WorthQueryDomainPackageValidationDenialKind::DuplicateGraphReadOperation
        }
        (DenialKind::ConflictingDefinition, Some(DefinitionKind::GraphReadOperation)) => {
            WorthQueryDomainPackageValidationDenialKind::ConflictingGraphReadOperation
        }
        (DenialKind::DuplicateDefinition, Some(DefinitionKind::DeclarationFamily)) => {
            WorthQueryDomainPackageValidationDenialKind::DuplicateDeclarationFamily
        }
        (DenialKind::ConflictingDefinition, Some(DefinitionKind::DeclarationFamily)) => {
            WorthQueryDomainPackageValidationDenialKind::ConflictingDeclarationFamily
        }
        _ => WorthQueryDomainPackageValidationDenialKind::InvalidPortablePackage,
    };
    WorthQueryDomainPackageValidationDenial::new(kind, denial.slot())
}
