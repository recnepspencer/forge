use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
        ApplicationCapabilityAllowRule, ApplicationCapabilityAmountDimension,
        ApplicationCapabilityCardinalityDimension, ApplicationCapabilityComposition,
        ApplicationCapabilityConflictRule, ApplicationCapabilityConstraintDefinition,
        ApplicationCapabilityContract, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityCurrentnessDefinition, ApplicationCapabilityDecisionComposition,
        ApplicationCapabilityDelegationDefinition, ApplicationCapabilityDelegationDepth,
        ApplicationCapabilityDelegationRule, ApplicationCapabilityDenyRule,
        ApplicationCapabilityDisclosureRule, ApplicationCapabilityDistinctActorRule,
        ApplicationCapabilityElevationDefinition, ApplicationCapabilityElevationRule,
        ApplicationCapabilityElevationStates, ApplicationCapabilityFieldBinding,
        ApplicationCapabilityFieldDimension, ApplicationCapabilityGraphClause,
        ApplicationCapabilityGraphRule, ApplicationCapabilityMandatoryReviewDefinition,
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

use super::super::super::{
    Account, AccountLabel, AccountStatus, IdentityExecutionSchema, Principal,
};
use super::super::declaration::{
    CapabilityAction, CapabilityActionField, CapabilityAmountField, CapabilityDelegationLimitField,
    CapabilityDisclosure, CapabilityDisclosureField, CapabilityGrant, CapabilityGrantee,
    CapabilityGrantor, CapabilityNotAfterField, CapabilityNotBeforeField, CapabilityParent,
    CapabilityProvenance, CapabilityPurpose, CapabilityPurposeField, CapabilityRequestContext,
    CapabilityResource, CapabilityStatus, CapabilityStatusField, CapabilityWorkflowField,
};
use super::{
    CapabilityElevation, CapabilityElevationApprover, CapabilityElevationFacts,
    CapabilityElevationGrant, CapabilityElevationIdentity, CapabilityElevationReason,
    CapabilityElevationRequester, CapabilityElevationReview, CapabilityElevationStatus,
    CapabilityElevationStatusField, CapabilityReview, CapabilityReviewFacts,
    CapabilityReviewIdentity, CapabilityReviewStatus, CapabilityReviewStatusField,
    CapabilityReviewer, ElevatedCapabilityTouchInput, ElevatedCapabilityTouchOperation,
    ElevatedTouchAccountCapability,
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
        .operation(ElevatedCapabilityTouchOperation::reference())
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
            state(CapabilityElevationStatus::Active),
            state(CapabilityElevationStatus::Revoked),
            state(CapabilityElevationStatus::ReviewRequired),
            state(CapabilityElevationStatus::Reviewed),
        ),
        ApplicationCapabilityRelationBinding::from_reference(
            CapabilityElevationRequester::reference(),
        ),
        ApplicationCapabilityRelationBinding::from_reference(
            CapabilityElevationApprover::reference(),
        ),
        ApplicationCapabilityRelationBinding::from_reference(CapabilityElevationGrant::reference()),
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
