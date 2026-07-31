use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};
use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityActorComposition, ApplicationCapabilityAllowRule,
        ApplicationCapabilityAmountDimension, ApplicationCapabilityCardinalityDimension,
        ApplicationCapabilityComposition, ApplicationCapabilityConstraintDefinition,
        ApplicationCapabilityContractBuilder, ApplicationCapabilityCurrentnessDefinition,
        ApplicationCapabilityDecisionComposition, ApplicationCapabilityDelegationDefinition,
        ApplicationCapabilityDelegationRule, ApplicationCapabilityDenyRule,
        ApplicationCapabilityDisclosureRule, ApplicationCapabilityFieldBinding,
        ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphClause,
        ApplicationCapabilityGraphRule, ApplicationCapabilityPropagationComposition,
        ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
        ApplicationCapabilityRequest, ApplicationCapabilityRequestContext,
        ApplicationCapabilityRequestProjection, ApplicationCapabilityRequestProjectionDenial,
        ApplicationCapabilitySeparationOfDutyRule, ApplicationCapabilityDistinctActorRule,
        ApplicationCapabilityConflictRule, ApplicationCapabilityTargetDefinition,
        ApplicationCapabilityValidityDefinition, ApplicationCapabilityValidityTimeline,
        ApplicationCapabilityValueBinding, ApplicationCapabilityWorkflowDefinition,
        ApplicationCapabilityEntitySelector,
    },
    application_schema::{
        ApplicationAuthorizationPathBuilder, ApplicationSchemaDeclarationBuilder,
        TypedApplicationReadableValue, TypedApplicationValue,
    },
};
use worth_query_declaration::{
    worth_query_aspect, worth_query_capability, worth_query_capability_context,
    worth_query_capability_provenance, worth_query_entity, worth_query_field,
    worth_query_operation, worth_query_operation_reads, worth_query_operation_writes,
    worth_query_relation,
};

use super::{
    Account, AccountIdentity, AccountLabel, AccountStatus, IdentityExecutionSchema, Principal,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CapabilityAction {
    Touch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CapabilityPurpose {
    AccountMaintenance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CapabilityStatus {
    Active,
    Revoked,
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

string_value!(CapabilityAction, { CapabilityAction::Touch => "touch" });
string_value!(CapabilityPurpose, {
    CapabilityPurpose::AccountMaintenance => "account-maintenance"
});
string_value!(CapabilityStatus, {
    CapabilityStatus::Active => "active",
    CapabilityStatus::Revoked => "revoked"
});

worth_query_entity!(pub CapabilityGrant in IdentityExecutionSchema);
worth_query_aspect!(pub CapabilityFacts in IdentityExecutionSchema, CapabilityGrant);
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
worth_query_relation!(
    pub CapabilityGrantee in IdentityExecutionSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityGrantor in IdentityExecutionSchema,
    Principal => CapabilityGrant
);
worth_query_relation!(
    pub CapabilityResource in IdentityExecutionSchema,
    CapabilityGrant => Account
);
worth_query_relation!(
    pub CapabilityParent in IdentityExecutionSchema,
    CapabilityGrant => CapabilityGrant
);
worth_query_capability_context!(pub CapabilityRequestContext in IdentityExecutionSchema);
worth_query_capability_provenance!(pub CapabilityProvenance in IdentityExecutionSchema);
worth_query_capability!(pub TouchAccountCapability in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CapabilityTouchInput {
    pub(super) account: String,
    pub(super) action: CapabilityAction,
    pub(super) purpose: CapabilityPurpose,
}

worth_query_operation!(
    pub CapabilityTouchOperation(CapabilityTouchInput) in IdentityExecutionSchema
);
worth_query_operation_reads!(CapabilityTouchOperation => [AccountLabel]);
worth_query_operation_writes!(CapabilityTouchOperation => [AccountLabel]);

impl ApplicationCapabilityRequest<IdentityExecutionSchema, TouchAccountCapability>
    for CapabilityTouchInput
{
    type Scope = Account;
    type Context = CapabilityRequestContext;

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<
            IdentityExecutionSchema,
            Self::Scope,
            Self::Context,
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
            ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference()),
        ))
    }
}

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    schema
        .entity(CapabilityGrant::reference())
        .aspect(CapabilityGrant::reference(), CapabilityFacts::reference())
        .field(CapabilityGrant::reference(), CapabilityIdentity::reference())
        .field(CapabilityGrant::reference(), CapabilityActionField::reference())
        .field(CapabilityGrant::reference(), CapabilityPurposeField::reference())
        .field(CapabilityGrant::reference(), CapabilityStatusField::reference())
        .field(CapabilityGrant::reference(), CapabilityWorkflowField::reference())
        .field(CapabilityGrant::reference(), CapabilityNotBeforeField::reference())
        .field(CapabilityGrant::reference(), CapabilityNotAfterField::reference())
        .field(
            CapabilityGrant::reference(),
            CapabilityDelegationLimitField::reference(),
        )
        .relation(
            CapabilityGrantee::reference(),
            Principal::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            CapabilityGrantor::reference(),
            Principal::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            CapabilityResource::reference(),
            CapabilityGrant::reference(),
            Account::reference(),
        )
        .relation(
            CapabilityParent::reference(),
            CapabilityGrant::reference(),
            CapabilityGrant::reference(),
        )
        .capability_context(CapabilityRequestContext::reference())
        .capability_provenance(CapabilityProvenance::reference())
        .operation(CapabilityTouchOperation::reference())
        .operation_decision_fact_budget(CapabilityTouchOperation::reference(), 1)
        .operation_projection_work_budget(CapabilityTouchOperation::reference(), 32)
        .operation_read_field(
            CapabilityTouchOperation::reference(),
            AccountLabel::reference(),
        )
        .operation_write(
            CapabilityTouchOperation::reference(),
            AccountLabel::reference(),
        )
        .capability(capability_contract())
}

fn capability_contract(
) -> worth_query_declaration::facade::application_capability::ApplicationCapabilityContractDefinition<
    IdentityExecutionSchema,
    TouchAccountCapability,
    CapabilityTouchOperation,
    CapabilityTouchInput,
> {
    ApplicationCapabilityContractBuilder::new(
        TouchAccountCapability::reference(),
        CapabilityTouchOperation::reference(),
        CapabilityGrant::reference(),
    )
    .target(ApplicationCapabilityTargetDefinition::new(
        ApplicationCapabilityValueBinding::new(
            CapabilityActionField::reference(),
            CapabilityAction::Touch,
        ),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityResource::reference()),
        ApplicationCapabilityRelationDimension::not_applicable(),
        ApplicationCapabilityFieldDimension::not_applicable(),
        ApplicationCapabilityValueBinding::new(
            CapabilityPurposeField::reference(),
            CapabilityPurpose::AccountMaintenance,
        ),
    ))
    .constraints(ApplicationCapabilityConstraintDefinition::new(
        ApplicationCapabilityAmountDimension::not_applicable(),
        ApplicationCapabilityCardinalityDimension::One,
        ApplicationCapabilityCurrentnessDefinition::new(
            ApplicationCapabilityValueBinding::new(
                CapabilityStatusField::reference(),
                CapabilityStatus::Active,
            ),
            ApplicationCapabilityWorkflowDefinition::new(
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityWorkflowField::reference(),
                ),
                ApplicationCapabilityFieldBinding::from_reference(AccountStatus::reference()),
            ),
            ApplicationCapabilityValidityDefinition::new(
                ApplicationCapabilityValidityTimeline::UnixEpochSeconds,
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityNotBeforeField::reference(),
                ),
                ApplicationCapabilityFieldBinding::from_reference(
                    CapabilityNotAfterField::reference(),
                ),
            ),
        ),
        CapabilityRequestContext::reference(),
    ))
    .delegation(ApplicationCapabilityDelegationDefinition::new(
        ApplicationCapabilityRelationBinding::from_reference(CapabilityParent::reference()),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityGrantor::reference()),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityGrantee::reference()),
        ApplicationCapabilityFieldBinding::from_reference(
            CapabilityDelegationLimitField::reference(),
        ),
        CapabilityProvenance::reference(),
    ))
    .composition(capability_composition())
    .build()
}

fn capability_composition() -> ApplicationCapabilityComposition {
    let allow = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityGrantee::reference())
        .forward(CapabilityResource::reference())
        .allow(Account::reference());
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(ApplicationCapabilityGraphRule::any([
                ApplicationCapabilityGraphClause::new(allow),
            ])),
            ApplicationCapabilityDenyRule::not_applicable(),
            ApplicationCapabilityConflictRule::not_applicable(),
        ),
        ApplicationCapabilityActorComposition::new(
            ApplicationCapabilitySeparationOfDutyRule::not_applicable(),
            ApplicationCapabilityDistinctActorRule::not_applicable(),
        ),
        ApplicationCapabilityPropagationComposition::new(
            ApplicationCapabilityDelegationRule::narrow_all_dimensions(),
            ApplicationCapabilityDisclosureRule::not_applicable(),
        ),
    )
}
