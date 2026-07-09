#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadRegistryAdmissionError {
    EmptyOperationName,
    EmptyDomainOwner,
    ZeroOperationVersion,
    MissingAdmittedReferences,
    MissingLoweringContract,
    DuplicateOperationKey,
    AmbiguousDomainReferenceAdmission,
}

impl WorthQueryGraphReadRegistryAdmissionError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyOperationName => "empty_operation_name",
            Self::EmptyDomainOwner => "empty_domain_owner",
            Self::ZeroOperationVersion => "zero_operation_version",
            Self::MissingAdmittedReferences => "missing_admitted_references",
            Self::MissingLoweringContract => "missing_lowering_contract",
            Self::DuplicateOperationKey => "duplicate_operation_key",
            Self::AmbiguousDomainReferenceAdmission => "ambiguous_domain_reference_admission",
        }
    }
}

impl From<crate::authoring::DomainGraphOperationDeclarationError>
    for WorthQueryGraphReadRegistryAdmissionError
{
    fn from(error: crate::authoring::DomainGraphOperationDeclarationError) -> Self {
        match error {
            crate::authoring::DomainGraphOperationDeclarationError::EmptyOperationName => {
                Self::EmptyOperationName
            }
            crate::authoring::DomainGraphOperationDeclarationError::EmptyDomainOwner => {
                Self::EmptyDomainOwner
            }
            crate::authoring::DomainGraphOperationDeclarationError::ZeroOperationVersion => {
                Self::ZeroOperationVersion
            }
            crate::authoring::DomainGraphOperationDeclarationError::EmptyAdmittedReference
            | crate::authoring::DomainGraphOperationDeclarationError::EmptySupportFamily => {
                Self::MissingAdmittedReferences
            }
        }
    }
}
