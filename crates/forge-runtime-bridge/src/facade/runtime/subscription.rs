use super::*;

impl RuntimeBridge {
    /// Declares and seals the canonical Milestone 16 subscription reference
    /// workload manifest. Canonicalization happens before any workload lane
    /// executes.
    pub fn declare_subscription_reference_workload_manifest(
        &self,
        product_ids: Vec<impl Into<std::sync::Arc<str>>>,
        component_ids: Vec<impl Into<std::sync::Arc<str>>>,
        lane_ids: Vec<impl Into<std::sync::Arc<str>>>,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadManifestSealed,
        BridgeSubscriptionReferenceWorkloadManifestRejection,
    > {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadManifestDraft::new(product_ids, component_ids, lane_ids)
            .seal()
    }

    /// Builds an indexed source-artifact view over retained subscription
    /// protocol artifacts. Later bundle assembly consumes this index rather
    /// than scanning host object graphs or retained history.
    pub fn build_subscription_certification_source_index(
        &self,
        inputs: Vec<BridgeSubscriptionSourceArtifactInput>,
    ) -> BridgeSubscriptionSourceArtifactIndex {
        let _ = self;
        BridgeSubscriptionSourceArtifactIndex::build(inputs)
    }

    /// Admits the certification assembly cost profile before bundle assembly.
    pub fn admit_subscription_certification_cost_profile(
        &self,
        density_posture: BridgeSubscriptionCertificationDensityPosture,
        max_source_artifact_entries: usize,
        max_bundle_field_count: usize,
        scratch_capacity: usize,
        rich_diagnostics_admitted: bool,
    ) -> Result<
        BridgeSubscriptionCertificationCostProfile,
        BridgeSubscriptionCertificationCostProfileRejection,
    > {
        let _ = self;
        BridgeSubscriptionCertificationCostProfile::admit(
            density_posture,
            max_source_artifact_entries,
            max_bundle_field_count,
            scratch_capacity,
            rich_diagnostics_admitted,
        )
    }

    /// Certifies the Milestone 16 cost posture matrix without assembling a
    /// semantic bundle. Dense and over-budget posture decisions are proven at
    /// admission time, before bundle assembly can allocate or reconstruct.
    pub fn certify_subscription_certification_cost_posture(
        &self,
    ) -> BridgeSubscriptionCertificationCostPostureReport {
        let _ = self;
        BridgeSubscriptionCertificationCostPostureReport::certify()
    }

    /// Certifies that bundle schema or digest incompatibility is the highest
    /// precedence comparison failure and short-circuits lower semantic drift.
    pub fn certify_subscription_certification_schema_compatibility(
        &self,
    ) -> BridgeSubscriptionCertificationSchemaCompatibilityReport {
        let _ = self;
        BridgeSubscriptionCertificationSchemaCompatibilityReport::certify()
    }

    /// Certifies multi-failure precedence using injected basis, checkpoint,
    /// replay, and diagnostics drift in one comparison.
    pub fn certify_subscription_certification_multi_failure_precedence(
        &self,
    ) -> BridgeSubscriptionCertificationMultiFailurePrecedenceReport {
        let _ = self;
        BridgeSubscriptionCertificationMultiFailurePrecedenceReport::certify()
    }

    /// Certifies that hostile retained-artifact insertion order cannot change
    /// source index, semantic digest, field ordering, or sealed bundle meaning.
    pub fn certify_subscription_certification_ordering_hostility(
        &self,
    ) -> BridgeSubscriptionCertificationOrderingHostilityReport {
        let _ = self;
        BridgeSubscriptionCertificationOrderingHostilityReport::certify()
    }

    /// Certifies that stale checkpoint drift localizes at the checkpoint/resume
    /// boundary without being misreported as retained replay mismatch.
    pub fn certify_subscription_certification_stale_checkpoint(
        &self,
    ) -> BridgeSubscriptionCertificationStaleCheckpointReport {
        let _ = self;
        BridgeSubscriptionCertificationStaleCheckpointReport::certify()
    }

    /// Certifies missing required bundle fields as typed bundle insufficiency.
    pub fn certify_subscription_certification_bundle_insufficiency(
        &self,
    ) -> BridgeSubscriptionCertificationBundleInsufficiencyReport {
        let _ = self;
        BridgeSubscriptionCertificationBundleInsufficiencyReport::certify()
    }

    /// Certifies retained historical basis evidence and rejects latest-truth
    /// fallback as basis drift.
    pub fn certify_subscription_certification_historical_basis(
        &self,
    ) -> BridgeSubscriptionCertificationHistoricalBasisReport {
        let _ = self;
        BridgeSubscriptionCertificationHistoricalBasisReport::certify()
    }

    /// Certifies family-aware strategy-lowering provenance without signal
    /// rediscovery.
    pub fn certify_subscription_certification_strategy_lowering(
        &self,
    ) -> BridgeSubscriptionCertificationStrategyLoweringReport {
        let _ = self;
        BridgeSubscriptionCertificationStrategyLoweringReport::certify()
    }

    /// Certifies shared fanout equivalence separately from incompatible
    /// sharing rejection.
    pub fn certify_subscription_certification_fanout(
        &self,
    ) -> BridgeSubscriptionCertificationFanoutReport {
        let _ = self;
        BridgeSubscriptionCertificationFanoutReport::certify()
    }

    /// Certifies authority-denied continuation localization before delivery
    /// drift can masquerade as subscription truth.
    pub fn certify_subscription_certification_denied_continuation(
        &self,
    ) -> BridgeSubscriptionCertificationDeniedContinuationReport {
        let _ = self;
        BridgeSubscriptionCertificationDeniedContinuationReport::certify()
    }

    /// Prepares the explicit scratch lifetime admitted by the certification
    /// cost profile. Bundle assembly consumes this proof instead of allocating
    /// per record group.
    pub fn prepare_subscription_certification_scratch(
        &self,
        cost_profile: &BridgeSubscriptionCertificationCostProfile,
    ) -> BridgeSubscriptionCertificationScratch {
        let _ = self;
        BridgeSubscriptionCertificationScratch::prepare(cost_profile)
    }

    /// Plans certification bundle assembly from a sealed manifest and indexed
    /// source artifact view.
    pub fn plan_subscription_certification_bundle(
        &self,
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
    ) -> BridgeSubscriptionCertificationAssemblyPlan {
        let _ = self;
        BridgeSubscriptionCertificationAssemblyPlan::plan(manifest, source_artifact_index)
    }

    /// Assembles a draft certification bundle from explicit plan, cost profile,
    /// and scratch proofs.
    pub fn assemble_subscription_certification_bundle(
        &self,
        assembly_plan: BridgeSubscriptionCertificationAssemblyPlan,
        cost_profile: BridgeSubscriptionCertificationCostProfile,
        scratch: BridgeSubscriptionCertificationScratch,
    ) -> Result<
        BridgeSubscriptionCertificationBundleDraft,
        BridgeSubscriptionCertificationAssemblyRejection,
    > {
        let _ = self;
        BridgeSubscriptionCertificationBundleDraft::assemble(assembly_plan, cost_profile, scratch)
    }

    /// Seals a draft bundle so later comparison phases cannot mutate the
    /// canonical field set after digest computation.
    pub fn seal_subscription_certification_bundle(
        &self,
        draft: BridgeSubscriptionCertificationBundleDraft,
    ) -> BridgeSubscriptionCertificationBundleSealed {
        let _ = self;
        draft.seal()
    }

    /// Admits a first-class comparison plan before sealed bundles are compared.
    pub fn plan_subscription_certification_comparison(
        &self,
        relationship: BridgeSubscriptionCertificationComparisonRelationship,
        expected_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
        divergence_axis: Option<BridgeSubscriptionCertificationDivergenceAxis>,
    ) -> Result<
        BridgeSubscriptionCertificationComparisonPlan,
        BridgeSubscriptionCertificationComparisonPlanRejection,
    > {
        let _ = self;
        BridgeSubscriptionCertificationComparisonPlan::admit(
            relationship,
            expected_failure_boundary,
            divergence_axis,
        )
    }

    /// Compares sealed certification bundles through an admitted relationship
    /// plan. Draft bundles cannot reach this phase.
    pub fn compare_subscription_certification_bundles(
        &self,
        plan: BridgeSubscriptionCertificationComparisonPlan,
        left: &BridgeSubscriptionCertificationBundleSealed,
        right: &BridgeSubscriptionCertificationBundleSealed,
    ) -> BridgeSubscriptionCertificationComparisonReport {
        let _ = self;
        BridgeSubscriptionCertificationComparisonReport::compare(plan, left, right)
    }

    /// Builds an offline audit index from sealed certification bundles. This is
    /// the only bundle collection shape accepted by the offline audit phase.
    pub fn build_subscription_offline_audit_bundle_index(
        &self,
        bundles: Vec<&BridgeSubscriptionCertificationBundleSealed>,
    ) -> BridgeSubscriptionOfflineAuditBundleIndex {
        let _ = self;
        BridgeSubscriptionOfflineAuditBundleIndex::build(bundles)
    }

    /// Admits an offline audit plan from sealed bundle indexes and comparison
    /// reports. Host logs and live runtime handles are explicitly rejected.
    pub fn plan_subscription_offline_audit(
        &self,
        bundle_index: &BridgeSubscriptionOfflineAuditBundleIndex,
        comparison_reports: Vec<&BridgeSubscriptionCertificationComparisonReport>,
        host_log_dependency_requested: bool,
        live_state_dependency_requested: bool,
    ) -> Result<BridgeSubscriptionOfflineAuditPlan, BridgeSubscriptionOfflineAuditPlanRejection>
    {
        let _ = self;
        BridgeSubscriptionOfflineAuditPlan::admit(
            bundle_index,
            comparison_reports,
            host_log_dependency_requested,
            live_state_dependency_requested,
        )
    }

    /// Diagnoses subscription certification offline from an admitted audit
    /// plan. This does not replay host behavior or query live runtime state.
    pub fn audit_subscription_certification_bundle_offline(
        &self,
        audit_plan: BridgeSubscriptionOfflineAuditPlan,
    ) -> BridgeSubscriptionOfflineAuditReport {
        let _ = self;
        BridgeSubscriptionOfflineAuditReport::audit(audit_plan)
    }

    /// Produces the public certification inspection view for the diagnostics
    /// entrypoint from an offline audit report.
    pub fn inspect_subscription_certification(
        &self,
        report: &BridgeSubscriptionOfflineAuditReport,
    ) -> BridgeSubscriptionCertificationInspection {
        let _ = self;
        BridgeSubscriptionCertificationInspection::from_offline_audit(report)
    }

    /// Produces the public certification inspection view for a complete
    /// Milestone 16 reference workload report without reopening sealed bundles
    /// or replaying host behavior.
    pub fn inspect_subscription_reference_workload_certification(
        &self,
        report: &BridgeSubscriptionReferenceWorkloadReport,
    ) -> BridgeSubscriptionReferenceWorkloadInspection {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadInspection::from_reference_workload(report)
    }

    /// Runs the Milestone 16 reference workload certification lanes from a
    /// sealed manifest. The returned report is derived entirely from emitted
    /// certification bundles, comparison reports, and offline audit evidence.
    pub fn run_subscription_reference_workload(
        &self,
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        lane_requests: Vec<BridgeSubscriptionReferenceWorkloadLaneRequest>,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadReport,
        BridgeSubscriptionReferenceWorkloadRejection,
    > {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadReport::run(manifest, lane_requests)
    }

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

    /// Records the preview-scoped routing, delivery, diagnostics, and
    /// continuation work descriptors that justify a later residue proof. This
    /// is intentionally separate from authoritative delivery windows.
    pub fn record_preview_subscription_work(
        &self,
        preview_active: &BridgePreviewActiveSubscription,
        inputs: Vec<BridgeSubscriptionPreviewWorkInput>,
    ) -> Result<BridgeSubscriptionPreviewWorkTrace, BridgeSubscriptionPreviewWorkTraceRejection>
    {
        let _ = self;
        BridgeSubscriptionPreviewWorkTrace::record(preview_active, inputs)
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

    /// Promotes a preview-scoped subscription only through an explicit
    /// speculation promotion record and a matching authoritative
    /// activation-ready boundary. This consumes the preview-active handle
    /// instead of mutating it into an authoritative subscription.
    pub fn promote_preview_subscription(
        &self,
        preview_active: BridgePreviewActiveSubscription,
        preview_work_trace: &BridgeSubscriptionPreviewWorkTrace,
        promotion_record: &BridgePreviewPromotionRecord,
        promoted_activation_ready: &BridgeSubscriptionActivationReady,
    ) -> Result<BridgeSubscriptionPreviewPromotionRecord, BridgeSubscriptionPreviewPromotionRejection>
    {
        let _ = self;
        BridgeSubscriptionPreviewPromotionRecord::promote(
            preview_active,
            preview_work_trace,
            promotion_record,
            promoted_activation_ready,
        )
    }

    /// Builds a compact explanation for a subscription preview-promotion
    /// boundary without reconstructing rich delivery diagnostics.
    pub fn inspect_subscription_preview_promotion_record(
        &self,
        record: &BridgeSubscriptionPreviewPromotionRecord,
    ) -> BridgeSubscriptionPreviewPromotionExplanation {
        let _ = self;
        BridgeSubscriptionPreviewPromotionExplanation::from_promotion_record(record)
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
