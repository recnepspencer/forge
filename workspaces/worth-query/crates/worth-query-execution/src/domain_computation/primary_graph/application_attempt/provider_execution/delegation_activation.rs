use worth_query_installation::facade::ApplicationSchema;

use super::super::{
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationIdempotencyBinding,
    WorthQueryDelegationActivationProgram,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn compare_and_commit_capability_delegation<Operation, Input, Scope>(
        &self,
        program: WorthQueryDelegationActivationProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> WorthQueryApplicationCommitOutcome {
        self.compare_and_commit_application_inner(program.into_inner(), idempotency)
    }
}
