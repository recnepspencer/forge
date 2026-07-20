use super::{WorthQueryDomainPackageValidationDenial, WorthQueryDomainPackageValidationDenialKind};
use worth_query_installation::facade::{
    WorthQueryPortableDefinitionKind as DefinitionKind,
    WorthQueryPortablePackageValidationDenialKind as DenialKind,
};

pub(super) fn map_portable_validation_denial(
    denial: worth_query_installation::facade::WorthQueryPortablePackageValidationDenial,
) -> WorthQueryDomainPackageValidationDenial {
    let kind = match (denial.kind(), denial.definition_kind()) {
        (DenialKind::InvalidDomainOperation, _) => {
            WorthQueryDomainPackageValidationDenialKind::InvalidDomainOperation
        }
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
        (DenialKind::DuplicateDefinition, Some(DefinitionKind::DomainOperation)) => {
            WorthQueryDomainPackageValidationDenialKind::DuplicateDomainOperation
        }
        (DenialKind::ConflictingDefinition, Some(DefinitionKind::DomainOperation)) => {
            WorthQueryDomainPackageValidationDenialKind::ConflictingDomainOperation
        }
        _ => WorthQueryDomainPackageValidationDenialKind::InvalidPortablePackage,
    };
    WorthQueryDomainPackageValidationDenial::new(kind, denial.slot())
}
