
use forge_query::facade::{
    ForgeQueryExistingTruthProbeRequest, ForgeQueryLiveView, ForgeQueryRuntime,
    ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use serde_json::Value;

fn mutation_common_path(
    runtime: &mut ForgeQueryRuntime,
    command: forge_query::facade::ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryRuntimeError> {
    let write_receipt = runtime.write_intent(command).execute()?;
    let _ = write_receipt.decision_trace_envelope();
    Ok(())
}

fn live_read_common_path<T>(
    workspace: &mut ForgeQueryWorkspace,
    view: &ForgeQueryLiveView<T>,
) -> Result<(), ForgeQueryRuntimeError> {
    let live_rows = workspace.read(view);
    let live_result = workspace.read_live_intent(view).execute()?;
    let _ = live_rows.len();
    let _ = live_result.receipt().decision_trace_envelope();
    Ok(())
}

fn inspection_common_path(
    workspace: &ForgeQueryWorkspace,
    target: &ForgeQueryLiveView<Value>,
) -> Result<(), ForgeQueryRuntimeError> {
    let inspection = workspace.inspect(target)?;
    let inspection_result = workspace.inspect_intent(target).execute()?;
    let _ = inspection;
    let _ = inspection_result.receipt().decision_trace_envelope();
    Ok(())
}

fn routing_common_path(
    runtime: &ForgeQueryRuntime,
    request: ForgeQueryExistingTruthProbeRequest,
) -> Result<(), ForgeQueryRuntimeError> {
    let probe = runtime.probe_existing(request.clone())?;
    let probe_result = runtime.probe_existing_intent(request).execute()?;
    let _ = probe.probe_digest();
    let _ = probe_result.receipt().decision_trace_envelope();
    Ok(())
}

fn main() {}
