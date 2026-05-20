#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AspectFrontDoorConstructionDenial {
    InvalidAspectKey(String),
    InvalidFieldKey(String),
    EmptyStructShape,
    DuplicateFieldDeclaration(String),
    EmptyFieldPath,
    DuplicateStructValueField(String),
}
