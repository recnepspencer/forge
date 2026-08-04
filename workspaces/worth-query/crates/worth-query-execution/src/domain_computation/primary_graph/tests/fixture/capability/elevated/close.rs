use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRequest,
    ApplicationCapabilityRequestContext, ApplicationCapabilityRequestProjection,
    ApplicationCapabilityRequestProjectionDenial,
};
use worth_query_declaration::{
    worth_query_capability, worth_query_operation, worth_query_operation_reads,
    worth_query_operation_writes,
};

use super::{
    CapabilityAction, CapabilityElevationApprover, CapabilityElevationGrant,
    CapabilityElevationIdentity, CapabilityElevationNotAfter, CapabilityElevationNotBefore,
    CapabilityElevationReason, CapabilityElevationRequester, CapabilityElevationReview,
    CapabilityElevationSlot, CapabilityElevationStatusField, CapabilityPurpose,
    CapabilityRequestContext, CapabilityReviewIdentity, CapabilityReviewKindField,
    CapabilityReviewResource, CapabilityReviewStatusField, CapabilityReviewer,
};
use crate::domain_computation::primary_graph::tests::fixture::{
    Account, AccountIdentity, IdentityExecutionSchema,
};

worth_query_capability!(pub RevokeElevationCapability in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseElevationInput {
    pub account: String,
    pub elevation: String,
}

worth_query_operation!(pub RevokeCapabilityElevationOperation(CloseElevationInput) in IdentityExecutionSchema);
worth_query_operation_reads!(RevokeCapabilityElevationOperation => [CapabilityElevationIdentity, CapabilityElevationReason, CapabilityElevationStatusField, CapabilityElevationNotBefore, CapabilityElevationNotAfter, CapabilityReviewIdentity, CapabilityReviewKindField, CapabilityReviewStatusField, CapabilityElevationRequester, CapabilityElevationApprover, CapabilityElevationGrant, CapabilityElevationReview, CapabilityReviewResource, CapabilityReviewer]);
worth_query_operation_writes!(RevokeCapabilityElevationOperation => [CapabilityElevationStatusField]);

impl ApplicationCapabilityRequest<IdentityExecutionSchema, RevokeElevationCapability>
    for CloseElevationInput
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
            CapabilityAction::RevokeElevation,
            CapabilityPurpose::AccountMaintenance,
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference()).entity(
                CapabilityElevationSlot::reference(),
                ApplicationCapabilityEntitySelector::new(
                    CapabilityElevationIdentity::reference(),
                    self.elevation.clone(),
                ),
            ),
        ))
    }
}
