use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
        ApplicationCapabilityAllowRule, ApplicationCapabilityAmountDimension,
        ApplicationCapabilityCardinalityDimension, ApplicationCapabilityComposition,
        ApplicationCapabilityConflictRule, ApplicationCapabilityConstraintDefinition,
        ApplicationCapabilityContract, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityCurrentnessDefinition, ApplicationCapabilityDecisionComposition,
        ApplicationCapabilityDelegationDefinition, ApplicationCapabilityDelegationRule,
        ApplicationCapabilityDenyRule, ApplicationCapabilityDisclosureRule,
        ApplicationCapabilityDistinctActorRule, ApplicationCapabilityFieldBinding,
        ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphClause,
        ApplicationCapabilityGraphRule, ApplicationCapabilityPropagationComposition,
        ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
        ApplicationCapabilityScopeGuard, ApplicationCapabilitySeparationOfDutyRule,
        ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
        ApplicationCapabilityValidityTimeline, ApplicationCapabilityValueBinding,
        ApplicationCapabilityWorkflowDefinition,
    },
    application_schema::{
        ApplicationAuthorizationPathBuilder, ApplicationSchemaDeclarationBuilder,
    },
};

use super::super::{Account, AccountLabel, AccountStatus, IdentityExecutionSchema, Principal};
use super::declaration::*;

pub(in super::super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    schema
        .entity(CapabilityGrant::reference())
        .aspect(CapabilityGrant::reference(), CapabilityFacts::reference())
        .field(
            CapabilityGrant::reference(),
            CapabilityIdentity::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityActionField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityPurposeField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityDisclosureField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityAmountField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityStatusField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityWorkflowField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityNotBeforeField::reference(),
        )
        .field(
            CapabilityGrant::reference(),
            CapabilityNotAfterField::reference(),
        )
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
            CapabilityCustodian::reference(),
            Principal::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            CapabilityResource::reference(),
            CapabilityGrant::reference(),
            Account::reference(),
        )
        .relation(
            CapabilityRelated::reference(),
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

fn capability_contract() -> ApplicationCapabilityContract<
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
        ApplicationCapabilityRelationDimension::bound(CapabilityRelated::reference()),
        ApplicationCapabilityFieldDimension::bound(CapabilityDisclosureField::reference()),
        ApplicationCapabilityValueBinding::new(
            CapabilityPurposeField::reference(),
            CapabilityPurpose::AccountMaintenance,
        ),
    ))
    .constraints(ApplicationCapabilityConstraintDefinition::new(
        ApplicationCapabilityAmountDimension::bound(CapabilityAmountField::reference()),
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
    let grantor = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityGrantor::reference())
        .forward(CapabilityResource::reference())
        .allow(Account::reference());
    let custodian = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityCustodian::reference())
        .forward(CapabilityResource::reference())
        .allow(Account::reference());
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(ApplicationCapabilityGraphRule::any([
                ApplicationCapabilityGraphClause::new(grantor),
                ApplicationCapabilityGraphClause::new(custodian),
            ])),
            ApplicationCapabilityDenyRule::not_applicable(),
            ApplicationCapabilityConflictRule::not_applicable(),
        ),
        ApplicationCapabilityActorComposition::new(
            ApplicationCapabilitySeparationOfDutyRule::not_applicable(),
            ApplicationCapabilityDistinctActorRule::not_applicable(),
        ),
        ApplicationCapabilityPropagationComposition::new(
            ApplicationCapabilityDelegationRule::narrow_all_dimensions(
                worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationDepth::new(2)
                    .unwrap(),
            ),
            ApplicationCapabilityDisclosureRule::permit([
                ApplicationCapabilityScopeGuard::requiring([
                    ApplicationCapabilityAcceptedValues::one_of(
                        CapabilityDisclosureField::reference(),
                        [
                            CapabilityDisclosure::AccountActivity,
                            CapabilityDisclosure::PrivateLabel,
                        ],
                    ),
                ]),
            ]),
        ),
    )
}
