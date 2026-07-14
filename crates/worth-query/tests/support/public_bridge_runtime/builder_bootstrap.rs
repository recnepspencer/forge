use super::*;

impl PublicBridgeRuntimeHarness {
    #[allow(dead_code)]
    pub fn bridge_backed_runtime_builder(&self) -> PublicBridgeRuntimeBootstrapBuilder {
        PublicBridgeRuntimeBootstrapBuilder {
            state: self.state.clone(),
        }
    }
}

impl PublicBridgeRuntimeBootstrapBuilder {
    #[allow(dead_code)]
    pub fn support_profile(
        self,
        support_profile: WorthQueryRuntimeSupportProfile,
    ) -> PublicBridgeRuntimeBootstrapWithSupportProfile {
        PublicBridgeRuntimeBootstrapWithSupportProfile {
            state: self.state,
            support_profile,
        }
    }
}

impl PublicBridgeRuntimeBootstrapWithSupportProfile {
    #[allow(dead_code)]
    pub fn build(self) -> WorthQueryRuntime {
        record_public_bridge_runtime_bootstrap_invocation(
            PublicBridgeRuntimeBootstrapPath::Builder,
        );

        WorthQueryRuntime::builder()
            .runtime_bridge(bridge::public_bridge())
            .schema_adapter(PublicSchemaAdapter)
            .source_adapter(PublicSourceAdapter::new(self.state.clone()))
            .existing_truth_verification(PublicExistingTruthVerificationAdapter::new(
                self.state.clone(),
            ))
            .write_authority(PublicWriteAuthorityAdapter::new(self.state.clone()))
            .snapshot_identity(PublicSnapshotIdentityAdapter::new(self.state))
            .signal_sink(PublicSignalSinkAdapter)
            .subscription_activation(PublicSubscriptionActivationAdapter)
            .preview_basis(PublicPreviewBasisAdapter)
            .inspector_evidence(PublicInspectorEvidenceAdapter)
            .support_profile(self.support_profile)
            .build_backend_from_parts()
            .build()
            .expect("public bridge-backed runtime should build")
    }
}
