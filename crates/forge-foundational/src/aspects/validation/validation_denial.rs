use crate::aspects::structs::FieldKey;
use crate::values::ScalarAspectType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractValidationDenial {
    ScalarTypeMismatch {
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
    StructValueRequired,
    ScalarValueRequired,
    MissingRequiredField(FieldKey),
    UnknownField(FieldKey),
    FieldTypeMismatch {
        field: FieldKey,
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
}
