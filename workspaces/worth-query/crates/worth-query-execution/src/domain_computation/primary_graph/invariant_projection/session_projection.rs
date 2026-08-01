use std::marker::PhantomData;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::sync::Arc;

use worth_query_installation::facade::ApplicationSchema;

use super::work::WorthQueryInvariantProjectionWorkBudget;
use super::{
    WorthQueryApplicationInvariantProjectionAuthority,
    WorthQueryApplicationInvariantProjectionReader, WorthQueryInvariantProjectionWork,
    WorthQueryRealizedProjectionScope,
};
use crate::domain_computation::primary_graph::application_attempt::snapshot_lease::WorthQueryApplicationSnapshotLease;

pub(super) struct WorthQueryCompletedSessionInvariantProjection<Output> {
    pub(super) output: Output,
    pub(super) work: WorthQueryInvariantProjectionWork,
    pub(super) realized_scope: WorthQueryRealizedProjectionScope,
}

impl<Schema> WorthQueryApplicationInvariantProjectionAuthority<Schema>
where
    Schema: ApplicationSchema,
{
    pub(super) fn project_session_bounded<Output>(
        &self,
        lease: &WorthQueryApplicationSnapshotLease,
        maximum_work: usize,
        projection: impl FnOnce(
            &mut WorthQueryApplicationInvariantProjectionReader<'_, Schema>,
        ) -> Output,
    ) -> Result<
        WorthQueryCompletedSessionInvariantProjection<Output>,
        super::locked_reader::WorthQueryInvariantProjectionWorkLimitExceeded,
    > {
        let projected = lease.handle().with_runtime_mut(|runtime| {
            catch_unwind(AssertUnwindSafe(|| {
                let mut reader = WorthQueryApplicationInvariantProjectionReader {
                    runtime,
                    layout: lease.layout(),
                    snapshot: lease.snapshot(),
                    runtime_authority: self.runtime_authority,
                    binding_identity: self.binding_identity.clone(),
                    authority_identity: self.authority_identity,
                    work: WorthQueryInvariantProjectionWork::default(),
                    work_budget: WorthQueryInvariantProjectionWorkBudget::bounded(maximum_work),
                    realized_scope: WorthQueryRealizedProjectionScope::default(),
                    aggregate_projections: Arc::clone(&self.graph.aggregate_projections),
                    _schema: PhantomData,
                };
                let output = projection(&mut reader);
                (
                    output,
                    reader.work,
                    reader.realized_scope,
                    reader.work_budget.exceeded(),
                )
            }))
        });
        match projected {
            Ok((_, _, _, true)) => {
                Err(super::locked_reader::WorthQueryInvariantProjectionWorkLimitExceeded)
            }
            Ok((output, work, realized_scope, false)) => {
                Ok(WorthQueryCompletedSessionInvariantProjection {
                    output,
                    work,
                    realized_scope,
                })
            }
            Err(payload) => resume_unwind(payload),
        }
    }
}
