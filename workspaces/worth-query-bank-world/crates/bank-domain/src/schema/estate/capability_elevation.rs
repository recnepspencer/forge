use worth_query_decl::facade::application_capability::{
    ApplicationCapabilityElevationDefinition, ApplicationCapabilityElevationRule,
    ApplicationCapabilityElevationStates, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityMandatoryReviewDefinition, ApplicationCapabilityRelationBinding,
    ApplicationCapabilityValueBinding,
};

use super::*;
use crate::estate::{
    EmergencyAccessStatus, EstateCapabilityOperation, EstateCapabilityPurpose,
    MandatoryReviewStatus,
};

pub(super) fn rule(
    action: EstateCapabilityOperation,
    purpose: EstateCapabilityPurpose,
) -> ApplicationCapabilityElevationRule {
    if action != EstateCapabilityOperation::ViewRestrictedEstate
        || purpose != EstateCapabilityPurpose::EmergencyProtection
    {
        return ApplicationCapabilityElevationRule::not_applicable();
    }
    ApplicationCapabilityElevationRule::governed(ApplicationCapabilityElevationDefinition::new(
        ApplicationCapabilityFieldBinding::from_reference(EmergencyAccessIdentityField::reference()),
        ApplicationCapabilityFieldBinding::from_reference(EmergencyAccessReasonField::reference()),
        ApplicationCapabilityFieldBinding::from_reference(EmergencyAccessStatusField::reference()),
        ApplicationCapabilityElevationStates::new(
            elevation_status(EmergencyAccessStatus::Requested),
            elevation_status(EmergencyAccessStatus::Approved),
            elevation_status(EmergencyAccessStatus::Active),
            elevation_status(EmergencyAccessStatus::Revoked),
            elevation_status(EmergencyAccessStatus::ReviewRequired),
            elevation_status(EmergencyAccessStatus::Reviewed),
        ),
        ApplicationCapabilityRelationBinding::from_reference(EmergencyRequester::reference()),
        ApplicationCapabilityRelationBinding::from_reference(EmergencyApprover::reference()),
        ApplicationCapabilityRelationBinding::from_reference(EmergencyGrant::reference()),
        ApplicationCapabilityMandatoryReviewDefinition::new(
            ApplicationCapabilityRelationBinding::from_reference(EmergencyReview::reference()),
            ApplicationCapabilityFieldBinding::from_reference(
                MandatoryReviewIdentityField::reference(),
            ),
            ApplicationCapabilityRelationBinding::from_reference(ReviewPrincipal::reference()),
            ApplicationCapabilityFieldBinding::from_reference(
                MandatoryReviewStatusField::reference(),
            ),
            review_status(MandatoryReviewStatus::Required),
            review_status(MandatoryReviewStatus::Completed),
        ),
    ))
}

fn elevation_status(status: EmergencyAccessStatus) -> ApplicationCapabilityValueBinding {
    ApplicationCapabilityValueBinding::new(EmergencyAccessStatusField::reference(), status)
}

fn review_status(status: MandatoryReviewStatus) -> ApplicationCapabilityValueBinding {
    ApplicationCapabilityValueBinding::new(MandatoryReviewStatusField::reference(), status)
}
