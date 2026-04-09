use super::*;

impl RuntimeBridge {
    pub fn publish_structural_remap_artifact(
        &self,
        reduced_match_set: &ReducedStructuralMatchSet,
    ) -> Result<PublishedStructuralRemapArtifact, BridgeDeliveryError> {
        PublishedStructuralRemapArtifact::from_reduced_match_set(reduced_match_set.clone())
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Reduced structural match set `{}` does not describe an advisory remap publication outcome.",
                        reduced_match_set.digest()
                    ),
                )
            })
    }

    pub fn publish_branch_comparison_artifact(
        &self,
        reduced_match_set: &ReducedStructuralMatchSet,
    ) -> Result<PublishedBranchComparisonArtifact, BridgeDeliveryError> {
        PublishedBranchComparisonArtifact::from_reduced_match_set(reduced_match_set.clone())
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Reduced structural match set `{}` does not describe a branch comparison publication outcome.",
                        reduced_match_set.digest()
                    ),
                )
            })
    }

    pub fn canonicalize_structural_remap_record(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
        reduced_match_set: &ReducedStructuralMatchSet,
        artifact: &PublishedStructuralRemapArtifact,
    ) -> BridgeCanonicalStructuralRemapRecord {
        let counters = BridgeStructuralCounters::from_structural_outcome(
            contract,
            planned_packet_set,
            reduced_match_set,
        );
        let record = BridgeCanonicalStructuralRemapRecord::new(BridgeStructuralRemapRecord::new(
            contract.clone(),
            planned_packet_set.clone(),
            reduced_match_set.clone(),
            artifact.clone(),
            counters,
        ));
        self.diagnostics.record_structural_remap(record.clone());
        record
    }

    pub fn canonicalize_structural_branch_comparison_record(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
        reduced_match_set: &ReducedStructuralMatchSet,
        artifact: &PublishedBranchComparisonArtifact,
    ) -> BridgeCanonicalStructuralBranchComparisonRecord {
        let counters = BridgeStructuralCounters::from_structural_outcome(
            contract,
            planned_packet_set,
            reduced_match_set,
        );
        let record = BridgeCanonicalStructuralBranchComparisonRecord::new(
            BridgeStructuralBranchComparisonRecord::new(
                contract.clone(),
                planned_packet_set.clone(),
                reduced_match_set.clone(),
                artifact.clone(),
                counters,
            ),
        );
        self.diagnostics
            .record_structural_branch_comparison(record.clone());
        record
    }
}
