use worth_proof::NonEmpty;

use super::super::ServingPhysicalRuntime;
use crate::physical_runtime::{
    CompletedPhysicalRootPublication, DataDispatchedPhysicalMutation,
    PhysicalCurrentRootAdvanceOutcome, PhysicalDataSettlementOutcome, PhysicalDurabilityGroupBasis,
    PhysicalRootNamespaceDurabilityOutcome, PhysicalRootPublicationPreparationOutcome,
    PhysicalRootReplacementOutcome,
};

impl ServingPhysicalRuntime {
    pub fn certification_complete_dispatched_mutation(
        &self,
        basis: PhysicalDurabilityGroupBasis,
        dispatched: DataDispatchedPhysicalMutation,
    ) -> CompletedPhysicalRootPublication {
        self.certification_complete_dispatched_group(basis, NonEmpty::new(dispatched, Vec::new()))
    }

    pub fn certification_complete_dispatched_group(
        &self,
        basis: PhysicalDurabilityGroupBasis,
        dispatched: NonEmpty<DataDispatchedPhysicalMutation>,
    ) -> CompletedPhysicalRootPublication {
        let submission = self.record_submission();
        let settled = settle_dispatched_group(dispatched);
        let joined = submission
            .join_data_settled_group(basis, settled)
            .unwrap_or_else(|rejected| {
                panic!(
                    "the exact settled group was rejected: {:?}",
                    rejected.cause()
                )
            });
        let prepared = match submission.prepare_root_publication(joined) {
            PhysicalRootPublicationPreparationOutcome::Prepared(prepared) => prepared,
            PhysicalRootPublicationPreparationOutcome::NotStarted(failure) => {
                panic!("root preparation did not start: {:?}", failure.cause())
            }
            PhysicalRootPublicationPreparationOutcome::InspectionRequired(failure) => {
                panic!(
                    "root preparation became indeterminate: {:?}",
                    failure.cause()
                )
            }
        };
        let replaced = match submission.replace_prepared_root(prepared) {
            PhysicalRootReplacementOutcome::Replaced(replaced) => replaced,
            PhysicalRootReplacementOutcome::NotStarted(failure) => {
                panic!("root replacement did not start: {:?}", failure.cause())
            }
            PhysicalRootReplacementOutcome::InspectionRequired(failure) => panic!(
                "root replacement became indeterminate: {:?} {:?}",
                failure.effect_fate(),
                failure.recovery_disposition()
            ),
        };
        let durable = match submission.synchronize_replaced_root_namespace(replaced) {
            PhysicalRootNamespaceDurabilityOutcome::Durable(durable) => durable,
            PhysicalRootNamespaceDurabilityOutcome::NotStarted(failure) => {
                panic!(
                    "namespace synchronization did not start: {:?}",
                    failure.cause()
                )
            }
            PhysicalRootNamespaceDurabilityOutcome::InspectionRequired(failure) => panic!(
                "namespace synchronization became indeterminate: {:?} {:?}",
                failure.effect_fate(),
                failure.recovery_disposition()
            ),
        };
        match submission.advance_namespace_durable_root(durable) {
            PhysicalCurrentRootAdvanceOutcome::Advanced(completed) => completed,
            PhysicalCurrentRootAdvanceOutcome::InspectionRequired(failure) => {
                panic!("current-root advance was rejected: {:?}", failure.cause())
            }
        }
    }
}

fn settle_dispatched_group(
    dispatched: NonEmpty<DataDispatchedPhysicalMutation>,
) -> NonEmpty<crate::physical_runtime::DataSettledPhysicalMutation> {
    let settled = dispatched
        .into_vec()
        .into_iter()
        .map(|dispatched| match dispatched.settle_exact_effects() {
            PhysicalDataSettlementOutcome::Settled(settled) => settled,
            PhysicalDataSettlementOutcome::InspectionRequired { cause, .. } => {
                panic!("the exact data effects must settle: {cause:?}")
            }
        })
        .collect::<Vec<_>>();
    let mut settled = settled.into_iter();
    NonEmpty::new(
        settled
            .next()
            .expect("a NonEmpty dispatched group yields a settled member"),
        settled.collect(),
    )
}
