use super::*;

impl RuntimeBridge {
    /// Seals the final merged temporal/async certification closeout artifact
    /// from already-proven lower certification bands.
    pub fn seal_subscription_temporal_async_certification_closeout(
        &self,
        request: BridgeSubscriptionTemporalAsyncCertificationCloseoutRequest,
    ) -> Result<
        BridgeSubscriptionTemporalAsyncCertificationCloseoutArtifact,
        BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection,
    > {
        let _ = self;
        BridgeSubscriptionTemporalAsyncCertificationCloseoutArtifact::seal(request)
    }

    /// Inspects the support matrix carried by a sealed Phase 18 closeout
    /// artifact without reopening lower bundle or workload semantics.
    pub fn inspect_subscription_temporal_async_certification_support_matrix<'a>(
        &self,
        artifact: &'a BridgeSubscriptionTemporalAsyncCertificationCloseoutArtifact,
    ) -> &'a BridgeSubscriptionTemporalAsyncCertificationSupportMatrix {
        let _ = self;
        artifact.support_matrix()
    }
}
