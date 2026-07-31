use worth_store::physical_runtime::{
    PhysicalDataDispatchOutcome, PhysicalDataSettlementOutcome, PhysicalRecordSubmission,
    WalDurablePhysicalMutation,
};

fn dispatch_and_settle_exact_data(
    submission: &PhysicalRecordSubmission,
    durable: WalDurablePhysicalMutation,
) {
    match submission.dispatch_wal_durable_data(durable) {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => {
            match dispatched.settle_exact_effects() {
                PhysicalDataSettlementOutcome::Settled(settled) => {
                    let _completed_identity = settled.mutation_identity();
                }
                PhysicalDataSettlementOutcome::InspectionRequired { dispatched, cause } => {
                    let _inspection_basis = (dispatched.mutation_identity(), cause);
                }
            }
        }
        PhysicalDataDispatchOutcome::NotStarted { durable, cause } => {
            let _preserved_authority = (durable.mutation_identity(), cause);
        }
        PhysicalDataDispatchOutcome::Indeterminate(indeterminate) => {
            let _inspection_basis = (
                indeterminate.mutation_identity(),
                indeterminate.completed_frames(),
                indeterminate.cause(),
            );
        }
    }
}

fn main() {
    let _ = dispatch_and_settle_exact_data;
}
