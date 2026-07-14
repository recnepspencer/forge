use super::*;
use crate::expression::model::SignalValue;

#[test]
fn compute_callbacks_reuse_slots_and_reject_stale_generations() {
    let first = register_native_compute(Box::new(|| Ok(SignalValue::Number(3.0))));
    assert_eq!(
        invoke_compute(first).unwrap().value,
        SignalValue::Number(3.0)
    );
    assert!(dispose_compute(first));

    let second = register_native_compute(Box::new(|| Ok(SignalValue::Number(4.0))));
    assert_eq!(first.slot, second.slot);
    assert!(second.generation > first.generation);

    let stale = invoke_compute(first).unwrap_err();
    assert_eq!(stale.class, ComputeCallbackFailureClass::GenerationMismatch);

    let fresh = invoke_compute(second).unwrap();
    assert_eq!(fresh.value, SignalValue::Number(4.0));

    let stats = compute_callback_stats();
    assert!(stats.compute_callback_reuse_count >= 1);
    assert!(stats.compute_callback_generation_mismatch_denial_count >= 1);
    assert!(dispose_compute(second));
}

#[test]
fn disposed_compute_callbacks_cannot_be_invoked() {
    let token = register_native_compute(Box::new(|| Ok(SignalValue::String("ok".to_owned()))));
    assert!(dispose_compute(token));

    let failure = invoke_compute(token).unwrap_err();
    assert_eq!(failure.class, ComputeCallbackFailureClass::Disposed);
}
