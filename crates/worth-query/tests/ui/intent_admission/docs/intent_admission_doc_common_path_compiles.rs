
use worth_query::facade::runtime::{WorthQueryEffectHandle, WorthQueryEffectIntentReceipt, WorthQueryIntentDeclaration, WorthQueryIntentReceipt, WorthQueryRuntime, WorthQueryRuntimeError};

fn authoritative_common_path(
    runtime: &mut WorthQueryRuntime,
    declaration: WorthQueryIntentDeclaration,
) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
    let receipt = runtime.intent(declaration).execute()?;
    let consumer = receipt.consumer_inspection();
    let _ = receipt.covered_entrypoint_label();
    let _ = receipt.decision_trace_envelope().trace_digest();
    let _ = receipt.execution_provenance_chain_digest();
    let _ = consumer.decision_trace_digest();
    let _ = consumer.execution_provenance_chain_digest();
    Ok(receipt)
}

fn effect_common_path<T>(
    runtime: &mut WorthQueryRuntime,
    effect: &WorthQueryEffectHandle<T>,
) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
    let receipt = runtime
        .next_effect_write_intent(effect, "1.0", "effect.intent.input.v1")
        .execute()?;
    let consumer = receipt.consumer_inspection();
    let _ = receipt.effect_name();
    let _ = receipt.intent_receipt().decision_trace_envelope().trace_digest();
    let _ = receipt.intent_receipt().execution_provenance_chain_digest();
    let _ = consumer.terminal_stage_label();
    Ok(receipt)
}

fn main() {}
