use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_physical_format::{
    PersistedPhysicalLayout, PlatformPhysicalFacade, PlatformPhysicalOpenRequest,
};

fn main() {
    let layout = PersistedPhysicalLayout::builder().build();
    let _reopened = PlatformPhysicalFacade::reopen_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
        layout,
    );
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(ROADMAP_2_S1_SCOPE, digest_set()).unwrap()
}

fn digest_set() -> HandoffEvidenceDigestSet {
    HandoffEvidenceDigestSet::new(
        digest("backend"),
        digest("deferred"),
        digest("harness"),
        digest("terms"),
        digest("audit"),
        digest("complexity"),
        digest("provenance"),
    )
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).unwrap()
}
