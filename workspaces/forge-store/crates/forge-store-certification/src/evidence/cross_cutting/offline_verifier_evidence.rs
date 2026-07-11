use crate::{
    OfflineVerifierObserver, PhysicalLayoutParityDenial, PhysicalLayoutParityReport,
    PhysicalSubstrateLane, RuntimeVerifierComparisonClassification,
    RuntimeVerifierComparisonDenial, RuntimeVerifierComparisonReport,
};
use forge_store_physical_format::{
    ExtentRecordDenialKind, MinimalManifestVerifierReport, OfflineVerifierDenial,
    OfflineVerifierDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOfflineVerifierEvidenceRow {
    MinimalManifestSmoke,
    RuntimeLayoutMatch,
    RuntimeDisagreementReported,
}

impl PhysicalOfflineVerifierEvidenceRow {
    pub const fn physical_format_required() -> [Self; 3] {
        [
            Self::MinimalManifestSmoke,
            Self::RuntimeLayoutMatch,
            Self::RuntimeDisagreementReported,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        PhysicalSubstrateLane::OfflineVerifier
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalOfflineVerifierEvidenceReport {
    row: PhysicalOfflineVerifierEvidenceRow,
    lane: PhysicalSubstrateLane,
    observed_reference_count: u32,
    semantic_decode_attempts: u32,
    comparison: Option<RuntimeVerifierComparisonClassification>,
}

impl PhysicalOfflineVerifierEvidenceReport {
    pub fn from_verifier_report(
        row: PhysicalOfflineVerifierEvidenceRow,
        report: &MinimalManifestVerifierReport,
    ) -> Result<Self, PhysicalOfflineVerifierEvidenceDenial> {
        if row != PhysicalOfflineVerifierEvidenceRow::MinimalManifestSmoke {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedReportRow(
                row,
            ));
        }
        if report.semantic_decode_attempts() != 0 {
            return Err(PhysicalOfflineVerifierEvidenceDenial::SemanticDecodeAttempted);
        }
        Ok(Self::new(
            row,
            report.layout().discovered_references().len() as u32,
            report.semantic_decode_attempts(),
        ))
    }

    pub fn from_parity_report(
        row: PhysicalOfflineVerifierEvidenceRow,
        report: PhysicalLayoutParityReport,
    ) -> Result<Self, PhysicalOfflineVerifierEvidenceDenial> {
        if row != PhysicalOfflineVerifierEvidenceRow::RuntimeLayoutMatch {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedReportRow(
                row,
            ));
        }
        Ok(Self::new(row, report.compared_references(), 0))
    }

    pub fn from_runtime_verifier_comparison(
        row: PhysicalOfflineVerifierEvidenceRow,
        report: &RuntimeVerifierComparisonReport,
    ) -> Result<Self, PhysicalOfflineVerifierEvidenceDenial> {
        if row != PhysicalOfflineVerifierEvidenceRow::RuntimeLayoutMatch {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedReportRow(
                row,
            ));
        }
        if report.classification() != RuntimeVerifierComparisonClassification::Equivalent {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedComparison(
                report.classification(),
            ));
        }
        Ok(Self::new_with_comparison(
            row,
            report.compared_references(),
            report.runtime_semantic_decode_attempts() + report.offline_semantic_decode_attempts(),
            Some(report.classification()),
        ))
    }

    pub fn from_parity_denial(
        row: PhysicalOfflineVerifierEvidenceRow,
        denial: PhysicalLayoutParityDenial,
    ) -> Result<Self, PhysicalOfflineVerifierEvidenceDenial> {
        if row != PhysicalOfflineVerifierEvidenceRow::RuntimeDisagreementReported {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedDenialRow(
                row,
            ));
        }
        Ok(Self::new(row, denial.offline_reference_count(), 0))
    }

    pub fn from_runtime_verifier_mismatch(
        row: PhysicalOfflineVerifierEvidenceRow,
        denial: &RuntimeVerifierComparisonDenial,
    ) -> Result<Self, PhysicalOfflineVerifierEvidenceDenial> {
        if row != PhysicalOfflineVerifierEvidenceRow::RuntimeDisagreementReported {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedDenialRow(
                row,
            ));
        }
        Ok(Self::new_with_comparison(
            row,
            denial.report().offline_reference_count(),
            denial.report().runtime_semantic_decode_attempts()
                + denial.report().offline_semantic_decode_attempts(),
            Some(denial.classification()),
        ))
    }

    pub fn from_verifier_denial(
        row: PhysicalOfflineVerifierEvidenceRow,
        denial: OfflineVerifierDenial,
    ) -> Result<Self, PhysicalOfflineVerifierEvidenceDenial> {
        if !denial_is_evidence(&denial) {
            return Err(PhysicalOfflineVerifierEvidenceDenial::UnexpectedVerifierDenial);
        }
        Ok(Self::new(
            row,
            0,
            denial.counters().semantic_decode_attempts(),
        ))
    }

    pub const fn row(&self) -> PhysicalOfflineVerifierEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn observed_reference_count(&self) -> u32 {
        self.observed_reference_count
    }

    pub const fn semantic_decode_attempts(&self) -> u32 {
        self.semantic_decode_attempts
    }

    pub const fn comparison(&self) -> Option<RuntimeVerifierComparisonClassification> {
        self.comparison
    }

    const fn new(
        row: PhysicalOfflineVerifierEvidenceRow,
        observed_reference_count: u32,
        semantic_decode_attempts: u32,
    ) -> Self {
        Self::new_with_comparison(
            row,
            observed_reference_count,
            semantic_decode_attempts,
            None,
        )
    }

    const fn new_with_comparison(
        row: PhysicalOfflineVerifierEvidenceRow,
        observed_reference_count: u32,
        semantic_decode_attempts: u32,
        comparison: Option<RuntimeVerifierComparisonClassification>,
    ) -> Self {
        Self {
            row,
            lane: row.physical_substrate_lane(),
            observed_reference_count,
            semantic_decode_attempts,
            comparison,
        }
    }
}

fn denial_is_evidence(denial: &OfflineVerifierDenial) -> bool {
    match denial.kind() {
        OfflineVerifierDenialKind::MissingRootManifest
        | OfflineVerifierDenialKind::AmbiguousRootManifest
        | OfflineVerifierDenialKind::BackendResidueDiscoverySource
        | OfflineVerifierDenialKind::MalformedManifestMembership
        | OfflineVerifierDenialKind::HeaderDecodeDenied => true,
        OfflineVerifierDenialKind::ExtentRecordDenied => {
            extent_record_denial_is_header_decode(denial)
        }
        _ => false,
    }
}

fn extent_record_denial_is_header_decode(denial: &OfflineVerifierDenial) -> bool {
    match denial.extent_denial() {
        Some(extent_denial) => {
            extent_denial.kind() == ExtentRecordDenialKind::HeaderDecodeDenied
                && extent_denial.header_denial().is_some()
        }
        None => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOfflineVerifierEvidenceDenial {
    UnexpectedReportRow(PhysicalOfflineVerifierEvidenceRow),
    UnexpectedDenialRow(PhysicalOfflineVerifierEvidenceRow),
    SemanticDecodeAttempted,
    UnexpectedVerifierDenial,
    EmptyOfflineObservation,
    UnexpectedComparison(RuntimeVerifierComparisonClassification),
}

pub fn offline_observer_requires_physical_references(
    observer: &OfflineVerifierObserver,
) -> Result<(), PhysicalOfflineVerifierEvidenceDenial> {
    if observer.discovered_references().is_empty() {
        return Err(PhysicalOfflineVerifierEvidenceDenial::EmptyOfflineObservation);
    }
    Ok(())
}
