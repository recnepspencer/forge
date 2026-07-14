use crate::aspects::keys::AspectKey;
use crate::aspects::masks::MaskAdmissibilityDenial;
use crate::aspects::structs::{FieldKey, StructAspectValueConstructionDenial};
use crate::aspects::validation::ContractValidationDenial;
use crate::values::ScalarAspectType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthoritativePatchConstructionDenial {
    DuplicateWholeAspectSet(AspectKey),
    EmptyFieldPatch,
    DuplicateFieldSet(FieldKey),
    FieldPatchRequiresStructAspect,
    FieldPatchRequiresFieldMask,
    MaskNotAdmitted(MaskAdmissibilityDenial),
    UnknownField(FieldKey),
    FieldNotSelectedByMutationMask(FieldKey),
    FieldTypeMismatch {
        field: FieldKey,
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
    RequiredFieldClearDenied(FieldKey),
    AmbiguousWholeAndFieldPatch(AspectKey),
    DuplicateFieldPatch(AspectKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthoritativePatchApplicationDenial {
    MissingAspectForFieldPatch(AspectKey),
    FieldPatchRequiresStructValue(AspectKey),
    StructValueConstructionDenied(StructAspectValueConstructionDenial),
    ContractValidationDenied(ContractValidationDenial),
}
