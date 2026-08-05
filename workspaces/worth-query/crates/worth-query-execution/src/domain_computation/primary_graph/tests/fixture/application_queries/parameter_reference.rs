use worth_query_declaration::facade::application_query::{
    ApplicationQueryParameterRef, ApplicationQueryResultFieldRef,
};

use super::{
    Activity, ActivityFacts, ActivitySequence, ActivitySequenceResultSlot, CrossRootQuery,
    IdentityExecutionSchema, StatusParameter,
};

pub(crate) fn status_parameter<Query>(
) -> ApplicationQueryParameterRef<Query, StatusParameter, String> {
    ApplicationQueryParameterRef::from_query_identifier("status")
}

pub(super) fn activity_sequence_result_field() -> ApplicationQueryResultFieldRef<
    CrossRootQuery,
    ActivitySequenceResultSlot,
    IdentityExecutionSchema,
    Activity,
    ActivityFacts,
    ActivitySequence,
    u64,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::NoEqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("sequence", ActivitySequence::reference())
}
