use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::storm_counters::{CoplanarOverlapStormCounterInput, CoplanarOverlapStormCounters};
use super::storm_receipt::CoplanarOverlapStormReceipt;
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};
use crate::workload_platform::workload_operators::CoplanarOverlapOperatorReceipt;

pub struct CoplanarOverlapStormWorkload<'a> {
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    operator_receipt: &'a CoplanarOverlapOperatorReceipt,
}

impl<'a> CoplanarOverlapStormWorkload<'a> {
    pub fn from_platform_evidence(
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        operator_receipt: &'a CoplanarOverlapOperatorReceipt,
    ) -> Self {
        Self {
            evidence_ledger,
            operator_receipt,
        }
    }

    pub fn certify(self) -> Result<CoplanarOverlapStormReceipt, CoplanarOverlapStormWorkloadError> {
        let counters = self.storm_counters()?;
        let workload_identity = self.workload_identity()?;
        let operator_identity = self.operator_receipt.operator_digest().to_string();
        let storm_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "coplanar-overlap-storm-workload".to_string(),
                workload_identity.clone(),
                operator_identity.clone(),
                format!("topology_faces:{}", counters.topology_face_count()),
                format!("topology_relations:{}", counters.topology_relation_count()),
                format!(
                    "transform_cancellation_steps:{}",
                    counters.transform_cancellation_step_count()
                ),
                format!("retained_artifacts:{}", counters.retained_artifact_count()),
                format!("replay_checkpoints:{}", counters.replay_checkpoint_count()),
                format!(
                    "overlap_extraction_receipts:{}",
                    counters.overlap_extraction_receipt_count()
                ),
                format!(
                    "overlap_candidate_pair_breadth:{}",
                    counters.overlap_candidate_pair_breadth()
                ),
                format!(
                    "overlap_segment_contacts:{}",
                    counters.overlap_segment_contacts_certified()
                ),
            ],
        );

        Ok(CoplanarOverlapStormReceipt::new(
            storm_digest,
            workload_identity,
            operator_identity,
            counters,
        ))
    }

    fn storm_counters(
        &self,
    ) -> Result<CoplanarOverlapStormCounters, CoplanarOverlapStormWorkloadError> {
        let topology = self.stage_counters(WorkloadEvidenceStage::Topology)?;
        let projection = self.stage_counters(WorkloadEvidenceStage::Projection)?;
        let transform = self.stage_counters(WorkloadEvidenceStage::Transform)?;
        let replay = self.stage_counters(WorkloadEvidenceStage::RetainedReplay)?;

        if topology.topology_face_count() == 0 || topology.topology_relation_count() == 0 {
            return Err(CoplanarOverlapStormWorkloadError::MissingTopologyEvidence);
        }
        if replay.retained_artifact_count() == 0 || replay.replay_checkpoint_count() == 0 {
            return Err(CoplanarOverlapStormWorkloadError::MissingRetainedReplayEvidence);
        }
        if transform.transform_changed_coordinate_count() == 0 {
            return Err(CoplanarOverlapStormWorkloadError::MissingTransformEvidence);
        }
        if projection.projected_entity_count() == 0 || projection.local_basis_part_count() == 0 {
            return Err(CoplanarOverlapStormWorkloadError::MissingProjectionEvidence);
        }
        if self.operator_receipt.operator_receipt_count() == 0 {
            return Err(CoplanarOverlapStormWorkloadError::MissingOperatorEvidence);
        }
        if self.operator_receipt.overlap_extraction_receipt_count() == 0 {
            return Err(CoplanarOverlapStormWorkloadError::MissingOverlapExtractionEvidence);
        }
        self.require_operator_link(WorkloadEvidenceStage::Projection)?;
        self.require_operator_link(WorkloadEvidenceStage::Transform)?;
        self.require_operator_link(WorkloadEvidenceStage::RetainedReplay)?;

        Ok(CoplanarOverlapStormCounters::new(
            CoplanarOverlapStormCounterInput {
                topology_entity_count: topology.topology_entity_count(),
                topology_face_count: topology.topology_face_count(),
                topology_relation_count: topology.topology_relation_count(),
                projected_entity_count: projection.projected_entity_count(),
                transform_step_count: transform.transform_step_count(),
                transform_cancellation_step_count: transform.transform_cancellation_step_count(),
                retained_artifact_count: replay.retained_artifact_count(),
                replay_checkpoint_count: replay.replay_checkpoint_count(),
                operator_input_count: self.operator_receipt.operator_input_count(),
                operator_receipt_count: self.operator_receipt.operator_receipt_count(),
                overlap_extraction_receipt_count: self
                    .operator_receipt
                    .overlap_extraction_receipt_count(),
                overlap_candidate_pair_breadth: self
                    .operator_receipt
                    .overlap_candidate_pair_breadth(),
                overlap_segment_contacts_certified: self
                    .operator_receipt
                    .overlap_segment_contacts_certified(),
                overlap_shared_intervals: self.operator_receipt.overlap_shared_intervals(),
                overlap_islands: self.operator_receipt.overlap_islands(),
                overlap_policy_required_exits: self
                    .operator_receipt
                    .overlap_policy_required_exits(),
                overlap_ambiguous_contacts: self.operator_receipt.overlap_ambiguous_contacts(),
            },
        ))
    }

    fn stage_counters(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters,
        CoplanarOverlapStormWorkloadError,
    > {
        self.evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .map(|row| row.counters())
            .ok_or(CoplanarOverlapStormWorkloadError::MissingReceiptBackedStage(stage))
    }

    fn require_operator_link(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<(), CoplanarOverlapStormWorkloadError> {
        let row = self
            .evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .ok_or(CoplanarOverlapStormWorkloadError::MissingReceiptBackedStage(stage))?;
        let expected_link = format!("{stage:?}:{}", row.evidence_identity());
        if self
            .operator_receipt
            .consumed_evidence_identities()
            .contains(&expected_link)
        {
            Ok(())
        } else {
            Err(CoplanarOverlapStormWorkloadError::MismatchedOperatorStageLink(stage))
        }
    }

    fn workload_identity(&self) -> Result<String, CoplanarOverlapStormWorkloadError> {
        let mut parts = Vec::new();
        for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
            let row = self
                .evidence_ledger
                .row_for_stage(stage)
                .filter(|row| row.is_receipt_backed() && row.is_admitted())
                .ok_or(CoplanarOverlapStormWorkloadError::MissingReceiptBackedStage(stage))?;
            parts.push(format!("{stage:?}:{}", row.evidence_identity()));
        }
        Ok(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "coplanar-overlap-storm-workload-ledger".to_string(),
                parts.join("|"),
            ],
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapStormWorkloadError {
    MissingReceiptBackedStage(WorkloadEvidenceStage),
    MissingTopologyEvidence,
    MissingProjectionEvidence,
    MissingTransformEvidence,
    MissingRetainedReplayEvidence,
    MissingOperatorEvidence,
    MissingOverlapExtractionEvidence,
    MismatchedOperatorStageLink(WorkloadEvidenceStage),
}

impl CoplanarOverlapStormWorkloadError {
    pub fn human_reason(self) -> String {
        match self {
            Self::MissingReceiptBackedStage(stage) => {
                format!(
                    "coplanar overlap storm requires receipt-backed {}",
                    stage.human_name()
                )
            }
            Self::MissingTopologyEvidence => {
                "coplanar overlap storm requires topology face and relation evidence".to_string()
            }
            Self::MissingProjectionEvidence => {
                "coplanar overlap storm requires projected entities and local-frame evidence"
                    .to_string()
            }
            Self::MissingTransformEvidence => {
                "coplanar overlap storm requires movement and rotation transform evidence"
                    .to_string()
            }
            Self::MissingRetainedReplayEvidence => {
                "coplanar overlap storm requires retained artifacts and replay checkpoints"
                    .to_string()
            }
            Self::MissingOperatorEvidence => {
                "coplanar overlap storm requires an executed overlap operator receipt".to_string()
            }
            Self::MissingOverlapExtractionEvidence => {
                "coplanar overlap storm requires overlap extraction receipts from the operator"
                    .to_string()
            }
            Self::MismatchedOperatorStageLink(stage) => {
                format!(
                    "coplanar overlap storm operator receipt must consume the same {} as the workload ledger",
                    stage.human_name()
                )
            }
        }
    }
}
