use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologyLegalityReceiptPosture {
    SelectedLegalityReceiptRequired,
    SelectedValidatorReceiptRequired,
    NotRequiredForFamilyDeclaration,
}

impl DerivedTopologyLegalityReceiptPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedLegalityReceiptRequired => "selected_legality_receipt_required",
            Self::SelectedValidatorReceiptRequired => "selected_validator_receipt_required",
            Self::NotRequiredForFamilyDeclaration => "not_required_for_family_declaration",
        }
    }
}
