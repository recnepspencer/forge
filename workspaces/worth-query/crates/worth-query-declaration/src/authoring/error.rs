#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoringError {
    EmptyRootEntityKey,
    EmptyProjectionSelector,
    EmptyOrderingSelector,
    EmptyProjectionSet,
    EmptyTraversalRelation,
    UnsupportedTraversalDepth { depth: u8 },
    EmptyResultFieldSource,
    EmptyDeliveredFieldName,
    EmptyResultShapeFieldSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoringFailureClass {
    InvalidAtom,
    InvalidAssembly,
}

impl AuthoringError {
    pub fn failure_class(&self) -> AuthoringFailureClass {
        match self {
            Self::EmptyRootEntityKey
            | Self::EmptyProjectionSelector
            | Self::EmptyOrderingSelector
            | Self::EmptyTraversalRelation
            | Self::UnsupportedTraversalDepth { .. }
            | Self::EmptyResultFieldSource
            | Self::EmptyDeliveredFieldName => AuthoringFailureClass::InvalidAtom,
            Self::EmptyProjectionSet | Self::EmptyResultShapeFieldSet => {
                AuthoringFailureClass::InvalidAssembly
            }
        }
    }
}
