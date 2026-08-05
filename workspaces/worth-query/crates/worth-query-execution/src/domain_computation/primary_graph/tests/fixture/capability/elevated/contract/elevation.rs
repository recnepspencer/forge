use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityContextEntitySlotBinding, ApplicationCapabilityElevationDefinition,
    ApplicationCapabilityElevationLifecycleDefinition, ApplicationCapabilityElevationRule,
    ApplicationCapabilityElevationStates, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityMandatoryReviewDefinition, ApplicationCapabilityRelationBinding,
    ApplicationCapabilityTransitionBinding, ApplicationCapabilityValidityDefinition,
    ApplicationCapabilityValidityTimeline, ApplicationCapabilityValueBinding,
};

use super::super::{
    ApproveCapabilityElevationOperation, ApproveElevationCapability, CapabilityElevationApprover,
    CapabilityElevationGrant, CapabilityElevationIdentity, CapabilityElevationNotAfter,
    CapabilityElevationNotBefore, CapabilityElevationReason, CapabilityElevationRequester,
    CapabilityElevationResource, CapabilityElevationReview, CapabilityElevationSlot,
    CapabilityElevationStatus, CapabilityElevationStatusField, CapabilityReviewIdentity,
    CapabilityReviewKind, CapabilityReviewKindField, CapabilityReviewResource,
    CapabilityReviewSlot, CapabilityReviewStatus, CapabilityReviewStatusField, CapabilityReviewer,
    CompleteCapabilityReviewOperation, CompleteElevationReviewCapability,
    RequestCapabilityElevationOperation, RequestElevationCapability,
    RevokeCapabilityElevationOperation, RevokeElevationCapability,
};

pub(super) fn definition() -> ApplicationCapabilityElevationRule {
    let state = |value| {
        ApplicationCapabilityValueBinding::new(CapabilityElevationStatusField::reference(), value)
    };
    ApplicationCapabilityElevationRule::governed(
        ApplicationCapabilityElevationDefinition::new(
            ApplicationCapabilityFieldBinding::from_reference(
                CapabilityElevationIdentity::reference(),
            ),
            ApplicationCapabilityFieldBinding::from_reference(
                CapabilityElevationReason::reference(),
            ),
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
            ApplicationCapabilityRelationBinding::from_reference(
                CapabilityElevationGrant::reference(),
            ),
            ApplicationCapabilityElevationLifecycleDefinition::new(
                ApplicationCapabilityContextEntitySlotBinding::from_reference(
                    CapabilityElevationSlot::reference(),
                ),
                ApplicationCapabilityContextEntitySlotBinding::from_reference(
                    CapabilityReviewSlot::reference(),
                ),
                ApplicationCapabilityTransitionBinding::from_references(
                    RequestElevationCapability::reference(),
                    RequestCapabilityElevationOperation::reference(),
                ),
                ApplicationCapabilityTransitionBinding::from_references(
                    ApproveElevationCapability::reference(),
                    ApproveCapabilityElevationOperation::reference(),
                ),
                ApplicationCapabilityTransitionBinding::from_references(
                    RevokeElevationCapability::reference(),
                    RevokeCapabilityElevationOperation::reference(),
                ),
                ApplicationCapabilityTransitionBinding::from_references(
                    CompleteElevationReviewCapability::reference(),
                    CompleteCapabilityReviewOperation::reference(),
                ),
            ),
            ApplicationCapabilityMandatoryReviewDefinition::new(
                ApplicationCapabilityRelationBinding::from_reference(
                    CapabilityElevationReview::reference(),
                ),
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityReviewIdentity::reference(),
                ),
                ApplicationCapabilityValueBinding::new(
                    CapabilityReviewKindField::reference(),
                    CapabilityReviewKind::Elevation,
                ),
                ApplicationCapabilityRelationBinding::from_reference(
                    CapabilityReviewResource::reference(),
                ),
                ApplicationCapabilityRelationBinding::from_reference(
                    CapabilityReviewer::reference(),
                ),
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
        )
        .with_resource_relation(ApplicationCapabilityRelationBinding::from_reference(
            CapabilityElevationResource::reference(),
        )),
    )
}
