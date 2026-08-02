use worth_proof::{NonEmpty, TransitionOutcome};
use worth_signal::facade::TemporalDuration;

use super::super::ServingPhysicalRuntime;
use crate::physical_runtime::{
    certification::CertificationDurableMutationInput, AdmittedRecordPlacementPolicy,
    PhysicalDurabilityGroupBasis, PhysicalManifestCapacityTransition, PhysicalMutationDeadline,
    PhysicalMutationIdempotencyMaterial, PhysicalMutationPreparationSuccess,
    PhysicalMutationRequest, PhysicalWalGroupAppendOutcome, PhysicalWalGroupBarrierOutcome,
    RecordAppendBatch, WalDurablePhysicalMutation,
};

impl ServingPhysicalRuntime {
    pub fn certification_prepare_single_wal_durable_mutation(
        &self,
        placement: AdmittedRecordPlacementPolicy,
        manifest_capacity_transition: PhysicalManifestCapacityTransition,
        material: PhysicalMutationIdempotencyMaterial,
        batch: RecordAppendBatch,
    ) -> (PhysicalDurabilityGroupBasis, WalDurablePhysicalMutation) {
        let (basis, durable) = self.certification_prepare_wal_durable_group(
            placement,
            manifest_capacity_transition,
            NonEmpty::new(
                CertificationDurableMutationInput::new(material, batch),
                Vec::new(),
            ),
        );
        let durable = durable
            .into_vec()
            .pop()
            .expect("a singleton group derives one durable member");
        (basis, durable)
    }

    pub fn certification_prepare_wal_durable_group(
        &self,
        placement: AdmittedRecordPlacementPolicy,
        manifest_capacity_transition: PhysicalManifestCapacityTransition,
        inputs: NonEmpty<CertificationDurableMutationInput>,
    ) -> (
        PhysicalDurabilityGroupBasis,
        NonEmpty<WalDurablePhysicalMutation>,
    ) {
        let submission = self.record_submission();
        let prepared = inputs
            .into_vec()
            .into_iter()
            .map(|input| {
                let (material, batch) = input.into_parts();
                let key = submission
                    .issue_idempotency_key(material)
                    .expect("the explicit certification identity must be admitted");
                match submission
                    .prepare_durable_append_with_manifest_capacity_transition(
                        batch,
                        placement,
                        manifest_capacity_transition,
                        PhysicalMutationRequest::platform_durable(
                            key,
                            PhysicalMutationDeadline::at(
                                TemporalDuration::temporal_duration(1_000)
                                    .expect("the fixed certification deadline is nonzero"),
                            ),
                        ),
                    )
                    .into_raw()
                {
                    TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(
                        prepared,
                    )) => prepared,
                    _ => panic!("canonical durable preparation must succeed"),
                }
            })
            .collect::<Vec<_>>();
        let mut prepared = prepared.into_iter();
        let prepared = NonEmpty::new(
            prepared
                .next()
                .expect("a NonEmpty input yields a prepared member"),
            prepared.collect(),
        );
        let appended = match submission.append_prepared_wal_group(prepared) {
            PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
            PhysicalWalGroupAppendOutcome::NotAdmitted { cause, .. } => {
                panic!("canonical WAL append was not admitted: {cause:?}")
            }
            PhysicalWalGroupAppendOutcome::AdmissionRejected(rejected) => panic!(
                "canonical WAL append admission was rejected: {:?}",
                rejected.cause()
            ),
            PhysicalWalGroupAppendOutcome::NotStarted(continuation) => panic!(
                "canonical WAL append did not start: {:?}",
                continuation.cause()
            ),
            PhysicalWalGroupAppendOutcome::PartiallyAppended(continuation) => panic!(
                "canonical WAL append stopped after a partial effect: {:?}",
                continuation.cause()
            ),
            PhysicalWalGroupAppendOutcome::Indeterminate(_) => {
                panic!("canonical WAL append became indeterminate")
            }
        };
        let basis = appended.basis();
        let durable = match submission.synchronize_appended_wal_group(appended) {
            PhysicalWalGroupBarrierOutcome::Durable(durable) => durable.into_members(),
            _ => panic!("canonical WAL barrier must succeed"),
        };
        (basis, durable)
    }
}
