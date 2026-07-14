
use worth_query::facade::runtime::{WorthQueryEffectHandle, WorthQueryEffectIntentReceipt, WorthQueryIntentDeclaration, WorthQueryIntentReceipt, WorthQueryRuntime, WorthQueryRuntimeError};

fn consumer_lane_on_receipt(
    runtime: &mut WorthQueryRuntime,
    declaration: WorthQueryIntentDeclaration,
) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
    let receipt = runtime.intent(declaration).execute()?;
    let consumer = receipt.consumer_inspection();
    let _ = consumer.outcome_class();
    let _ = consumer.decision_trace_digest();
    let _ = consumer.terminal_stage_label();
    let _ = consumer.execution_provenance_chain_digest();
    Ok(receipt)
}

fn consumer_lane_on_review<T>(
    runtime: &mut WorthQueryRuntime,
    effect: &WorthQueryEffectHandle<T>,
) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
    let review = runtime
        .next_effect_write_intent(effect, "1.0", "effect.intent.input.v1")
        .review()?;
    let consumer = review.consumer_inspection();
    let _ = consumer.outcome_class();
    let _ = consumer.terminal_stage_label();
    let admitted = review.admit()?;
    let receipt = admitted.execute()?;
    let _ = receipt.consumer_inspection().decision_trace_digest();
    Ok(receipt)
}

fn main() {}
