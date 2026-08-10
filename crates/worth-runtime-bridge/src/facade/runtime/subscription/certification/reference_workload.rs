use super::*;

impl RuntimeBridge {
    /// Runs the Milestone 16 reference workload certification lanes from a
    /// sealed manifest. The returned report is derived entirely from emitted
    /// certification bundles, comparison reports, and offline audit evidence.
    pub fn plan_subscription_reference_workload(
        &self,
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        lane_requests: Vec<BridgeSubscriptionReferenceWorkloadLaneRequest>,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadDeclaration,
        BridgeSubscriptionReferenceWorkloadRejection,
    > {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadDeclaration::plan(manifest, lane_requests)
    }

    /// Admits the sealed lane-artifact set for a planned reference workload
    /// before workload sufficiency is proven.
    pub fn admit_subscription_reference_workload_lane_artifacts(
        &self,
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        declaration: &BridgeSubscriptionReferenceWorkloadDeclaration,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
        BridgeSubscriptionReferenceWorkloadRejection,
    > {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadLaneArtifactSet::admit(manifest, declaration)
    }

    /// Proves that a sealed lane-artifact set covers the required Phase 17
    /// workload facets and hostile lanes before the workload can close.
    pub fn prove_subscription_reference_workload_coverage(
        &self,
        lane_artifact_set: BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
    ) -> Result<
        BridgeSubscriptionReferenceWorkloadCoverageProof,
        BridgeSubscriptionReferenceWorkloadRejection,
    > {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadCoverageProof::prove(lane_artifact_set)
    }

    /// Seals Phase 17 reference-workload sufficiency from explicit manifest,
    /// declaration, lane-artifact, and coverage-proof phases.
    pub fn seal_subscription_reference_workload_sufficiency(
        &self,
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        declaration: &BridgeSubscriptionReferenceWorkloadDeclaration,
        lane_artifact_set: BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
        coverage_proof: &BridgeSubscriptionReferenceWorkloadCoverageProof,
        fixture_evidence_digest: &str,
    ) -> BridgeSubscriptionReferenceWorkloadSufficiency {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadReport::seal(
            manifest,
            declaration,
            lane_artifact_set,
            coverage_proof,
            fixture_evidence_digest,
        )
    }

    /// Runs the broad reference workload report surface without requiring full
    /// Phase 17 sufficiency closure. This remains useful for partial lane
    /// audits and hostile lane-local certification.
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
