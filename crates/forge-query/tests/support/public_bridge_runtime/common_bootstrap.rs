use super::*;

impl PublicBridgeRuntimeHarness {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(PublicBridgeRuntimeState::default())),
        }
    }

    #[allow(dead_code)]
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
            .snapshot_identity(PublicSnapshotIdentityAdapter::new(self.state.clone()))
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
    pub fn seed_backend_authoritative_truth(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_path: &str,
        value: Value,
    ) -> PublicExistingTruthSeedRecord {
        let record = PublicExistingTruthSeedRecord::new(binding, aspect_path);
        let key = (
            record.binding_digest.clone(),
            record.target_collection.clone(),
            record.aspect_path.clone(),
        );
        self.state
            .borrow_mut()
            .existing_truth_values
            .insert(key, value);
        record
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicExistingTruthSeedRecord {
    binding_digest: String,
    target_collection: String,
    aspect_path: String,
}

impl PublicExistingTruthSeedRecord {
    fn new(binding: &ForgeQueryExistingTruthTargetBinding, aspect_path: &str) -> Self {
        Self {
            binding_digest: binding.binding_digest(),
            target_collection: binding.target_collection().unwrap_or("none").to_string(),
            aspect_path: aspect_path.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    #[allow(dead_code)]
    pub fn target_collection(&self) -> &str {
        &self.target_collection
    }

    #[allow(dead_code)]
    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }
}

#[allow(dead_code)]
pub fn reset_public_bridge_runtime_bootstrap_invocations() {
    BOOTSTRAP_INVOCATIONS.with(|counts| {
        *counts.borrow_mut() = [0; 2];
    });
}

#[allow(dead_code)]
pub fn public_bridge_runtime_bootstrap_invocation_count(
    path: PublicBridgeRuntimeBootstrapPath,
) -> usize {
    BOOTSTRAP_INVOCATIONS.with(|counts| counts.borrow()[bootstrap_index(path)])
}
