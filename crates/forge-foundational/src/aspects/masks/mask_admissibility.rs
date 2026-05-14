#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaskAdmissibilityDenial {
    ModeNotAllowed,
    FieldMaskRequiresStruct,
    UnknownField,
}
