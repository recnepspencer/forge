use crate::aspects::structs::FieldKey;
use crate::values::ScalarAspectType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractValidationDenial {
    ScalarTypeMismatch {
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
    NonCanonicalScalarValue(ScalarAspectType),
    StructValueRequired,
    ScalarValueRequired,
    MissingRequiredField(FieldKey),
    UnknownField(FieldKey),
    FieldTypeMismatch {
        field: FieldKey,
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
    NonCanonicalFieldValue {
        field: FieldKey,
        family: ScalarAspectType,
    },
}
