use topology::facade::{TopologySeedReceipt, TopologyWorkloadReceipt};

use super::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageBinding,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::workload_platform::geometry_binding::GeometryBindingReceiptSet;
use crate::workload_platform::planar_boolean_overlap_region_extraction::CoplanarOverlapOperatorReceipt;
use crate::workload_platform::projection_workload::ProjectionReceiptSet;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;
use crate::workload_platform::surface_support::{
    SurfaceSupportReceiptSet, UnsupportedSurfaceSupportReceipt,
};
use crate::workload_platform::transform_workload::TransformReceiptSet;
use crate::workload_platform::user_response::WorthUserResponseReceipt;
use crate::workload_platform::vocabulary::{
    DiagnosticWorkloadReceipt, GeometryBindingWorkloadReceipt, ProjectionWorkloadReceipt,
    ResponseWorkloadReceipt, RetainedReplayWorkloadReceipt, SurfaceSupportWorkloadReceipt,
    TransformWorkloadReceipt,
};

impl WorkloadEvidenceRow {
    pub(crate) fn from_operator_evidence(
        evidence_identity: impl Into<String>,
        operator_input_count: usize,
        operator_receipt_count: usize,
    ) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Operator,
            evidence_identity,
            WorkloadEvidenceStageCounters::operator(operator_input_count, operator_receipt_count),
        )
    }

    pub fn from_coplanar_overlap_operator_receipt(
        receipt: &CoplanarOverlapOperatorReceipt,
    ) -> Self {
        Self::from_operator_evidence(
            receipt.operator_digest(),
            receipt.operator_input_count(),
            receipt.operator_receipt_count(),
        )
    }

    pub fn from_topology_receipt(receipt: &TopologyWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Topology,
            receipt.identity().name(),
            WorkloadEvidenceStageCounters::default(),
        )
    }

    pub fn from_topology_seed_receipt(receipt: &TopologySeedReceipt) -> Self {
        Self::from_topology_workload_and_seed_receipts(
            receipt.query_receipts().declaration_receipt(),
            receipt,
        )
    }

    pub fn from_topology_workload_and_seed_receipts(
        workload_receipt: &TopologyWorkloadReceipt,
        seed_receipt: &TopologySeedReceipt,
    ) -> Self {
        let counters = seed_receipt.counters();
        Self::receipt_backed(
            WorkloadEvidenceStage::Topology,
            workload_receipt.identity().name(),
            WorkloadEvidenceStageCounters::topology(
                counters.total_topology_entities(),
                counters.face_count(),
                counters.loop_count() + counters.half_edge_count() + counters.edge_count(),
            ),
        )
    }

    pub fn from_geometry_binding_receipt(receipt: &GeometryBindingWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::GeometryBinding,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::default(),
        )
    }

    pub fn from_geometry_binding_receipt_set(receipt: &GeometryBindingReceiptSet) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::GeometryBinding,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceStageCounters::binding(receipt.counters().topology_targets()),
        )
    }

    pub fn from_surface_support_receipt(receipt: &SurfaceSupportWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::SurfaceSupport,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::default(),
        )
    }

    pub fn from_surface_support_receipt_set(receipt: &SurfaceSupportReceiptSet) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::SurfaceSupport,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceStageCounters::surface_support(
                receipt.counters().classified_families(),
            ),
        )
    }

    pub fn from_unsupported_surface_support_receipt(
        receipt: &UnsupportedSurfaceSupportReceipt,
    ) -> Self {
        Self::receipt_backed_with_support(
            WorkloadEvidenceStage::SurfaceSupport,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceSupport::Unsupported,
            WorkloadEvidenceStageCounters::surface_support(
                receipt.counters().classified_families(),
            ),
        )
    }

    pub fn from_projection_receipt(receipt: &ProjectionWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Projection,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::default(),
        )
    }

    pub fn from_projection_receipt_set(receipt: &ProjectionReceiptSet) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Projection,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceStageCounters::projection(
                receipt.counters().projected_topology_entities(),
                receipt.counters().local_basis_parts(),
            ),
        )
    }

    pub fn from_transform_receipt(receipt: &TransformWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Transform,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::default(),
        )
    }

    pub fn from_transform_receipt_set(receipt: &TransformReceiptSet) -> Self {
        Self::receipt_backed_with_stage_binding(
            WorkloadEvidenceStage::Transform,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceStageCounters::transform(
                receipt.counters().transform_steps(),
                receipt.counters().changed_coordinate_rows(),
                receipt.counters().cancellation_steps(),
            ),
            WorkloadEvidenceStageBinding::new(
                WorkloadEvidenceStage::Projection,
                receipt.projected_workload_identity(),
            ),
        )
    }

    pub fn from_retained_replay_receipt(receipt: &RetainedReplayWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::RetainedReplay,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::default(),
        )
    }

    pub fn from_replay_receipt_set(receipt: &ReplayReceiptSet) -> Self {
        Self::receipt_backed_with_stage_binding(
            WorkloadEvidenceStage::RetainedReplay,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceStageCounters::retained_replay(
                receipt.counters().retained_artifact_rows(),
                receipt.counters().replay_rows(),
            ),
            WorkloadEvidenceStageBinding::new(
                WorkloadEvidenceStage::Transform,
                receipt.transformed_workload_identity(),
            ),
        )
    }

    pub fn from_diagnostic_receipt(receipt: &DiagnosticWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Diagnostics,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::diagnostics(1),
        )
    }

    pub fn from_planar_diagnostic_receipt(receipt: &PlanarDiagnosticBundleReceipt) -> Self {
        let counters = receipt.counters();
        Self::receipt_backed(
            WorkloadEvidenceStage::Diagnostics,
            receipt.diagnostic_bundle_digest(),
            WorkloadEvidenceStageCounters::diagnostics(
                counters.source_receipts_inspected()
                    + counters.topology_surfaces_inspected()
                    + counters.causal_references_resolved()
                    + counters.locality_rows_emitted()
                    + counters.denied_evidence_rows(),
            ),
        )
    }

    pub fn from_response_receipt(receipt: &ResponseWorkloadReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Response,
            receipt.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::response(1),
        )
    }

    pub fn from_user_response_receipt(receipt: &WorthUserResponseReceipt) -> Self {
        Self::receipt_backed(
            WorkloadEvidenceStage::Response,
            receipt.stage_identity().receipt_identity(),
            WorkloadEvidenceStageCounters::response(1),
        )
    }
}
