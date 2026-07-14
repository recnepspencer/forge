#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AspectFrontDoorConstructionDenial {
    InvalidAspectKey(String),
    InvalidFieldKey(String),
    EmptyStructShape,
    DuplicateFieldDeclaration(String),
    EmptyFieldPath,
    FieldPathMustTargetSingleField,
    DuplicateStructValueField(String),
    OpaqueMaskContractMustBeDiagnosticOnly,
}
