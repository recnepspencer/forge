//! Compatibility execution for callers not yet migrated into managed runs.
//!
//! This adapter deliberately returns only a legacy provider receipt. It owns
//! no managed-run, Signal, checkpoint, artifact, cleanup, or convergence
//! authority and cannot satisfy any managed typestate transition.

use std::sync::Arc;

use super::provider_anchor::{
    WorthQueryGraphProviderAnchor, WorthQueryGraphProviderStartInvocation,
};
use super::{
    WorthQueryGraphProviderExecutionStart, WorthQueryGraphProviderMemoryArena,
    WorthQueryGraphProviderStep, WorthQueryGraphProviderStepCompletion,
    WorthQueryOwnedGraphProviderExecution,
};
use crate::domain_computation::{
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCallKind, WorthQueryGraphProviderFailure,
    WorthQueryGraphProviderReceipt, WorthQueryGraphReadMaterial, WorthQueryProviderWorkReport,
};

#[doc(hidden)]
pub fn execute_legacy_one_shot(
    anchor: &WorthQueryGraphProviderAnchor,
    call: &WorthQueryGraphProviderCall,
) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
    let contract = call
        .resource_envelope()
        .bounded_step_contract()
        .map_err(WorthQueryGraphProviderFailure::new)?;
    let memory = WorthQueryGraphProviderMemoryArena::new(contract.retained_bytes_ceiling());
    let mut start = WorthQueryGraphProviderExecutionStart::new(memory.clone());
    let invocation = anchor.begin(call, &mut start);
    let unreturned_execution_release = start.release_unreturned_execution();
    let start_contract = start.finish();
    let mut execution = match invocation {
        WorthQueryGraphProviderStartInvocation::Returned(Ok(execution)) => execution,
        WorthQueryGraphProviderStartInvocation::Returned(Err(failure)) => {
            return Err(legacy_start_failure(failure, unreturned_execution_release))
        }
        WorthQueryGraphProviderStartInvocation::Panicked => {
            return Err(WorthQueryGraphProviderFailure::new(
                "legacy provider execution construction panicked",
            ))
        }
    };
    if let Err(denial) = start_contract {
        let _ = WorthQueryOwnedGraphProviderExecution::new(execution).release();
        return Err(WorthQueryGraphProviderFailure::new(denial.detail()));
    }
    let mut completed_work_units = 0u64;
    let mut applied_effect_count = 0u64;
    let mut peak_scratch_bytes = 0u64;
    let mut projection_rows = Vec::new();
    loop {
        let mut step =
            WorthQueryGraphProviderStep::new(call.kind(), &contract, None, memory.clone());
        let disposition = match execution.advance(&mut step) {
            Ok(disposition) => disposition,
            Err(failure) => {
                let _ = step.finish_rejected(failure.clone());
                return Err(failure);
            }
        };
        let mut report = step
            .finish(disposition)
            .map_err(|(denial, _)| WorthQueryGraphProviderFailure::new(denial.detail()))?;
        completed_work_units = completed_work_units.saturating_add(report.completed_work_units());
        applied_effect_count = applied_effect_count.saturating_add(report.applied_effect_count());
        peak_scratch_bytes = peak_scratch_bytes.max(report.peak_scratch_bytes());
        if let Some(material) = report.take_projection() {
            projection_rows.extend(material.into_rows());
        }
        match report.completion() {
            WorthQueryGraphProviderStepCompletion::Continue => {}
            WorthQueryGraphProviderStepCompletion::Complete => {
                let provider_receipt = Arc::<str>::from(
                    report
                        .provider_receipt()
                        .expect("complete disposition carries provider receipt"),
                );
                let work = WorthQueryProviderWorkReport::new(
                    completed_work_units,
                    applied_effect_count,
                    usize::try_from(peak_scratch_bytes).unwrap_or(usize::MAX),
                    usize::try_from(report.retained_bytes()).unwrap_or(usize::MAX),
                );
                return if call.kind() == WorthQueryGraphProviderCallKind::Project {
                    call.projected(
                        provider_receipt,
                        WorthQueryGraphReadMaterial::new(projection_rows),
                        work,
                    )
                } else {
                    Ok(call.completed(provider_receipt, work))
                };
            }
            WorthQueryGraphProviderStepCompletion::Failed => {
                return Err(WorthQueryGraphProviderFailure::new(
                    "provider step failed without a provider failure",
                ));
            }
        }
    }
}

fn legacy_start_failure(
    failure: WorthQueryGraphProviderFailure,
    release: Option<super::WorthQueryProviderExecutionReleaseEvidence>,
) -> WorthQueryGraphProviderFailure {
    if release
        .as_ref()
        .is_some_and(super::WorthQueryProviderExecutionReleaseEvidence::recovery_required)
    {
        WorthQueryGraphProviderFailure::new(format!(
            "{}; admitted provider execution required physical-release recovery",
            failure.detail()
        ))
    } else {
        failure
    }
}
