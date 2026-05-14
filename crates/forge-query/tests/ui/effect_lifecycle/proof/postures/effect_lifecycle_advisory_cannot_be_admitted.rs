use forge_query::facade::{admit_effect_intent, AdvisoryEffectEligibility};

fn main() {
    let advisory: AdvisoryEffectEligibility = unsafe { std::mem::zeroed() };
    let _ = admit_effect_intent(advisory);
}
