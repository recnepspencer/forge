
use worth_query::facade::runtime::{WorthQueryExistingTruthProbeRequest, WorthQueryLiveView, WorthQueryRuntime, WorthQueryNativeRow, WorthQueryRuntimeError, WorthQueryWorkspace};

fn mutation_common_path(
    runtime: &mut WorthQueryRuntime,
    command: worth_query::facade::runtime::WorthQueryWriteCommand,
) -> Result<(), WorthQueryRuntimeError> {
    let write_receipt = runtime.write_intent(command).execute()?;
    let _ = write_receipt.decision_trace_envelope();
    Ok(())
}

fn live_read_common_path<T>(
    workspace: &mut WorthQueryWorkspace,
    view: &WorthQueryLiveView<T>,
) -> Result<(), WorthQueryRuntimeError> {
    let live_rows = workspace.read(view);
    let live_result = workspace.read_live_intent(view).execute()?;
    let _ = live_rows.len();
    let _ = live_result.receipt().decision_trace_envelope();
    Ok(())
}

fn inspection_common_path(
    workspace: &WorthQueryWorkspace,
    target: &WorthQueryLiveView<WorthQueryNativeRow>,
) -> Result<(), WorthQueryRuntimeError> {
    let inspection = workspace.inspect(target)?;
    let inspection_result = workspace.inspect_intent(target).execute()?;
    let _ = inspection;
    let _ = inspection_result.receipt().decision_trace_envelope();
    Ok(())
}

fn routing_common_path(
    runtime: &WorthQueryRuntime,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<(), WorthQueryRuntimeError> {
    let probe = runtime.probe_existing(request.clone())?;
    let probe_result = runtime.probe_existing_intent(request).execute()?;
    let _ = probe.probe_digest();
    let _ = probe_result.receipt().decision_trace_envelope();
    Ok(())
}

fn main() {}
