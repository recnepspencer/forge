use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityContextEntitySlotBinding, ApplicationCapabilityElevationDefinition,
    ApplicationCapabilityElevationLifecycleDefinition, ApplicationCapabilityElevationRule,
    ApplicationCapabilityElevationStates, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityMandatoryReviewDefinition, ApplicationCapabilityOperationBinding,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityValidityDefinition,
    ApplicationCapabilityValidityTimeline, ApplicationCapabilityValueBinding,
};

use super::super::{
    ApproveCapabilityElevationOperation, CapabilityElevationApprover, CapabilityElevationGrant,
    CapabilityElevationIdentity, CapabilityElevationNotAfter, CapabilityElevationNotBefore,
    CapabilityElevationReason, CapabilityElevationRequester, CapabilityElevationReview,
    CapabilityElevationSlot, CapabilityElevationStatus, CapabilityElevationStatusField,
    CapabilityReviewIdentity, CapabilityReviewSlot, CapabilityReviewStatus,
    CapabilityReviewStatusField, CapabilityReviewer, CompleteCapabilityReviewOperation,
    RequestCapabilityElevationOperation, RevokeCapabilityElevationOperation,
};

pub(super) fn definition() -> ApplicationCapabilityElevationRule {
    let state = |value| {
        ApplicationCapabilityValueBinding::new(CapabilityElevationStatusField::reference(), value)
    };
    ApplicationCapabilityElevationRule::governed(ApplicationCapabilityElevationDefinition::new(
        ApplicationCapabilityFieldBinding::from_reference(CapabilityElevationIdentity::reference()),
        ApplicationCapabilityFieldBinding::from_reference(CapabilityElevationReason::reference()),
        ApplicationCapabilityFieldBinding::from_reference(
            CapabilityElevationStatusField::reference(),
        ),
        ApplicationCapabilityElevationStates::new(
            state(CapabilityElevationStatus::Requested),
            state(CapabilityElevationStatus::Approved),
            state(CapabilityElevationStatus::Expired),
            state(CapabilityElevationStatus::Revoked),
        ),
        ApplicationCapabilityValidityDefinition::new(
            ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
            ApplicationCapabilityFieldBinding::from_reference(
                CapabilityElevationNotBefore::reference(),
            ),
            ApplicationCapabilityFieldBinding::from_reference(
                CapabilityElevationNotAfter::reference(),
            ),
        ),
        std::time::Duration::from_secs(20 * 60),
        ApplicationCapabilityRelationBinding::from_reference(
            CapabilityElevationRequester::reference(),
        ),
        ApplicationCapabilityRelationBinding::from_reference(
            CapabilityElevationApprover::reference(),
        ),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityElevationGrant::reference()),
        ApplicationCapabilityElevationLifecycleDefinition::new(
            ApplicationCapabilityContextEntitySlotBinding::from_reference(
                CapabilityElevationSlot::reference(),
            ),
            ApplicationCapabilityContextEntitySlotBinding::from_reference(
                CapabilityReviewSlot::reference(),
            ),
            ApplicationCapabilityOperationBinding::from_reference(
                RequestCapabilityElevationOperation::reference(),
            ),
            ApplicationCapabilityOperationBinding::from_reference(
                ApproveCapabilityElevationOperation::reference(),
            ),
            ApplicationCapabilityOperationBinding::from_reference(
                RevokeCapabilityElevationOperation::reference(),
            ),
            ApplicationCapabilityOperationBinding::from_reference(
                CompleteCapabilityReviewOperation::reference(),
            ),
        ),
        ApplicationCapabilityMandatoryReviewDefinition::new(
            ApplicationCapabilityRelationBinding::from_reference(
                CapabilityElevationReview::reference(),
            ),
            ApplicationCapabilityFieldBinding::from_reference(CapabilityReviewIdentity::reference()),
            ApplicationCapabilityRelationBinding::from_reference(CapabilityReviewer::reference()),
            ApplicationCapabilityFieldBinding::from_reference(
                CapabilityReviewStatusField::reference(),
            ),
            ApplicationCapabilityValueBinding::new(
                CapabilityReviewStatusField::reference(),
                CapabilityReviewStatus::Required,
            ),
            ApplicationCapabilityValueBinding::new(
                CapabilityReviewStatusField::reference(),
                CapabilityReviewStatus::Completed,
            ),
        ),
    ))
}
