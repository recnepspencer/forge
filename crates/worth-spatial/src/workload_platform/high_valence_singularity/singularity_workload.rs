use topology::facade::TopologySeedNeighborhoodReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::singularity_counters::{
    HighValenceSingularityCounterInput, HighValenceSingularityCounters,
};
use super::singularity_receipt::HighValenceSingularityReceipt;
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

pub const HIGH_VALENCE_SINGULARITY_MAX_ADMITTED_VALENCE: usize = 128;

pub struct HighValenceSingularityWorkload<'a> {
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    topology_neighborhood: Option<&'a TopologySeedNeighborhoodReceipt>,
    rebuild_motion_compatibility: HighValenceRebuildMotionCompatibility,
    predicate_certification: HighValencePredicateCertification,
    singularity_policy: HighValenceSingularityPolicy,
    evidence_integrity: HighValenceEvidenceIntegrity,
}

impl<'a> HighValenceSingularityWorkload<'a> {
    pub fn from_platform_evidence(
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        topology_neighborhood: Option<&'a TopologySeedNeighborhoodReceipt>,
    ) -> Self {
        Self {
            evidence_ledger,
            topology_neighborhood,
            rebuild_motion_compatibility: HighValenceRebuildMotionCompatibility::Compatible,
            predicate_certification: HighValencePredicateCertification::Certified,
            singularity_policy: HighValenceSingularityPolicy::Admit,
            evidence_integrity: HighValenceEvidenceIntegrity::Consistent,
        }
    }

    pub fn requiring_rebuild_motion_compatibility(
        mut self,
        compatibility: HighValenceRebuildMotionCompatibility,
    ) -> Self {
        self.rebuild_motion_compatibility = compatibility;
        self
    }

    pub fn requiring_predicate_certification(
        mut self,
        predicate_certification: HighValencePredicateCertification,
    ) -> Self {
        self.predicate_certification = predicate_certification;
        self
    }

    pub fn requiring_singularity_policy(mut self, policy: HighValenceSingularityPolicy) -> Self {
        self.singularity_policy = policy;
        self
    }

    pub fn requiring_evidence_integrity(mut self, integrity: HighValenceEvidenceIntegrity) -> Self {
        self.evidence_integrity = integrity;
        self
    }

    pub fn certify(
        self,
    ) -> Result<HighValenceSingularityReceipt, HighValenceSingularityWorkloadError> {
        let neighborhood = self.required_topology_neighborhood()?;
        self.require_admitted_valence(neighborhood)?;
        self.require_predicate_certification()?;
        self.require_singularity_policy()?;
        self.require_rebuild_motion_compatibility()?;
        self.require_evidence_integrity()?;

        let counters = self.singularity_counters(neighborhood)?;
        let workload_identity = self.workload_identity()?;
        let center_vertex_identity = format!("{:?}", neighborhood.center_vertex_id());
        let local_rebuild_evidence_digest = self.local_rebuild_evidence_digest(neighborhood)?;
        let singularity_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "high-valence-singularity-workload".to_string(),
                workload_identity.clone(),
                center_vertex_identity.clone(),
                format!("neighborhood_valence:{}", counters.neighborhood_valence()),
                format!("topology_faces:{}", counters.topology_face_count()),
                format!("topology_relations:{}", counters.topology_relation_count()),
                format!("binding_targets:{}", counters.binding_target_count()),
                format!("projected_entities:{}", counters.projected_entity_count()),
                format!("local_basis_parts:{}", counters.local_basis_part_count()),
                format!(
                    "local_rebuild_rows:{}",
                    counters.local_rebuild_evidence_row_count()
                ),
                local_rebuild_evidence_digest.clone(),
                format!("retained_artifacts:{}", counters.retained_artifact_count()),
                format!("replay_checkpoints:{}", counters.replay_checkpoint_count()),
            ],
        );

        Ok(HighValenceSingularityReceipt::new(
            singularity_digest,
            workload_identity,
            center_vertex_identity,
            local_rebuild_evidence_digest,
            counters,
        ))
    }

    fn required_topology_neighborhood(
        &self,
    ) -> Result<&TopologySeedNeighborhoodReceipt, HighValenceSingularityWorkloadError> {
        self.topology_neighborhood
            .filter(|neighborhood| neighborhood.valence() > 0)
            .ok_or(HighValenceSingularityWorkloadError::MissingTopologyNeighborhood)
    }

    fn require_admitted_valence(
        &self,
        neighborhood: &TopologySeedNeighborhoodReceipt,
    ) -> Result<(), HighValenceSingularityWorkloadError> {
        let valence = neighborhood.valence();
        if (3..=HIGH_VALENCE_SINGULARITY_MAX_ADMITTED_VALENCE).contains(&valence) {
            Ok(())
        } else {
            Err(HighValenceSingularityWorkloadError::UnsupportedValence { valence })
        }
    }

    fn require_rebuild_motion_compatibility(
        &self,
    ) -> Result<(), HighValenceSingularityWorkloadError> {
        match &self.rebuild_motion_compatibility {
            HighValenceRebuildMotionCompatibility::Compatible => Ok(()),
            HighValenceRebuildMotionCompatibility::Incompatible { reason } => Err(
                HighValenceSingularityWorkloadError::RebuildMotionIncompatible {
                    reason: reason.clone(),
                },
            ),
        }
    }

    fn require_predicate_certification(&self) -> Result<(), HighValenceSingularityWorkloadError> {
        match self.predicate_certification {
            HighValencePredicateCertification::Certified => Ok(()),
            HighValencePredicateCertification::Uncertain => {
                Err(HighValenceSingularityWorkloadError::PredicateUncertain)
            }
        }
    }

    fn require_singularity_policy(&self) -> Result<(), HighValenceSingularityWorkloadError> {
        match self.singularity_policy {
            HighValenceSingularityPolicy::Admit => Ok(()),
            HighValenceSingularityPolicy::RequiresUserDecision => {
                Err(HighValenceSingularityWorkloadError::PolicyRequired)
            }
        }
    }

    fn require_evidence_integrity(&self) -> Result<(), HighValenceSingularityWorkloadError> {
        match self.evidence_integrity {
            HighValenceEvidenceIntegrity::Consistent => Ok(()),
            HighValenceEvidenceIntegrity::MismatchedProjectedNeighborhood { stage } => {
                Err(HighValenceSingularityWorkloadError::IntegrityMismatch { stage })
            }
        }
    }

    fn singularity_counters(
        &self,
        neighborhood: &TopologySeedNeighborhoodReceipt,
    ) -> Result<HighValenceSingularityCounters, HighValenceSingularityWorkloadError> {
        let topology = self.stage_counters(WorkloadEvidenceStage::Topology)?;
        let binding = self.stage_counters(WorkloadEvidenceStage::GeometryBinding)?;
        let support = self.stage_counters(WorkloadEvidenceStage::SurfaceSupport)?;
        let projection = self.stage_counters(WorkloadEvidenceStage::Projection)?;
        let transform = self.stage_counters(WorkloadEvidenceStage::Transform)?;
        let replay = self.stage_counters(WorkloadEvidenceStage::RetainedReplay)?;
        let diagnostics = self.stage_counters(WorkloadEvidenceStage::Diagnostics)?;
        let response = self.stage_counters(WorkloadEvidenceStage::Response)?;

        if topology.topology_relation_count() < neighborhood.valence() {
            return Err(HighValenceSingularityWorkloadError::MissingTopologyEvidence);
        }
        if topology.topology_face_count() < neighborhood.valence() {
            return Err(HighValenceSingularityWorkloadError::MissingTopologyEvidence);
        }
        if binding.binding_target_count() < neighborhood.valence() * 2 {
            return Err(HighValenceSingularityWorkloadError::MissingGeometryBindingEvidence);
        }
        if support.surface_support_count() == 0 {
            return Err(HighValenceSingularityWorkloadError::MissingSurfaceSupportEvidence);
        }
        if projection.projected_entity_count() < neighborhood.valence() * 2 {
            return Err(HighValenceSingularityWorkloadError::MissingProjectionEvidence);
        }
        if projection.local_basis_part_count() == 0 {
            return Err(HighValenceSingularityWorkloadError::MissingProjectionEvidence);
        }
        if transform.transform_step_count() == 0 {
            return Err(HighValenceSingularityWorkloadError::MissingTransformEvidence);
        }
        if replay.retained_artifact_count() == 0 || replay.replay_checkpoint_count() == 0 {
            return Err(HighValenceSingularityWorkloadError::MissingRetainedReplayEvidence);
        }
        if diagnostics.diagnostic_count() == 0 {
            return Err(HighValenceSingularityWorkloadError::MissingDiagnosticEvidence);
        }
        if response.user_outcome_count() == 0 {
            return Err(HighValenceSingularityWorkloadError::MissingResponseEvidence);
        }

        Ok(HighValenceSingularityCounters::new(
            HighValenceSingularityCounterInput {
                topology_entity_count: topology.topology_entity_count(),
                topology_face_count: topology.topology_face_count(),
                topology_relation_count: topology.topology_relation_count(),
                binding_target_count: binding.binding_target_count(),
                surface_support_count: support.surface_support_count(),
                neighborhood_valence: neighborhood.valence(),
                projected_entity_count: projection.projected_entity_count(),
                local_basis_part_count: projection.local_basis_part_count(),
                transform_step_count: transform.transform_step_count(),
                local_rebuild_evidence_row_count: 1,
                retained_artifact_count: replay.retained_artifact_count(),
                replay_checkpoint_count: replay.replay_checkpoint_count(),
                diagnostic_count: diagnostics.diagnostic_count(),
                user_outcome_count: response.user_outcome_count(),
            },
        ))
    }

    fn local_rebuild_evidence_digest(
        &self,
        neighborhood: &TopologySeedNeighborhoodReceipt,
    ) -> Result<String, HighValenceSingularityWorkloadError> {
        let transform = self.receipt_backed_stage_row(WorkloadEvidenceStage::Transform)?;
        let replay = self.receipt_backed_stage_row(WorkloadEvidenceStage::RetainedReplay)?;
        Ok(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "high-valence-local-rebuild-evidence".to_string(),
                format!("center:{:?}", neighborhood.center_vertex_id()),
                format!("valence:{}", neighborhood.valence()),
                format!("incident_half_edges:{}", neighborhood.valence()),
                format!("transform:{}", transform.evidence_identity()),
                format!("retained_replay:{}", replay.evidence_identity()),
            ],
        ))
    }

    fn stage_counters(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<WorkloadEvidenceStageCounters, HighValenceSingularityWorkloadError> {
        self.evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .map(|row| row.counters())
            .ok_or(HighValenceSingularityWorkloadError::MissingReceiptBackedStage(stage))
    }

    fn receipt_backed_stage_row(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<
        &crate::workload_platform::evidence_ledger::WorkloadEvidenceRow,
        HighValenceSingularityWorkloadError,
    > {
        self.evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .ok_or(HighValenceSingularityWorkloadError::MissingReceiptBackedStage(stage))
    }

    fn workload_identity(&self) -> Result<String, HighValenceSingularityWorkloadError> {
        let mut parts = Vec::new();
        for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
            let row = self
                .evidence_ledger
                .row_for_stage(stage)
                .filter(|row| row.is_receipt_backed() && row.is_admitted())
                .ok_or(HighValenceSingularityWorkloadError::MissingReceiptBackedStage(stage))?;
            parts.push(format!("{stage:?}:{}", row.evidence_identity()));
        }
        Ok(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "high-valence-singularity-workload-ledger".to_string(),
                parts.join("|"),
            ],
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HighValenceRebuildMotionCompatibility {
    Compatible,
    Incompatible { reason: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HighValencePredicateCertification {
    Certified,
    Uncertain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HighValenceSingularityPolicy {
    Admit,
    RequiresUserDecision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HighValenceEvidenceIntegrity {
    Consistent,
    MismatchedProjectedNeighborhood { stage: WorkloadEvidenceStage },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HighValenceSingularityWorkloadError {
    MissingReceiptBackedStage(WorkloadEvidenceStage),
    MissingTopologyNeighborhood,
    MissingTopologyEvidence,
    MissingGeometryBindingEvidence,
    MissingSurfaceSupportEvidence,
    MissingProjectionEvidence,
    MissingTransformEvidence,
    MissingRetainedReplayEvidence,
    MissingDiagnosticEvidence,
    MissingResponseEvidence,
    UnsupportedValence { valence: usize },
    RebuildMotionIncompatible { reason: String },
    PredicateUncertain,
    PolicyRequired,
    IntegrityMismatch { stage: WorkloadEvidenceStage },
}

impl HighValenceSingularityWorkloadError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingReceiptBackedStage(stage) => {
                format!(
                    "high-valence singularity requires receipt-backed {}",
                    stage.human_name()
                )
            }
            Self::MissingTopologyNeighborhood => {
                "high-valence singularity requires a topology neighborhood receipt".to_string()
            }
            Self::MissingTopologyEvidence => {
                "high-valence singularity requires topology face and relation evidence for every incident half-edge".to_string()
            }
            Self::MissingGeometryBindingEvidence => {
                "high-valence singularity requires geometry binding evidence for every incident face and edge".to_string()
            }
            Self::MissingSurfaceSupportEvidence => {
                "high-valence singularity requires certified planar surface support for the incident faces".to_string()
            }
            Self::MissingProjectionEvidence => {
                "high-valence singularity requires projected incident face and edge evidence with a local basis"
                    .to_string()
            }
            Self::MissingTransformEvidence => {
                "high-valence singularity requires movement and rotation transform evidence"
                    .to_string()
            }
            Self::MissingRetainedReplayEvidence => {
                "high-valence singularity requires retained artifacts and replay checkpoints"
                    .to_string()
            }
            Self::MissingDiagnosticEvidence => {
                "high-valence singularity requires diagnostic evidence naming the singular neighborhood".to_string()
            }
            Self::MissingResponseEvidence => {
                "high-valence singularity requires a user response receipt".to_string()
            }
            Self::UnsupportedValence { valence } => {
                format!(
                    "high-valence singularity supports valence 3 through {HIGH_VALENCE_SINGULARITY_MAX_ADMITTED_VALENCE} today; valence {valence} needs an explicit widening phase"
                )
            }
            Self::RebuildMotionIncompatible { reason } => reason.clone(),
            Self::PredicateUncertain => {
                "predicate authority could not certify the high-valence neighborhood; inspect exact predicate evidence before rebuild".to_string()
            }
            Self::PolicyRequired => {
                "high-valence singularity needs a user policy decision before local rebuild"
                    .to_string()
            }
            Self::IntegrityMismatch { stage } => {
                format!(
                    "high-valence singularity evidence must consume the same {} as the topology neighborhood",
                    stage.human_name()
                )
            }
        }
    }
}
