use worth_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisReadyArtifact;
use worth_foundational::FoundationalPerformanceClaimSurface;

use crate::canonical_basis::canonical_basis_domains::validate_store_native_basis_domain;
use crate::{
    StoreCanonicalBasisDomainMismatch, StoreCanonicalBasisFamily,
    StoreCompletedBoundaryReceiptEvidence, StoreDiagnosticSupportReportEvidence,
    StorePerformanceReceiptEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreReadinessHandoffArtifact<PerformanceClaim>
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    canonical_basis: CanonicalBasisReadyArtifact,
    completed_receipts: Vec<StoreCompletedBoundaryReceiptEvidence>,
    diagnostics: Vec<StoreDiagnosticSupportReportEvidence>,
    performance: Vec<StorePerformanceReceiptEvidence<PerformanceClaim>>,
}

impl<PerformanceClaim> StoreReadinessHandoffArtifact<PerformanceClaim>
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    pub fn new(
        canonical_basis: CanonicalBasisReadyArtifact,
        completed_receipts: Vec<StoreCompletedBoundaryReceiptEvidence>,
        diagnostics: Vec<StoreDiagnosticSupportReportEvidence>,
        performance: Vec<StorePerformanceReceiptEvidence<PerformanceClaim>>,
    ) -> Result<Self, StoreReadinessHandoffDenial> {
        validate_store_native_basis_domain(
            StoreCanonicalBasisFamily::ReadinessHandoff,
            &canonical_basis,
        )
        .map_err(StoreReadinessHandoffDenial::CanonicalBasisDomain)?;

        if completed_receipts.is_empty() {
            return Err(StoreReadinessHandoffDenial::MissingBoundaryReceipt);
        }
        if diagnostics.is_empty() {
            return Err(StoreReadinessHandoffDenial::MissingDiagnosticEvidence);
        }
        if performance.is_empty() {
            return Err(StoreReadinessHandoffDenial::MissingPerformanceEvidence);
        }

        Ok(Self {
            canonical_basis,
            completed_receipts,
            diagnostics,
            performance,
        })
    }

    pub const fn canonical_basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.canonical_basis
    }

    pub fn completed_receipts(&self) -> &[StoreCompletedBoundaryReceiptEvidence] {
        &self.completed_receipts
    }

    pub fn diagnostics(&self) -> &[StoreDiagnosticSupportReportEvidence] {
        &self.diagnostics
    }

    pub fn performance(&self) -> &[StorePerformanceReceiptEvidence<PerformanceClaim>] {
        &self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreReadinessHandoffDenial {
    CanonicalBasisDomain(StoreCanonicalBasisDomainMismatch),
    MissingBoundaryReceipt,
    MissingDiagnosticEvidence,
    MissingPerformanceEvidence,
}
