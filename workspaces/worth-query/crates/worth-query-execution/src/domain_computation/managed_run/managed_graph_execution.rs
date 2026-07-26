use std::sync::Arc;

use super::step_contract_admission::WorthQueryAdmittedManagedStepContract;
use crate::domain_computation::provider_session::graph_provider::bounded_step::{
    provider_anchor::WorthQueryGraphProviderAnchor, WorthQueryGraphProviderMemoryArena,
    WorthQueryGraphProviderStepArtifactContext, WorthQueryGraphProviderStepCompletion,
    WorthQueryOwnedGraphProviderExecution, WorthQueryProviderExecutionInvocation,
};
use crate::domain_computation::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphProviderCall,
    WorthQueryGraphProviderCallKind, WorthQueryGraphProviderExecution, WorthQueryGraphProviderStep,
    WorthQueryGraphProviderStepReport, WorthQueryGraphProviderStepRetainedEvidence,
    WorthQueryGraphReadMaterial, WorthQueryGraphReadStreamAccumulator,
    WorthQueryProviderWorkReport,
};

pub(super) struct WorthQueryManagedGraphExecution {
    pub(super) call: WorthQueryGraphProviderCall,
    pub(super) execution: WorthQueryOwnedGraphProviderExecution,
    pub(super) anchor: Arc<WorthQueryGraphProviderAnchor>,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) memory: WorthQueryGraphProviderMemoryArena,
    pub(super) completed_work_units: u64,
    pub(super) applied_effect_count: u64,
    pub(super) peak_scratch_bytes: u64,
    pub(super) retained_bytes: u64,
    pub(super) projection: Option<WorthQueryGraphReadStreamAccumulator>,
    pub(super) artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
    pub(super) produced_artifact_count: usize,
    pub(super) retained_artifact_count: usize,
    pub(super) disposed_artifact_count: usize,
    last_checkpoint_available: bool,
    last_retained: WorthQueryGraphProviderStepRetainedEvidence,
}

pub(super) struct WorthQueryManagedGraphExecutionStartParts {
    pub(super) call: WorthQueryGraphProviderCall,
    pub(super) execution: Box<dyn WorthQueryGraphProviderExecution>,
    pub(super) anchor: Arc<WorthQueryGraphProviderAnchor>,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
    pub(super) memory: WorthQueryGraphProviderMemoryArena,
}

pub(super) struct WorthQueryRestoredManagedGraphExecutionParts {
    pub(super) call: WorthQueryGraphProviderCall,
    pub(super) execution: Box<dyn WorthQueryGraphProviderExecution>,
    pub(super) anchor: Arc<WorthQueryGraphProviderAnchor>,
    pub(super) contract: WorthQueryAdmittedManagedStepContract,
    pub(super) memory: WorthQueryGraphProviderMemoryArena,
    pub(super) completed_work_units: u64,
    pub(super) applied_effect_count: u64,
    pub(super) peak_scratch_bytes: u64,
    pub(super) retained_bytes: u64,
    pub(super) projection: Option<WorthQueryGraphReadStreamAccumulator>,
    pub(super) artifact_context: Option<WorthQueryGraphProviderStepArtifactContext>,
    pub(super) produced_artifact_count: usize,
    pub(super) retained_artifact_count: usize,
    pub(super) disposed_artifact_count: usize,
}

pub(super) enum WorthQueryManagedProviderStep {
    Continue(WorthQueryManagedProviderStepEvidence),
    Complete(WorthQueryManagedProviderStepEvidence),
    Failed(WorthQueryManagedProviderStepEvidence),
}

pub(super) struct WorthQueryManagedProviderStepEvidence {
    admission: super::provider_step_admission::WorthQueryAdmittedProviderStep,
    report: WorthQueryGraphProviderStepReport,
}

impl WorthQueryManagedProviderStepEvidence {
    fn new(
        admission: super::provider_step_admission::WorthQueryAdmittedProviderStep,
        report: WorthQueryGraphProviderStepReport,
    ) -> Self {
        Self { admission, report }
    }

    pub(super) fn into_report(self) -> WorthQueryGraphProviderStepReport {
        let _ = self.admission.observation();
        self.report
    }
}

impl WorthQueryManagedGraphExecution {
    pub(super) fn new(parts: WorthQueryManagedGraphExecutionStartParts) -> Self {
        let projection = (parts.call.kind() == WorthQueryGraphProviderCallKind::Project)
            .then(|| WorthQueryGraphReadStreamAccumulator::new(&parts.call));
        Self {
            call: parts.call,
            execution: WorthQueryOwnedGraphProviderExecution::new(parts.execution),
            anchor: parts.anchor,
            contract: parts.contract,
            memory: parts.memory,
            completed_work_units: 0,
            applied_effect_count: 0,
            peak_scratch_bytes: 0,
            retained_bytes: 0,
            projection,
            artifact_context: parts.artifact_context,
            produced_artifact_count: 0,
            retained_artifact_count: 0,
            disposed_artifact_count: 0,
            last_checkpoint_available: false,
            last_retained: WorthQueryGraphProviderStepRetainedEvidence::default(),
        }
    }

    pub(super) fn restored(parts: WorthQueryRestoredManagedGraphExecutionParts) -> Self {
        Self {
            call: parts.call,
            execution: WorthQueryOwnedGraphProviderExecution::new(parts.execution),
            anchor: parts.anchor,
            contract: parts.contract,
            memory: parts.memory,
            completed_work_units: parts.completed_work_units,
            applied_effect_count: parts.applied_effect_count,
            peak_scratch_bytes: parts.peak_scratch_bytes,
            retained_bytes: parts.retained_bytes,
            projection: parts.projection,
            artifact_context: parts.artifact_context,
            produced_artifact_count: parts.produced_artifact_count,
            retained_artifact_count: parts.retained_artifact_count,
            disposed_artifact_count: parts.disposed_artifact_count,
            last_checkpoint_available: false,
            last_retained: WorthQueryGraphProviderStepRetainedEvidence::default(),
        }
    }

    pub(super) fn admit_provider_step(
        &self,
        observation: super::WorthQueryManagedSafePointObservation,
    ) -> super::provider_step_admission::WorthQueryProviderStepAdmissionOutcome {
        super::provider_step_admission::admit_provider_step(
            self.call.kind(),
            self.contract.installed(),
            observation,
        )
    }

    pub(super) fn advance_provider(
        &mut self,
        admission: super::provider_step_admission::WorthQueryAdmittedProviderStep,
    ) -> WorthQueryManagedProviderStep {
        let mut step = WorthQueryGraphProviderStep::new(
            self.call.kind(),
            self.contract.installed(),
            self.artifact_context.clone(),
            self.memory.clone(),
        );
        let disposition = match self.execution.advance(&mut step) {
            WorthQueryProviderExecutionInvocation::Returned(Ok(disposition)) => disposition,
            WorthQueryProviderExecutionInvocation::Returned(Err(failure)) => {
                return WorthQueryManagedProviderStep::Failed(
                    WorthQueryManagedProviderStepEvidence::new(
                        admission,
                        step.finish_rejected(failure),
                    ),
                )
            }
            WorthQueryProviderExecutionInvocation::Panicked => {
                return WorthQueryManagedProviderStep::Failed(
                    WorthQueryManagedProviderStepEvidence::new(admission, step.finish_panicked()),
                )
            }
        };
        let report = match step.finish(disposition) {
            Ok(report) => report,
            Err((_denial, report)) => {
                return WorthQueryManagedProviderStep::Failed(
                    WorthQueryManagedProviderStepEvidence::new(admission, report),
                )
            }
        };
        let completion = report.completion();
        let evidence = WorthQueryManagedProviderStepEvidence::new(admission, report);
        match completion {
            WorthQueryGraphProviderStepCompletion::Continue => {
                WorthQueryManagedProviderStep::Continue(evidence)
            }
            WorthQueryGraphProviderStepCompletion::Complete => {
                WorthQueryManagedProviderStep::Complete(evidence)
            }
            WorthQueryGraphProviderStepCompletion::Failed => {
                WorthQueryManagedProviderStep::Failed(evidence)
            }
        }
    }

    pub(super) fn admit_report(&mut self, report: &mut WorthQueryGraphProviderStepReport) {
        self.completed_work_units = self
            .completed_work_units
            .saturating_add(report.completed_work_units());
        self.applied_effect_count = self
            .applied_effect_count
            .saturating_add(report.applied_effect_count());
        self.peak_scratch_bytes = self.peak_scratch_bytes.max(report.peak_scratch_bytes());
        self.retained_bytes = report.retained_bytes();
        self.last_checkpoint_available = report.checkpoint_available();
        self.last_retained = report.retained_evidence();
        let artifacts = report.artifact_evidence();
        self.produced_artifact_count = self
            .produced_artifact_count
            .saturating_add(artifacts.produced_artifact_count());
        self.retained_artifact_count = artifacts.retained_artifact_count();
        self.disposed_artifact_count = self
            .disposed_artifact_count
            .saturating_add(artifacts.disposed_artifact_count());
    }

    pub(super) fn yield_safe_point(
        &self,
        observation: super::WorthQueryManagedSafePointObservation,
    ) -> super::yield_eligibility::WorthQueryManagedYieldSafePoint {
        super::yield_eligibility::WorthQueryManagedYieldSafePoint::new(
            observation,
            self.last_checkpoint_available,
            self.last_retained,
        )
    }

    pub(super) const fn applied_effect_count(&self) -> u64 {
        self.applied_effect_count
    }

    pub(super) fn provider_call_identity(&self) -> &str {
        self.call.call_identity()
    }

    pub(super) fn admit_projection_chunk(&mut self, material: &WorthQueryGraphReadMaterial) {
        self.projection
            .as_mut()
            .expect("only projection executions admit projection chunks")
            .admit_chunk(material);
    }

    pub(super) fn release_projection_chunk(&mut self, retained_bytes: usize) -> bool {
        let retained_bytes = u64::try_from(retained_bytes).unwrap_or(u64::MAX);
        let Some(remaining) = self.retained_bytes.checked_sub(retained_bytes) else {
            return false;
        };
        if !self.last_retained.release_projection_bytes(retained_bytes) {
            return false;
        }
        self.retained_bytes = remaining;
        true
    }

    pub(super) fn seal_completion(
        &mut self,
        report: &WorthQueryGraphProviderStepReport,
    ) -> Result<WorthQueryBoundGraphExecutionReceipt, ()> {
        let provider_receipt = Arc::<str>::from(report.provider_receipt().ok_or(())?);
        let work = WorthQueryProviderWorkReport::new(
            self.completed_work_units,
            self.applied_effect_count,
            usize::try_from(self.peak_scratch_bytes).unwrap_or(usize::MAX),
            usize::try_from(self.retained_bytes).unwrap_or(usize::MAX),
        )
        .with_artifact_disposition(
            self.produced_artifact_count,
            self.retained_artifact_count,
            self.disposed_artifact_count,
        )
        .ok_or(())?;
        let receipt = if self.call.kind() == WorthQueryGraphProviderCallKind::Project {
            let stream = self.projection.take().ok_or(())?.finish(&self.call);
            self.call
                .streamed(provider_receipt, stream, work)
                .map_err(|_| ())?
        } else {
            self.call.completed(provider_receipt, work)
        };
        self.call.admit_receipt(receipt).map_err(|_| ())
    }
}
