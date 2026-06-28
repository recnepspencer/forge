use forge_foundational::{
    AspectValue, CurrentBasisBoundaryBundle, FoundationalBoundaryMaterializationBundle,
    FoundationalCertifiedDiagnosticBundle, FoundationalDiagnosticSupportReport,
};

use super::super::denial::RecoveryEvidenceDenial;
use super::super::diagnostics::RecoverySourceDecisionReport;
use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::super::lineage_provenance::RecoveryEvidenceLineageReport;
use super::super::performance::RecoveryCounterPerformanceReceipt;
use super::super::proof_progression::ProofProgressionRecoveryTrace;
use super::bundle_materialization::materialize_bundle;
use super::canonical_basis::RecoveryEvidenceCanonicalBasis;
use super::current_basis::RecoveryCurrentBasisEvidence;
use super::diagnostic_certification::{
    certify_diagnostic_support_bundle, readmit_diagnostic_support_bundle,
};
use super::receipt::RecoveryPhysicsReceipt;
use super::report::RecoveryPhysicsReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEvidenceBundlePrimary {
    recovered_physical_root: String,
    exact_counter_assertions: usize,
}

impl RecoveryEvidenceBundlePrimary {
    pub(crate) fn from_members(
        receipt: &RecoveryPhysicsReceipt,
        performance: &RecoveryCounterPerformanceReceipt,
    ) -> Self {
        Self {
            recovered_physical_root: receipt.recovered_physical_root().to_string(),
            exact_counter_assertions: performance.exact_counter_assertions(),
        }
    }

    pub fn recovered_physical_root(&self) -> &str {
        &self.recovered_physical_root
    }

    pub const fn exact_counter_assertions(&self) -> usize {
        self.exact_counter_assertions
    }
}

pub type MaterializedFoundationalRecoveryEvidenceBundle =
    FoundationalBoundaryMaterializationBundle<RecoveryEvidenceBundlePrimary, AspectValue>;
pub type RecoveryCurrentBasisBoundaryBundle =
    CurrentBasisBoundaryBundle<RecoveryEvidenceBundlePrimary, AspectValue>;
pub type RecoveryCertifiedDiagnosticSupportBundle = FoundationalCertifiedDiagnosticBundle<
    RecoveryCurrentBasisBoundaryBundle,
    FoundationalDiagnosticSupportReport,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalRecoveryEvidenceBundle {
    receipt: RecoveryPhysicsReceipt,
    report: RecoveryPhysicsReport,
    performance: RecoveryCounterPerformanceReceipt,
    source_decisions: RecoverySourceDecisionReport,
    canonical_basis: RecoveryEvidenceCanonicalBasis,
    current_basis: RecoveryCurrentBasisEvidence,
    lineage: RecoveryEvidenceLineageReport,
    proof_trace: ProofProgressionRecoveryTrace,
    materialized: MaterializedFoundationalRecoveryEvidenceBundle,
}

impl FoundationalRecoveryEvidenceBundle {
    pub fn from_source(
        source: &RecoveryPhysicsEvidenceSource,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        let receipt = RecoveryPhysicsReceipt::from_executed_source(source);
        let report = RecoveryPhysicsReport::from_executed_source(source);
        let performance = RecoveryCounterPerformanceReceipt::from_source(source);
        let source_decisions = RecoverySourceDecisionReport::from_source(source);
        let lineage = RecoveryEvidenceLineageReport::from_source(source);
        let proof_trace = ProofProgressionRecoveryTrace::from_source(source);
        let materialized = materialize_bundle(&receipt, &report, &performance);
        let canonical_basis = RecoveryEvidenceCanonicalBasis::full_from_evidence_surfaces(
            &materialized,
            &source_decisions,
            &performance,
        )?;
        let current_basis = RecoveryCurrentBasisEvidence::from_materialized_bundle(
            &materialized,
            receipt.handle(),
            receipt.epoch(),
        )?;
        Ok(Self {
            receipt,
            report,
            performance,
            source_decisions,
            canonical_basis,
            current_basis,
            lineage,
            proof_trace,
            materialized,
        })
    }

    pub const fn receipt(&self) -> &RecoveryPhysicsReceipt {
        &self.receipt
    }

    pub const fn report(&self) -> &RecoveryPhysicsReport {
        &self.report
    }

    pub const fn performance(&self) -> &RecoveryCounterPerformanceReceipt {
        &self.performance
    }

    pub const fn source_decisions(&self) -> &RecoverySourceDecisionReport {
        &self.source_decisions
    }

    pub const fn canonical_basis(&self) -> &RecoveryEvidenceCanonicalBasis {
        &self.canonical_basis
    }

    pub const fn current_basis(&self) -> &RecoveryCurrentBasisEvidence {
        &self.current_basis
    }

    pub const fn lineage(&self) -> &RecoveryEvidenceLineageReport {
        &self.lineage
    }

    pub const fn proof_trace(&self) -> &ProofProgressionRecoveryTrace {
        &self.proof_trace
    }

    pub const fn materialized(&self) -> &MaterializedFoundationalRecoveryEvidenceBundle {
        &self.materialized
    }

    pub fn certified_diagnostic_support_bundle(
        &self,
    ) -> Result<RecoveryCertifiedDiagnosticSupportBundle, RecoveryEvidenceDenial> {
        certify_diagnostic_support_bundle(&self.materialized, &self.source_decisions)
    }

    pub fn readmitted_diagnostic_support_bundle(
        &self,
    ) -> Result<RecoveryCertifiedDiagnosticSupportBundle, RecoveryEvidenceDenial> {
        readmit_diagnostic_support_bundle(self.certified_diagnostic_support_bundle()?)
    }
}
