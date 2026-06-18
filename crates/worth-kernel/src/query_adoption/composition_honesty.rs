use forge_query::facade::consumer_kit::{
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportScope,
    ForgeQueryBoundaryAuditError, ForgeQuerySupportPinningError,
};
use source_status::WorthKernelCompositionSourceStatus;
use topology::facade::WorthTopoQueryConsumerKitAdoptionError;
use workload_snapshot::WorthKernelRepresentativeWorkloadSnapshot;
use worth_spatial::facade::query_adoption::WorthSpatialQueryConsumerKitAdoptionError;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

use crate::workload_composition::WorkloadCatalogError;

mod source_status;
mod workload_snapshot;
pub use source_status::WorthKernelCompositionSourceKind;

const LOWER_CRATE_RECEIPT_FAMILY_COUNT: usize = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthKernelCompositionHonestyReport {
    evidence_report_identity: String,
    digest_participation_identity: String,
    kernel_composition_source_count: usize,
    lower_crate_receipt_family_count: usize,
    kernel_workload_receipt_family_count: usize,
    spatial_workload_support_pin_row_count: usize,
    representative_workload_evidence_row_count: usize,
    representative_spatial_receipt_identity_count: usize,
}

impl WorthKernelCompositionHonestyReport {
    pub fn evidence_report_identity(&self) -> &str {
        &self.evidence_report_identity
    }

    pub fn digest_participation_identity(&self) -> &str {
        &self.digest_participation_identity
    }

    pub const fn kernel_composition_source_count(&self) -> usize {
        self.kernel_composition_source_count
    }

    pub const fn lower_crate_receipt_family_count(&self) -> usize {
        self.lower_crate_receipt_family_count
    }

    pub const fn kernel_workload_receipt_family_count(&self) -> usize {
        self.kernel_workload_receipt_family_count
    }

    pub const fn spatial_workload_support_pin_row_count(&self) -> usize {
        self.spatial_workload_support_pin_row_count
    }

    pub const fn representative_workload_evidence_row_count(&self) -> usize {
        self.representative_workload_evidence_row_count
    }

    pub const fn representative_spatial_receipt_identity_count(&self) -> usize {
        self.representative_spatial_receipt_identity_count
    }

    fn from_sources(
        sources: Vec<WorthKernelCompositionSourceStatus>,
    ) -> Result<Self, WorthKernelCompositionHonestyError> {
        let workload_snapshot = WorthKernelRepresentativeWorkloadSnapshot::current()?;
        Self::from_sources_and_workload_snapshot(sources, workload_snapshot)
    }

    fn from_sources_and_workload_snapshot(
        sources: Vec<WorthKernelCompositionSourceStatus>,
        workload_snapshot: WorthKernelRepresentativeWorkloadSnapshot,
    ) -> Result<Self, WorthKernelCompositionHonestyError> {
        validate_sources(&sources)?;
        validate_representative_workload_snapshot(&workload_snapshot)?;
        let spatial_workload_support_pin_row_count = sources
            .iter()
            .find(|source| source.kind == WorthKernelCompositionSourceKind::Spatial)
            .map(|source| source.workload_support_pin_row_count)
            .unwrap_or_default();
        let kernel_workload_receipt_family_count = WorkloadEvidenceStage::AUTHORITY_STAGES.len();
        let evidence_report = composition_evidence_report(
            &sources,
            &workload_snapshot,
            spatial_workload_support_pin_row_count,
            kernel_workload_receipt_family_count,
        )
        .map_err(WorthKernelCompositionHonestyError::EvidenceReport)?;

        Ok(Self {
            evidence_report_identity: evidence_report
                .report_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            digest_participation_identity: evidence_report
                .digest_participation_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            kernel_composition_source_count: sources.len(),
            lower_crate_receipt_family_count: LOWER_CRATE_RECEIPT_FAMILY_COUNT,
            kernel_workload_receipt_family_count,
            spatial_workload_support_pin_row_count,
            representative_workload_evidence_row_count: workload_snapshot.evidence_row_count(),
            representative_spatial_receipt_identity_count: workload_snapshot
                .spatial_receipt_identities()
                .len(),
        })
    }
}

#[derive(Debug)]
pub enum WorthKernelCompositionHonestyError {
    KernelSupportPinning(ForgeQuerySupportPinningError),
    KernelBoundaryAudit(ForgeQueryBoundaryAuditError),
    TopologyAdoption(WorthTopoQueryConsumerKitAdoptionError),
    SpatialAdoption(WorthSpatialQueryConsumerKitAdoptionError),
    RepresentativeWorkload(WorkloadCatalogError),
    EvidenceReport(EvidenceReportError),
    MissingKernelReceipts,
    MissingTopologyReceipts,
    MissingSpatialReceipts,
    DuplicateCompositionSource(WorthKernelCompositionSourceKind),
    StaleSupportPins {
        source: WorthKernelCompositionSourceKind,
        blocking_finding_count: usize,
    },
    ForgedEvidenceReport {
        source: WorthKernelCompositionSourceKind,
        field: &'static str,
    },
    DirtyHardProhibitionBoundary(WorthKernelCompositionSourceKind),
}

pub fn current_kernel_composition_honesty_report(
) -> Result<WorthKernelCompositionHonestyReport, WorthKernelCompositionHonestyError> {
    WorthKernelCompositionHonestyReport::from_sources(WorthKernelCompositionSourceStatus::current()?)
}

fn validate_sources(
    sources: &[WorthKernelCompositionSourceStatus],
) -> Result<(), WorthKernelCompositionHonestyError> {
    require_single_source(sources, WorthKernelCompositionSourceKind::Kernel)?;
    require_single_source(sources, WorthKernelCompositionSourceKind::Topology)?;
    let spatial = require_single_source(sources, WorthKernelCompositionSourceKind::Spatial)?;

    if spatial.workload_support_pin_row_count == 0 {
        return Err(WorthKernelCompositionHonestyError::MissingSpatialReceipts);
    }

    for source in sources {
        if source.support_blocking_finding_count > 0 {
            return Err(WorthKernelCompositionHonestyError::StaleSupportPins {
                source: source.kind,
                blocking_finding_count: source.support_blocking_finding_count,
            });
        }
        if !source.hard_prohibition_audit_clean {
            return Err(
                WorthKernelCompositionHonestyError::DirtyHardProhibitionBoundary(source.kind),
            );
        }
        require_identity_field(
            source,
            source.support_pin_report_digest.as_str(),
            "support_pin_report_digest",
        )?;
        require_identity_field(
            source,
            source.evidence_report_identity.as_str(),
            "evidence_report_identity",
        )?;
        require_identity_field(
            source,
            source.evidence_digest_participation_identity.as_str(),
            "evidence_digest_participation_identity",
        )?;
        require_identity_field(
            source,
            source.boundary_audit_report_identity.as_str(),
            "boundary_audit_report_identity",
        )?;
    }

    Ok(())
}

fn require_single_source(
    sources: &[WorthKernelCompositionSourceStatus],
    kind: WorthKernelCompositionSourceKind,
) -> Result<&WorthKernelCompositionSourceStatus, WorthKernelCompositionHonestyError> {
    let mut matching = sources.iter().filter(|source| source.kind == kind);
    let source = matching.next().ok_or_else(|| missing_source_error(kind))?;
    if matching.next().is_some() {
        return Err(WorthKernelCompositionHonestyError::DuplicateCompositionSource(kind));
    }
    Ok(source)
}

const fn missing_source_error(
    kind: WorthKernelCompositionSourceKind,
) -> WorthKernelCompositionHonestyError {
    match kind {
        WorthKernelCompositionSourceKind::Kernel => {
            WorthKernelCompositionHonestyError::MissingKernelReceipts
        }
        WorthKernelCompositionSourceKind::Topology => {
            WorthKernelCompositionHonestyError::MissingTopologyReceipts
        }
        WorthKernelCompositionSourceKind::Spatial => {
            WorthKernelCompositionHonestyError::MissingSpatialReceipts
        }
    }
}

fn validate_representative_workload_snapshot(
    snapshot: &WorthKernelRepresentativeWorkloadSnapshot,
) -> Result<(), WorthKernelCompositionHonestyError> {
    require_snapshot_identity_field(
        snapshot.topology_receipt_identity(),
        "topology_receipt_identity",
    )?;
    if snapshot.spatial_receipt_identities().len()
        != WorkloadEvidenceStage::AUTHORITY_STAGES.len() - 1
    {
        return Err(WorthKernelCompositionHonestyError::MissingSpatialReceipts);
    }
    if snapshot.evidence_row_count() != WorkloadEvidenceStage::AUTHORITY_STAGES.len() {
        return Err(WorthKernelCompositionHonestyError::MissingSpatialReceipts);
    }
    for identity in snapshot.spatial_receipt_identities() {
        require_snapshot_identity_field(identity, "spatial_receipt_identity")?;
    }
    Ok(())
}

fn require_snapshot_identity_field(
    value: &str,
    field: &'static str,
) -> Result<(), WorthKernelCompositionHonestyError> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() || normalized.contains("forged") || normalized.contains("synthetic") {
        return Err(WorthKernelCompositionHonestyError::ForgedEvidenceReport {
            source: WorthKernelCompositionSourceKind::Kernel,
            field,
        });
    }
    Ok(())
}

fn require_identity_field(
    source: &WorthKernelCompositionSourceStatus,
    value: &str,
    field: &'static str,
) -> Result<(), WorthKernelCompositionHonestyError> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() || normalized.contains("forged") || normalized.contains("synthetic") {
        return Err(WorthKernelCompositionHonestyError::ForgedEvidenceReport {
            source: source.kind,
            field,
        });
    }
    Ok(())
}

fn composition_evidence_report(
    sources: &[WorthKernelCompositionSourceStatus],
    workload_snapshot: &WorthKernelRepresentativeWorkloadSnapshot,
    spatial_workload_support_pin_row_count: usize,
    kernel_workload_receipt_family_count: usize,
) -> Result<EvidenceReport, EvidenceReportError> {
    let source_fingerprints = sources.iter().map(|source| source.fingerprint());

    EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-kernel.query-adoption.phase-seven")?,
        "worth-kernel-composition-honesty",
    )?
    .shape_participating("crate", "worth-kernel")?
    .value_sequence_participating("composition_sources", source_fingerprints)?
    .value_participating(
        "representative_topology_receipt_identity",
        workload_snapshot.topology_receipt_identity(),
    )?
    .value_sequence_participating(
        "representative_spatial_receipt_identities",
        workload_snapshot.spatial_receipt_identities().iter().cloned(),
    )?
    .usize_participating("kernel_composition_source_count", sources.len())?
    .usize_participating(
        "lower_crate_receipt_family_count",
        LOWER_CRATE_RECEIPT_FAMILY_COUNT,
    )?
    .usize_participating(
        "kernel_workload_receipt_family_count",
        kernel_workload_receipt_family_count,
    )?
    .usize_participating(
        "spatial_workload_support_pin_rows",
        spatial_workload_support_pin_row_count,
    )?
    .usize_participating(
        "representative_workload_evidence_rows",
        workload_snapshot.evidence_row_count(),
    )?
    .bool_participating("kernel_is_composition_only", true)?
    .diagnostic_value_nonparticipating(
        "authority_boundary",
        "worth-kernel composes lower-crate receipts and Query evidence; topology and spatial truth stay crate-owned",
    )?
    .seal()
}

#[cfg(test)]
mod tests;
