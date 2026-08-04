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
        ApplicationCapabilityGraphRule, ApplicationCapabilityPathContextAnchor,
        ApplicationCapabilityPropagationComposition, ApplicationCapabilityRelationBinding,
        ApplicationCapabilityRelationDimension, ApplicationCapabilityScopeGuard,
        ApplicationCapabilitySeparationOfDutyRule, ApplicationCapabilityTargetDefinition,
        ApplicationCapabilityValidityDefinition, ApplicationCapabilityValidityTimeline,
        ApplicationCapabilityValueBinding, ApplicationCapabilityWorkflowDefinition,
    },
    application_schema::{
        ApplicationAuthorizationPathBuilder, ApplicationSchemaDeclarationBuilder,
    },
};

use super::super::{
    Account, AccountLabel, AccountOwner, AccountStatus, IdentityExecutionSchema, Principal,
};
use super::declaration::*;

pub(in super::super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    let schema = install_grant_facts(schema);
    let schema = install_grant_relations(schema);
    install_capability_operations(schema)
}

fn install_grant_facts(
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
        .entity(CapabilityActionRecord::reference())
        .aspect(
            CapabilityActionRecord::reference(),
            CapabilityActionRecordFacts::reference(),
        )
        .field(
            CapabilityActionRecord::reference(),
            CapabilityActionRecordIdentity::reference(),
        )
}

fn install_grant_relations(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    schema
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
        .relation(
            CapabilityExplicitDeny::reference(),
            Principal::reference(),
            Account::reference(),
        )
        .relation(
            CapabilityConflictingBeneficiary::reference(),
            Principal::reference(),
            Account::reference(),
        )
        .relation(
            CapabilityRequestActor::reference(),
            Principal::reference(),
            CapabilityActionRecord::reference(),
        )
        .relation(
            CapabilityPriorActor::reference(),
            Principal::reference(),
            CapabilityActionRecord::reference(),
        )
        .relation(
            CapabilityActionResource::reference(),
            CapabilityActionRecord::reference(),
            Account::reference(),
        )
}

fn install_capability_operations(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    schema
        .capability_context(CapabilityRequestContext::reference())
        .capability_context_entity_slot(CapabilityRequestActorSlot::reference())
        .capability_context_entity_slot(CapabilityPriorActorSlot::reference())
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
        .operation(ComposedCapabilityTouchOperation::reference())
        .operation_decision_fact_budget(ComposedCapabilityTouchOperation::reference(), 1)
        .operation_projection_work_budget(ComposedCapabilityTouchOperation::reference(), 32)
        .operation_read_field(
            ComposedCapabilityTouchOperation::reference(),
            AccountLabel::reference(),
        )
        .operation_write(
            ComposedCapabilityTouchOperation::reference(),
            AccountLabel::reference(),
        )
        .capability(capability_contract())
        .capability(composed_capability_contract())
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
    .target(capability_target())
    .constraints(capability_constraints())
    .delegation(capability_delegation())
    .composition(capability_composition())
    .build()
}

fn composed_capability_contract() -> ApplicationCapabilityContract<
    IdentityExecutionSchema,
    ComposedTouchAccountCapability,
    ComposedCapabilityTouchOperation,
    CapabilityTouchInput,
> {
    ApplicationCapabilityContractBuilder::new(
        ComposedTouchAccountCapability::reference(),
        ComposedCapabilityTouchOperation::reference(),
        CapabilityGrant::reference(),
    )
    .target(capability_target())
    .constraints(capability_constraints())
    .delegation(capability_delegation())
    .composition(composed_capability_composition())
    .build()
}

fn capability_target() -> ApplicationCapabilityTargetDefinition {
    ApplicationCapabilityTargetDefinition::new(
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
    )
}

fn capability_constraints() -> ApplicationCapabilityConstraintDefinition {
    ApplicationCapabilityConstraintDefinition::new(
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
    )
}

fn capability_delegation() -> ApplicationCapabilityDelegationDefinition {
    ApplicationCapabilityDelegationDefinition::new(
        ApplicationCapabilityRelationBinding::from_reference(CapabilityParent::reference()),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityGrantor::reference()),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityGrantee::reference()),
        ApplicationCapabilityFieldBinding::from_reference(
            CapabilityDelegationLimitField::reference(),
        ),
        CapabilityProvenance::reference(),
    )
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
        capability_propagation(),
    )
}

fn composed_capability_composition() -> ApplicationCapabilityComposition {
    let assignment = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(AccountOwner::reference())
        .allow(Account::reference());
    let explicit_deny = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityExplicitDeny::reference())
        .deny(Account::reference());
    let conflict = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityConflictingBeneficiary::reference())
        .deny(Account::reference());
    let request_actor = ApplicationCapabilityGraphClause::new(
        ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
            .forward(CapabilityRequestActor::reference())
            .forward(CapabilityActionResource::reference())
            .deny(Account::reference()),
    )
    .anchored([ApplicationCapabilityPathContextAnchor::after_forward(
        CapabilityRequestActor::reference(),
        CapabilityRequestActorSlot::reference(),
    )]);
    let prior_actor = ApplicationCapabilityGraphClause::new(
        ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
            .forward(CapabilityPriorActor::reference())
            .forward(CapabilityActionResource::reference())
            .deny(Account::reference()),
    )
    .anchored([ApplicationCapabilityPathContextAnchor::after_forward(
        CapabilityPriorActor::reference(),
        CapabilityPriorActorSlot::reference(),
    )]);
    let graph =
        |path| ApplicationCapabilityGraphRule::any([ApplicationCapabilityGraphClause::new(path)]);
    let anchored_graph = |clause| ApplicationCapabilityGraphRule::any([clause]);
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(graph(assignment)),
            ApplicationCapabilityDenyRule::when(graph(explicit_deny)),
            ApplicationCapabilityConflictRule::when(graph(conflict)),
        ),
        ApplicationCapabilityActorComposition::new(
            ApplicationCapabilitySeparationOfDutyRule::when(anchored_graph(request_actor)),
            ApplicationCapabilityDistinctActorRule::when(anchored_graph(prior_actor)),
        ),
        capability_propagation(),
    )
}

fn capability_propagation() -> ApplicationCapabilityPropagationComposition {
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
    )
}
