#![allow(dead_code)]

use forge_query::facade::{
    ForgeQueryEffectHandle, ForgeQueryEffectIntentReceipt, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentDeclaration, ForgeQueryIntentReceipt, ForgeQueryRuntime,
    ForgeQueryRuntimeError,
};

fn authoritative_common_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    declaration: ForgeQueryIntentDeclaration,
) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
    runtime.intent(declaration).execute()
}

fn authoritative_advanced_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    declaration: ForgeQueryIntentDeclaration,
) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
    let review = runtime.intent(declaration).review()?;

    let _ = review.request().family();
    let _ = review.request().entrypoint();
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision_trace_envelope();
    if let Some(plan) = review.admitted_plan() {
        let _ = plan.family();
        let _ = plan.entrypoint();
        let _ = plan.execution_seam();
        let _ = plan.decision_digest();
    }

    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(_) => {}
        ForgeQueryIntentAdmissionDecision::Advisory(_) => {}
        ForgeQueryIntentAdmissionDecision::Violation(_) => {}
    }

    let admitted = review.admit()?;
    let _ = admitted.request().request_digest();
    let _ = admitted.eligibility().eligibility_digest();
    let _ = admitted.decision();
    let _ = admitted.handoff().handoff_digest();
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn effect_common_path_compiles<T>(
    runtime: &mut ForgeQueryRuntime,
    effect: &ForgeQueryEffectHandle<T>,
) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
    let receipt = runtime
        .next_effect_write_intent(effect, "1.0", "effect.intent.input.v1")
        .execute()?;
    let _ = receipt.execution_binding_digest();
    let _ = receipt.execution_provenance_chain_digest();
    let _ = receipt.decision_trace_envelope().trace_digest();
    Ok(receipt)
}

fn effect_advanced_path_compiles<T>(
    runtime: &mut ForgeQueryRuntime,
    effect: &ForgeQueryEffectHandle<T>,
) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
    let review = runtime
        .next_effect_write_intent(effect, "1.0", "effect.intent.input.v1")
        .review()?;

    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review.pending_delivery().commit_identity();
    let admitted = review.admit()?;
    let _ = admitted.handoff().handoff_digest();
    let _ = admitted.execution_binding().binding_digest();
    let _ = admitted.pending_delivery().effect_name();
    admitted.execute()
}
