use crate::physical_runtime::{PhysicalWorkObservation, PhysicalWorkShutdownObservation};

use super::identity::PhysicalWorkCourtroomIdentity;
use super::{
    causal_lowering::lower_causal,
    evidence::{
        PhysicalWorkCourtroomEvidenceParts, PhysicalWorkShutdownEvidence,
        PhysicalWorkShutdownEvidenceParts,
    },
    validation::validate_execution,
    PhysicalWorkArtifactBinding, PhysicalWorkCourtroomEvidence, PhysicalWorkCourtroomRunBinding,
    PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence,
};
use crate::physical_runtime::{LifecycleGeneration, RuntimeIdentity};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

pub struct PhysicalWorkCourtroomBinding {
    identity: PhysicalWorkCourtroomIdentity,
    observation: PhysicalWorkObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkCourtroomFinishDenial {
    ShutdownNotObserved,
}

impl PhysicalWorkCourtroomBinding {
    pub(in crate::physical_runtime) const fn new(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        generation: LifecycleGeneration,
        observation: PhysicalWorkObservation,
    ) -> Self {
        Self {
            identity: PhysicalWorkCourtroomIdentity::new(store, runtime, generation),
            observation,
        }
    }

    pub fn finish(
        self,
        run: PhysicalWorkCourtroomRunBinding,
        artifacts: impl IntoIterator<Item = PhysicalWorkArtifactBinding>,
        oracle: PhysicalWorkOracleEvidence,
        mutants: impl IntoIterator<Item = PhysicalWorkMutantLocalization>,
    ) -> Result<PhysicalWorkCourtroomEvidence, PhysicalWorkCourtroomFinishDenial> {
        let terminal = self
            .observation
            .terminal()
            .ok_or(PhysicalWorkCourtroomFinishDenial::ShutdownNotObserved)?;
        let records = self.observation.causal().records();
        let causal_overflow = self.observation.causal().overflow();
        let artifacts = artifacts.into_iter().collect::<Vec<_>>();
        let mutants = mutants.into_iter().collect::<Vec<_>>();
        let mut findings = validate_execution(
            self.identity,
            &records,
            causal_overflow,
            terminal,
            &artifacts,
            &oracle,
            &mutants,
        );
        let (causal, backend_profile) = lower_causal(self.identity, records, &mut findings);
        Ok(PhysicalWorkCourtroomEvidence::from_parts(
            PhysicalWorkCourtroomEvidenceParts {
                store: self.identity.store().bytes(),
                runtime: self.identity.runtime().get(),
                generation: self.identity.generation().get(),
                backend_profile,
                run,
                causal,
                causal_overflow,
                shutdown: lower_shutdown(terminal),
                artifacts: artifacts.into_boxed_slice(),
                oracle,
                mutants: mutants.into_boxed_slice(),
                findings,
            },
        ))
    }
}

fn lower_shutdown(terminal: &PhysicalWorkShutdownObservation) -> PhysicalWorkShutdownEvidence {
    let drain = terminal.drain();
    PhysicalWorkShutdownEvidence::from_parts(PhysicalWorkShutdownEvidenceParts {
        declared: terminal.declared(),
        blocked: terminal.blocked(),
        ready: terminal.ready(),
        queued: terminal.queued(),
        dispatched: terminal.dispatched(),
        settling: terminal.settling(),
        terminal_observations: terminal.terminal().len() as u64,
        residual: terminal.residual(),
        unaccounted_terminal: terminal.unaccounted_terminal(),
        settled: drain.settled().len() as u64,
        cancelled_before_dispatch: drain.cancelled_before_dispatch().len() as u64,
        continued_after_cancellation: drain.continued_after_consumer_cancellation().len() as u64,
        inspection_required: drain.inspection_required().len() as u64,
        released_before_dispatch: drain.released_before_dispatch().len() as u64,
        drain_residual: drain.residual().len() as u64,
        reconciliation_deferred: drain.derived_reconciliation_deferred().len() as u64,
        drain_evidence_overflow: drain.evidence_overflow(),
    })
}
