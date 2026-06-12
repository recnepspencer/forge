use topology::facade::{NmtTopologyConstructionReceipt, NmtTopologyPattern, NmtTopologyPosture};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::denial::NmtRadialFanDenial;
use super::receipt::{
    NmtRadialFanCounterInput, NmtRadialFanCounters, NmtRadialFanReceipt, NmtRadialFanReceiptInput,
};
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;
use crate::workload_platform::transform_workload::{
    TransformReceiptSet, UnsupportedTransformWorkload,
};

pub struct NmtRadialFanWorkload<'a> {
    topology_construction: &'a NmtTopologyConstructionReceipt,
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    projected_workload: &'a ProjectedPlanarWorkload,
    transform_receipts: &'a TransformReceiptSet,
    replay_receipts: &'a ReplayReceiptSet,
}

impl<'a> NmtRadialFanWorkload<'a> {
    pub fn from_platform_evidence(
        topology_construction: &'a NmtTopologyConstructionReceipt,
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        projected_workload: &'a ProjectedPlanarWorkload,
        transform_receipts: &'a TransformReceiptSet,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Self {
        Self {
            topology_construction,
            evidence_ledger,
            projected_workload,
            transform_receipts,
            replay_receipts,
        }
    }

    pub fn certify(self) -> Result<NmtRadialFanReceipt, NmtRadialFanDenial> {
        self.require_open_radial_fan()?;
        self.require_open_non_manifold_posture()?;
        self.require_receipt_backed_authority_stages()?;
        self.require_platform_receipt_identity_links()?;
        let counters = self.counters()?;
        Ok(NmtRadialFanReceipt::new(NmtRadialFanReceiptInput {
            workload_identity: self.workload_identity()?,
            topology_construction_identity: self
                .topology_construction
                .pattern_identity()
                .identity_digest()
                .to_string(),
            topology_posture: format!(
                "{:?}",
                self.topology_construction.topology_posture().posture()
            ),
            projected_workload_identity: self
                .projected_workload
                .receipts()
                .stage_identity()
                .receipt_identity(),
            open_boundary_digest: self
                .topology_construction
                .open_boundary()
                .boundary_digest()
                .to_string(),
            radial_adjacency_digest: self
                .topology_construction
                .radial_adjacency()
                .radial_digest()
                .to_string(),
            transform_posture_identity: self
                .transform_receipts
                .transform_posture_receipt()
                .posture_identity()
                .to_string(),
            retained_replay_identity: self
                .replay_receipts
                .replay_checkpoint_identity()
                .to_string(),
            retained_artifact_identity: self
                .replay_receipts
                .retained_artifact_identity()
                .to_string(),
            transformed_workload_identity: self
                .replay_receipts
                .transformed_workload_identity()
                .to_string(),
            counters,
        }))
    }

    pub fn denied_transform_from_platform_evidence(
        topology_construction: &'a NmtTopologyConstructionReceipt,
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        projected_workload: &'a ProjectedPlanarWorkload,
        unsupported_transform: &UnsupportedTransformWorkload,
    ) -> Result<NmtRadialFanDenial, NmtRadialFanDenial> {
        super::denied_transform::denied_transform_from_platform_evidence(
            topology_construction,
            evidence_ledger,
            projected_workload,
            unsupported_transform,
        )
    }

    fn require_open_radial_fan(&self) -> Result<(), NmtRadialFanDenial> {
        match self.topology_construction.pattern() {
            NmtTopologyPattern::OpenRadialFan(spec) => {
                if spec.incident_face_count() < 3 {
                    return Err(NmtRadialFanDenial::InsufficientIncidentFaces {
                        incident_faces: spec.incident_face_count(),
                    });
                }
                Ok(())
            }
            pattern => Err(NmtRadialFanDenial::WrongTopologyPattern {
                pattern_name: nmt_topology_pattern_label(pattern).to_string(),
            }),
        }
    }

    fn require_open_non_manifold_posture(&self) -> Result<(), NmtRadialFanDenial> {
        let posture = self.topology_construction.topology_posture().posture();
        if posture != NmtTopologyPosture::OpenNonManifold {
            return Err(NmtRadialFanDenial::WrongTopologyPosture {
                posture: format!("{posture:?}"),
            });
        }
        if self
            .topology_construction
            .open_boundary()
            .boundary_half_edge_count()
            == 0
        {
            return Err(NmtRadialFanDenial::MissingOpenBoundaryEvidence);
        }
        if self
            .topology_construction
            .radial_adjacency()
            .non_manifold_edge_count()
            == 0
        {
            return Err(NmtRadialFanDenial::MissingRadialAdjacencyEvidence);
        }
        Ok(())
    }

    fn require_receipt_backed_authority_stages(&self) -> Result<(), NmtRadialFanDenial> {
        for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
            self.stage_counters(stage)?;
        }
        Ok(())
    }

    fn require_platform_receipt_identity_links(&self) -> Result<(), NmtRadialFanDenial> {
        self.require_stage_identity(
            WorkloadEvidenceStage::Topology,
            self.topology_construction
                .topology_seed_receipt()
                .query_receipts()
                .declaration_receipt()
                .identity()
                .name(),
            NmtRadialFanDenial::MismatchedTopologyConstructionReceipt,
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::Projection,
            self.projected_workload
                .receipts()
                .stage_identity()
                .receipt_identity()
                .as_str(),
            NmtRadialFanDenial::MismatchedProjectionReceipt,
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::Transform,
            self.transform_receipts
                .stage_identity()
                .receipt_identity()
                .as_str(),
            NmtRadialFanDenial::MismatchedTransformReceipt,
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::RetainedReplay,
            self.replay_receipts
                .stage_identity()
                .receipt_identity()
                .as_str(),
            NmtRadialFanDenial::MismatchedRetainedReplayReceipt,
        )
    }

    fn require_stage_identity(
        &self,
        stage: WorkloadEvidenceStage,
        expected_identity: &str,
        denial: NmtRadialFanDenial,
    ) -> Result<(), NmtRadialFanDenial> {
        let row = self
            .evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .ok_or(NmtRadialFanDenial::MissingReceiptBackedStage(stage))?;
        if row.evidence_identity() == expected_identity {
            Ok(())
        } else {
            Err(denial)
        }
    }

    fn counters(&self) -> Result<NmtRadialFanCounters, NmtRadialFanDenial> {
        let topology = self.stage_counters(WorkloadEvidenceStage::Topology)?;
        let projection = self.stage_counters(WorkloadEvidenceStage::Projection)?;
        let transform = self.stage_counters(WorkloadEvidenceStage::Transform)?;
        let replay = self.stage_counters(WorkloadEvidenceStage::RetainedReplay)?;
        let diagnostics = self.stage_counters(WorkloadEvidenceStage::Diagnostics)?;
        let response = self.stage_counters(WorkloadEvidenceStage::Response)?;
        let construction = self.topology_construction.counters();

        if topology.topology_face_count() < construction.face_count() {
            return Err(NmtRadialFanDenial::MissingTopologyEvidence);
        }
        if projection.projected_entity_count() < construction.face_count()
            || projection.local_basis_part_count() == 0
        {
            return Err(NmtRadialFanDenial::MissingProjectionEvidence);
        }
        if transform.transform_step_count() == 0 {
            return Err(NmtRadialFanDenial::MissingTransformEvidence);
        }
        if transform.transform_changed_coordinate_count() == 0
            || self.transform_receipts.counters().changed_coordinate_rows() == 0
        {
            return Err(NmtRadialFanDenial::LabelOnlyMotion);
        }
        if replay.retained_artifact_count() == 0
            || replay.replay_checkpoint_count() == 0
            || self.replay_receipts.counters().replay_rows() == 0
        {
            return Err(NmtRadialFanDenial::MissingRetainedReplayEvidence);
        }

        Ok(NmtRadialFanCounters::new(NmtRadialFanCounterInput {
            incident_face_count: construction.face_count(),
            open_boundary_half_edge_count: self
                .topology_construction
                .open_boundary()
                .boundary_half_edge_count(),
            non_manifold_edge_count: self
                .topology_construction
                .radial_adjacency()
                .non_manifold_edge_count(),
            topology,
            projection,
            transform,
            replay,
            diagnostics,
            response,
        }))
    }

    fn stage_counters(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<WorkloadEvidenceStageCounters, NmtRadialFanDenial> {
        self.evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .map(|row| row.counters())
            .ok_or(NmtRadialFanDenial::MissingReceiptBackedStage(stage))
    }

    fn workload_identity(&self) -> Result<String, NmtRadialFanDenial> {
        let mut parts = Vec::new();
        for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
            let row = self
                .evidence_ledger
                .row_for_stage(stage)
                .filter(|row| row.is_receipt_backed() && row.is_admitted())
                .ok_or(NmtRadialFanDenial::MissingReceiptBackedStage(stage))?;
            parts.push(format!("{stage:?}:{}", row.evidence_identity()));
        }
        Ok(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-open-radial-fan-platform-ledger".to_string(),
                parts.join("|"),
                self.transform_receipts
                    .transform_posture_receipt()
                    .posture_identity()
                    .to_string(),
                self.replay_receipts
                    .replay_checkpoint_identity()
                    .to_string(),
            ],
        ))
    }
}

fn nmt_topology_pattern_label(pattern: &NmtTopologyPattern) -> &'static str {
    match pattern {
        NmtTopologyPattern::OpenWireChain(_) => "open wire chain topology",
        NmtTopologyPattern::OpenSheetPatch(_) => "open sheet patch topology",
        NmtTopologyPattern::OpenRadialFan(_) => "open radial fan topology",
        NmtTopologyPattern::OpenLayerStack(_) => "open layer stack topology",
    }
}
