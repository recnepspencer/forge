use worth_query_declaration::{
    worth_query_operation, worth_query_operation_emits, worth_query_operation_expects_fact,
    worth_query_operation_reads, worth_query_operation_requires, worth_query_operation_writes,
};

use super::{
    AccountLabel, AccountStatus, IdentityExecutionSchema, LiveActivityEffect, ViewAccount,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct PublishLiveActivityInput;

worth_query_operation!(
    pub PublishLiveActivityOperation(PublishLiveActivityInput) in IdentityExecutionSchema
);
worth_query_operation_requires!(PublishLiveActivityOperation => [ViewAccount]);
worth_query_operation_expects_fact!(PublishLiveActivityOperation => [AccountStatus]);
worth_query_operation_reads!(PublishLiveActivityOperation => [AccountStatus, AccountLabel]);
worth_query_operation_writes!(PublishLiveActivityOperation => [AccountLabel]);
worth_query_operation_emits!(PublishLiveActivityOperation => [LiveActivityEffect]);
