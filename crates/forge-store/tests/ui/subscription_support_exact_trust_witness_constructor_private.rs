#![allow(invalid_value)]

use forge_store::{
    ExactSupportTrustWitness, SupportExactTrustTranslation, SupportTrustFreshnessWitness,
    SupportTrustStrengthProvenance,
};

fn main() {
    let _ = ExactSupportTrustWitness {
        translation: unsafe { std::mem::zeroed::<SupportExactTrustTranslation>() },
        trust: unsafe { std::mem::zeroed::<SupportTrustStrengthProvenance>() },
        freshness: unsafe { std::mem::zeroed::<SupportTrustFreshnessWitness>() },
    };
}
