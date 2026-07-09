use worth_query::facade::{admit_effect_intent, DeferredEffectEligibility};

fn main() {
    let deferred: DeferredEffectEligibility = unsafe { std::mem::zeroed() };
    let _ = admit_effect_intent(deferred);
}
