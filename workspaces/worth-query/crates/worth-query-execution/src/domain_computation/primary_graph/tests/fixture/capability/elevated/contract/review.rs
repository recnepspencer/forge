use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityActorComposition, ApplicationCapabilityAllowRule,
        ApplicationCapabilityComposition, ApplicationCapabilityConflictRule,
        ApplicationCapabilityContract, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityDecisionComposition, ApplicationCapabilityDenyRule,
        ApplicationCapabilityDistinctActorRule, ApplicationCapabilityElevationRule,
        ApplicationCapabilityGraphClause, ApplicationCapabilityGraphRule,
        ApplicationCapabilityPathContextAnchor, ApplicationCapabilitySeparationOfDutyRule,
    },
    application_schema::{
        ApplicationAuthorizationPathBuilder, ApplicationSchemaDeclarationBuilder,
    },
};

use super::super::{
    CapabilityConflictingBeneficiary, CapabilityElevationApprover, CapabilityElevationGrant,
    CapabilityElevationIdentity, CapabilityElevationNotAfter, CapabilityElevationNotBefore,
    CapabilityElevationReason, CapabilityElevationRequester, CapabilityElevationReview,
    CapabilityElevationSlot, CapabilityElevationStatusField, CapabilityGrant, CapabilityGrantor,
    CapabilityResource, CapabilityReviewIdentity, CapabilityReviewStatusField, CapabilityReviewer,
    CompleteCapabilityReviewOperation, CompleteElevationReviewCapability,
    CompleteElevationReviewInput,
};
use super::{constraints, delegation, propagation, target};
use crate::domain_computation::primary_graph::tests::fixture::{
    Account, IdentityExecutionSchema, Principal,
};

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    let operation = CompleteCapabilityReviewOperation::reference();
    schema
        .operation_decision_fact_budget(operation, 12)
        .operation_projection_work_budget(operation, 96)
        .operation_read_field(operation, CapabilityElevationIdentity::reference())
        .operation_read_field(operation, CapabilityElevationReason::reference())
        .operation_read_field(operation, CapabilityElevationStatusField::reference())
        .operation_read_field(operation, CapabilityElevationNotBefore::reference())
        .operation_read_field(operation, CapabilityElevationNotAfter::reference())
        .operation_read_field(operation, CapabilityReviewIdentity::reference())
        .operation_read_field(operation, CapabilityReviewStatusField::reference())
        .operation_read_relation(operation, CapabilityElevationRequester::reference())
        .operation_read_relation(operation, CapabilityElevationApprover::reference())
        .operation_read_relation(operation, CapabilityElevationGrant::reference())
        .operation_read_relation(operation, CapabilityElevationReview::reference())
        .operation_read_relation(operation, CapabilityReviewer::reference())
        .operation_write(operation, CapabilityReviewStatusField::reference())
        .operation_link(operation, CapabilityReviewer::reference())
        .capability(contract())
}

fn contract() -> ApplicationCapabilityContract<
    IdentityExecutionSchema,
    CompleteElevationReviewCapability,
    CompleteCapabilityReviewOperation,
    CompleteElevationReviewInput,
> {
    ApplicationCapabilityContractBuilder::new(
        CompleteElevationReviewCapability::reference(),
        CompleteCapabilityReviewOperation::reference(),
        CapabilityGrant::reference(),
    )
    .target(target())
    .constraints(constraints())
    .delegation(delegation())
    .composition(composition())
    .elevation(ApplicationCapabilityElevationRule::not_applicable())
    .build()
}

fn composition() -> ApplicationCapabilityComposition {
    let allow = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityGrantor::reference())
        .forward(CapabilityResource::reference())
        .allow(Account::reference());
    let conflict = ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(CapabilityConflictingBeneficiary::reference())
        .deny(Account::reference());
    let requester = ApplicationCapabilityGraphClause::new(
        ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
            .forward(CapabilityElevationRequester::reference())
            .forward(CapabilityElevationGrant::reference())
            .forward(CapabilityResource::reference())
            .deny(Account::reference()),
    )
    .anchored([ApplicationCapabilityPathContextAnchor::after_forward(
        CapabilityElevationRequester::reference(),
        CapabilityElevationSlot::reference(),
    )]);
    let approver = ApplicationCapabilityGraphClause::new(
        ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
            .forward(CapabilityElevationApprover::reference())
            .forward(CapabilityElevationGrant::reference())
            .forward(CapabilityResource::reference())
            .deny(Account::reference()),
    )
    .anchored([ApplicationCapabilityPathContextAnchor::after_forward(
        CapabilityElevationApprover::reference(),
        CapabilityElevationSlot::reference(),
    )]);
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
            ApplicationCapabilityDistinctActorRule::when(ApplicationCapabilityGraphRule::any([
                requester, approver,
            ])),
        ),
        propagation(),
    )
}
