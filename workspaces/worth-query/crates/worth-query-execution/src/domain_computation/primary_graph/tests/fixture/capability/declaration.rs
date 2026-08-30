use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};
use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityEntitySelector, ApplicationCapabilityGovernedInputIdentity,
        ApplicationCapabilityRelatedEntitySelector, ApplicationCapabilityRequest,
        ApplicationCapabilityRequestContext, ApplicationCapabilityRequestProjection,
        ApplicationCapabilityRequestProjectionDenial,
    },
    application_schema::{TypedApplicationReadableValue, TypedApplicationValue},
};
use worth_query_declaration::{
    worth_query_aspect, worth_query_capability, worth_query_capability_context,
    worth_query_capability_context_entity_slot, worth_query_capability_provenance,
    worth_query_entity, worth_query_field, worth_query_operation, worth_query_operation_reads,
    worth_query_operation_writes, worth_query_relation,
};

use super::super::{Account, AccountIdentity, AccountLabel, IdentityExecutionSchema, Principal};
use super::governed_input::CapabilityGovernedInputIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityAction {
    Touch,
    Inspect,
    Disburse,
    RequestElevation,
    ApproveElevation,
    RevokeElevation,
    CompleteReview,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityPurpose {
    AccountMaintenance,
    Audit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityStatus {
    Active,
    Revoked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityDisclosure {
    AccountActivity,
    PrivateLabel,
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
                let AspectValue::String(InternedString::Raw(value)) = value else {
                    return None;
                };
                match value.as_str() {
                    $($value => Some($variant),)+
                    _ => None,
                }
            }
        }
    };
}

string_value!(CapabilityAction, {
    CapabilityAction::Touch => "touch",
    CapabilityAction::Inspect => "inspect",
    CapabilityAction::Disburse => "disburse",
    CapabilityAction::RequestElevation => "request-elevation",
    CapabilityAction::ApproveElevation => "approve-elevation",
    CapabilityAction::RevokeElevation => "revoke-elevation",
    CapabilityAction::CompleteReview => "complete-review"
});
string_value!(CapabilityPurpose, {
    CapabilityPurpose::AccountMaintenance => "account-maintenance",
    CapabilityPurpose::Audit => "audit"
});
string_value!(CapabilityStatus, {
    CapabilityStatus::Active => "active",
    CapabilityStatus::Revoked => "revoked"
});
string_value!(CapabilityDisclosure, {
    CapabilityDisclosure::AccountActivity => "account-activity",
    CapabilityDisclosure::PrivateLabel => "private-label"
});

worth_query_entity!(pub CapabilityGrant in IdentityExecutionSchema);
worth_query_aspect!(pub CapabilityFacts in IdentityExecutionSchema, CapabilityGrant; identity = AspectIdentity(0x91611036), revision = AspectContractRevision(1),);
worth_query_field!(
    pub CapabilityIdentity in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    String, read_only, equality
);
worth_query_field!(
    pub CapabilityActionField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    CapabilityAction, read_only, no_equality
);
worth_query_field!(
    pub CapabilityPurposeField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    CapabilityPurpose, read_only, no_equality
);
worth_query_field!(
    pub CapabilityDisclosureField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    CapabilityDisclosure, read_only, no_equality
);
worth_query_field!(
    pub CapabilityAmountField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    u64, read_write, no_equality
);
worth_query_field!(
    pub CapabilityStatusField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    CapabilityStatus, read_write, no_equality
);
worth_query_field!(
    pub CapabilityWorkflowField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    String, read_write, no_equality
);
worth_query_field!(
    pub CapabilityNotBeforeField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    u64, read_write, no_equality
);
worth_query_field!(
    pub CapabilityNotAfterField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    u64, read_write, no_equality
);
worth_query_field!(
    pub CapabilityDelegationLimitField in IdentityExecutionSchema, CapabilityGrant, CapabilityFacts:
    u64, read_write, no_equality
);
worth_query_entity!(pub CapabilityActionRecord in IdentityExecutionSchema);
worth_query_aspect!(pub CapabilityActionRecordFacts in IdentityExecutionSchema,
    CapabilityActionRecord; identity = AspectIdentity(0x91611037), revision = AspectContractRevision(1),);
worth_query_field!(
    pub CapabilityActionRecordIdentity in IdentityExecutionSchema,
    CapabilityActionRecord, CapabilityActionRecordFacts:
    String, read_only, equality
);
worth_query_relation!(
    pub CapabilityGrantee in IdentityExecutionSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityGrantor in IdentityExecutionSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityCustodian in IdentityExecutionSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityResource in IdentityExecutionSchema,
    CapabilityGrant => Account
);
worth_query_relation!(
    pub CapabilityRelated in IdentityExecutionSchema,
    CapabilityGrant => Account
);
worth_query_relation!(
    pub CapabilityParent in IdentityExecutionSchema,
    CapabilityGrant => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityExplicitDeny in IdentityExecutionSchema,
    Principal => Account
);
worth_query_relation!(
    pub CapabilityConflictingBeneficiary in IdentityExecutionSchema,
    Principal => Account
);
worth_query_relation!(
    pub CapabilityRequestActor in IdentityExecutionSchema,
    Principal => CapabilityActionRecord
);
worth_query_relation!(
    pub CapabilityPriorActor in IdentityExecutionSchema,
    Principal => CapabilityActionRecord
);
worth_query_relation!(
    pub CapabilityActionResource in IdentityExecutionSchema,
    CapabilityActionRecord => Account
);
worth_query_capability_context!(pub CapabilityRequestContext in IdentityExecutionSchema);
worth_query_capability_context_entity_slot!(
    pub CapabilityRequestActorSlot in IdentityExecutionSchema,
    CapabilityRequestContext => CapabilityActionRecord
);
worth_query_capability_context_entity_slot!(
    pub CapabilityPriorActorSlot in IdentityExecutionSchema,
    CapabilityRequestContext => CapabilityActionRecord
);
worth_query_capability_provenance!(pub CapabilityProvenance in IdentityExecutionSchema);
worth_query_capability!(pub TouchAccountCapability in IdentityExecutionSchema);
worth_query_capability!(pub ComposedTouchAccountCapability in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityTouchInput {
    pub account: String,
    pub action: CapabilityAction,
    pub purpose: CapabilityPurpose,
    pub disclosure: CapabilityDisclosure,
    pub related_account: String,
    pub request_record: String,
    pub prior_record: String,
    pub amount: u64,
    pub caller_time: u64,
    pub governed_input_identity: CapabilityGovernedInputIdentity,
}
worth_query_declaration::worth_query_portable_type!(CapabilityAction => "worth.query.test.execution.capability.action.v1");
worth_query_declaration::worth_query_portable_type!(CapabilityPurpose => "worth.query.test.execution.capability.purpose.v1");
worth_query_declaration::worth_query_portable_type!(CapabilityStatus => "worth.query.test.execution.capability.status.v1");
worth_query_declaration::worth_query_portable_type!(CapabilityDisclosure => "worth.query.test.execution.capability.disclosure.v1");
worth_query_declaration::worth_query_portable_type!(
    CapabilityTouchInput => "worth.query.test.capability-touch-input.v1"
);

worth_query_operation!(
    pub CapabilityTouchOperation(CapabilityTouchInput) in IdentityExecutionSchema
);
worth_query_operation!(
    pub ComposedCapabilityTouchOperation(CapabilityTouchInput) in IdentityExecutionSchema
);
worth_query_operation_reads!(CapabilityTouchOperation => [AccountLabel]);
worth_query_operation_writes!(CapabilityTouchOperation => [AccountLabel]);
worth_query_operation_reads!(ComposedCapabilityTouchOperation => [AccountLabel]);
worth_query_operation_writes!(ComposedCapabilityTouchOperation => [AccountLabel]);

impl ApplicationCapabilityRequest<IdentityExecutionSchema, TouchAccountCapability>
    for CapabilityTouchInput
{
    type Scope = Account;
    type Context = CapabilityRequestContext;

    fn governed_input_identity(&self) -> Option<ApplicationCapabilityGovernedInputIdentity> {
        self.governed_input_identity.materialize(self.amount)
    }

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<IdentityExecutionSchema, Self::Scope, Self::Context>,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        self.project_capability_request(ApplicationCapabilityRequestContext::new(
            CapabilityRequestContext::reference(),
        ))
    }
}

impl ApplicationCapabilityRequest<IdentityExecutionSchema, ComposedTouchAccountCapability>
    for CapabilityTouchInput
{
    type Scope = Account;
    type Context = CapabilityRequestContext;

    fn governed_input_identity(&self) -> Option<ApplicationCapabilityGovernedInputIdentity> {
        self.governed_input_identity.materialize(self.amount)
    }

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<IdentityExecutionSchema, Self::Scope, Self::Context>,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        self.project_capability_request(
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference())
                .entity(
                    CapabilityRequestActorSlot::reference(),
                    ApplicationCapabilityEntitySelector::new(
                        CapabilityActionRecordIdentity::reference(),
                        self.request_record.clone(),
                    ),
                )
                .entity(
                    CapabilityPriorActorSlot::reference(),
                    ApplicationCapabilityEntitySelector::new(
                        CapabilityActionRecordIdentity::reference(),
                        self.prior_record.clone(),
                    ),
                ),
        )
    }
}

impl CapabilityTouchInput {
    fn project_capability_request(
        &self,
        context: ApplicationCapabilityRequestContext<
            IdentityExecutionSchema,
            CapabilityRequestContext,
        >,
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
            self.action,
            self.purpose,
            context,
        )
        .related_entity(ApplicationCapabilityRelatedEntitySelector::new(
            CapabilityRelated::reference(),
            ApplicationCapabilityEntitySelector::new(
                AccountIdentity::reference(),
                self.related_account.clone(),
            ),
        ))
        .field(self.disclosure)
        .magnitude(self.amount))
    }
}
