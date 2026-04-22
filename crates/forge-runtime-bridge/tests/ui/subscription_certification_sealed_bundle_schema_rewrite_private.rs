use forge_runtime_bridge::facade::BridgeSubscriptionCertificationBundleSealed;

fn sealed_bundle() -> BridgeSubscriptionCertificationBundleSealed {
    unimplemented!()
}

fn main() {
    let _forged = sealed_bundle()
        .with_schema_digest_identity_for_certification("fake-schema", "fake-digest");
}
