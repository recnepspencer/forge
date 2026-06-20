use super::{
    ForgeQueryGraphReadMaterializationAdmittedLimits, ForgeQueryGraphReadMaterializationCheckpoint,
    ForgeQueryGraphReadMaterializationCounters, ForgeQueryGraphReadMaterializationJob,
    ForgeQueryGraphReadMaterializationProgress, ForgeQueryGraphReadMaterializationRequest,
};
use crate::runtime::{ForgeQueryRuntime, ForgeQueryRuntimeError};

pub struct ForgeQueryGraphReadMaterializationRuntime<'a> {
    runtime: &'a mut ForgeQueryRuntime,
}

impl<'a> ForgeQueryGraphReadMaterializationRuntime<'a> {
    pub(crate) fn new(runtime: &'a mut ForgeQueryRuntime) -> Self {
        Self { runtime }
    }

    pub fn admit(
        self,
        request: ForgeQueryGraphReadMaterializationRequest,
    ) -> Result<ForgeQueryGraphReadMaterializationAdmittedJob, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(crate::runtime::ForgeQueryRuntimeFacadeFamily::Read)?;
        Ok(ForgeQueryGraphReadMaterializationAdmittedJob {
            snapshot_identity: self
                .runtime
                .current_snapshot_identity()
                .evidence_identity()
                .as_str()
                .to_string(),
            request,
        })
    }
}

pub struct ForgeQueryGraphReadMaterializationAdmittedJob {
    request: ForgeQueryGraphReadMaterializationRequest,
    snapshot_identity: String,
}

impl ForgeQueryGraphReadMaterializationAdmittedJob {
    pub fn request(&self) -> &ForgeQueryGraphReadMaterializationRequest {
        &self.request
    }

    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }

    pub fn start(self) -> Result<ForgeQueryGraphReadMaterializationJob, ForgeQueryRuntimeError> {
        let target_progress = materialization_progress_for_request(&self.request);
        let initial_progress = initial_progress_for_target(&self.request, &target_progress);
        let initial_checkpoint = ForgeQueryGraphReadMaterializationCheckpoint::from_progress(
            self.request.digest(),
            0,
            0,
            0,
            0,
        );
        Ok(ForgeQueryGraphReadMaterializationJob::running(
            self.request,
            self.snapshot_identity,
            initial_progress,
            target_progress,
            initial_checkpoint,
        ))
    }
}

fn initial_progress_for_target(
    request: &ForgeQueryGraphReadMaterializationRequest,
    target_progress: &ForgeQueryGraphReadMaterializationProgress,
) -> ForgeQueryGraphReadMaterializationProgress {
    ForgeQueryGraphReadMaterializationProgress::from_request_parts(
        request.digest(),
        target_progress.admitted_limits().clone(),
        ForgeQueryGraphReadMaterializationCounters::new(0, 0, 0, 0, 0, 0),
    )
}

fn materialization_progress_for_request(
    request: &ForgeQueryGraphReadMaterializationRequest,
) -> ForgeQueryGraphReadMaterializationProgress {
    let admitted_limits =
        ForgeQueryGraphReadMaterializationAdmittedLimits::from_policy(request.policy());
    let touched_edges = request.estimated_touched_edges();
    let frontier_pages = touched_edges.div_ceil(64).max(1);
    let allocated_bytes = request.estimated_resident_bytes();
    let emitted_rows = request.estimated_emitted_rows();
    let checkpoint_count = frontier_pages
        .div_ceil(request.policy().checkpoint_interval().frontier_page_count())
        .max(1);
    let counters = ForgeQueryGraphReadMaterializationCounters::new(
        touched_edges,
        frontier_pages,
        allocated_bytes,
        emitted_rows,
        checkpoint_count,
        1,
    );
    ForgeQueryGraphReadMaterializationProgress::from_request_parts(
        request.digest(),
        admitted_limits,
        counters,
    )
}
