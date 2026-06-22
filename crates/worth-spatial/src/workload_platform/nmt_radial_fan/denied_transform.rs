use topology::facade::{NmtTopologyConstructionReceipt, NmtTopologyPattern, NmtTopologyPosture};

use super::denial::NmtRadialFanDenial;
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::transform_workload::{
    UnsupportedTransformReasonCode, UnsupportedTransformWorkload,
};

pub(super) fn denied_transform_from_platform_evidence(
    topology_construction: &NmtTopologyConstructionReceipt,
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
    projected_workload: &ProjectedPlanarWorkload,
    unsupported_transform: &UnsupportedTransformWorkload,
) -> Result<NmtRadialFanDenial, NmtRadialFanDenial> {
    let verifier =
        DeniedTransformVerifier::new(topology_construction, evidence_ledger, projected_workload);
    verifier.require_open_radial_fan()?;
    verifier.require_open_non_manifold_posture()?;
    verifier.require_receipt_backed_stage_identity(
        WorkloadEvidenceStage::Topology,
        topology_construction
            .topology_seed_receipt()
            .query_receipts()
            .declaration_receipt()
            .identity()
            .name(),
        NmtRadialFanDenial::MismatchedTopologyConstructionReceipt,
    )?;
    verifier.require_receipt_backed_stage_identity(
        WorkloadEvidenceStage::Projection,
        projected_workload
            .receipts()
            .stage_identity()
            .receipt_identity()
            .as_str(),
        NmtRadialFanDenial::MismatchedProjectionReceipt,
    )?;
    match unsupported_transform.reason_code() {
        UnsupportedTransformReasonCode::LabelOnlyMotionEvidence => {
            Ok(NmtRadialFanDenial::LabelOnlyMotion)
        }
        _ => Err(NmtRadialFanDenial::MissingTransformEvidence),
    }
}

struct DeniedTransformVerifier<'a> {
    topology_construction: &'a NmtTopologyConstructionReceipt,
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
}

impl<'a> DeniedTransformVerifier<'a> {
    fn new(
        topology_construction: &'a NmtTopologyConstructionReceipt,
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        _projected_workload: &'a ProjectedPlanarWorkload,
    ) -> Self {
        Self {
            topology_construction,
            evidence_ledger,
        }
    }

    fn require_open_radial_fan(&self) -> Result<(), NmtRadialFanDenial> {
        match self.topology_construction.pattern() {
            NmtTopologyPattern::OpenRadialFan(spec) if spec.incident_face_count() >= 3 => Ok(()),
            NmtTopologyPattern::OpenRadialFan(spec) => {
                Err(NmtRadialFanDenial::InsufficientIncidentFaces {
                    incident_faces: spec.incident_face_count(),
                })
            }
            pattern => Err(NmtRadialFanDenial::WrongTopologyPattern {
                pattern_name: nmt_topology_pattern_label(pattern).to_string(),
            }),
        }
    }

    fn require_open_non_manifold_posture(&self) -> Result<(), NmtRadialFanDenial> {
        if self.topology_construction.topology_posture().posture()
            != NmtTopologyPosture::OpenNonManifold
        {
            return Err(NmtRadialFanDenial::WrongTopologyPosture {
                posture: format!(
                    "{:?}",
                    self.topology_construction.topology_posture().posture()
                ),
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

    fn require_receipt_backed_stage_identity(
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
}

fn nmt_topology_pattern_label(pattern: &NmtTopologyPattern) -> &'static str {
    match pattern {
        NmtTopologyPattern::OpenWireChain(_) => "open wire chain topology",
        NmtTopologyPattern::OpenSheetPatch(_) => "open sheet patch topology",
        NmtTopologyPattern::OpenRadialFan(_) => "open radial fan topology",
        NmtTopologyPattern::OpenLayerStack(_) => "open layer stack topology",
    }
}
