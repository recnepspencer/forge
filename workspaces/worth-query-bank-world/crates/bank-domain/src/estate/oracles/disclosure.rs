use crate::estate::{EstateAction, EstateDenial};

pub(super) fn validate(action: EstateAction) -> Result<(), EstateDenial> {
    let EstateAction::ViewRestrictedEstate { field, purpose, .. } = action else {
        return Ok(());
    };
    if !field.permits(purpose) {
        return Err(EstateDenial::DisclosurePurposeMismatch);
    }
    Ok(())
}
