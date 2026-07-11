use crate::handoff_gate_proof_scan::scan_current_foundational_handoff_gate_proof_surfaces;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S0HandoffDeniedInputKind {
    TerminalJsonProjection,
    UnclassifiedResidue,
    RawStringIdentity,
    GenericSerdeAuthority,
    NonNativeDigestBasis,
}

impl S0HandoffDeniedInputKind {
    pub const REQUIRED: [Self; 5] = [
        Self::TerminalJsonProjection,
        Self::UnclassifiedResidue,
        Self::RawStringIdentity,
        Self::GenericSerdeAuthority,
        Self::NonNativeDigestBasis,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct S0HandoffNegativeProof {
    denied_input: S0HandoffDeniedInputKind,
}

impl S0HandoffNegativeProof {
    const fn terminal_json_projection_denied() -> Self {
        Self::new(S0HandoffDeniedInputKind::TerminalJsonProjection)
    }

    const fn unclassified_residue_denied() -> Self {
        Self::new(S0HandoffDeniedInputKind::UnclassifiedResidue)
    }

    const fn raw_string_identity_denied() -> Self {
        Self::new(S0HandoffDeniedInputKind::RawStringIdentity)
    }

    const fn generic_serde_authority_denied() -> Self {
        Self::new(S0HandoffDeniedInputKind::GenericSerdeAuthority)
    }

    const fn non_native_digest_basis_denied() -> Self {
        Self::new(S0HandoffDeniedInputKind::NonNativeDigestBasis)
    }

    const fn new(denied_input: S0HandoffDeniedInputKind) -> Self {
        Self { denied_input }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct S0HandoffNegativeProofSet {
    proofs: Vec<S0HandoffNegativeProof>,
}

impl S0HandoffNegativeProofSet {
    fn contains(&self, denied_input: S0HandoffDeniedInputKind) -> bool {
        self.proofs
            .iter()
            .any(|proof| proof.denied_input == denied_input)
    }

    fn len(&self) -> usize {
        self.proofs.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S0CurrentResidueScanEvidence {
    classified_occurrence_count: usize,
}

impl S0CurrentResidueScanEvidence {
    fn new(classified_occurrence_count: usize) -> Result<Self, S0HandoffGateProofEvidenceDenial> {
        if classified_occurrence_count == 0 {
            return Err(S0HandoffGateProofEvidenceDenial::MissingCurrentResidueScan);
        }
        Ok(Self {
            classified_occurrence_count,
        })
    }

    pub const fn classified_occurrence_count(self) -> usize {
        self.classified_occurrence_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S0TerminalProjectionBoundaryEvidence {
    terminal_boundary_count: usize,
}

impl S0TerminalProjectionBoundaryEvidence {
    fn new(terminal_boundary_count: usize) -> Result<Self, S0HandoffGateProofEvidenceDenial> {
        if terminal_boundary_count == 0 {
            return Err(S0HandoffGateProofEvidenceDenial::MissingTerminalProjectionBoundary);
        }
        Ok(Self {
            terminal_boundary_count,
        })
    }

    pub const fn terminal_boundary_count(self) -> usize {
        self.terminal_boundary_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S0FoundationalAdoptionEvidence {
    adopted_family_count: usize,
}

impl S0FoundationalAdoptionEvidence {
    fn new(adopted_family_count: usize) -> Result<Self, S0HandoffGateProofEvidenceDenial> {
        if adopted_family_count == 0 {
            return Err(S0HandoffGateProofEvidenceDenial::MissingFoundationalAdoption);
        }
        Ok(Self {
            adopted_family_count,
        })
    }

    pub const fn adopted_family_count(self) -> usize {
        self.adopted_family_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S0PublicFacadeEvidence {
    exported_surface_count: usize,
}

impl S0PublicFacadeEvidence {
    fn new(exported_surface_count: usize) -> Result<Self, S0HandoffGateProofEvidenceDenial> {
        if exported_surface_count == 0 {
            return Err(S0HandoffGateProofEvidenceDenial::MissingPublicFacadeProof);
        }
        Ok(Self {
            exported_surface_count,
        })
    }

    pub const fn exported_surface_count(self) -> usize {
        self.exported_surface_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S0NativeHarnessEvidence {
    native_fixture_surface_count: usize,
}

impl S0NativeHarnessEvidence {
    fn new(native_fixture_surface_count: usize) -> Result<Self, S0HandoffGateProofEvidenceDenial> {
        if native_fixture_surface_count == 0 {
            return Err(S0HandoffGateProofEvidenceDenial::MissingNativeHarnessProof);
        }
        Ok(Self {
            native_fixture_surface_count,
        })
    }

    pub const fn native_fixture_surface_count(self) -> usize {
        self.native_fixture_surface_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S0HandoffGateProofEvidence {
    current_residue_scan: S0CurrentResidueScanEvidence,
    terminal_projection_boundary: S0TerminalProjectionBoundaryEvidence,
    foundational_adoption: S0FoundationalAdoptionEvidence,
    public_facade: S0PublicFacadeEvidence,
    native_harness: S0NativeHarnessEvidence,
    negative_proofs: S0HandoffNegativeProofSet,
}

impl S0HandoffGateProofEvidence {
    fn new(
        current_residue_scan: S0CurrentResidueScanEvidence,
        terminal_projection_boundary: S0TerminalProjectionBoundaryEvidence,
        foundational_adoption: S0FoundationalAdoptionEvidence,
        public_facade: S0PublicFacadeEvidence,
        native_harness: S0NativeHarnessEvidence,
    ) -> Self {
        Self {
            current_residue_scan,
            terminal_projection_boundary,
            foundational_adoption,
            public_facade,
            native_harness,
            negative_proofs: required_negative_proofs(),
        }
    }

    pub const fn current_residue_scan(&self) -> S0CurrentResidueScanEvidence {
        self.current_residue_scan
    }

    pub const fn terminal_projection_boundary(&self) -> S0TerminalProjectionBoundaryEvidence {
        self.terminal_projection_boundary
    }

    pub const fn foundational_adoption(&self) -> S0FoundationalAdoptionEvidence {
        self.foundational_adoption
    }

    pub const fn public_facade(&self) -> S0PublicFacadeEvidence {
        self.public_facade
    }

    pub const fn native_harness(&self) -> S0NativeHarnessEvidence {
        self.native_harness
    }

    pub fn contains_negative_proof(&self, denied_input: S0HandoffDeniedInputKind) -> bool {
        self.negative_proofs.contains(denied_input)
    }

    pub fn negative_proof_count(&self) -> usize {
        self.negative_proofs.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S0HandoffGateProofEvidenceDenial {
    MissingCurrentResidueScan,
    MissingTerminalProjectionBoundary,
    MissingFoundationalAdoption,
    MissingPublicFacadeProof,
    MissingNativeHarnessProof,
    SourceReadFailed(String),
}

pub fn certify_current_foundational_handoff_gate_proof_evidence(
) -> Result<S0HandoffGateProofEvidence, S0HandoffGateProofEvidenceDenial> {
    let counts = scan_current_foundational_handoff_gate_proof_surfaces()?;
    S0HandoffGateProofEvidence::from_current_workspace_evidence(
        S0CurrentResidueScanEvidence::new(counts.current_residue_scan)?,
        S0TerminalProjectionBoundaryEvidence::new(counts.terminal_projection_boundary)?,
        S0FoundationalAdoptionEvidence::new(counts.foundational_adoption)?,
        S0PublicFacadeEvidence::new(counts.public_facade)?,
        S0NativeHarnessEvidence::new(counts.native_harness)?,
    )
}

impl S0HandoffGateProofEvidence {
    fn from_current_workspace_evidence(
        current_residue_scan: S0CurrentResidueScanEvidence,
        terminal_projection_boundary: S0TerminalProjectionBoundaryEvidence,
        foundational_adoption: S0FoundationalAdoptionEvidence,
        public_facade: S0PublicFacadeEvidence,
        native_harness: S0NativeHarnessEvidence,
    ) -> Result<Self, S0HandoffGateProofEvidenceDenial> {
        Ok(Self::new(
            current_residue_scan,
            terminal_projection_boundary,
            foundational_adoption,
            public_facade,
            native_harness,
        ))
    }
}

fn required_negative_proofs() -> S0HandoffNegativeProofSet {
    S0HandoffNegativeProofSet {
        proofs: vec![
            S0HandoffNegativeProof::terminal_json_projection_denied(),
            S0HandoffNegativeProof::unclassified_residue_denied(),
            S0HandoffNegativeProof::raw_string_identity_denied(),
            S0HandoffNegativeProof::generic_serde_authority_denied(),
            S0HandoffNegativeProof::non_native_digest_basis_denied(),
        ],
    }
}
