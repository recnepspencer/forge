use std::sync::atomic::Ordering;

use super::{PhysicalWorkStopKind, PhysicalWorkSubmissionOwner};
use crate::physical_runtime::work::{
    command_storage::DrainedPhysicalCommand, PhysicalWorkShutdownObservation,
    PhysicalWorkTerminalDisposition, PhysicalWorkTerminalEvent, PhysicalWorkTerminalStage,
};

pub(in crate::physical_runtime) struct PhysicalWorkSafeCancellation {
    drained: Vec<DrainedPhysicalCommand>,
    candidates: Box<[crate::physical_runtime::PhysicalWorkConsumerHandle]>,
    disposition: PhysicalWorkTerminalDisposition,
    completed_before_cancellation: u64,
}

impl PhysicalWorkSubmissionOwner {
    pub(in crate::physical_runtime) fn stop_admission(&self) {
        self.shared.stop_accepting();
    }

    pub(in crate::physical_runtime) fn cancel_safe_work(
        &self,
        kind: PhysicalWorkStopKind,
    ) -> PhysicalWorkSafeCancellation {
        self.shared.await_submissions();
        self.shared.abandonment.await_idle();
        let disposition = disposition(kind);
        let completed_before_cancellation = self.shared.accounting.terminal();
        let drained = self.shared.commands.drain_before_dispatch();
        for command in &drained {
            self.release_drained(command);
        }
        let mut candidates = drained
            .iter()
            .filter_map(|command| command.consumer)
            .collect::<Vec<_>>();
        candidates.extend(self.shared.terminal_ledger.cancellation_candidates());
        candidates.sort_by_key(|consumer| consumer.identity().operation().get());
        candidates.dedup_by_key(|consumer| consumer.identity());
        PhysicalWorkSafeCancellation {
            drained,
            candidates: candidates.into_boxed_slice(),
            disposition,
            completed_before_cancellation,
        }
    }

    pub(in crate::physical_runtime) fn settle_dispatches(
        &self,
        cancellation: PhysicalWorkSafeCancellation,
    ) -> PhysicalWorkShutdownObservation {
        self.shared.abandonment.await_idle();
        self.shared.await_idle();
        let residual = self.shared.commands.active_stages();
        let observation = PhysicalWorkShutdownObservation::from_active(
            self.shared.accounting.declared(),
            cancellation.completed_before_cancellation,
            cancellation
                .drained
                .iter()
                .map(|command| (command.identity, command.stage, command.consumer)),
            cancellation.disposition,
        )
        .with_additional_cancellation_candidates(cancellation.candidates.iter().copied())
        .with_drain(self.shared.terminal_ledger.observe(residual));
        if !self.shared.terminal_published.swap(true, Ordering::AcqRel) {
            self.observation.publish(observation.clone());
        }
        observation
    }

    pub(in crate::physical_runtime) fn stop(
        &self,
        kind: PhysicalWorkStopKind,
    ) -> PhysicalWorkShutdownObservation {
        self.stop_admission();
        let cancellation = self.cancel_safe_work(kind);
        self.settle_dispatches(cancellation)
    }

    fn release_drained(&self, command: &DrainedPhysicalCommand) {
        if !command.release.claim_shutdown_release() {
            return;
        }
        self.shared
            .release_capacity(command.scope_members, command.semantic_bytes);
        self.shared
            .accounting
            .record_terminal(command.operation, command.pressure);
        if !matches!(
            command.stage,
            PhysicalWorkTerminalStage::Dispatched | PhysicalWorkTerminalStage::Settling
        ) {
            self.shared
                .terminal_ledger
                .record(PhysicalWorkTerminalEvent::CancelledBeforeDispatch(
                    command.identity,
                ));
        }
    }
}

impl PhysicalWorkSafeCancellation {
    pub(in crate::physical_runtime) const fn cancellation_candidates(
        &self,
    ) -> &[crate::physical_runtime::PhysicalWorkConsumerHandle] {
        &self.candidates
    }
}

const fn disposition(kind: PhysicalWorkStopKind) -> PhysicalWorkTerminalDisposition {
    match kind {
        PhysicalWorkStopKind::Close => PhysicalWorkTerminalDisposition::ClosedBeforeReadiness,
        PhysicalWorkStopKind::Abort => PhysicalWorkTerminalDisposition::AbortedBeforeReadiness,
        PhysicalWorkStopKind::Drop => PhysicalWorkTerminalDisposition::DroppedBeforeReadiness,
    }
}
