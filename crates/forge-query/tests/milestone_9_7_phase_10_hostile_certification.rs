mod support;

use support::public_bridge_runtime::{
    certify_public_bridge_hostile_schedule, public_graph_support_profile,
    PublicBridgeRuntimeBootstrapPath, PublicBridgeRuntimeHarness,
};

#[test]
fn public_bridge_hostile_certification_schedule_replays_identically() {
    let first = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Common,
        public_graph_support_profile(),
    );
    let replay = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Builder,
        public_graph_support_profile(),
    );
    let second = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Common,
        public_graph_support_profile(),
    );

    assert_eq!(first, replay);
    assert_eq!(first, second);
}
