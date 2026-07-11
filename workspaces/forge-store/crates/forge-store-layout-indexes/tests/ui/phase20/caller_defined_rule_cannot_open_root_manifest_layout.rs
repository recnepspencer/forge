use forge_store_physical_format::{
    PlatformPhysicalFacade, PlatformPhysicalOpenRequest,
};

struct FakeRule;

fn main() {
    let mut facade = PlatformPhysicalFacade::open_s1(
        forge_store_contracts::AcceptedHandoffReadiness::from_s0_artifacts(
            forge_store_contracts::ROADMAP_2_S1_SCOPE,
            forge_store_contracts::HandoffEvidenceDigestSet::new(
                digest("backend"),
                digest("deferred"),
                digest("harness"),
                digest("terms"),
                digest("audit"),
                digest("complexity"),
                digest("provenance"),
            )
            .unwrap(),
        )
        .unwrap(),
        PlatformPhysicalOpenRequest::s1_canonical(),
    )
    .unwrap();

    let _ = facade.root_manifest_layout(&FakeRule);
}

fn digest(name: &str) -> forge_store_contracts::StableDigest {
    forge_store_contracts::StableDigest::new(format!("sha256:{name}")).unwrap()
}
