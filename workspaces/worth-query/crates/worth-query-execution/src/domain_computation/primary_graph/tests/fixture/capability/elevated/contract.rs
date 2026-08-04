use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
        ApplicationCapabilityAllowRule, ApplicationCapabilityAmountDimension,
        ApplicationCapabilityCardinalityDimension, ApplicationCapabilityComposition,
        ApplicationCapabilityConflictRule, ApplicationCapabilityConstraintDefinition,
        ApplicationCapabilityContextEntitySlotBinding, ApplicationCapabilityContract,
        ApplicationCapabilityContractBuilder, ApplicationCapabilityCurrentnessDefinition,
        ApplicationCapabilityDecisionComposition, ApplicationCapabilityDelegationDefinition,
        ApplicationCapabilityDelegationDepth, ApplicationCapabilityDelegationRule,
        ApplicationCapabilityDenyRule, ApplicationCapabilityDisclosureRule,
        ApplicationCapabilityDistinctActorRule, ApplicationCapabilityElevationDefinition,
        ApplicationCapabilityElevationLifecycleDefinition, ApplicationCapabilityElevationRule,
        ApplicationCapabilityElevationStates, ApplicationCapabilityFieldBinding,
        ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphClause,
        ApplicationCapabilityGraphRule, ApplicationCapabilityMandatoryReviewDefinition,
        ApplicationCapabilityOperationBinding, ApplicationCapabilityPropagationComposition,
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

use super::super::super::{
    Account, AccountLabel, AccountStatus, IdentityExecutionSchema, Principal,
};
use super::super::declaration::{
    CapabilityAction, CapabilityActionField, CapabilityAmountField,
    CapabilityConflictingBeneficiary, CapabilityDelegationLimitField, CapabilityDisclosure,
    CapabilityDisclosureField, CapabilityGrant, CapabilityGrantee, CapabilityGrantor,
    CapabilityNotAfterField, CapabilityNotBeforeField, CapabilityParent, CapabilityProvenance,
    CapabilityPurpose, CapabilityPurposeField, CapabilityRequestContext, CapabilityResource,
    CapabilityStatus, CapabilityStatusField, CapabilityWorkflowField,
};
use super::{
    ApproveCapabilityElevationOperation, CapabilityElevation, CapabilityElevationApprover,
    CapabilityElevationFacts, CapabilityElevationGrant, CapabilityElevationIdentity,
    CapabilityElevationNotAfter, CapabilityElevationNotBefore, CapabilityElevationReason,
    CapabilityElevationRequester, CapabilityElevationReview, CapabilityElevationSlot,
    CapabilityElevationStatus, CapabilityElevationStatusField, CapabilityReview,
    CapabilityReviewFacts, CapabilityReviewIdentity, CapabilityReviewSlot, CapabilityReviewStatus,
    CapabilityReviewStatusField, CapabilityReviewer, CompleteCapabilityReviewOperation,
    ElevatedCapabilityTouchInput, ElevatedCapabilityTouchOperation, ElevatedTouchAccountCapability,
    RequestCapabilityElevationOperation, RevokeCapabilityElevationOperation,
};

pub(in crate::domain_computation::primary_graph::tests::fixture::capability) fn install(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    schema
        .entity(CapabilityElevation::reference())
        .aspect(
            CapabilityElevation::reference(),
            CapabilityElevationFacts::reference(),
        )
        .field(
            CapabilityElevation::reference(),
            CapabilityElevationIdentity::reference(),
        )
        .field(
            CapabilityElevation::reference(),
            CapabilityElevationReason::reference(),
        )
        .field(
            CapabilityElevation::reference(),
            CapabilityElevationStatusField::reference(),
        )
        .field(
            CapabilityElevation::reference(),
            CapabilityElevationNotBefore::reference(),
        )
        .field(
            CapabilityElevation::reference(),
            CapabilityElevationNotAfter::reference(),
        )
        .entity(CapabilityReview::reference())
        .aspect(
            CapabilityReview::reference(),
            CapabilityReviewFacts::reference(),
        )
        .field(
            CapabilityReview::reference(),
            CapabilityReviewIdentity::reference(),
        )
        .field(
            CapabilityReview::reference(),
            CapabilityReviewStatusField::reference(),
        )
        .relation(
            CapabilityElevationRequester::reference(),
            Principal::reference(),
            CapabilityElevation::reference(),
        )
        .relation(
            CapabilityElevationApprover::reference(),
            Principal::reference(),
            CapabilityElevation::reference(),
        )
        .relation(
            CapabilityElevationGrant::reference(),
            CapabilityElevation::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            CapabilityElevationReview::reference(),
            CapabilityElevation::reference(),
            CapabilityReview::reference(),
        )
        .relation(
            CapabilityReviewer::reference(),
            Principal::reference(),
            CapabilityReview::reference(),
        )
        .capability_context_entity_slot(CapabilityElevationSlot::reference())
        .capability_context_entity_slot(CapabilityReviewSlot::reference())
        .operation(ElevatedCapabilityTouchOperation::reference())
        .operation(RequestCapabilityElevationOperation::reference())
        .operation(ApproveCapabilityElevationOperation::reference())
        .operation(RevokeCapabilityElevationOperation::reference())
        .operation(CompleteCapabilityReviewOperation::reference())
        .operation_decision_fact_budget(ElevatedCapabilityTouchOperation::reference(), 1)
        .operation_projection_work_budget(ElevatedCapabilityTouchOperation::reference(), 32)
        .operation_read_field(
            ElevatedCapabilityTouchOperation::reference(),
            AccountLabel::reference(),
        )
        .operation_write(
            ElevatedCapabilityTouchOperation::reference(),
            AccountLabel::reference(),
        )
        .capability(elevated_contract())
}

fn elevated_contract() -> ApplicationCapabilityContract<
    IdentityExecutionSchema,
    ElevatedTouchAccountCapability,
    ElevatedCapabilityTouchOperation,
    ElevatedCapabilityTouchInput,
> {
    ApplicationCapabilityContractBuilder::new(
        ElevatedTouchAccountCapability::reference(),
        ElevatedCapabilityTouchOperation::reference(),
        CapabilityGrant::reference(),
    )
    .target(ApplicationCapabilityTargetDefinition::new(
        ApplicationCapabilityValueBinding::new(
            CapabilityActionField::reference(),
            CapabilityAction::Touch,
        ),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityResource::reference()),
        ApplicationCapabilityRelationDimension::not_applicable(),
        ApplicationCapabilityFieldDimension::bound(CapabilityDisclosureField::reference()),
        ApplicationCapabilityValueBinding::new(
            CapabilityPurposeField::reference(),
            CapabilityPurpose::AccountMaintenance,
        ),
    ))
    .constraints(constraints())
    .delegation(delegation())
    .composition(composition())
    .elevation(elevation())
    .build()
}

fn constraints() -> ApplicationCapabilityConstraintDefinition {
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

fn delegation() -> ApplicationCapabilityDelegationDefinition {
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

fn composition() -> ApplicationCapabilityComposition {
    let allow = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityGrantor::reference())
        .forward(CapabilityResource::reference())
        .allow(Account::reference());
    let conflict = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityConflictingBeneficiary::reference())
        .deny(Account::reference());
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(ApplicationCapabilityGraphRule::any([
                ApplicationCapabilityGraphClause::new(allow),
            ])),
            ApplicationCapabilityDenyRule::not_applicable(),
            ApplicationCapabilityConflictRule::when(ApplicationCapabilityGraphRule::any([
                ApplicationCapabilityGraphClause::new(conflict),
            ])),
        ),
        ApplicationCapabilityActorComposition::new(
            ApplicationCapabilitySeparationOfDutyRule::not_applicable(),
            ApplicationCapabilityDistinctActorRule::not_applicable(),
        ),
        ApplicationCapabilityPropagationComposition::new(
            ApplicationCapabilityDelegationRule::narrow_all_dimensions(
                ApplicationCapabilityDelegationDepth::new(2).unwrap(),
            ),
            ApplicationCapabilityDisclosureRule::permit([
                ApplicationCapabilityScopeGuard::requiring([
                    ApplicationCapabilityAcceptedValues::one_of(
                        CapabilityDisclosureField::reference(),
                        [CapabilityDisclosure::AccountActivity],
                    ),
                ]),
            ]),
        ),
    )
}

fn elevation() -> ApplicationCapabilityElevationRule {
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
