use super::{
    WorthQueryLoweredMutationPlan, WorthQueryMutationAftermath, WorthQueryMutationCompletion,
    WorthQueryMutationCounters, WorthQueryMutationOutcome, WorthQueryMutationRequest,
    WorthQueryMutationStop, WorthQueryMutationStopSource,
};
use crate::runtime::{
    WorthQueryOrdinaryAuthorityDrift, WorthQueryOrdinaryAuthorityFamily, WorthQueryWorkspace,
};

impl WorthQueryMutationRequest {
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryMutationOutcome {
        let counters = WorthQueryMutationCounters::context_checked();
        debug_assert_eq!(
            self.context.authority.family(),
            WorthQueryOrdinaryAuthorityFamily::Mutation
        );
        match workspace.ordinary_authority_drift(&self.context.authority) {
            WorthQueryOrdinaryAuthorityDrift::ForeignOwner => {
                return WorthQueryMutationOutcome::Stopped(WorthQueryMutationStop::authority(
                    WorthQueryMutationStopSource::ForeignAuthority,
                    counters,
                ));
            }
            WorthQueryOrdinaryAuthorityDrift::StaleSnapshot => {
                return WorthQueryMutationOutcome::Stopped(WorthQueryMutationStop::authority(
                    WorthQueryMutationStopSource::StaleBasis,
                    counters,
                ));
            }
            WorthQueryOrdinaryAuthorityDrift::Current => {}
        }

        let counters = counters.execution_attempted();
        let execution = match workspace
            .execute_ordinary_authoritative_mutation(self.declaration.into_command())
        {
            Ok(execution) => execution,
            Err(error) => {
                return WorthQueryMutationOutcome::Stopped(WorthQueryMutationStop::runtime(
                    error, counters,
                ));
            }
        };
        let plan = WorthQueryLoweredMutationPlan::new(
            execution.request_identity().clone(),
            execution.handoff_identity().clone(),
        );
        let receipt_identity = execution.receipt_identity().clone();
        let inspection_identity = execution.inspection_identity().clone();
        let receipt = execution.into_receipt();
        let aftermath =
            WorthQueryMutationAftermath::new(&receipt, receipt_identity, inspection_identity);
        WorthQueryMutationOutcome::Completed(WorthQueryMutationCompletion::new(
            plan,
            receipt,
            aftermath,
            counters.execution_completed(),
        ))
    }
}
