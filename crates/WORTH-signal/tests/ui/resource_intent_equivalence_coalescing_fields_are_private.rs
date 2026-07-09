use worth_signal::facade::{
    AdmittedResourceRequest, ResourceIntentEquivalenceCoalescing, ResourceLifecycleTransition,
    ResourcePolicyDigest, ResourceRequestHandle, ResourceRequestIntentDigest,
    ResourceSupersessionOrdinal,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn WORTHd_request() -> AdmittedResourceRequest {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn WORTHd_intent_digest() -> ResourceRequestIntentDigest {
    loop {}
}

fn WORTHd_policy_digest() -> ResourcePolicyDigest {
    loop {}
}

fn main() {
    let _ = ResourceIntentEquivalenceCoalescing {
        supersession_ordinal: ResourceSupersessionOrdinal::new(0),
        winner: WORTHd_handle(),
        coalesced_request: WORTHd_request(),
        intent_digest: WORTHd_intent_digest(),
        policy_decision_digest: WORTHd_policy_digest(),
        lifecycle_transition: WORTHd_transition(),
    };
}
