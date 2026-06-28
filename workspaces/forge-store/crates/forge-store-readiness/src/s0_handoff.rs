use forge_foundational::FoundationalPerformanceClaimSurface;
use forge_store_aspect_native::{StoreS0ReadinessHandoffArtifact, StoreTerminalJsonProjection};
use forge_store_s0_reclassification::{S0HandoffDeniedInputKind, S0HandoffGateProofEvidence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S0AspectNativeGateHandoff<PerformanceClaim>
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    artifact: StoreS0ReadinessHandoffArtifact<PerformanceClaim>,
    gate_proof_evidence: S0HandoffGateProofEvidence,
}

impl<PerformanceClaim> S0AspectNativeGateHandoff<PerformanceClaim>
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    pub fn new(
        artifact: StoreS0ReadinessHandoffArtifact<PerformanceClaim>,
        gate_proof_evidence: S0HandoffGateProofEvidence,
    ) -> Result<Self, S0AspectNativeGateHandoffDenial> {
        for required in S0HandoffDeniedInputKind::REQUIRED {
            if !gate_proof_evidence.contains_negative_proof(required) {
                return Err(S0AspectNativeGateHandoffDenial::MissingNegativeProof(
                    required,
                ));
            }
        }

        Ok(Self {
            artifact,
            gate_proof_evidence,
        })
    }

    pub const fn artifact(&self) -> &StoreS0ReadinessHandoffArtifact<PerformanceClaim> {
        &self.artifact
    }

    pub const fn gate_proof_evidence(&self) -> &S0HandoffGateProofEvidence {
        &self.gate_proof_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S0AspectNativeGateHandoffVerdict {
    canonical_basis_entry_count: usize,
    receipt_count: usize,
    diagnostic_count: usize,
    performance_receipt_count: usize,
    denied_input_count: usize,
    residue_scan_occurrence_count: usize,
    foundational_adoption_family_count: usize,
}

impl S0AspectNativeGateHandoffVerdict {
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
pub enum S0AspectNativeGateHandoffDenial {
    TerminalJsonProjectionInput,
    MissingNegativeProof(S0HandoffDeniedInputKind),
}

pub fn accept_s0_aspect_native_gate_handoff<PerformanceClaim>(
    handoff: S0AspectNativeGateHandoff<PerformanceClaim>,
) -> S0AspectNativeGateHandoffVerdict
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    reconstruct_s0_handoff_verdict_from_native_evidence(&handoff)
}

pub fn reject_terminal_json_projection_as_s0_handoff(
    _projection: StoreTerminalJsonProjection,
) -> S0AspectNativeGateHandoffDenial {
    S0AspectNativeGateHandoffDenial::TerminalJsonProjectionInput
}

pub fn reconstruct_s0_handoff_verdict_from_native_evidence<PerformanceClaim>(
    handoff: &S0AspectNativeGateHandoff<PerformanceClaim>,
) -> S0AspectNativeGateHandoffVerdict
where
    PerformanceClaim: FoundationalPerformanceClaimSurface,
{
    S0AspectNativeGateHandoffVerdict {
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
