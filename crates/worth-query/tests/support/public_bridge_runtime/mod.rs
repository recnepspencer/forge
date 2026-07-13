mod adapters;
mod bridge;
mod builder_bootstrap;
mod common_bootstrap;
mod existing_truth_adapter;
mod external_row;
#[allow(dead_code)]
mod hostile_certification;
mod profiles;
mod reader_lane_honesty;
mod state;

use std::cell::RefCell;
use std::rc::Rc;

use worth_foundational::facade::AspectValue;
use worth_query::facade::runtime::{
    WorthQueryAspectTouch, WorthQueryExistingTruthTargetBinding, WorthQueryRuntime,
    WorthQueryRuntimeSupportProfile,
};

use self::adapters::{
    PublicInspectorEvidenceAdapter, PublicPreviewBasisAdapter, PublicSchemaAdapter,
    PublicSignalSinkAdapter, PublicSnapshotIdentityAdapter, PublicSourceAdapter,
    PublicSubscriptionActivationAdapter, PublicWriteAuthorityAdapter,
};
use self::existing_truth_adapter::PublicExistingTruthVerificationAdapter;
use self::state::{PublicBridgeRuntimeState, PublicExistingTruthKey};

type SharedRuntimeState = Rc<RefCell<PublicBridgeRuntimeState>>;

pub use self::profiles::public_graph_support_profile;
#[allow(unused_imports)]
pub use common_bootstrap::{
    public_bridge_runtime_bootstrap_invocation_count,
    reset_public_bridge_runtime_bootstrap_invocations,
};
#[allow(unused_imports)]
pub use hostile_certification::{
    certify_public_bridge_hostile_schedule, PublicBridgeHostileCertificationArtifact,
};
#[allow(unused_imports)]
pub use reader_lane_honesty::{
    direct_materialization_read_count, public_bridge_certification_inventory,
    public_bridge_certification_inventory_paths, public_bridge_direct_materialization_sabotage,
    sabotaged_public_bridge_certification_inventory,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicBridgeRuntimeBootstrapPath {
    Common,
    Builder,
}

pub struct PublicBridgeRuntimeHarness {
    state: SharedRuntimeState,
}

#[allow(dead_code)]
pub struct PublicBridgeRuntimeBootstrapBuilder {
    state: SharedRuntimeState,
}

#[allow(dead_code)]
pub struct PublicBridgeRuntimeBootstrapWithSupportProfile {
    state: SharedRuntimeState,
    support_profile: WorthQueryRuntimeSupportProfile,
}

thread_local! {
    static BOOTSTRAP_INVOCATIONS: RefCell<[usize; 2]> = const { RefCell::new([0; 2]) };
}

fn record_public_bridge_runtime_bootstrap_invocation(path: PublicBridgeRuntimeBootstrapPath) {
    BOOTSTRAP_INVOCATIONS.with(|counts| {
        counts.borrow_mut()[bootstrap_index(path)] += 1;
    });
}

fn bootstrap_index(path: PublicBridgeRuntimeBootstrapPath) -> usize {
    match path {
        PublicBridgeRuntimeBootstrapPath::Common => 0,
        PublicBridgeRuntimeBootstrapPath::Builder => 1,
    }
}
