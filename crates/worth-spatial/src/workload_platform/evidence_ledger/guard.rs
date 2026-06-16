use std::fmt;

use super::{WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct};

pub struct WorkloadEvidenceGuard<'a> {
    stage_index: &'a WorkloadEvidenceStageIndexProduct,
}

impl fmt::Debug for WorkloadEvidenceGuard<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage_index_identity = self.stage_index.index_identity();
        formatter
            .debug_struct("WorkloadEvidenceGuard")
            .field("stage_index_identity", &stage_index_identity)
            .finish()
    }
}

impl<'a> WorkloadEvidenceGuard<'a> {
    pub(crate) fn new(stage_index: &'a WorkloadEvidenceStageIndexProduct) -> Self {
        Self { stage_index }
    }

    pub fn assert_uses_real_topology(self) -> Result<Self, WorkloadEvidenceGuardError> {
        let row = self.receipt_backed_row(WorkloadEvidenceStage::Topology)?;
        let counters = row.counters();
        if counters.topology_entity_count() == 0 || counters.topology_relation_count() == 0 {
            return Err(WorkloadEvidenceGuardError::SyntheticTopology);
        }
        Ok(self)
    }

    pub fn assert_binding_is_receipt_backed(self) -> Result<Self, WorkloadEvidenceGuardError> {
        self.receipt_backed_row(WorkloadEvidenceStage::GeometryBinding)?;
        Ok(self)
    }

    pub fn assert_projection_is_receipt_backed(self) -> Result<Self, WorkloadEvidenceGuardError> {
        self.receipt_backed_row(WorkloadEvidenceStage::Projection)?;
        Ok(self)
    }

    pub fn assert_transform_changed_geometry(self) -> Result<Self, WorkloadEvidenceGuardError> {
        let row = self.receipt_backed_row(WorkloadEvidenceStage::Transform)?;
        if row.counters().transform_changed_coordinate_count() == 0 {
            return Err(WorkloadEvidenceGuardError::LabelOnlyMotion);
        }
        Ok(self)
    }

    pub fn assert_replay_consumed_retained_artifact(
        self,
    ) -> Result<Self, WorkloadEvidenceGuardError> {
        let row = self.receipt_backed_row(WorkloadEvidenceStage::RetainedReplay)?;
        let counters = row.counters();
        if counters.retained_artifact_count() == 0 || counters.replay_checkpoint_count() == 0 {
            return Err(WorkloadEvidenceGuardError::SyntheticReplay);
        }
        Ok(self)
    }

    pub fn assert_counters_are_receipt_backed(self) -> Result<Self, WorkloadEvidenceGuardError> {
        let missing_counter = WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.stage_index
                    .row_for_stage(*stage)
                    .is_some_and(|row| row.counters().total_receipt_backed_counters() == 0)
            });
        if let Some(stage) = missing_counter {
            return Err(WorkloadEvidenceGuardError::MissingReceiptBackedCounters(
                stage,
            ));
        }
        Ok(self)
    }

    pub fn assert_no_fixture_arithmetic_as_truth(self) -> Result<Self, WorkloadEvidenceGuardError> {
        if let Some(stage) = self.manual_authority_stage() {
            return Err(WorkloadEvidenceGuardError::FixtureArithmeticAsTruth(stage));
        }
        Ok(self)
    }

    pub fn assert_no_synthetic_end_to_end_claim(self) -> Result<Self, WorkloadEvidenceGuardError> {
        if let Some(stage) = self.stage_index.missing_authority_stage() {
            return Err(WorkloadEvidenceGuardError::IncompleteEndToEndClaim(stage));
        }
        if let Some(stage) = self.manual_authority_stage() {
            return Err(WorkloadEvidenceGuardError::SyntheticEndToEndClaim(stage));
        }
        if let Some(stage) = self.unadmitted_authority_stage() {
            return Err(WorkloadEvidenceGuardError::UnsupportedEndToEndClaim(stage));
        }
        Ok(self)
    }

    fn receipt_backed_row(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<&super::WorkloadEvidenceRow, WorkloadEvidenceGuardError> {
        let row = self
            .stage_index
            .row_for_stage(stage)
            .ok_or(WorkloadEvidenceGuardError::MissingReceiptBackedStage(stage))?;
        if row.is_receipt_backed() {
            Ok(row)
        } else {
            Err(WorkloadEvidenceGuardError::ManualStage(stage))
        }
    }

    fn manual_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.stage_index
                    .row_for_stage(*stage)
                    .is_some_and(|row| !row.is_receipt_backed())
            })
    }

    fn unadmitted_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.stage_index
                    .row_for_stage(*stage)
                    .is_some_and(|row| row.is_receipt_backed() && !row.is_admitted())
            })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceGuardError {
    MissingReceiptBackedStage(WorkloadEvidenceStage),
    ManualStage(WorkloadEvidenceStage),
    SyntheticTopology,
    LabelOnlyMotion,
    SyntheticReplay,
    MissingReceiptBackedCounters(WorkloadEvidenceStage),
    FixtureArithmeticAsTruth(WorkloadEvidenceStage),
    IncompleteEndToEndClaim(WorkloadEvidenceStage),
    SyntheticEndToEndClaim(WorkloadEvidenceStage),
    UnsupportedEndToEndClaim(WorkloadEvidenceStage),
}

impl WorkloadEvidenceGuardError {
    pub fn human_reason(self) -> &'static str {
        match self {
            Self::MissingReceiptBackedStage(_) => {
                "workload evidence guard requires a source receipt for this stage"
            }
            Self::ManualStage(_) => "workload evidence guard rejects hand-filled stage evidence",
            Self::SyntheticTopology => {
                "workload evidence guard requires real topology entity and relation evidence"
            }
            Self::LabelOnlyMotion => {
                "workload evidence guard requires coordinate-changing transform evidence"
            }
            Self::SyntheticReplay => {
                "workload evidence guard requires retained artifacts and replay checkpoints"
            }
            Self::MissingReceiptBackedCounters(_) => {
                "workload evidence guard requires receipt-backed counters for every stage"
            }
            Self::FixtureArithmeticAsTruth(_) => {
                "workload evidence guard rejects fixture arithmetic as truth evidence"
            }
            Self::IncompleteEndToEndClaim(_) => {
                "workload evidence guard rejects incomplete end-to-end claims"
            }
            Self::SyntheticEndToEndClaim(_) => {
                "workload evidence guard rejects synthetic end-to-end claims"
            }
            Self::UnsupportedEndToEndClaim(_) => {
                "workload evidence guard rejects unsupported stages in end-to-end claims"
            }
        }
    }
}
