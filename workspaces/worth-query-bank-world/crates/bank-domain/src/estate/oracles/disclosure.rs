use crate::estate::{EstateAction, EstateDenial};

pub(super) fn validate(action: EstateAction) -> Result<(), EstateDenial> {
    let (field, purpose) = match action {
        EstateAction::ViewRestrictedEstate { field, purpose, .. } => (field, purpose),
        EstateAction::ViewRestrictedEstateWithEmergencyAccess { field, .. } => {
            (field, action.purpose())
        }
        _ => return Ok(()),
    };
    if !field.permits(purpose) {
        return Err(EstateDenial::DisclosurePurposeMismatch);
    }
    Ok(())
}
