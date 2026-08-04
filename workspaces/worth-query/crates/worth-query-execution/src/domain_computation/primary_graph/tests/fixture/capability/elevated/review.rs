use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRequest,
    ApplicationCapabilityRequestContext, ApplicationCapabilityRequestProjection,
    ApplicationCapabilityRequestProjectionDenial,
};
use worth_query_declaration::{
    worth_query_capability, worth_query_operation, worth_query_operation_links,
    worth_query_operation_reads, worth_query_operation_writes,
};

use super::{
    CapabilityAction, CapabilityElevationApprover, CapabilityElevationGrant,
    CapabilityElevationIdentity, CapabilityElevationNotAfter, CapabilityElevationNotBefore,
    CapabilityElevationReason, CapabilityElevationRequester, CapabilityElevationReview,
    CapabilityElevationSlot, CapabilityElevationStatusField, CapabilityPurpose,
    CapabilityRequestContext, CapabilityReviewIdentity, CapabilityReviewKindField,
    CapabilityReviewResource, CapabilityReviewSlot, CapabilityReviewStatusField,
    CapabilityReviewer,
};
use crate::domain_computation::primary_graph::tests::fixture::{
    Account, AccountIdentity, IdentityExecutionSchema,
};

worth_query_capability!(pub CompleteElevationReviewCapability in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteElevationReviewInput {
    pub account: String,
    pub elevation: String,
    pub review: String,
}

worth_query_operation!(pub CompleteCapabilityReviewOperation(CompleteElevationReviewInput) in IdentityExecutionSchema);
worth_query_operation_reads!(CompleteCapabilityReviewOperation => [CapabilityElevationIdentity, CapabilityElevationReason, CapabilityElevationStatusField, CapabilityElevationNotBefore, CapabilityElevationNotAfter, CapabilityReviewIdentity, CapabilityReviewKindField, CapabilityReviewStatusField, CapabilityElevationRequester, CapabilityElevationApprover, CapabilityElevationGrant, CapabilityElevationReview, CapabilityReviewResource, CapabilityReviewer]);
worth_query_operation_writes!(CompleteCapabilityReviewOperation => [CapabilityReviewStatusField]);
worth_query_operation_links!(CompleteCapabilityReviewOperation => [CapabilityReviewer]);

impl ApplicationCapabilityRequest<IdentityExecutionSchema, CompleteElevationReviewCapability>
    for CompleteElevationReviewInput
{
    type Scope = Account;
    type Context = CapabilityRequestContext;

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<
            IdentityExecutionSchema,
            Account,
            CapabilityRequestContext,
        >,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        Ok(ApplicationCapabilityRequestProjection::new(
            ApplicationCapabilityEntitySelector::new(
                AccountIdentity::reference(),
                self.account.clone(),
            ),
            CapabilityAction::CompleteReview,
            CapabilityPurpose::AccountMaintenance,
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference())
                .entity(
                    CapabilityElevationSlot::reference(),
                    ApplicationCapabilityEntitySelector::new(
                        CapabilityElevationIdentity::reference(),
                        self.elevation.clone(),
                    ),
                )
                .entity(
                    CapabilityReviewSlot::reference(),
                    ApplicationCapabilityEntitySelector::new(
                        CapabilityReviewIdentity::reference(),
                        self.review.clone(),
                    ),
                ),
        ))
    }
}
