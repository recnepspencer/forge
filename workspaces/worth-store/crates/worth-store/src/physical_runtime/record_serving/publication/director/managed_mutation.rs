use worth_proof::NonEmpty;

use super::RecordPublicationDirector;
use crate::physical_runtime::{
    CompletedPhysicalMutationFact, IndeterminatePhysicalMutation,
    PhysicalCurrentRootAdvanceOutcome, PhysicalDataDispatchOutcome, PhysicalDataSettlementOutcome,
    PhysicalMutationAttempt, PhysicalMutationIndeterminateStage, PhysicalMutationProgressPhase,
    PhysicalMutationProvenNoEffectCause, PhysicalMutationTerminalFact,
    PhysicalPreSealCancellationOutcome, PhysicalRootNamespaceDurabilityOutcome,
    PhysicalRootPublicationPreparationOutcome, PhysicalRootReplacementOutcome,
    PhysicalWalGroupAppendOutcome, PhysicalWalGroupBarrierOutcome, PreparedPhysicalMutation,
};

impl RecordPublicationDirector {
    pub(in crate::physical_runtime) fn execute_managed_mutation(
        &self,
        prepared: PreparedPhysicalMutation,
        attempt: &PhysicalMutationAttempt,
    ) -> PhysicalMutationTerminalFact {
        let effect_cutover = attempt.effect_cutover();
        if attempt.cancellation_requested() {
            return self.pre_effect_terminal(
                prepared,
                attempt,
                PhysicalMutationProvenNoEffectCause::CancelledBeforeGroupSeal,
            );
        }
        match self.deadline_elapsed(attempt) {
            Ok(true) => {
                return self.pre_effect_terminal(
                    prepared,
                    attempt,
                    PhysicalMutationProvenNoEffectCause::DeadlineElapsedBeforeGroupSeal,
                )
            }
            Err(()) => {
                return self.pre_effect_terminal(
                    prepared,
                    attempt,
                    PhysicalMutationProvenNoEffectCause::WorkerUnavailableBeforeGroupSeal,
                )
            }
            Ok(false) => {}
        }

        attempt.enter(PhysicalMutationProgressPhase::WalAppend);
        let planned = match self.plan_prepared_group_for_wal(NonEmpty::new(prepared, Vec::new())) {
            Ok(planned) => planned,
            Err((members, _)) => {
                return self.pre_effect_terminal(
                    one(members),
                    attempt,
                    PhysicalMutationProvenNoEffectCause::AdmissionDeniedBeforeGroupSeal,
                )
            }
        };
        let appended = self.wal.append_prepared_group(planned);
        let appended = match appended {
            PhysicalWalGroupAppendOutcome::Appended(appended) => {
                attempt.commit_settlement();
                drop(effect_cutover);
                #[cfg(feature = "certification-test-authority")]
                self.mutations.reach_certification_checkpoint(
                    crate::physical_runtime::durability::
                        CertificationPhysicalMutationCheckpoint::AfterGroupSeal,
                );
                appended
            }
            PhysicalWalGroupAppendOutcome::NotAdmitted { members, .. } => {
                return self.pre_effect_terminal(
                    one(members),
                    attempt,
                    PhysicalMutationProvenNoEffectCause::AdmissionDeniedBeforeGroupSeal,
                )
            }
            PhysicalWalGroupAppendOutcome::AdmissionRejected(rejected) => {
                return self.pre_effect_terminal(
                    one(rejected.into_members()),
                    attempt,
                    PhysicalMutationProvenNoEffectCause::AdmissionDeniedBeforeGroupSeal,
                )
            }
            PhysicalWalGroupAppendOutcome::NotStarted(_)
            | PhysicalWalGroupAppendOutcome::PartiallyAppended(_)
            | PhysicalWalGroupAppendOutcome::Indeterminate(_) => {
                attempt.commit_settlement();
                drop(effect_cutover);
                return indeterminate(attempt, PhysicalMutationIndeterminateStage::WalAppend, 0);
            }
        };

        attempt.enter(PhysicalMutationProgressPhase::WalDurabilityBarrier);
        let durable = match self.wal_barrier.synchronize_appended_group(appended) {
            PhysicalWalGroupBarrierOutcome::Durable(durable) => durable,
            PhysicalWalGroupBarrierOutcome::BarrierNotStarted { .. }
            | PhysicalWalGroupBarrierOutcome::Indeterminate(_) => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::WalDurabilityBarrier,
                    1,
                )
            }
        };
        let basis = durable.basis();
        let durable = one(durable.into_members());
        #[cfg(feature = "certification-test-authority")]
        self.mutations.reach_certification_checkpoint(
            crate::physical_runtime::durability::CertificationPhysicalMutationCheckpoint::
                AfterWalDurability,
        );

        attempt.enter(PhysicalMutationProgressPhase::DataDispatch);
        let dispatched = match self.dispatch_wal_durable_data(durable) {
            PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
            PhysicalDataDispatchOutcome::RetryableAfterCleanup(_)
            | PhysicalDataDispatchOutcome::NotStarted { .. }
            | PhysicalDataDispatchOutcome::Indeterminate(_) => {
                return indeterminate(attempt, PhysicalMutationIndeterminateStage::DataDispatch, 1)
            }
        };

        attempt.enter(PhysicalMutationProgressPhase::DataSettlement);
        #[cfg(feature = "certification-test-authority")]
        self.mutations.reach_certification_checkpoint(
            crate::physical_runtime::durability::CertificationPhysicalMutationCheckpoint::
                DuringDataSettlement,
        );
        let settled = match dispatched.settle_exact_effects() {
            PhysicalDataSettlementOutcome::Settled(settled) => settled,
            PhysicalDataSettlementOutcome::InspectionRequired { .. } => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::DataSettlement,
                    1,
                )
            }
        };
        let settled = match crate::physical_runtime::DataSettledPhysicalMutationMembers::admit(
            basis,
            NonEmpty::new(settled, Vec::new()),
        ) {
            Ok(settled) => settled,
            Err(_) => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::DataSettlement,
                    1,
                )
            }
        };

        attempt.enter(PhysicalMutationProgressPhase::RootPreparation);
        let prepared_root = match PhysicalRootPublicationPreparationOutcome::from_result(
            self.prepare_settled_root_publication(settled),
        ) {
            PhysicalRootPublicationPreparationOutcome::Prepared(prepared) => prepared,
            PhysicalRootPublicationPreparationOutcome::NotStarted(_)
            | PhysicalRootPublicationPreparationOutcome::InspectionRequired(_) => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::RootPreparation,
                    1,
                )
            }
        };
        #[cfg(feature = "certification-test-authority")]
        self.mutations.reach_certification_checkpoint(
            crate::physical_runtime::durability::CertificationPhysicalMutationCheckpoint::
                DuringRootPublication,
        );

        attempt.enter(PhysicalMutationProgressPhase::RootReplacement);
        let replaced = match self.replace_prepared_root(prepared_root) {
            PhysicalRootReplacementOutcome::Replaced(replaced) => replaced,
            PhysicalRootReplacementOutcome::NotStarted(_)
            | PhysicalRootReplacementOutcome::InspectionRequired(_) => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::RootReplacement,
                    1,
                )
            }
        };

        attempt.enter(PhysicalMutationProgressPhase::RootNamespaceDurability);
        let namespace_durable = match self.synchronize_replaced_root_namespace(replaced) {
            PhysicalRootNamespaceDurabilityOutcome::Durable(durable) => durable,
            PhysicalRootNamespaceDurabilityOutcome::NotStarted(_)
            | PhysicalRootNamespaceDurabilityOutcome::InspectionRequired(_) => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::RootNamespaceDurability,
                    2,
                )
            }
        };

        attempt.enter(PhysicalMutationProgressPhase::CurrentRootAdvance);
        let completed = match self.advance_namespace_durable_root(namespace_durable) {
            PhysicalCurrentRootAdvanceOutcome::Advanced(completed) => completed,
            PhysicalCurrentRootAdvanceOutcome::InspectionRequired(_) => {
                return indeterminate(
                    attempt,
                    PhysicalMutationIndeterminateStage::CurrentRootAdvance,
                    3,
                )
            }
        };
        let member = completed
            .settled_members()
            .first()
            .expect("one-member managed group completes one exact member");
        debug_assert_eq!(member.mutation_identity(), attempt.identity());
        PhysicalMutationTerminalFact::Completed(CompletedPhysicalMutationFact::from_root_member(
            member,
            attempt.fingerprint(),
            completed.current_root().generation(),
        ))
    }

    fn deadline_elapsed(&self, attempt: &PhysicalMutationAttempt) -> Result<bool, ()> {
        self.runtime
            .upgrade()
            .ok_or(())?
            .signal
            .clock_observation()
            .map(|clock| attempt.deadline().signal_deadline().get() <= clock.current_tick())
            .map_err(|_| ())
    }

    fn pre_effect_terminal(
        &self,
        prepared: PreparedPhysicalMutation,
        attempt: &PhysicalMutationAttempt,
        cause: PhysicalMutationProvenNoEffectCause,
    ) -> PhysicalMutationTerminalFact {
        match self.settle_prepared_before_group_seal(prepared, cause) {
            PhysicalPreSealCancellationOutcome::ProvenNoEffect(terminal) => {
                PhysicalMutationTerminalFact::ProvenNoEffect(terminal)
            }
            PhysicalPreSealCancellationOutcome::NotCancelled { .. } => {
                indeterminate(attempt, PhysicalMutationIndeterminateStage::WalAppend, 0)
            }
        }
    }
}

fn one<T>(members: NonEmpty<T>) -> T {
    let mut members = members.into_vec().into_iter();
    let member = members
        .next()
        .expect("a managed physical mutation group is nonempty");
    debug_assert!(members.next().is_none());
    member
}

fn indeterminate(
    attempt: &PhysicalMutationAttempt,
    stage: PhysicalMutationIndeterminateStage,
    completed_effects: usize,
) -> PhysicalMutationTerminalFact {
    PhysicalMutationTerminalFact::Indeterminate(IndeterminatePhysicalMutation::possible_effect(
        attempt.identity(),
        attempt.idempotency_identity(),
        attempt.fingerprint(),
        stage,
        completed_effects,
    ))
}
