#![allow(invalid_value)]

use worth_store::{
    OperationalSupportTrustReport, SupportTrustClassificationCostSurface, SupportTrustClass,
    SupportTrustOperationalWitness, SupportTrustProvenance, SupportTrustStrength,
    SupportTrustUseBoundary,
};

fn main() {
    let _ = OperationalSupportTrustReport {
        witness: unsafe { std::mem::zeroed::<SupportTrustOperationalWitness>() },
        trust_class: SupportTrustClass::ExactSupportTrusted,
        trust_strength: SupportTrustStrength::Exact,
        provenance: SupportTrustProvenance::NativePublished,
        use_boundary: SupportTrustUseBoundary::StoreLocalOperational,
        cost_surface: unsafe { std::mem::zeroed::<SupportTrustClassificationCostSurface>() },
    };
}
