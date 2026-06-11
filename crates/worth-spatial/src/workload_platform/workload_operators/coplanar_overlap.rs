use crate::planar_contracts::coplanar_overlap_contract::CoplanarOverlapContractReceipt;
use crate::workload_platform::evidence_ledger::{WorkloadEvidenceRow, WorkloadEvidenceStage};

use super::coplanar_overlap_extractions::{
    extraction_summary, operator_digest, CoplanarOverlapOperatorExtraction,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapWorkloadOperator {
    consumed_evidence: Vec<WorkloadEvidenceRow>,
    overlap_extractions: Vec<CoplanarOverlapOperatorExtraction>,
}

impl CoplanarOverlapWorkloadOperator {
    pub fn from_consumed_evidence(consumed_evidence: &[WorkloadEvidenceRow]) -> Self {
        Self {
            consumed_evidence: consumed_evidence.to_vec(),
            overlap_extractions: Vec::new(),
        }
    }

    pub fn with_overlap_extractions(mut self, receipts: &[CoplanarOverlapContractReceipt]) -> Self {
        self.overlap_extractions = receipts
            .iter()
            .map(CoplanarOverlapOperatorExtraction::from_receipt)
            .collect();
        self
    }

    pub fn execute(self) -> Result<CoplanarOverlapOperatorReceipt, CoplanarOverlapOperatorDenial> {
        require_honest_stage(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Projection,
        )?;
        require_honest_stage(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Transform,
        )?;
        require_honest_stage(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::RetainedReplay,
        )?;
        let projection = stage_identity(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Projection,
        )?;
        let transform = stage_identity(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Transform,
        )?;
        let retained_replay = stage_identity(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::RetainedReplay,
        )?;
        let extraction_summary = extraction_summary(&self.overlap_extractions)?;
        let operator_digest = operator_digest(
            projection,
            transform,
            retained_replay,
            &extraction_summary.extraction_identities,
            &extraction_summary,
        );
        Ok(CoplanarOverlapOperatorReceipt {
            operator_digest,
            consumed_evidence_identities: consumed_identities(&self.consumed_evidence),
            overlap_extraction_identities: extraction_summary.extraction_identities,
            operator_input_count: self.consumed_evidence.len() + extraction_summary.receipt_count,
            operator_receipt_count: 1,
            overlap_extraction_receipt_count: extraction_summary.receipt_count,
            overlap_candidate_pair_breadth: extraction_summary.candidate_pair_breadth,
            overlap_segment_contacts_certified: extraction_summary.segment_contacts_certified,
            overlap_shared_intervals: extraction_summary.shared_intervals,
            overlap_islands: extraction_summary.overlap_islands,
            overlap_containment_relations: extraction_summary.containment_relations,
            overlap_policy_required_exits: extraction_summary.policy_required_exits,
            overlap_ambiguous_contacts: extraction_summary.ambiguous_contacts,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapOperatorReceipt {
    operator_digest: String,
    consumed_evidence_identities: Vec<String>,
    overlap_extraction_identities: Vec<String>,
    operator_input_count: usize,
    operator_receipt_count: usize,
    overlap_extraction_receipt_count: usize,
    overlap_candidate_pair_breadth: usize,
    overlap_segment_contacts_certified: usize,
    overlap_shared_intervals: usize,
    overlap_islands: usize,
    overlap_containment_relations: usize,
    overlap_policy_required_exits: usize,
    overlap_ambiguous_contacts: usize,
}

impl CoplanarOverlapOperatorReceipt {
    pub fn operator_digest(&self) -> &str {
        &self.operator_digest
    }

    pub fn consumed_evidence_identities(&self) -> &[String] {
        &self.consumed_evidence_identities
    }

    pub fn overlap_extraction_identities(&self) -> &[String] {
        &self.overlap_extraction_identities
    }

    pub fn operator_input_count(&self) -> usize {
        self.operator_input_count
    }

    pub fn operator_receipt_count(&self) -> usize {
        self.operator_receipt_count
    }

    pub fn overlap_extraction_receipt_count(&self) -> usize {
        self.overlap_extraction_receipt_count
    }

    pub fn overlap_candidate_pair_breadth(&self) -> usize {
        self.overlap_candidate_pair_breadth
    }

    pub fn overlap_segment_contacts_certified(&self) -> usize {
        self.overlap_segment_contacts_certified
    }

    pub fn overlap_shared_intervals(&self) -> usize {
        self.overlap_shared_intervals
    }

    pub fn overlap_islands(&self) -> usize {
        self.overlap_islands
    }

    pub fn overlap_containment_relations(&self) -> usize {
        self.overlap_containment_relations
    }

    pub fn overlap_policy_required_exits(&self) -> usize {
        self.overlap_policy_required_exits
    }

    pub fn overlap_ambiguous_contacts(&self) -> usize {
        self.overlap_ambiguous_contacts
    }

    pub fn links_to_stage(&self, stage: WorkloadEvidenceStage) -> bool {
        self.consumed_evidence_identities
            .iter()
            .any(|identity| identity.starts_with(&format!("{stage:?}:")))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RequiredOperatorEvidenceStage {
    Projection,
    Transform,
    RetainedReplay,
}

impl RequiredOperatorEvidenceStage {
    fn evidence_stage(self) -> WorkloadEvidenceStage {
        match self {
            Self::Projection => WorkloadEvidenceStage::Projection,
            Self::Transform => WorkloadEvidenceStage::Transform,
            Self::RetainedReplay => WorkloadEvidenceStage::RetainedReplay,
        }
    }

    fn missing_denial(self) -> CoplanarOverlapOperatorDenial {
        match self {
            Self::Projection => CoplanarOverlapOperatorDenial::MissingProjectedWorkload,
            Self::Transform => CoplanarOverlapOperatorDenial::MissingTransformWorkload,
            Self::RetainedReplay => CoplanarOverlapOperatorDenial::MissingRetainedReplayWorkload,
        }
    }

    fn manual_denial(self) -> CoplanarOverlapOperatorDenial {
        match self {
            Self::Projection => CoplanarOverlapOperatorDenial::ManualProjectedWorkload,
            Self::Transform => CoplanarOverlapOperatorDenial::ManualTransformWorkload,
            Self::RetainedReplay => CoplanarOverlapOperatorDenial::ManualRetainedReplayWorkload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapOperatorDenial {
    MissingProjectedWorkload,
    MissingTransformWorkload,
    MissingRetainedReplayWorkload,
    ManualProjectedWorkload,
    ManualTransformWorkload,
    ManualRetainedReplayWorkload,
    SyntheticProjectedWorkload,
    SyntheticTransformWorkload,
    SyntheticRetainedReplayWorkload,
    MissingOverlapExtractionReceipts,
    SyntheticOverlapExtraction,
}

impl CoplanarOverlapOperatorDenial {
    pub fn human_reason(self) -> &'static str {
        match self {
            Self::MissingProjectedWorkload => {
                "coplanar overlap operator requires projected planar workload evidence"
            }
            Self::MissingTransformWorkload => {
                "coplanar overlap operator requires transform workload evidence"
            }
            Self::MissingRetainedReplayWorkload => {
                "coplanar overlap operator requires retained replay workload evidence"
            }
            Self::ManualProjectedWorkload => {
                "coplanar overlap operator rejects hand-filled projection evidence"
            }
            Self::ManualTransformWorkload => {
                "coplanar overlap operator rejects hand-filled transform evidence"
            }
            Self::ManualRetainedReplayWorkload => {
                "coplanar overlap operator rejects hand-filled retained replay evidence"
            }
            Self::SyntheticProjectedWorkload => {
                "coplanar overlap operator requires projected entities and local-basis evidence"
            }
            Self::SyntheticTransformWorkload => {
                "coplanar overlap operator requires real transform step evidence"
            }
            Self::SyntheticRetainedReplayWorkload => {
                "coplanar overlap operator requires retained artifact and replay checkpoint evidence"
            }
            Self::MissingOverlapExtractionReceipts => {
                "coplanar overlap operator requires real overlap extraction receipts"
            }
            Self::SyntheticOverlapExtraction => {
                "coplanar overlap operator requires overlap extraction receipts with candidate pairs and retained overlap facts"
            }
        }
    }
}

fn require_honest_stage(
    consumed_evidence: &[WorkloadEvidenceRow],
    required_stage: RequiredOperatorEvidenceStage,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    let stage = required_stage.evidence_stage();
    let row = consumed_evidence
        .iter()
        .find(|row| row.stage() == stage)
        .ok_or_else(|| required_stage.missing_denial())?;
    if !row.is_receipt_backed() || !row.is_admitted() {
        return Err(required_stage.manual_denial());
    }
    let counters = row.counters();
    match required_stage {
        RequiredOperatorEvidenceStage::Projection
            if counters.projected_entity_count() == 0 || counters.local_basis_part_count() == 0 =>
        {
            Err(CoplanarOverlapOperatorDenial::SyntheticProjectedWorkload)
        }
        RequiredOperatorEvidenceStage::Transform
            if counters.transform_changed_coordinate_count() == 0 =>
        {
            Err(CoplanarOverlapOperatorDenial::SyntheticTransformWorkload)
        }
        RequiredOperatorEvidenceStage::RetainedReplay
            if counters.retained_artifact_count() == 0
                || counters.replay_checkpoint_count() == 0 =>
        {
            Err(CoplanarOverlapOperatorDenial::SyntheticRetainedReplayWorkload)
        }
        _ => Ok(()),
    }
}

fn stage_identity(
    consumed_evidence: &[WorkloadEvidenceRow],
    required_stage: RequiredOperatorEvidenceStage,
) -> Result<&str, CoplanarOverlapOperatorDenial> {
    consumed_evidence
        .iter()
        .find(|row| row.stage() == required_stage.evidence_stage())
        .map(WorkloadEvidenceRow::evidence_identity)
        .ok_or_else(|| required_stage.missing_denial())
}

fn consumed_identities(consumed_evidence: &[WorkloadEvidenceRow]) -> Vec<String> {
    consumed_evidence
        .iter()
        .map(|row| format!("{:?}:{}", row.stage(), row.evidence_identity()))
        .collect()
}
