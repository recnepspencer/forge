mod adapters;
mod bridge;
mod payload;
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

#[allow(unused_imports)]
pub use self::profiles::{
    public_graph_support_profile, public_multi_verified_relation_profile,
    public_verified_relation_profile,
};

pub struct PublicBridgeRuntimeHarness {
    state: SharedRuntimeState,
}

impl PublicBridgeRuntimeHarness {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(PublicBridgeRuntimeState::default())),
        }
    }

    pub fn runtime(&self, profile: ForgeQueryRuntimeSupportProfile) -> ForgeQueryRuntime {
        ForgeQueryRuntime::builder()
            .runtime_bridge(bridge::public_bridge())
            .schema_adapter(PublicSchemaAdapter)
            .source_adapter(PublicSourceAdapter::new(self.state.clone()))
            .existing_truth_verification(PublicExistingTruthVerificationAdapter::new(
                self.state.clone(),
            ))
            .write_authority(PublicWriteAuthorityAdapter::new(self.state.clone()))
            .signal_sink(PublicSignalSinkAdapter)
            .subscription_activation(PublicSubscriptionActivationAdapter)
            .preview_basis(PublicPreviewBasisAdapter)
            .inspector_evidence(PublicInspectorEvidenceAdapter)
            .support_profile(profile)
            .build_backend_from_parts()
            .build()
            .expect("public bridge-backed runtime should build")
    }

    pub fn seed_existing_truth_value(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_path: &str,
        value: Value,
    ) {
        let mut state = self.state.borrow_mut();
        state.existing_truth_values.insert(
            (
                binding.binding_digest(),
                binding.target_collection().unwrap_or("none").to_string(),
                aspect_path.to_string(),
            ),
            value,
        );
    }
}
