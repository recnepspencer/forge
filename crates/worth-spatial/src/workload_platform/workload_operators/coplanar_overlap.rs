use crate::workload_platform::evidence_ledger::{WorkloadEvidenceRow, WorkloadEvidenceStage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapWorkloadOperator {
    consumed_evidence: Vec<WorkloadEvidenceRow>,
}

impl CoplanarOverlapWorkloadOperator {
    pub fn from_consumed_evidence(consumed_evidence: &[WorkloadEvidenceRow]) -> Self {
        Self {
            consumed_evidence: consumed_evidence.to_vec(),
        }
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
        let operator_digest =
            format!("coplanar-overlap-operator:{projection}:{transform}:{retained_replay}");
        Ok(CoplanarOverlapOperatorReceipt {
            operator_digest,
            consumed_evidence_identities: consumed_identities(&self.consumed_evidence),
            operator_input_count: self.consumed_evidence.len(),
            operator_receipt_count: 1,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapOperatorReceipt {
    operator_digest: String,
    consumed_evidence_identities: Vec<String>,
    operator_input_count: usize,
    operator_receipt_count: usize,
}

impl CoplanarOverlapOperatorReceipt {
    pub fn operator_digest(&self) -> &str {
        &self.operator_digest
    }

    pub fn consumed_evidence_identities(&self) -> &[String] {
        &self.consumed_evidence_identities
    }

    pub fn operator_input_count(&self) -> usize {
        self.operator_input_count
    }

    pub fn operator_receipt_count(&self) -> usize {
        self.operator_receipt_count
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
        RequiredOperatorEvidenceStage::Transform if counters.transform_step_count() == 0 => {
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
