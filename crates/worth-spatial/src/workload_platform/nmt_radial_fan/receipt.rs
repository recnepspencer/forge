use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::denial::NmtRadialFanDenial;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters;
use crate::workload_platform::nmt_certification_context::NmtCertifiedScopeContext;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtRadialFanCounters {
    incident_face_count: usize,
    open_boundary_half_edge_count: usize,
    non_manifold_edge_count: usize,
    topology_face_count: usize,
    projected_entity_count: usize,
    transform_step_count: usize,
    changed_coordinate_count: usize,
    retained_artifact_count: usize,
    replay_checkpoint_count: usize,
    diagnostic_count: usize,
    user_outcome_count: usize,
}

impl NmtRadialFanCounters {
    pub(crate) fn new(input: NmtRadialFanCounterInput) -> Self {
        Self {
            incident_face_count: input.incident_face_count,
            open_boundary_half_edge_count: input.open_boundary_half_edge_count,
            non_manifold_edge_count: input.non_manifold_edge_count,
            topology_face_count: input.topology.topology_face_count(),
            projected_entity_count: input.projection.projected_entity_count(),
            transform_step_count: input.transform.transform_step_count(),
            changed_coordinate_count: input.transform.transform_changed_coordinate_count(),
            retained_artifact_count: input.replay.retained_artifact_count(),
            replay_checkpoint_count: input.replay.replay_checkpoint_count(),
            diagnostic_count: input.diagnostics.diagnostic_count(),
            user_outcome_count: input.response.user_outcome_count(),
        }
    }

    pub(crate) fn from_certified_scope(scope: &NmtCertifiedScopeContext) -> Self {
        let topology = scope.topology_scope().counters();
        let projection = scope.projection().counters();
        let motion = scope.motion().counters();
        let replay = scope.retained_replay().counters();
        Self {
            incident_face_count: topology.face_count(),
            open_boundary_half_edge_count: topology.boundary_half_edge_count(),
            non_manifold_edge_count: topology.non_manifold_edge_count(),
            topology_face_count: topology.face_count(),
            projected_entity_count: projection.scope_projected_entities_consumed(),
            transform_step_count: motion.transform_steps(),
            changed_coordinate_count: motion.changed_coordinate_rows(),
            retained_artifact_count: replay.scope_retained_artifact_rows(),
            replay_checkpoint_count: replay.scope_checkpoints_consumed(),
            diagnostic_count: 1,
            user_outcome_count: 1,
        }
    }

    pub fn incident_face_count(self) -> usize {
        self.incident_face_count
    }

    pub fn open_boundary_half_edge_count(self) -> usize {
        self.open_boundary_half_edge_count
    }

    pub fn non_manifold_edge_count(self) -> usize {
        self.non_manifold_edge_count
    }

    pub fn topology_face_count(self) -> usize {
        self.topology_face_count
    }

    pub fn projected_entity_count(self) -> usize {
        self.projected_entity_count
    }

    pub fn transform_step_count(self) -> usize {
        self.transform_step_count
    }

    pub fn changed_coordinate_count(self) -> usize {
        self.changed_coordinate_count
    }

    pub fn retained_artifact_count(self) -> usize {
        self.retained_artifact_count
    }

    pub fn replay_checkpoint_count(self) -> usize {
        self.replay_checkpoint_count
    }

    pub fn diagnostic_count(self) -> usize {
        self.diagnostic_count
    }

    pub fn user_outcome_count(self) -> usize {
        self.user_outcome_count
    }
}

pub(crate) struct NmtRadialFanCounterInput {
    pub incident_face_count: usize,
    pub open_boundary_half_edge_count: usize,
    pub non_manifold_edge_count: usize,
    pub topology: WorkloadEvidenceStageCounters,
    pub projection: WorkloadEvidenceStageCounters,
    pub transform: WorkloadEvidenceStageCounters,
    pub replay: WorkloadEvidenceStageCounters,
    pub diagnostics: WorkloadEvidenceStageCounters,
    pub response: WorkloadEvidenceStageCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtRadialFanReceipt {
    workload_identity: String,
    fan_digest: String,
    topology_construction_identity: String,
    topology_posture: String,
    projected_workload_identity: String,
    open_boundary_digest: String,
    radial_adjacency_digest: String,
    transform_posture_identity: String,
    retained_replay_identity: String,
    retained_artifact_identity: String,
    transformed_workload_identity: String,
    counters: NmtRadialFanCounters,
}

pub(crate) struct NmtRadialFanReceiptInput {
    pub workload_identity: String,
    pub topology_construction_identity: String,
    pub topology_posture: String,
    pub projected_workload_identity: String,
    pub open_boundary_digest: String,
    pub radial_adjacency_digest: String,
    pub transform_posture_identity: String,
    pub retained_replay_identity: String,
    pub retained_artifact_identity: String,
    pub transformed_workload_identity: String,
    pub counters: NmtRadialFanCounters,
}

impl NmtRadialFanReceipt {
    pub(crate) fn new(input: NmtRadialFanReceiptInput) -> Self {
        let fan_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-open-radial-fan-workload".to_string(),
                input.workload_identity.clone(),
                input.topology_construction_identity.clone(),
                input.topology_posture.clone(),
                input.projected_workload_identity.clone(),
                input.open_boundary_digest.clone(),
                input.radial_adjacency_digest.clone(),
                input.transform_posture_identity.clone(),
                input.retained_replay_identity.clone(),
                input.retained_artifact_identity.clone(),
                input.transformed_workload_identity.clone(),
                format!("incident_faces:{}", input.counters.incident_face_count()),
                format!(
                    "boundary_half_edges:{}",
                    input.counters.open_boundary_half_edge_count()
                ),
                format!(
                    "non_manifold_edges:{}",
                    input.counters.non_manifold_edge_count()
                ),
                format!("diagnostics:{}", input.counters.diagnostic_count()),
                format!("user_outcomes:{}", input.counters.user_outcome_count()),
            ],
        );
        Self {
            workload_identity: input.workload_identity,
            fan_digest,
            topology_construction_identity: input.topology_construction_identity,
            topology_posture: input.topology_posture,
            projected_workload_identity: input.projected_workload_identity,
            open_boundary_digest: input.open_boundary_digest,
            radial_adjacency_digest: input.radial_adjacency_digest,
            transform_posture_identity: input.transform_posture_identity,
            retained_replay_identity: input.retained_replay_identity,
            retained_artifact_identity: input.retained_artifact_identity,
            transformed_workload_identity: input.transformed_workload_identity,
            counters: input.counters,
        }
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn fan_digest(&self) -> &str {
        &self.fan_digest
    }

    pub fn topology_construction_identity(&self) -> &str {
        &self.topology_construction_identity
    }

    pub fn topology_posture(&self) -> &str {
        &self.topology_posture
    }

    pub fn topology_posture_label(&self) -> &'static str {
        match self.topology_posture.as_str() {
            "OpenNonManifold" => "open non-manifold",
            "OpenSheet" => "open sheet",
            "OpenWire" => "open wire",
            "LayeredOpen" => "layered open",
            _ => "unknown open topology",
        }
    }

    pub fn projected_workload_identity(&self) -> &str {
        &self.projected_workload_identity
    }

    pub fn open_boundary_digest(&self) -> &str {
        &self.open_boundary_digest
    }

    pub fn radial_adjacency_digest(&self) -> &str {
        &self.radial_adjacency_digest
    }

    pub fn transform_posture_identity(&self) -> &str {
        &self.transform_posture_identity
    }

    pub fn retained_replay_identity(&self) -> &str {
        &self.retained_replay_identity
    }

    pub fn retained_artifact_identity(&self) -> &str {
        &self.retained_artifact_identity
    }

    pub fn transformed_workload_identity(&self) -> &str {
        &self.transformed_workload_identity
    }

    pub fn counters(&self) -> NmtRadialFanCounters {
        self.counters
    }

    pub fn require_matching_retained_replay(
        &self,
        replay_receipts: &ReplayReceiptSet,
    ) -> Result<(), NmtRadialFanDenial> {
        if replay_receipts.replay_checkpoint_identity() == self.retained_replay_identity
            && replay_receipts.retained_artifact_identity() == self.retained_artifact_identity
            && replay_receipts.transformed_workload_identity() == self.transformed_workload_identity
        {
            Ok(())
        } else {
            Err(NmtRadialFanDenial::ClosedManifoldLaunderingAttempt {
                source_identity: replay_receipts.replay_checkpoint_identity().to_string(),
            })
        }
    }
}
