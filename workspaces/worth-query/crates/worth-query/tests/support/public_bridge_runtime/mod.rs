mod adapters;
mod aspect_contracts;
mod bridge;
mod builder_bootstrap;
mod common_bootstrap;
mod existing_truth_adapter;
mod external_row;
mod profiles;
mod relational_merge;
mod state;

use std::cell::RefCell;
use std::rc::Rc;

use worth_foundational::facade::AspectValue;
use worth_query::facade::runtime::{
    WorthQueryAspectTouch, WorthQueryExistingTruthTargetBinding, WorthQueryRuntime,
    WorthQueryRuntimeBuilder, WorthQueryRuntimeSupportProfile,
};

use self::adapters::{
    PublicInspectorEvidenceAdapter, PublicPreviewBasisAdapter, PublicSchemaAdapter,
    PublicSignalSinkAdapter, PublicSnapshotIdentityAdapter, PublicSourceAdapter,
    PublicSubscriptionActivationAdapter, PublicWriteAuthorityAdapter,
};
use self::aspect_contracts::public_bridge_aspect_contracts;
use self::existing_truth_adapter::PublicExistingTruthVerificationAdapter;
use self::state::{PublicBridgeRuntimeState, PublicExistingTruthKey};

type SharedRuntimeState = Rc<RefCell<PublicBridgeRuntimeState>>;

pub use self::profiles::public_graph_support_profile;
pub use self::relational_merge::public_relational_merge_runtime;
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
pub struct PublicBridgeRuntimeBootstrapBuilder {
    state: SharedRuntimeState,
}
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
