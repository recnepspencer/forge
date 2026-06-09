use super::*;

impl PublicBridgeRuntimeHarness {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(PublicBridgeRuntimeState::default())),
        }
    }

    pub fn bridge_backed_runtime(&self) -> ForgeQueryRuntime {
        self.bridge_backed_runtime_with_support(public_graph_support_profile())
    }

    pub fn bridge_backed_runtime_with_support(
        &self,
        profile: ForgeQueryRuntimeSupportProfile,
    ) -> ForgeQueryRuntime {
        record_public_bridge_runtime_bootstrap_invocation(PublicBridgeRuntimeBootstrapPath::Common);

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

    #[allow(dead_code)]
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

pub fn reset_public_bridge_runtime_bootstrap_invocations() {
    BOOTSTRAP_INVOCATIONS.with(|counts| {
        *counts.borrow_mut() = [0; 2];
    });
}

pub fn public_bridge_runtime_bootstrap_invocation_count(
    path: PublicBridgeRuntimeBootstrapPath,
) -> usize {
    BOOTSTRAP_INVOCATIONS.with(|counts| counts.borrow()[bootstrap_index(path)])
}
