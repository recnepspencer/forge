use super::*;

impl RuntimeBridge {
    /// Declares and seals the canonical Milestone 16 subscription reference
    /// workload manifest. Canonicalization happens before any workload lane
    /// executes.
    pub fn declare_subscription_reference_workload_manifest(
        &self,
        product_ids: BridgeSubscriptionReferenceWorkloadProductIdSet,
        component_ids: BridgeSubscriptionReferenceWorkloadComponentIdSet,
        lane_ids: BridgeSubscriptionReferenceWorkloadLaneIdSet,
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

    /// Certifies that bundle schema or digest divergence is the highest
    /// precedence comparison failure and shadows lower semantic drift.
    pub fn certify_subscription_certification_schema_parity(
        &self,
    ) -> BridgeSubscriptionCertificationSchemaParityReport {
        let _ = self;
        BridgeSubscriptionCertificationSchemaParityReport::certify()
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
    /// reconstruction as basis drift.
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

    /// Certifies shared fanout equivalence separately from divergent
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
}
