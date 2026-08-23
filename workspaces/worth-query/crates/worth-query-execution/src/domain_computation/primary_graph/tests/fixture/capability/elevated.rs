use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};
use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityElevationRequest, ApplicationCapabilityElevationRequestProjection,
        ApplicationCapabilityElevationRequestProjectionDenial, ApplicationCapabilityEntitySelector,
        ApplicationCapabilityRequest, ApplicationCapabilityRequestContext,
        ApplicationCapabilityRequestProjection, ApplicationCapabilityRequestProjectionDenial,
        ApplicationCapabilityValueBinding,
    },
    application_schema::{TypedApplicationReadableValue, TypedApplicationValue},
};
use worth_query_declaration::{
    worth_query_aspect, worth_query_capability, worth_query_capability_context_entity_slot,
    worth_query_entity, worth_query_field, worth_query_operation, worth_query_operation_creates,
    worth_query_operation_links, worth_query_operation_reads, worth_query_operation_writes,
    worth_query_relation,
};

use super::super::{Account, AccountIdentity, AccountLabel, IdentityExecutionSchema, Principal};
use super::declaration::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityElevationStatus {
    Requested,
    Approved,
    Expired,
    Revoked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityReviewStatus {
    Required,
    Completed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityReviewKind {
    Elevation,
}

macro_rules! string_value {
    ($type:ty, {$($variant:path => $value:literal),+ $(,)?}) => {
        impl TypedApplicationValue for $type {
            const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;
            fn into_foundational_value(self) -> AspectValue {
                let value = match self { $($variant => $value),+ };
                AspectValue::String(InternedString::from(value))
            }
        }
        impl TypedApplicationReadableValue for $type {
            fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                let AspectValue::String(InternedString::Raw(value)) = value else { return None; };
                match value.as_str() { $($value => Some($variant),)+ _ => None }
            }
        }
    };
}

string_value!(CapabilityElevationStatus, {
    CapabilityElevationStatus::Requested => "requested",
    CapabilityElevationStatus::Approved => "approved",
    CapabilityElevationStatus::Expired => "expired",
    CapabilityElevationStatus::Revoked => "revoked"
});
string_value!(CapabilityReviewStatus, {
    CapabilityReviewStatus::Required => "required",
    CapabilityReviewStatus::Completed => "completed"
});
string_value!(CapabilityReviewKind, {
    CapabilityReviewKind::Elevation => "elevation"
});

worth_query_entity!(pub CapabilityElevation in IdentityExecutionSchema);
worth_query_aspect!(pub CapabilityElevationFacts in IdentityExecutionSchema, CapabilityElevation; identity = AspectIdentity(0x91611038), revision = AspectContractRevision(1),);
worth_query_field!(pub CapabilityElevationIdentity in IdentityExecutionSchema, CapabilityElevation, CapabilityElevationFacts: String, read_only, equality);
worth_query_field!(pub CapabilityElevationReason in IdentityExecutionSchema, CapabilityElevation, CapabilityElevationFacts: String, read_only, no_equality);
worth_query_field!(pub CapabilityElevationStatusField in IdentityExecutionSchema, CapabilityElevation, CapabilityElevationFacts: CapabilityElevationStatus, read_write, no_equality);
worth_query_field!(pub CapabilityElevationNotBefore in IdentityExecutionSchema, CapabilityElevation, CapabilityElevationFacts: u64, read_write, no_equality);
worth_query_field!(pub CapabilityElevationNotAfter in IdentityExecutionSchema, CapabilityElevation, CapabilityElevationFacts: u64, read_write, no_equality);
worth_query_entity!(pub CapabilityReview in IdentityExecutionSchema);
worth_query_aspect!(pub CapabilityReviewFacts in IdentityExecutionSchema, CapabilityReview; identity = AspectIdentity(0x91611039), revision = AspectContractRevision(1),);
worth_query_field!(pub CapabilityReviewIdentity in IdentityExecutionSchema, CapabilityReview, CapabilityReviewFacts: String, read_only, equality);
worth_query_field!(pub CapabilityReviewKindField in IdentityExecutionSchema, CapabilityReview, CapabilityReviewFacts: CapabilityReviewKind, read_only, equality);
worth_query_field!(pub CapabilityReviewStatusField in IdentityExecutionSchema, CapabilityReview, CapabilityReviewFacts: CapabilityReviewStatus, read_write, no_equality);
worth_query_relation!(pub CapabilityElevationRequester in IdentityExecutionSchema, Principal => CapabilityElevation);
worth_query_relation!(pub CapabilityElevationApprover in IdentityExecutionSchema, Principal => CapabilityElevation);
worth_query_relation!(pub CapabilityElevationGrant in IdentityExecutionSchema, CapabilityElevation => CapabilityGrant);
worth_query_relation!(pub CapabilityElevationResource in IdentityExecutionSchema, CapabilityElevation => Account);
worth_query_relation!(pub CapabilityElevationReview in IdentityExecutionSchema, CapabilityElevation => CapabilityReview);
worth_query_relation!(pub CapabilityReviewResource in IdentityExecutionSchema, CapabilityReview => Account);
worth_query_relation!(pub CapabilityReviewer in IdentityExecutionSchema, Principal => CapabilityReview);
worth_query_capability_context_entity_slot!(pub CapabilityElevationSlot in IdentityExecutionSchema, CapabilityRequestContext => CapabilityElevation);
worth_query_capability_context_entity_slot!(pub CapabilityReviewSlot in IdentityExecutionSchema, CapabilityRequestContext => CapabilityReview);
worth_query_capability!(pub ElevatedTouchAccountCapability in IdentityExecutionSchema);
worth_query_capability!(pub RequestElevationCapability in IdentityExecutionSchema);
worth_query_capability!(pub ApproveElevationCapability in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElevatedCapabilityTouchInput {
    pub account: String,
    pub elevation: Option<String>,
    pub substitute_resource_selector: bool,
    pub action: CapabilityAction,
    pub purpose: CapabilityPurpose,
    pub disclosure: CapabilityDisclosure,
    pub amount: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestElevationInput {
    pub account: String,
    pub target_account: String,
    pub grant: String,
    pub elevation_key: String,
    pub elevation_identity: String,
    pub review_key: String,
    pub review_identity: String,
    pub reason: String,
    pub duration: std::time::Duration,
    pub action: CapabilityAction,
    pub target_purpose: CapabilityPurpose,
    pub disclosure: CapabilityDisclosure,
    pub amount: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproveElevationInput {
    pub account: String,
    pub elevation: String,
}

worth_query_operation!(pub ElevatedCapabilityTouchOperation(ElevatedCapabilityTouchInput) in IdentityExecutionSchema);
worth_query_operation!(pub RequestCapabilityElevationOperation(RequestElevationInput) in IdentityExecutionSchema);
worth_query_operation!(pub ApproveCapabilityElevationOperation(ApproveElevationInput) in IdentityExecutionSchema);
worth_query_operation_reads!(ElevatedCapabilityTouchOperation => [AccountLabel]);
worth_query_operation_writes!(ElevatedCapabilityTouchOperation => [AccountLabel]);
worth_query_operation_reads!(RequestCapabilityElevationOperation => [AccountLabel]);
worth_query_operation_creates!(RequestCapabilityElevationOperation => [CapabilityElevation, CapabilityReview]);
worth_query_operation_writes!(RequestCapabilityElevationOperation => [CapabilityElevationIdentity, CapabilityElevationReason, CapabilityElevationStatusField, CapabilityElevationNotBefore, CapabilityElevationNotAfter, CapabilityReviewIdentity, CapabilityReviewKindField, CapabilityReviewStatusField]);
worth_query_operation_links!(RequestCapabilityElevationOperation => [CapabilityElevationRequester, CapabilityElevationGrant, CapabilityElevationResource, CapabilityElevationReview, CapabilityReviewResource]);
worth_query_operation_reads!(ApproveCapabilityElevationOperation => [CapabilityElevationIdentity, CapabilityElevationReason, CapabilityElevationStatusField, CapabilityElevationNotBefore, CapabilityElevationNotAfter, CapabilityReviewIdentity, CapabilityReviewKindField, CapabilityReviewStatusField, CapabilityElevationRequester, CapabilityElevationApprover, CapabilityElevationGrant, CapabilityElevationResource, CapabilityElevationReview, CapabilityReviewResource, CapabilityReviewer]);
worth_query_operation_writes!(ApproveCapabilityElevationOperation => [CapabilityElevationStatusField]);
worth_query_operation_links!(ApproveCapabilityElevationOperation => [CapabilityElevationApprover]);

impl ApplicationCapabilityRequest<IdentityExecutionSchema, ElevatedTouchAccountCapability>
    for ElevatedCapabilityTouchInput
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
        let projection = ApplicationCapabilityRequestProjection::new(
            ApplicationCapabilityEntitySelector::new(
                AccountIdentity::reference(),
                self.account.clone(),
            ),
            self.action,
            self.purpose,
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference()),
        );
        let projection = match (&self.elevation, self.substitute_resource_selector) {
            (Some(_), true) => projection.elevation(ApplicationCapabilityEntitySelector::new(
                AccountIdentity::reference(),
                self.account.clone(),
            )),
            (Some(elevation), false) => {
                projection.elevation(ApplicationCapabilityEntitySelector::new(
                    CapabilityElevationIdentity::reference(),
                    elevation.clone(),
                ))
            }
            (None, _) => projection,
        };
        Ok(projection.field(self.disclosure).magnitude(self.amount))
    }
}

impl ApplicationCapabilityRequest<IdentityExecutionSchema, RequestElevationCapability>
    for RequestElevationInput
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
        Ok(self.ordinary_projection())
    }
}

impl ApplicationCapabilityRequest<IdentityExecutionSchema, ApproveElevationCapability>
    for ApproveElevationInput
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
            CapabilityAction::ApproveElevation,
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

impl
    ApplicationCapabilityElevationRequest<
        IdentityExecutionSchema,
        RequestCapabilityElevationOperation,
    > for RequestElevationInput
{
    type Scope = Account;
    type Context = CapabilityRequestContext;

    fn elevation_request(
        &self,
    ) -> Result<
        ApplicationCapabilityElevationRequestProjection<
            IdentityExecutionSchema,
            Account,
            CapabilityRequestContext,
        >,
        ApplicationCapabilityElevationRequestProjectionDenial,
    > {
        ApplicationCapabilityElevationRequestProjection::new(
            self.target_projection(),
            ApplicationCapabilityEntitySelector::new(
                CapabilityIdentity::reference(),
                self.grant.clone(),
            ),
            self.elevation_key.clone(),
            ApplicationCapabilityValueBinding::new(
                CapabilityElevationIdentity::reference(),
                self.elevation_identity.clone(),
            ),
            self.review_key.clone(),
            ApplicationCapabilityValueBinding::new(
                CapabilityReviewIdentity::reference(),
                self.review_identity.clone(),
            ),
            ApplicationCapabilityValueBinding::new(
                CapabilityElevationReason::reference(),
                self.reason.clone(),
            ),
            self.duration,
        )
    }
}

impl RequestElevationInput {
    fn ordinary_projection(
        &self,
    ) -> ApplicationCapabilityRequestProjection<
        IdentityExecutionSchema,
        Account,
        CapabilityRequestContext,
    > {
        ApplicationCapabilityRequestProjection::new(
            ApplicationCapabilityEntitySelector::new(
                AccountIdentity::reference(),
                self.account.clone(),
            ),
            CapabilityAction::RequestElevation,
            CapabilityPurpose::AccountMaintenance,
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference()),
        )
    }

    fn target_projection(
        &self,
    ) -> ApplicationCapabilityRequestProjection<
        IdentityExecutionSchema,
        Account,
        CapabilityRequestContext,
    > {
        ApplicationCapabilityRequestProjection::new(
            ApplicationCapabilityEntitySelector::new(
                AccountIdentity::reference(),
                self.target_account.clone(),
            ),
            self.action,
            self.target_purpose,
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference()),
        )
        .field(self.disclosure)
        .magnitude(self.amount)
    }
}

#[path = "elevated/account_activity_query.rs"]
mod account_activity_query;
#[path = "elevated/close.rs"]
mod close;
#[path = "elevated/contract.rs"]
mod contract;
#[path = "elevated/review.rs"]
mod review;
pub(in crate::domain_computation::primary_graph) use account_activity_query::{
    elevated_account_activity_definition, elevated_account_activity_parameters,
    ElevatedAccountActivityCause, ElevatedAccountActivityQuery, ElevatedAccountActivityResult,
};
pub use close::*;
pub(super) use contract::install;
pub use review::*;
