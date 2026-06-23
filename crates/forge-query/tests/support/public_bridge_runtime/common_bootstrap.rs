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
        aspect_touch: ForgeQueryAspectTouch,
        value: AspectValue,
    ) -> PublicExistingTruthSeedRecord {
        let record = PublicExistingTruthSeedRecord::new(binding, aspect_touch);
        self.state
            .borrow_mut()
            .existing_truth_values
            .insert(record.key.clone(), value);
        record
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicExistingTruthSeedRecord {
    key: PublicExistingTruthKey,
}

impl PublicExistingTruthSeedRecord {
    fn new(
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspect_touch: ForgeQueryAspectTouch,
    ) -> Self {
        Self {
            key: PublicExistingTruthKey::new(binding, aspect_touch),
        }
    }

    #[allow(dead_code)]
    pub fn binding_digest(&self) -> &str {
        self.key.binding_digest()
    }

    #[allow(dead_code)]
    pub fn target_collection(&self) -> &str {
        self.key.target_collection()
    }

    #[allow(dead_code)]
    pub fn admitted_aspect_touch_reporting_projection(&self) -> String {
        self.key.admitted_aspect_touch_reporting_projection()
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
