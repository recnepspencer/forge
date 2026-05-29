use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaskAdmissibilityDenial {
    ModeNotAllowed,
    FieldMaskRequiresStruct,
    UnknownField,
}
