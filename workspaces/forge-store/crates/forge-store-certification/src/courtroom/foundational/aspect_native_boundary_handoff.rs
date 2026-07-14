use forge_foundational::FoundationalPerformanceClaimSurface;
use forge_store_aspect_native::{StoreReadinessHandoffArtifact, StoreTerminalJsonProjection};

use super::aspect_native_boundary_audit::{
    AspectNativeBoundaryAudit, AspectNativeRejectedInputKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectNativeBoundaryHandoff<PerformanceClaim>
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    artifact: StoreReadinessHandoffArtifact<PerformanceClaim>,
    gate_proof_evidence: AspectNativeBoundaryAudit,
}

impl<PerformanceClaim> AspectNativeBoundaryHandoff<PerformanceClaim>
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    pub fn new(
        artifact: StoreReadinessHandoffArtifact<PerformanceClaim>,
        gate_proof_evidence: AspectNativeBoundaryAudit,
    ) -> Result<Self, AspectNativeBoundaryHandoffDenial> {
        for required in AspectNativeRejectedInputKind::REQUIRED {
            if !gate_proof_evidence.contains_negative_proof(required) {
                return Err(AspectNativeBoundaryHandoffDenial::MissingNegativeProof(
                    required,
                ));
            }
        }

        Ok(Self {
            artifact,
            gate_proof_evidence,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectNativeBoundaryHandoffVerdict {
    canonical_basis_entry_count: usize,
    receipt_count: usize,
    diagnostic_count: usize,
    performance_receipt_count: usize,
    denied_input_count: usize,
    residue_scan_occurrence_count: usize,
    foundational_adoption_family_count: usize,
}

impl AspectNativeBoundaryHandoffVerdict {
    pub const fn canonical_basis_entry_count(&self) -> usize {
        self.canonical_basis_entry_count
    }

    pub const fn receipt_count(&self) -> usize {
        self.receipt_count
    }

    pub const fn diagnostic_count(&self) -> usize {
        self.diagnostic_count
    }

    pub const fn performance_receipt_count(&self) -> usize {
        self.performance_receipt_count
    }

    pub const fn denied_input_count(&self) -> usize {
        self.denied_input_count
    }

    pub const fn residue_scan_occurrence_count(&self) -> usize {
        self.residue_scan_occurrence_count
    }

    pub const fn foundational_adoption_family_count(&self) -> usize {
        self.foundational_adoption_family_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AspectNativeBoundaryHandoffDenial {
    TerminalJsonProjectionInput,
    MissingNegativeProof(AspectNativeRejectedInputKind),
}

pub fn accept_aspect_native_boundary_handoff<PerformanceClaim>(
    handoff: AspectNativeBoundaryHandoff<PerformanceClaim>,
) -> AspectNativeBoundaryHandoffVerdict
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    reconstruct_aspect_native_boundary_verdict(&handoff)
}

pub fn reject_terminal_json_projection_as_boundary_handoff(
    _projection: StoreTerminalJsonProjection,
) -> AspectNativeBoundaryHandoffDenial {
    AspectNativeBoundaryHandoffDenial::TerminalJsonProjectionInput
}

pub fn reconstruct_aspect_native_boundary_verdict<PerformanceClaim>(
    handoff: &AspectNativeBoundaryHandoff<PerformanceClaim>,
) -> AspectNativeBoundaryHandoffVerdict
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    AspectNativeBoundaryHandoffVerdict {
        canonical_basis_entry_count: handoff.artifact.canonical_basis().payload().entries().len(),
        receipt_count: handoff.artifact.completed_receipts().len(),
        diagnostic_count: handoff.artifact.diagnostics().len(),
        performance_receipt_count: handoff.artifact.performance().len(),
        denied_input_count: handoff.gate_proof_evidence.negative_proof_count(),
        residue_scan_occurrence_count: handoff
            .gate_proof_evidence
            .current_residue_scan()
            .classified_occurrence_count(),
        foundational_adoption_family_count: handoff
            .gate_proof_evidence
            .foundational_adoption()
            .adopted_family_count(),
    }
}
