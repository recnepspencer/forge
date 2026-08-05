use crate::estate::{EmergencyAccessId, EstateAction, EstateCaseId, RestrictedBankField};

use super::estate_emergency_access_activity;

#[test]
fn activity_request_binds_the_exact_estate_access_and_field() {
    let estate = EstateCaseId::new(17).unwrap();
    let access = EmergencyAccessId::new(23).unwrap();
    let request = estate_emergency_access_activity(estate, access);

    assert_eq!(request.estate(), estate);
    assert_eq!(
        request.capability_request(),
        EstateAction::ViewRestrictedEstateWithEmergencyAccess {
            estate,
            access,
            field: RestrictedBankField::EmergencyAccessActivity,
        }
    );
}
