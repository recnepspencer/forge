use super::*;

impl RuntimeBridge {
    /// Retains one temporal resume-basis packet from an admitted temporal basis
    /// and explicit wake posture.
    pub fn capture_temporal_subscription_resume_basis(
        &self,
        admitted_temporal_basis: &AdmittedBridgeTemporalBasis,
        wake_posture: BridgeRetainedTemporalWakePosture,
        previous_value_evidence: Option<&RetainedHistoricalPreviousValueEvidence>,
        retention_complete: bool,
    ) -> BridgeRetainedTemporalResumeBasis {
        let _ = self;
        BridgeRetainedTemporalResumeBasis::capture(
            admitted_temporal_basis,
            wake_posture,
            previous_value_evidence,
            retention_complete,
        )
    }

    /// Retains one inflight async resume-basis packet from an admitted async
    /// request identity.
    pub fn capture_inflight_async_subscription_resume_basis(
        &self,
        request_identity: &AdmittedBridgeAsyncRequestIdentity,
        retention_complete: bool,
    ) -> BridgeRetainedInflightAsyncResumeBasis {
        let _ = self;
        BridgeRetainedInflightAsyncResumeBasis::capture(request_identity, retention_complete)
    }

    /// Retains one shared-delivery resume basis packet from a sealed bundle,
    /// consumer projection, and admitted acknowledgement frontier.
    pub fn capture_shared_delivery_subscription_resume_basis(
        &self,
        bundle: &BridgeSharedConsumerDeliveryBundleSealed,
        projection: &BridgeSharedConsumerDeliveryProjection,
        acknowledgement: &BridgeSharedDeliveryAcknowledgementFrontier,
        retention_complete: bool,
    ) -> Result<BridgeRetainedDeliveryResumeBasis, BridgeSubscriptionResumeBasisRejection> {
        let _ = self;
        BridgeRetainedDeliveryResumeBasis::capture(
            bundle,
            projection,
            acknowledgement,
            retention_complete,
        )
    }

    /// Captures one retained subscription resume-basis packet from explicit
    /// checkpoint, temporal, inflight-async, and delivery basis inputs.
    pub fn capture_subscription_resume_basis(
        &self,
        active_subscription: &BridgeActiveSubscription,
        checkpoint: &BridgeSubscriptionCheckpoint,
        temporal_resume_basis: Option<BridgeRetainedTemporalResumeBasis>,
        inflight_async_resume_basis: Option<BridgeRetainedInflightAsyncResumeBasis>,
        delivery_resume_basis: Option<BridgeRetainedDeliveryResumeBasis>,
        retention_complete: bool,
    ) -> BridgeRetainedSubscriptionResumeBasis {
        let _ = self;
        BridgeRetainedSubscriptionResumeBasis::capture(
            active_subscription,
            checkpoint,
            temporal_resume_basis,
            inflight_async_resume_basis,
            delivery_resume_basis,
            retention_complete,
        )
    }

    /// Admits one retained subscription resume basis packet after explicit
    /// basis, branch, and retained-artifact validation.
    pub fn admit_subscription_resume_basis(
        &self,
        retained_basis: &BridgeRetainedSubscriptionResumeBasis,
    ) -> Result<AdmittedBridgeSubscriptionResumeBasis, BridgeSubscriptionResumeBasisRejection> {
        let _ = self;
        AdmittedBridgeSubscriptionResumeBasis::admit(retained_basis)
    }

    /// Projects admitted retained basis into replay-readiness without yet
    /// executing resumed delivery.
    pub fn prepare_subscription_replay_readiness(
        &self,
        admitted_resume_basis: &AdmittedBridgeSubscriptionResumeBasis,
    ) -> BridgeSubscriptionReplayReadiness {
        let _ = self;
        BridgeSubscriptionReplayReadiness::prepare(admitted_resume_basis)
    }

    /// Lowers admitted retained resume basis plus replay-readiness into the
    /// existing delivery-replay resume admission surface without reopening
    /// basis reconstruction from raw checkpoints.
    pub fn admit_subscription_resume_from_basis(
        &self,
        active_subscription: &BridgeActiveSubscription,
        admitted_resume_basis: &AdmittedBridgeSubscriptionResumeBasis,
        replay_readiness: &BridgeSubscriptionReplayReadiness,
    ) -> Result<BridgeSubscriptionResumeAdmission, BridgeSubscriptionResumeBasisRejection> {
        let _ = self;
        admitted_resume_basis.lower_resume_admission(active_subscription, replay_readiness)
    }
}
