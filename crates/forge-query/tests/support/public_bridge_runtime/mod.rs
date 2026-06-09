mod adapters;
mod bridge;
mod builder_bootstrap;
mod common_bootstrap;
mod external_row;
mod profiles;
mod state;

use std::cell::RefCell;
use std::rc::Rc;

use forge_query::facade::{
    ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntime, ForgeQueryRuntimeSupportProfile,
};
use serde_json::Value;

use self::adapters::{
    PublicExistingTruthVerificationAdapter, PublicInspectorEvidenceAdapter,
    PublicPreviewBasisAdapter, PublicSchemaAdapter, PublicSignalSinkAdapter, PublicSourceAdapter,
    PublicSubscriptionActivationAdapter, PublicWriteAuthorityAdapter,
};
use self::state::PublicBridgeRuntimeState;

type SharedRuntimeState = Rc<RefCell<PublicBridgeRuntimeState>>;

pub use self::profiles::public_graph_support_profile;
pub use common_bootstrap::{
    public_bridge_runtime_bootstrap_invocation_count,
    reset_public_bridge_runtime_bootstrap_invocations,
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
    support_profile: ForgeQueryRuntimeSupportProfile,
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
