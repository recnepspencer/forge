use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityContract, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityElevationRule,
    },
    application_schema::ApplicationSchemaDeclarationBuilder,
};

use super::super::{
    CapabilityElevation, CapabilityElevationGrant, CapabilityElevationIdentity,
    CapabilityElevationNotAfter, CapabilityElevationNotBefore, CapabilityElevationReason,
    CapabilityElevationRequester, CapabilityElevationReview, CapabilityElevationStatusField,
    CapabilityGrant, CapabilityReview, CapabilityReviewIdentity, CapabilityReviewStatusField,
    RequestCapabilityElevationOperation, RequestElevationCapability, RequestElevationInput,
};
use super::{composition, constraints, delegation, target};
use crate::domain_computation::primary_graph::tests::fixture::{
    AccountLabel, IdentityExecutionSchema,
};

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema>,
) -> ApplicationSchemaDeclarationBuilder<IdentityExecutionSchema> {
    schema
        .operation_decision_fact_budget(RequestCapabilityElevationOperation::reference(), 1)
        .operation_projection_work_budget(RequestCapabilityElevationOperation::reference(), 32)
        .operation_read_field(
            RequestCapabilityElevationOperation::reference(),
            AccountLabel::reference(),
        )
        .operation_create(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevation::reference(),
        )
        .operation_create(
            RequestCapabilityElevationOperation::reference(),
            CapabilityReview::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationIdentity::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationReason::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationStatusField::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationNotBefore::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationNotAfter::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityReviewIdentity::reference(),
        )
        .operation_write(
            RequestCapabilityElevationOperation::reference(),
            CapabilityReviewStatusField::reference(),
        )
        .operation_link(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationRequester::reference(),
        )
        .operation_link(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationGrant::reference(),
        )
        .operation_link(
            RequestCapabilityElevationOperation::reference(),
            CapabilityElevationReview::reference(),
        )
        .capability(request_contract())
}

fn request_contract() -> ApplicationCapabilityContract<
    IdentityExecutionSchema,
    RequestElevationCapability,
    RequestCapabilityElevationOperation,
    RequestElevationInput,
> {
    ApplicationCapabilityContractBuilder::new(
        RequestElevationCapability::reference(),
        RequestCapabilityElevationOperation::reference(),
        CapabilityGrant::reference(),
    )
    .target(target())
    .constraints(constraints())
    .delegation(delegation())
    .composition(composition())
    .elevation(ApplicationCapabilityElevationRule::not_applicable())
    .build()
}
