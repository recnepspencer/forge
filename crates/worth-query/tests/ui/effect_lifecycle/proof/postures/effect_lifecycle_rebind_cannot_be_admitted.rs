use worth_query::facade::foundation::{admit_effect_intent, RebindRequiredEffectEligibility};

fn main() {
    let rebind: RebindRequiredEffectEligibility = unsafe { std::mem::zeroed() };
    let _ = admit_effect_intent(rebind);
}
