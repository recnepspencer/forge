use super::source_scan::scan_current_aspect_native_boundary_surfaces;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectNativeRejectedInputKind {
    TerminalJsonProjection,
    UnclassifiedResidue,
    RawStringIdentity,
    GenericSerdeAuthority,
    NonNativeDigestBasis,
}

impl AspectNativeRejectedInputKind {
    pub const REQUIRED: [Self; 5] = [
        Self::TerminalJsonProjection,
        Self::UnclassifiedResidue,
        Self::RawStringIdentity,
        Self::GenericSerdeAuthority,
        Self::NonNativeDigestBasis,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AspectNativeRejectedInputProof {
    denied_input: AspectNativeRejectedInputKind,
}

impl AspectNativeRejectedInputProof {
    const fn terminal_json_projection_denied() -> Self {
        Self::new(AspectNativeRejectedInputKind::TerminalJsonProjection)
    }

    const fn unclassified_residue_denied() -> Self {
        Self::new(AspectNativeRejectedInputKind::UnclassifiedResidue)
    }

    const fn raw_string_identity_denied() -> Self {
        Self::new(AspectNativeRejectedInputKind::RawStringIdentity)
    }

    const fn generic_serde_authority_denied() -> Self {
        Self::new(AspectNativeRejectedInputKind::GenericSerdeAuthority)
    }

    const fn non_native_digest_basis_denied() -> Self {
        Self::new(AspectNativeRejectedInputKind::NonNativeDigestBasis)
    }

    const fn new(denied_input: AspectNativeRejectedInputKind) -> Self {
        Self { denied_input }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AspectNativeRejectedInputProofSet {
    proofs: Vec<AspectNativeRejectedInputProof>,
}

impl AspectNativeRejectedInputProofSet {
    fn contains(&self, denied_input: AspectNativeRejectedInputKind) -> bool {
        self.proofs
            .iter()
            .any(|proof| proof.denied_input == denied_input)
    }

    fn len(&self) -> usize {
        self.proofs.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AspectNativeResidueScanEvidence {
    classified_occurrence_count: usize,
}

impl AspectNativeResidueScanEvidence {
    fn new(classified_occurrence_count: usize) -> Result<Self, AspectNativeBoundaryAuditDenial> {
        if classified_occurrence_count == 0 {
            return Err(AspectNativeBoundaryAuditDenial::MissingCurrentResidueScan);
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
pub struct TerminalProjectionBoundaryEvidence {
    terminal_boundary_count: usize,
}

impl TerminalProjectionBoundaryEvidence {
    fn new(terminal_boundary_count: usize) -> Result<Self, AspectNativeBoundaryAuditDenial> {
        if terminal_boundary_count == 0 {
            return Err(AspectNativeBoundaryAuditDenial::MissingTerminalProjectionBoundary);
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
pub struct FoundationalSurfaceAdoptionEvidence {
    adopted_family_count: usize,
}

impl FoundationalSurfaceAdoptionEvidence {
    fn new(adopted_family_count: usize) -> Result<Self, AspectNativeBoundaryAuditDenial> {
        if adopted_family_count == 0 {
            return Err(AspectNativeBoundaryAuditDenial::MissingFoundationalAdoption);
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
pub struct StorePublicFacadeEvidence {
    exported_surface_count: usize,
}

impl StorePublicFacadeEvidence {
    fn new(exported_surface_count: usize) -> Result<Self, AspectNativeBoundaryAuditDenial> {
        if exported_surface_count == 0 {
            return Err(AspectNativeBoundaryAuditDenial::MissingPublicFacadeProof);
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
pub struct NativeHarnessSurfaceEvidence {
    native_fixture_surface_count: usize,
}

impl NativeHarnessSurfaceEvidence {
    fn new(native_fixture_surface_count: usize) -> Result<Self, AspectNativeBoundaryAuditDenial> {
        if native_fixture_surface_count == 0 {
            return Err(AspectNativeBoundaryAuditDenial::MissingNativeHarnessProof);
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
pub struct AspectNativeBoundaryAudit {
    current_residue_scan: AspectNativeResidueScanEvidence,
    terminal_projection_boundary: TerminalProjectionBoundaryEvidence,
    foundational_adoption: FoundationalSurfaceAdoptionEvidence,
    public_facade: StorePublicFacadeEvidence,
    native_harness: NativeHarnessSurfaceEvidence,
    negative_proofs: AspectNativeRejectedInputProofSet,
}

impl AspectNativeBoundaryAudit {
    fn new(
        current_residue_scan: AspectNativeResidueScanEvidence,
        terminal_projection_boundary: TerminalProjectionBoundaryEvidence,
        foundational_adoption: FoundationalSurfaceAdoptionEvidence,
        public_facade: StorePublicFacadeEvidence,
        native_harness: NativeHarnessSurfaceEvidence,
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

    pub const fn current_residue_scan(&self) -> AspectNativeResidueScanEvidence {
        self.current_residue_scan
    }

    pub const fn terminal_projection_boundary(&self) -> TerminalProjectionBoundaryEvidence {
        self.terminal_projection_boundary
    }

    pub const fn foundational_adoption(&self) -> FoundationalSurfaceAdoptionEvidence {
        self.foundational_adoption
    }

    pub const fn public_facade(&self) -> StorePublicFacadeEvidence {
        self.public_facade
    }

    pub const fn native_harness(&self) -> NativeHarnessSurfaceEvidence {
        self.native_harness
    }

    pub fn contains_negative_proof(&self, denied_input: AspectNativeRejectedInputKind) -> bool {
        self.negative_proofs.contains(denied_input)
    }

    pub fn negative_proof_count(&self) -> usize {
        self.negative_proofs.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AspectNativeBoundaryAuditDenial {
    MissingCurrentResidueScan,
    MissingTerminalProjectionBoundary,
    MissingFoundationalAdoption,
    MissingPublicFacadeProof,
    MissingNativeHarnessProof,
    SourceReadFailed(String),
}

pub fn audit_current_aspect_native_boundaries(
) -> Result<AspectNativeBoundaryAudit, AspectNativeBoundaryAuditDenial> {
    let counts = scan_current_aspect_native_boundary_surfaces()?;
    AspectNativeBoundaryAudit::from_current_workspace_evidence(
        AspectNativeResidueScanEvidence::new(counts.current_residue_scan)?,
        TerminalProjectionBoundaryEvidence::new(counts.terminal_projection_boundary)?,
        FoundationalSurfaceAdoptionEvidence::new(counts.foundational_adoption)?,
        StorePublicFacadeEvidence::new(counts.public_facade)?,
        NativeHarnessSurfaceEvidence::new(counts.native_harness)?,
    )
}

impl AspectNativeBoundaryAudit {
    fn from_current_workspace_evidence(
        current_residue_scan: AspectNativeResidueScanEvidence,
        terminal_projection_boundary: TerminalProjectionBoundaryEvidence,
        foundational_adoption: FoundationalSurfaceAdoptionEvidence,
        public_facade: StorePublicFacadeEvidence,
        native_harness: NativeHarnessSurfaceEvidence,
    ) -> Result<Self, AspectNativeBoundaryAuditDenial> {
        Ok(Self::new(
            current_residue_scan,
            terminal_projection_boundary,
            foundational_adoption,
            public_facade,
            native_harness,
        ))
    }
}

fn required_negative_proofs() -> AspectNativeRejectedInputProofSet {
    AspectNativeRejectedInputProofSet {
        proofs: vec![
            AspectNativeRejectedInputProof::terminal_json_projection_denied(),
            AspectNativeRejectedInputProof::unclassified_residue_denied(),
            AspectNativeRejectedInputProof::raw_string_identity_denied(),
            AspectNativeRejectedInputProof::generic_serde_authority_denied(),
            AspectNativeRejectedInputProof::non_native_digest_basis_denied(),
        ],
    }
}
