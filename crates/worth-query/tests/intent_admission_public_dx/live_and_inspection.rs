//! Live-read, inspection, and probe compile-time DX evidence.

use super::*;

pub(super) fn live_read_common_path_compiles<T>(
    workspace: &mut WorthQueryWorkspace,
    live_view: &worth_query::facade::runtime::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::runtime::WorthQueryLiveReadResult, WorthQueryRuntimeError> {
    let result = workspace.read_live_intent(live_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

pub(super) fn live_read_advanced_path_compiles<T>(
    workspace: &mut WorthQueryWorkspace,
    live_view: &worth_query::facade::runtime::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::runtime::WorthQueryLiveReadResult, WorthQueryRuntimeError> {
    let review = workspace.read_live_intent(live_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

pub(super) fn derived_materialization_common_path_compiles<T>(
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::runtime::WorthQueryDerivedViewHandle<T>,
) -> Result<
    worth_query::facade::runtime::WorthQueryDerivedMaterializationResult,
    WorthQueryRuntimeError,
> {
    let result = workspace.materialize_intent(derived_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

pub(super) fn derived_materialization_advanced_path_compiles<T>(
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::runtime::WorthQueryDerivedViewHandle<T>,
) -> Result<
    worth_query::facade::runtime::WorthQueryDerivedMaterializationResult,
    WorthQueryRuntimeError,
> {
    let review = workspace.materialize_intent(derived_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

pub(super) fn derived_inspection_common_path_compiles<T>(
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::runtime::WorthQueryDerivedViewHandle<T>,
) -> Result<worth_query::facade::runtime::WorthQueryDerivedInspectionResult, WorthQueryRuntimeError>
{
    let result = workspace.inspect_derived_intent(derived_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

pub(super) fn derived_inspection_advanced_path_compiles<T>(
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::runtime::WorthQueryDerivedViewHandle<T>,
) -> Result<worth_query::facade::runtime::WorthQueryDerivedInspectionResult, WorthQueryRuntimeError>
{
    let review = workspace.inspect_derived_intent(derived_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

pub(super) fn generic_inspection_common_path_compiles<T>(
    workspace: &WorthQueryWorkspace,
    live_view: &worth_query::facade::runtime::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::runtime::WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError>
{
    let result = workspace.inspect_intent(live_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

pub(super) fn generic_inspection_advanced_path_compiles<T>(
    workspace: &WorthQueryWorkspace,
    live_view: &worth_query::facade::runtime::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::runtime::WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError>
{
    let review = workspace.inspect_intent(live_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

pub(super) fn existing_truth_probe_common_path_compiles(
    runtime: &WorthQueryRuntime,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError> {
    let result = runtime.probe_existing_intent(request).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

pub(super) fn workspace_existing_truth_probe_common_path_compiles(
    workspace: &WorthQueryWorkspace,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError> {
    let result = workspace.probe_existing_intent(request).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    Ok(result)
}

pub(super) fn existing_truth_probe_advanced_path_compiles(
    runtime: &WorthQueryRuntime,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError> {
    let review = runtime.probe_existing_intent(request).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

pub(super) fn existing_truth_probe_request_typecheck(
    binding: WorthQueryExistingTruthTargetBinding,
) -> Result<
    WorthQueryExistingTruthProbeRequest,
    worth_query::facade::foundation::WorthQueryWorkspaceError,
> {
    WorthQueryExistingTruthProbeRequest::new(binding, [touch("identity.id")])
}
