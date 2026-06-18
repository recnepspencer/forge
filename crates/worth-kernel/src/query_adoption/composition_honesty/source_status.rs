use topology::facade::{
    current_topology_query_consumer_kit_adoption_status, WorthTopoQueryConsumerKitAdoptionStatus,
};
use worth_spatial::facade::query_adoption::{
    current_spatial_query_consumer_kit_adoption_status, WorthSpatialQueryConsumerKitAdoptionStatus,
};

use super::WorthKernelCompositionHonestyError;
use crate::query_adoption::boundary_audit::kernel_query_hard_prohibition_boundary_audit;
use crate::query_adoption::evidence_reports::kernel_query_adoption_evidence_report;
use crate::query_adoption::support_pins::evaluate_current_kernel_support_pins;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthKernelCompositionSourceKind {
    Kernel,
    Topology,
    Spatial,
}

impl WorthKernelCompositionSourceKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Kernel => "worth-kernel",
            Self::Topology => "worth-topo",
            Self::Spatial => "worth-spatial",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthKernelCompositionSourceStatus {
    pub(super) kind: WorthKernelCompositionSourceKind,
    pub(super) support_pin_report_digest: String,
    pub(super) support_blocking_finding_count: usize,
    pub(super) evidence_report_identity: String,
    pub(super) evidence_digest_participation_identity: String,
    pub(super) boundary_audit_report_identity: String,
    pub(super) hard_prohibition_audit_clean: bool,
    pub(super) workload_support_pin_row_count: usize,
}

impl WorthKernelCompositionSourceStatus {
    pub(super) fn current() -> Result<Vec<Self>, WorthKernelCompositionHonestyError> {
        let topology = current_topology_query_consumer_kit_adoption_status()
            .map_err(WorthKernelCompositionHonestyError::TopologyAdoption)?;
        let spatial = current_spatial_query_consumer_kit_adoption_status()
            .map_err(WorthKernelCompositionHonestyError::SpatialAdoption)?;

        Ok(vec![
            Self::kernel()?,
            Self::topology(topology),
            Self::spatial(spatial),
        ])
    }

    pub(super) fn kernel() -> Result<Self, WorthKernelCompositionHonestyError> {
        let support_report = evaluate_current_kernel_support_pins()
            .map_err(WorthKernelCompositionHonestyError::KernelSupportPinning)?;
        support_report
            .assert_satisfied()
            .map_err(WorthKernelCompositionHonestyError::KernelSupportPinning)?;
        let boundary_report = kernel_query_hard_prohibition_boundary_audit()
            .map_err(WorthKernelCompositionHonestyError::KernelBoundaryAudit)?;
        boundary_report.assert_clean();
        let evidence_report =
            kernel_query_adoption_evidence_report(&support_report, &boundary_report)
                .map_err(WorthKernelCompositionHonestyError::EvidenceReport)?;

        Ok(Self {
            kind: WorthKernelCompositionSourceKind::Kernel,
            support_pin_report_digest: support_report.report_digest().to_string(),
            support_blocking_finding_count: support_report.blocking_finding_count(),
            evidence_report_identity: evidence_report
                .report_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            evidence_digest_participation_identity: evidence_report
                .digest_participation_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            boundary_audit_report_identity: boundary_report
                .report_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            hard_prohibition_audit_clean: boundary_report.is_clean(),
            workload_support_pin_row_count: 0,
        })
    }

    fn topology(status: WorthTopoQueryConsumerKitAdoptionStatus) -> Self {
        Self {
            kind: WorthKernelCompositionSourceKind::Topology,
            support_pin_report_digest: status.support_pin_report_digest().to_string(),
            support_blocking_finding_count: status.support_blocking_finding_count(),
            evidence_report_identity: status.evidence_report_identity().to_string(),
            evidence_digest_participation_identity: status
                .evidence_digest_participation_identity()
                .to_string(),
            boundary_audit_report_identity: status.boundary_audit_report_identity().to_string(),
            hard_prohibition_audit_clean: status.hard_prohibition_audit_clean(),
            workload_support_pin_row_count: 0,
        }
    }

    fn spatial(status: WorthSpatialQueryConsumerKitAdoptionStatus) -> Self {
        Self {
            kind: WorthKernelCompositionSourceKind::Spatial,
            support_pin_report_digest: status.support_pin_report_digest().to_string(),
            support_blocking_finding_count: status.support_blocking_finding_count(),
            evidence_report_identity: status.evidence_report_identity().to_string(),
            evidence_digest_participation_identity: status
                .evidence_digest_participation_identity()
                .to_string(),
            boundary_audit_report_identity: status.boundary_audit_report_identity().to_string(),
            hard_prohibition_audit_clean: status.hard_prohibition_audit_clean(),
            workload_support_pin_row_count: status.workload_support_pin_row_count(),
        }
    }

    pub(super) fn fingerprint(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}",
            self.kind.label(),
            self.support_pin_report_digest,
            self.support_blocking_finding_count,
            self.evidence_report_identity,
            self.evidence_digest_participation_identity,
            self.boundary_audit_report_identity,
            self.workload_support_pin_row_count
        )
    }
}
