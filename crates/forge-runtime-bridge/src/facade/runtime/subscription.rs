use super::*;

impl RuntimeBridge {
    /// Declares one bridge subscription artifact using the runtime's frozen
    /// Phase 1 declaration-family registry.
    pub fn declare_subscription(
        &self,
        requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
        normalized_slice_intents: Vec<NormalizedSubscriptionSliceIntent>,
        delivery_intent_class: BridgeSubscriptionDeliveryIntentClass,
    ) -> Result<BridgeSubscriptionDeclaration, BridgeSubscriptionDeclarationRejection> {
        let family_registration = self
            .subscription_family_registry
            .family_for_kind(requested_family_kind);

        BridgeSubscriptionDeclaration::new(
            requested_family_kind,
            delivery_intent_class,
            normalized_slice_intents,
            family_registration,
        )
    }

    /// Returns the frozen subscription-family registry identity bound into this
    /// runtime.
    pub fn subscription_family_registry_identity(
        &self,
    ) -> &BridgeSubscriptionFamilyRegistryIdentity {
        self.subscription_family_registry.registry_identity()
    }

    /// Returns the frozen subscription-family registry counters bound into this
    /// runtime.
    pub fn subscription_family_registry_counters(&self) -> &BridgeSubscriptionCounters {
        self.subscription_family_registry.counters()
    }

    /// Admits one declared bridge subscription against an explicit truth basis
    /// and lowers it to a canonical signal strategy descriptor.
    pub fn admit_subscription(
        &self,
        declaration: &BridgeSubscriptionDeclaration,
        basis_request: BridgeSubscriptionBasisRequest,
    ) -> Result<AdmittedBridgeSubscription, BridgeSubscriptionAdmissionRejection> {
        AdmittedBridgeSubscription::admit(self, declaration, basis_request)
    }

    /// Prepares an admitted subscription for activation without performing any
    /// signal registration or delivery fanout.
    pub fn prepare_subscription_activation(
        &self,
        admitted: &AdmittedBridgeSubscription,
    ) -> BridgeSubscriptionActivationReady {
        BridgeSubscriptionActivationReady::prepare(
            self.subscription_family_registry_identity(),
            admitted,
        )
    }

    /// Produces a deactivated retained artifact from one activation-ready
    /// subscription handle.
    pub fn deactivate_subscription(
        &self,
        activation_ready: BridgeSubscriptionActivationReady,
    ) -> BridgeSubscriptionDeactivated {
        let _ = self;
        activation_ready.deactivate()
    }

    /// Builds a retained explanation for one activation-ready subscription.
    pub fn inspect_activation_ready_subscription(
        &self,
        activation_ready: &BridgeSubscriptionActivationReady,
    ) -> BridgeSubscriptionExplanation {
        let _ = self;
        BridgeSubscriptionExplanation::from_activation_ready(activation_ready)
    }

    /// Builds a retained explanation for one deactivated subscription.
    pub fn inspect_deactivated_subscription(
        &self,
        deactivated: &BridgeSubscriptionDeactivated,
    ) -> BridgeSubscriptionExplanation {
        let _ = self;
        BridgeSubscriptionExplanation::from_deactivated(deactivated)
    }

    /// Builds a retained explanation for one subscription admission rejection.
    pub fn inspect_subscription_admission_rejection(
        &self,
        rejection: &BridgeSubscriptionAdmissionRejection,
    ) -> BridgeSubscriptionExplanation {
        let _ = self;
        BridgeSubscriptionExplanation::from_rejection(rejection)
    }

    /// Reconstructs retained subscription meaning from a canonical bundle.
    pub fn replay_subscription(
        &self,
        retained_bundle: &BridgeRetainedSubscriptionBundle,
    ) -> Result<BridgeSubscriptionReplaySummary, BridgeSubscriptionReplayMismatch> {
        BridgeSubscriptionReplaySummary::replay(
            self.subscription_family_registry_identity(),
            retained_bundle,
        )
    }

    /// Admits one Phase 1 delivery cost profile before active delivery.
    pub fn admit_subscription_delivery_cost_profile(
        &self,
        density_posture: BridgeSubscriptionDeliveryDensityPosture,
        max_member_count: usize,
        max_coalesced_member_width: usize,
        max_fanout_width: usize,
    ) -> Result<BridgeSubscriptionDeliveryCostProfile, BridgeSubscriptionDeliveryCostProfileRejection>
    {
        let _ = self;
        BridgeSubscriptionDeliveryCostProfile::admit(
            density_posture,
            max_member_count,
            max_coalesced_member_width,
            max_fanout_width,
        )
    }

    /// Admits a single-consumer Phase 1 contract. Callback and channel
    /// identity are intentionally absent from this API.
    pub fn admit_subscription_consumer_contract(
        &self,
        family: BridgeSubscriptionConsumerContractFamily,
        pacing_capability: BridgeSubscriptionConsumerPacingCapability,
        backpressure_posture: BridgeSubscriptionConsumerBackpressurePosture,
        coalescing_admitted: bool,
        diagnostics_retention: BridgeSubscriptionConsumerDiagnosticsRetention,
    ) -> Result<BridgeSubscriptionConsumerContract, BridgeSubscriptionConsumerContractRejection>
    {
        let _ = self;
        BridgeSubscriptionConsumerContract::admit(
            family,
            pacing_capability,
            backpressure_posture,
            coalescing_admitted,
            diagnostics_retention,
        )
    }

    /// Consumes an activation-ready subscription into an active delivery proof.
    pub fn activate_subscription_delivery(
        &self,
        activation_ready: BridgeSubscriptionActivationReady,
        cost_profile: BridgeSubscriptionDeliveryCostProfile,
        consumer_contract: BridgeSubscriptionConsumerContract,
    ) -> BridgeActiveSubscription {
        let _ = self;
        BridgeActiveSubscription::activate(activation_ready, cost_profile, consumer_contract)
    }

    /// Opens a phase-typed delivery window for one active subscription.
    pub fn open_subscription_delivery_window(
        &self,
        active_subscription: &BridgeActiveSubscription,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
        delivery_window_sequence: u64,
    ) -> BridgeSubscriptionDeliveryWindowOpen {
        let _ = self;
        BridgeSubscriptionDeliveryWindowOpen::open(
            active_subscription,
            delivery_family_kind,
            delivery_window_sequence,
        )
    }

    /// Seals a delivery window into canonical delivery member records.
    pub fn seal_subscription_delivery_window(
        &self,
        delivery_window: BridgeSubscriptionDeliveryWindowOpen,
        members: Vec<BridgeSubscriptionDeliveryMemberInput>,
    ) -> Result<BridgeSubscriptionDeliveryWindowSealed, BridgeSubscriptionDeliveryWindowRejection>
    {
        let _ = self;
        delivery_window.seal(members)
    }

    /// Returns the hot-path diagnostics reference for a sealed delivery window.
    pub fn inspect_subscription_delivery_reference<'a>(
        &self,
        sealed: &'a BridgeSubscriptionDeliveryWindowSealed,
    ) -> &'a BridgeSubscriptionDeliveryDiagnosticsReference {
        let _ = self;
        sealed.diagnostics_reference()
    }

    /// Admits additional equivalent consumers to share one active subscription.
    pub fn plan_shared_subscription_fanout(
        &self,
        active_subscription: &BridgeActiveSubscription,
        additional_consumers: Vec<BridgeSubscriptionConsumerContract>,
    ) -> Result<BridgeSubscriptionFanoutPlan, BridgeSubscriptionFanoutPlanRejection> {
        let _ = self;
        BridgeSubscriptionFanoutPlan::plan(active_subscription, additional_consumers)
    }

    /// Lowers an admitted fanout plan into a compact indexed layout before
    /// delivery projection.
    pub fn build_subscription_fanout_layout(
        &self,
        fanout_plan: BridgeSubscriptionFanoutPlan,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
    ) -> BridgeSubscriptionFanoutLayout {
        let _ = self;
        BridgeSubscriptionFanoutLayout::build(fanout_plan, delivery_family_kind)
    }

    /// Projects one sealed canonical delivery window through a prebuilt fanout
    /// layout without cloning canonical member records per consumer.
    pub fn project_subscription_delivery_to_fanout(
        &self,
        fanout_layout: &BridgeSubscriptionFanoutLayout,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> Result<
        BridgeSubscriptionFanoutDeliveryProjectionSet,
        BridgeSubscriptionFanoutProjectionRejection,
    > {
        let _ = self;
        BridgeSubscriptionFanoutDeliveryProjection::project(fanout_layout, sealed_window)
    }

    /// Retains descriptor evidence for a sealed delivery window without
    /// reconstructing payloads or rich diagnostics.
    pub fn retain_subscription_delivery_window_seed(
        &self,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> BridgeSubscriptionRetainedDeliveryWindowSeed {
        let _ = self;
        BridgeSubscriptionRetainedDeliveryWindowSeed::retain(sealed_window)
    }

    /// Retains descriptor evidence for a fanout projection set without replaying
    /// delivery.
    pub fn retain_subscription_fanout_projection_seed(
        &self,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
    ) -> BridgeSubscriptionRetainedDeliveryReplaySeed {
        let _ = self;
        BridgeSubscriptionRetainedDeliveryReplaySeed::retain(projection_set)
    }

    /// Validates a fanout projection set against its layout as a compact proof.
    pub fn validate_subscription_fanout_projection(
        &self,
        fanout_layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
    ) -> Result<
        BridgeSubscriptionFanoutProjectionValidation,
        BridgeSubscriptionFanoutProjectionValidationRejection,
    > {
        let _ = self;
        BridgeSubscriptionFanoutProjectionValidation::validate(fanout_layout, projection_set)
    }

    /// Inspects replay readiness without executing retained replay.
    pub fn inspect_subscription_delivery_replay_readiness(
        &self,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> BridgeSubscriptionDeliveryWindowReplayReadiness {
        let _ = self;
        BridgeSubscriptionDeliveryWindowReplayReadiness::inspect(sealed_window)
    }

    /// Plans retained delivery replay from checkpoint-approved resume evidence.
    /// This only admits ordered retained window descriptors; it does not
    /// dispatch callbacks, replay payloads, or materialize rich diagnostics.
    pub fn plan_subscription_delivery_replay(
        &self,
        active_subscription: &BridgeActiveSubscription,
        resume_admission: BridgeSubscriptionResumeAdmission,
        retained_window_seeds: Vec<BridgeSubscriptionRetainedDeliveryWindowSeed>,
    ) -> Result<BridgeSubscriptionDeliveryReplayPlan, BridgeSubscriptionDeliveryReplayPlanRejection>
    {
        let _ = self;
        BridgeSubscriptionDeliveryReplayPlan::plan(
            active_subscription,
            resume_admission,
            retained_window_seeds,
        )
    }

    /// Admits a preview-scoped subscription basis from an active preview
    /// session and its matching retained preview execution record. Ordinary
    /// branch-head or snapshot basis artifacts cannot satisfy this API.
    pub fn admit_subscription_preview_basis(
        &self,
        active_preview_session: &BridgePreviewSession<PreviewActive>,
        preview_execution_record: &BridgePreviewExecutionRecord,
    ) -> Result<BridgeSubscriptionPreviewBasisBinding, BridgeSubscriptionPreviewBasisRejection>
    {
        let _ = self;
        BridgeSubscriptionPreviewBasisBinding::admit(
            active_preview_session,
            preview_execution_record,
        )
    }

    /// Activates a preview-scoped subscription delivery proof. The returned
    /// type is intentionally distinct from authoritative active subscriptions.
    pub fn activate_preview_subscription_delivery(
        &self,
        activation_ready: BridgeSubscriptionActivationReady,
        preview_basis: BridgeSubscriptionPreviewBasisBinding,
        cost_profile: BridgeSubscriptionDeliveryCostProfile,
        consumer_contract: BridgeSubscriptionConsumerContract,
    ) -> BridgePreviewActiveSubscription {
        let _ = self;
        BridgePreviewActiveSubscription::activate(
            activation_ready,
            preview_basis,
            cost_profile,
            consumer_contract,
        )
    }

    /// Builds a preview-residue scope index from explicit scope-local artifact
    /// descriptors. This does not scan authoritative or global registries.
    pub fn build_subscription_preview_residue_scope_index(
        &self,
        preview_active: &BridgePreviewActiveSubscription,
        artifact_inputs: Vec<BridgeSubscriptionPreviewResidueArtifactInput>,
    ) -> BridgeSubscriptionPreviewResidueScopeIndex {
        let _ = self;
        BridgeSubscriptionPreviewResidueScopeIndex::build(preview_active, artifact_inputs)
    }

    /// Discards a preview subscription only after scope-indexed residue proof
    /// establishes zero authoritative and bridge-visible residue.
    pub fn discard_preview_subscription(
        &self,
        preview_active: BridgePreviewActiveSubscription,
        residue_scope_index: BridgeSubscriptionPreviewResidueScopeIndex,
    ) -> Result<
        BridgeSubscriptionPreviewDiscardResidueProof,
        BridgeSubscriptionPreviewDiscardResidueRejection,
    > {
        let _ = self;
        BridgeSubscriptionPreviewDiscardResidueProof::prove(preview_active, residue_scope_index)
    }

    /// Admits an acknowledged canonical frontier from a sealed delivery window.
    pub fn admit_subscription_acknowledgement_frontier(
        &self,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        acknowledged_sequence: usize,
        acknowledged_member_identity: &BridgeSubscriptionDeliveryMemberIdentity,
        acknowledged_member_digest: &str,
    ) -> Result<
        BridgeSubscriptionAcknowledgementFrontier,
        BridgeSubscriptionAcknowledgementFrontierRejection,
    > {
        let _ = self;
        BridgeSubscriptionAcknowledgementFrontier::admit(
            sealed_window,
            acknowledged_sequence,
            acknowledged_member_identity,
            acknowledged_member_digest,
        )
    }

    /// Converts an admitted acknowledgement frontier into checkpoint-ready
    /// evidence without publishing a replay token yet.
    pub fn prepare_subscription_checkpoint(
        &self,
        frontier: BridgeSubscriptionAcknowledgementFrontier,
    ) -> BridgeSubscriptionCheckpointReady {
        let _ = self;
        BridgeSubscriptionCheckpointReady::prepare(frontier)
    }

    /// Publishes a checkpoint from checkpoint-ready evidence and the matching
    /// active subscription proof.
    pub fn publish_subscription_checkpoint(
        &self,
        checkpoint_ready: BridgeSubscriptionCheckpointReady,
        active_subscription: &BridgeActiveSubscription,
        duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    ) -> Result<BridgeSubscriptionCheckpoint, BridgeSubscriptionCheckpointRejection> {
        let _ = self;
        BridgeSubscriptionCheckpoint::publish(
            checkpoint_ready,
            active_subscription,
            duplicate_replay_policy_kind,
            None,
        )
    }

    /// Publishes a checkpoint from checkpoint-ready evidence while binding the
    /// compact fanout layout that delivered the acknowledged window.
    pub fn publish_subscription_fanout_checkpoint(
        &self,
        checkpoint_ready: BridgeSubscriptionCheckpointReady,
        active_subscription: &BridgeActiveSubscription,
        fanout_layout: &BridgeSubscriptionFanoutLayout,
        duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    ) -> Result<BridgeSubscriptionCheckpoint, BridgeSubscriptionCheckpointRejection> {
        let _ = self;
        BridgeSubscriptionCheckpoint::publish(
            checkpoint_ready,
            active_subscription,
            duplicate_replay_policy_kind,
            Some(fanout_layout),
        )
    }

    /// Admits a checkpoint for resume against the matching active subscription
    /// without reopening delivery or reconstructing retained payloads.
    pub fn admit_subscription_resume(
        &self,
        active_subscription: &BridgeActiveSubscription,
        checkpoint: &BridgeSubscriptionCheckpoint,
    ) -> Result<BridgeSubscriptionResumeAdmission, BridgeSubscriptionResumeAdmissionRejection> {
        let _ = self;
        BridgeSubscriptionResumeAdmission::admit(active_subscription, checkpoint)
    }

    /// Lowers resume admission into a descriptor plan for the next canonical
    /// member sequence. Execution remains a later batch.
    pub fn plan_subscription_resume(
        &self,
        resume_admission: BridgeSubscriptionResumeAdmission,
    ) -> BridgeSubscriptionResumePlan {
        let _ = self;
        BridgeSubscriptionResumePlan::plan(resume_admission)
    }

    /// Builds a locality-scoped continuation index from explicit truth-owned
    /// candidate descriptors. This does not scan active subscription registries.
    pub fn build_subscription_continuation_index(
        &self,
        active_subscription: &BridgeActiveSubscription,
        candidate_inputs: Vec<BridgeSubscriptionContinuationCandidateInput>,
    ) -> Result<BridgeSubscriptionContinuationIndex, BridgeSubscriptionContinuationIndexRejection>
    {
        let _ = self;
        BridgeSubscriptionContinuationIndex::build(active_subscription, candidate_inputs)
    }

    /// Plans one typed subscription continuation from a prebuilt locality index.
    pub fn plan_subscription_continuation(
        &self,
        active_subscription: &BridgeActiveSubscription,
        continuation_index: &BridgeSubscriptionContinuationIndex,
        candidate_slot: usize,
    ) -> Result<BridgeSubscriptionContinuationDecision, BridgeSubscriptionContinuationRejection>
    {
        let _ = self;
        BridgeSubscriptionContinuationDecision::plan(
            active_subscription,
            continuation_index,
            candidate_slot,
        )
    }
}
