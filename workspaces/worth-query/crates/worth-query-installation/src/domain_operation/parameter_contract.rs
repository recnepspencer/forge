use worth_foundational::facade::{AspectIdentity, AspectKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationParameterContract {
    NotRequired,
    Declared {
        fields: Vec<WorthQueryOperationParameterField>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryOperationParameterField {
    pub name: String,
    pub value_family: WorthQueryOperationValueFamily,
    pub required: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationValueFamily {
    Bool,
    I64,
    U64,
    Text,
    EntityIdentity,
    NativeAspect {
        key: AspectKey,
        identity: AspectIdentity,
    },
}
