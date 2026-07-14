use worth_query::facade::foundation::{admit_effect_intent, DeniedEffectEligibility};

fn main() {
    let denied: DeniedEffectEligibility = unsafe { std::mem::zeroed() };
    let _ = admit_effect_intent(denied);
}
