use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{PlatformPhysicalFacade, PlatformPhysicalOpenRequest};

struct FakePageRule;

fn main() {
    let mut facade = PlatformPhysicalFacade::open_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
    )
    .unwrap();
    let fake = FakePageRule;
    let _ = facade.page_layout(&fake);
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
