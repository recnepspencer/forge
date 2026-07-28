use super::WorthQueryPortableDefinitionKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPortablePackageValidationDenialKind {
    EmptyDomainOwner,
    EmptyDefinitionSlot,
    EmptyDefinitionSemantics,
    EmptyRequirement,
    DuplicateContributionCategory,
    DuplicateDefinition,
    ConflictingDefinition,
    InvalidDomainOperation,
    DuplicateArtifactContract,
    ConflictingArtifactContract,
    ApplicationSchemaIdentityMismatch,
    DuplicateApplicationSchema,
    ConflictingApplicationSchema,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortablePackageValidationDenial {
    kind: WorthQueryPortablePackageValidationDenialKind,
    definition_kind: Option<WorthQueryPortableDefinitionKind>,
    slot: String,
}

impl WorthQueryPortablePackageValidationDenial {
    pub(super) fn empty_domain_owner() -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::EmptyDomainOwner,
            None,
            "domain-owner",
        )
    }

    pub(super) fn empty_definition_slot(kind: WorthQueryPortableDefinitionKind) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::EmptyDefinitionSlot,
            Some(kind),
            "definition-slot",
        )
    }

    pub(super) fn empty_definition_semantics(
        kind: WorthQueryPortableDefinitionKind,
        slot: impl Into<String>,
    ) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::EmptyDefinitionSemantics,
            Some(kind),
            slot,
        )
    }

    pub(super) fn empty_requirement(slot: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::EmptyRequirement,
            None,
            slot,
        )
    }

    pub(super) fn duplicate_contribution_category() -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::DuplicateContributionCategory,
            None,
            "contribution",
        )
    }

    pub(super) fn duplicate_definition(
        kind: WorthQueryPortableDefinitionKind,
        slot: impl Into<String>,
    ) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::DuplicateDefinition,
            Some(kind),
            slot,
        )
    }

    pub(super) fn conflicting_definition(
        kind: WorthQueryPortableDefinitionKind,
        slot: impl Into<String>,
    ) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::ConflictingDefinition,
            Some(kind),
            slot,
        )
    }

    pub(super) fn invalid_domain_operation(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::InvalidDomainOperation,
            Some(WorthQueryPortableDefinitionKind::DomainOperation),
            subject,
        )
    }

    pub(super) fn duplicate_artifact_contract(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::DuplicateArtifactContract,
            None,
            subject,
        )
    }

    pub(super) fn conflicting_artifact_contract(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::ConflictingArtifactContract,
            None,
            subject,
        )
    }

    pub(super) fn application_schema_identity_mismatch(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::ApplicationSchemaIdentityMismatch,
            None,
            subject,
        )
    }

    pub(super) fn duplicate_application_schema(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::DuplicateApplicationSchema,
            None,
            subject,
        )
    }

    pub(super) fn conflicting_application_schema(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortablePackageValidationDenialKind::ConflictingApplicationSchema,
            None,
            subject,
        )
    }

    fn new(
        kind: WorthQueryPortablePackageValidationDenialKind,
        definition_kind: Option<WorthQueryPortableDefinitionKind>,
        slot: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            definition_kind,
            slot: slot.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryPortablePackageValidationDenialKind {
        self.kind
    }

    pub fn definition_kind(&self) -> Option<WorthQueryPortableDefinitionKind> {
        self.definition_kind
    }

    pub fn slot(&self) -> &str {
        &self.slot
    }
}

impl std::fmt::Display for WorthQueryPortablePackageValidationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "portable package validation denied: {:?} ({})",
            self.kind, self.slot
        )
    }
}

impl std::error::Error for WorthQueryPortablePackageValidationDenial {}
