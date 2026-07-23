use crate::physical_runtime::{
    record_serving::{RecordServingTerminalObservation, ServingShutdownOutcome},
    AbortedRuntime, ClosedRuntime, MediaShutdownOutcome,
};

use super::PhysicalStoreInstanceParts;
use crate::physical_runtime::work::{PhysicalWorkStopKind, PhysicalWorkSubmissionOwner};

impl PhysicalStoreInstanceParts {
    pub(in crate::physical_runtime) fn close(self) -> ServingShutdownOutcome<ClosedRuntime> {
        self.shutdown(PhysicalWorkStopKind::Close, |core| core.close())
    }

    pub(in crate::physical_runtime) fn abort(self) -> ServingShutdownOutcome<AbortedRuntime> {
        self.shutdown(PhysicalWorkStopKind::Abort, |core| core.abort())
    }

    fn shutdown<Terminal>(
        self,
        work_stop: PhysicalWorkStopKind,
        terminate_core: impl FnOnce(crate::physical_runtime::runtime::PhysicalRuntimeCore) -> Terminal,
    ) -> ServingShutdownOutcome<Terminal> {
        let Self {
            termination,
            work_admission: _work_admission,
            work_submission,
            signal_owner,
            scheduler_admission: _scheduler_admission,
            executor,
            core,
            record_owner,
            format: _,
            access: _,
            current_root: _,
            free_space: _,
            allocation_frontier: _,
            publication_residue,
            health,
            frame_ports,
        } = self;

        let work = stop_and_release_work_submission(work_submission, work_stop);
        let signal = signal_owner.dispose();
        let residency = frame_ports.close();
        drop(termination);
        let record_counters = record_owner.into_terminal_snapshot();
        let records = RecordServingTerminalObservation::new(
            health.requires_inspection()
                || !publication_residue.is_empty()
                || residency.requires_inspection(),
            publication_residue,
            record_counters,
        );
        let media_release = executor.into_media().close();
        let media = MediaShutdownOutcome::new(terminate_core(core), media_release);

        ServingShutdownOutcome::new(media, records, residency, work, signal)
    }
}

fn stop_and_release_work_submission(
    work_submission: PhysicalWorkSubmissionOwner,
    stop: PhysicalWorkStopKind,
) -> crate::physical_runtime::PhysicalWorkShutdownObservation {
    let observation = work_submission.stop(stop);
    drop(work_submission);
    observation
}
