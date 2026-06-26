use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologyQueryReceiptPosture {
    ProjectionConsumptionRequired,
    NativeReadReceiptRequired,
    NativeWriteReceiptRequired,
    NotRequiredForFamilyDeclaration,
}

impl DerivedTopologyQueryReceiptPosture {
    pub const fn requires_query_support(self) -> bool {
        !matches!(self, Self::NotRequiredForFamilyDeclaration)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectionConsumptionRequired => "projection_consumption_required",
            Self::NativeReadReceiptRequired => "native_read_receipt_required",
            Self::NativeWriteReceiptRequired => "native_write_receipt_required",
            Self::NotRequiredForFamilyDeclaration => "not_required_for_family_declaration",
        }
    }
}
