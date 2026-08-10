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
}
