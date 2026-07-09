use worth_query::facade::policy::{
    BasisEligibilityCounters, BridgeLowerRuntimeEvidenceReference, LowerRuntimeBoundObservationBasis,
    ObservationBasisCapability,
};

fn main() {
    let _ = LowerRuntimeBoundObservationBasis {
        capability: unsafe { std::mem::zeroed::<ObservationBasisCapability>() },
        evidence: unsafe { std::mem::zeroed::<BridgeLowerRuntimeEvidenceReference>() },
        binding_digest: String::new(),
        counters: unsafe { std::mem::zeroed::<BasisEligibilityCounters>() },
    };
}
