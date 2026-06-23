#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainGraphOperationDeclarationError {
    EmptyOperationName,
    EmptyDomainOwner,
    ZeroOperationVersion,
    EmptyAdmittedReference,
    EmptySupportFamily,
}
