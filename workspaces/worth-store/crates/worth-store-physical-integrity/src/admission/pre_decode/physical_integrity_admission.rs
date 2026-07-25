use crate::{
    execute_declared_checksum, ChecksumAlgorithmClaim, ChecksumAlgorithmId,
    ChecksumAlgorithmMismatchDenial, ChecksumDeclarationAdmission, ChecksumScopeDeclaration,
    DeclaredPhysicalChecksum, IntegrityCheckedFrame, IntegrityCheckedPage,
    IntegrityInspectionLease, PhysicalIntegrityAdmissionRequest, PreDecodeAdmissionCounters,
    PreDecodePhysicalDenial, PreDecodePhysicalDenialKind,
};
use worth_store_physical_format::{
    PhysicalFrameKind, PhysicalHeaderDecodeWitness, PhysicalHeaderKind, PhysicalPageKind,
    PhysicalReferenceValidationWitness,
};

#[derive(Debug, Clone, Copy)]
struct PageIntegrityAdmissionBasis {
    cell: worth_store_physical_format::PageGenerationCell,
    header_witness: PhysicalHeaderDecodeWitness,
    expected_kind: PhysicalPageKind,
    expected_checksum: DeclaredPhysicalChecksum,
}

#[derive(Debug, Clone, Copy)]
struct FrameIntegrityAdmissionBasis {
    validation: PhysicalReferenceValidationWitness,
    header_witness: PhysicalHeaderDecodeWitness,
    expected_kind: PhysicalFrameKind,
    expected_checksum: DeclaredPhysicalChecksum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityAdmissionSeed<'lease> {
    lease: IntegrityInspectionLease<'lease>,
}

impl<'lease> PhysicalIntegrityAdmissionSeed<'lease> {
    pub const fn entry_witness(&self) -> crate::IntegrityEntryWitness {
        self.lease.entry_witness()
    }

    pub fn with_checksum_declaration(
        self,
        declaration: ChecksumDeclarationAdmission,
    ) -> Result<PhysicalIntegrityAdmission<'lease>, PreDecodePhysicalDenial> {
        if declaration.entry_witness() != self.lease.entry_witness() {
            return Err(PreDecodePhysicalDenial::new(
                PreDecodePhysicalDenialKind::EntryWitnessMismatch,
                self.lease.protected_bytes(),
            ));
        }
        Ok(PhysicalIntegrityAdmission {
            lease: self.lease,
            declaration,
        })
    }

    pub fn with_checksum_claim(
        self,
        claim: ChecksumAlgorithmClaim<'_>,
        scope: ChecksumScopeDeclaration,
    ) -> Result<PhysicalIntegrityAdmission<'lease>, PreDecodePhysicalDenial> {
        let algorithm = admit_algorithm_claim(claim, self.lease)?;
        let declaration = algorithm
            .declare_for_scope(scope)
            .map_err(|denial| unsupported_checksum(self.lease, denial))?
            .admit_for_physical_integrity_entry(self.lease.entry_witness());
        self.with_checksum_declaration(declaration)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityAdmission<'lease> {
    lease: IntegrityInspectionLease<'lease>,
    declaration: ChecksumDeclarationAdmission,
}

impl<'lease> PhysicalIntegrityAdmission<'lease> {
    pub const fn from_entry(
        lease: IntegrityInspectionLease<'lease>,
    ) -> PhysicalIntegrityAdmissionSeed<'lease> {
        PhysicalIntegrityAdmissionSeed { lease }
    }

    pub fn admit_page(
        &self,
        request: PhysicalIntegrityAdmissionRequest,
    ) -> Result<IntegrityCheckedPage<'lease>, PreDecodePhysicalDenial> {
        let page_basis = self.require_page_request(request)?;
        self.verify_page_witness(page_basis)?;
        self.verify_page_length(page_basis.header_witness)?;
        let checksum =
            self.verify_declared_checksum(page_basis.header_witness, page_basis.expected_checksum)?;
        Ok(self.build_checked_page(page_basis.header_witness, checksum))
    }

    pub fn admit_frame(
        &self,
        request: PhysicalIntegrityAdmissionRequest,
    ) -> Result<IntegrityCheckedFrame<'lease>, PreDecodePhysicalDenial> {
        let frame_basis = self.require_frame_request(request)?;
        self.verify_frame_witness(frame_basis)?;
        self.verify_frame_length(frame_basis.header_witness)?;
        let checksum = self
            .verify_declared_checksum(frame_basis.header_witness, frame_basis.expected_checksum)?;
        Ok(self.build_checked_frame(frame_basis.header_witness, checksum))
    }

    fn require_page_request(
        &self,
        request: PhysicalIntegrityAdmissionRequest,
    ) -> Result<PageIntegrityAdmissionBasis, PreDecodePhysicalDenial> {
        let PhysicalIntegrityAdmissionRequest::Page {
            cell,
            header_witness,
            expected_kind,
            expected_checksum,
        } = request
        else {
            return Err(self.reject_wrong_integrity_request_kind());
        };
        Ok(PageIntegrityAdmissionBasis {
            cell,
            header_witness,
            expected_kind,
            expected_checksum,
        })
    }

    fn require_frame_request(
        &self,
        request: PhysicalIntegrityAdmissionRequest,
    ) -> Result<FrameIntegrityAdmissionBasis, PreDecodePhysicalDenial> {
        let PhysicalIntegrityAdmissionRequest::Frame {
            validation,
            header_witness,
            expected_kind,
            expected_checksum,
        } = request
        else {
            return Err(self.reject_wrong_integrity_request_kind());
        };
        Ok(FrameIntegrityAdmissionBasis {
            validation,
            header_witness,
            expected_kind,
            expected_checksum,
        })
    }

    fn verify_page_witness(
        &self,
        basis: PageIntegrityAdmissionBasis,
    ) -> Result<(), PreDecodePhysicalDenial> {
        reject_page_witness(
            basis.cell.owner(),
            basis.header_witness,
            basis.expected_kind,
            self.lease,
        )
    }

    fn verify_page_length(
        &self,
        witness: PhysicalHeaderDecodeWitness,
    ) -> Result<(), PreDecodePhysicalDenial> {
        reject_truncated_page(self.lease, witness)
    }

    fn verify_frame_witness(
        &self,
        basis: FrameIntegrityAdmissionBasis,
    ) -> Result<(), PreDecodePhysicalDenial> {
        reject_frame_witness(
            basis.validation,
            basis.header_witness,
            basis.expected_kind,
            self.lease,
        )
    }

    fn verify_frame_length(
        &self,
        witness: PhysicalHeaderDecodeWitness,
    ) -> Result<(), PreDecodePhysicalDenial> {
        reject_truncated_frame(self.lease, witness)
    }

    fn verify_declared_checksum(
        &self,
        witness: PhysicalHeaderDecodeWitness,
        expected_checksum: DeclaredPhysicalChecksum,
    ) -> Result<crate::ExecutedPhysicalChecksum, PreDecodePhysicalDenial> {
        require_matching_checksum(
            self.lease,
            self.declared_checksum_algorithm(),
            witness,
            expected_checksum,
        )
    }

    fn build_checked_page(
        &self,
        witness: PhysicalHeaderDecodeWitness,
        checksum: crate::ExecutedPhysicalChecksum,
    ) -> IntegrityCheckedPage<'lease> {
        IntegrityCheckedPage::new(
            self.lease.protected_bytes(),
            witness,
            checksum,
            self.admitted_pre_decode_counters(),
            self.declaration.declaration().coverage_basis().clone(),
        )
    }

    fn build_checked_frame(
        &self,
        witness: PhysicalHeaderDecodeWitness,
        checksum: crate::ExecutedPhysicalChecksum,
    ) -> IntegrityCheckedFrame<'lease> {
        IntegrityCheckedFrame::new(
            self.lease.protected_bytes(),
            witness,
            checksum,
            self.admitted_pre_decode_counters(),
            self.declaration.declaration().coverage_basis().clone(),
        )
    }

    fn declared_checksum_algorithm(&self) -> ChecksumAlgorithmId {
        self.declaration
            .declaration()
            .coverage_basis()
            .algorithm_id()
    }

    fn admitted_pre_decode_counters(&self) -> PreDecodeAdmissionCounters {
        PreDecodeAdmissionCounters::admitted(self.lease.protected_bytes().len_bytes() as u64)
    }

    fn reject_wrong_integrity_request_kind(&self) -> PreDecodePhysicalDenial {
        PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::PhysicalHeaderDenied,
            self.lease.protected_bytes(),
        )
    }
}

fn admit_algorithm_claim(
    claim: ChecksumAlgorithmClaim<'_>,
    lease: IntegrityInspectionLease<'_>,
) -> Result<ChecksumAlgorithmId, PreDecodePhysicalDenial> {
    ChecksumAlgorithmId::admit_claim(claim).map_err(|denial| unsupported_checksum(lease, denial))
}

fn unsupported_checksum(
    lease: IntegrityInspectionLease<'_>,
    denial: ChecksumAlgorithmMismatchDenial,
) -> PreDecodePhysicalDenial {
    PreDecodePhysicalDenial::new(
        PreDecodePhysicalDenialKind::UnsupportedChecksumAlgorithm,
        lease.protected_bytes(),
    )
    .with_checksum_denial(denial)
}

fn reject_truncated_page(
    lease: IntegrityInspectionLease<'_>,
    witness: PhysicalHeaderDecodeWitness,
) -> Result<(), PreDecodePhysicalDenial> {
    reject_invalid_extent(
        lease,
        witness,
        PreDecodePhysicalDenialKind::TruncatedPhysicalPage,
    )
}

fn reject_truncated_frame(
    lease: IntegrityInspectionLease<'_>,
    witness: PhysicalHeaderDecodeWitness,
) -> Result<(), PreDecodePhysicalDenial> {
    reject_invalid_extent(
        lease,
        witness,
        PreDecodePhysicalDenialKind::TruncatedPhysicalFrame,
    )
}

fn reject_invalid_extent(
    lease: IntegrityInspectionLease<'_>,
    witness: PhysicalHeaderDecodeWitness,
    truncated: PreDecodePhysicalDenialKind,
) -> Result<(), PreDecodePhysicalDenial> {
    let expected = witness
        .payload_offset()
        .checked_add(witness.payload_length() as usize);
    let actual = lease.protected_bytes().len_bytes();
    let kind = match expected {
        Some(expected) if actual == expected => return Ok(()),
        Some(expected) if actual > expected => PreDecodePhysicalDenialKind::PhysicalHeaderDenied,
        _ => truncated,
    };
    Err(PreDecodePhysicalDenial::new(kind, lease.protected_bytes()).with_locality(witness.owner()))
}

fn require_matching_checksum(
    lease: IntegrityInspectionLease<'_>,
    algorithm: ChecksumAlgorithmId,
    witness: PhysicalHeaderDecodeWitness,
    expected: DeclaredPhysicalChecksum,
) -> Result<crate::ExecutedPhysicalChecksum, PreDecodePhysicalDenial> {
    let actual = execute_declared_checksum(algorithm, lease.protected_bytes().as_bytes());
    if actual.value() != expected.value() {
        return Err(PreDecodePhysicalDenial::after_checksum(
            PreDecodePhysicalDenialKind::ChecksumMismatch,
            lease.protected_bytes(),
        )
        .with_checksum_values(expected.value(), actual.value())
        .with_locality(witness.owner()));
    }
    Ok(actual)
}

fn reject_page_witness(
    expected_owner: worth_store_physical_format::PhysicalGenerationOwner,
    witness: PhysicalHeaderDecodeWitness,
    expected_kind: PhysicalPageKind,
    lease: IntegrityInspectionLease<'_>,
) -> Result<(), PreDecodePhysicalDenial> {
    if witness.kind() != PhysicalHeaderKind::Page(expected_kind) {
        return Err(PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::PhysicalHeaderDenied,
            lease.protected_bytes(),
        )
        .with_observed_kind(witness.kind())
        .with_locality(witness.owner()));
    }
    if witness.owner() != expected_owner {
        return Err(PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::StaleGeneration,
            lease.protected_bytes(),
        )
        .with_locality(witness.owner()));
    }
    Ok(())
}

fn reject_frame_witness(
    validation: PhysicalReferenceValidationWitness,
    witness: PhysicalHeaderDecodeWitness,
    expected_kind: PhysicalFrameKind,
    lease: IntegrityInspectionLease<'_>,
) -> Result<(), PreDecodePhysicalDenial> {
    if witness.kind() != PhysicalHeaderKind::Frame(expected_kind) {
        return Err(PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::PhysicalHeaderDenied,
            lease.protected_bytes(),
        )
        .with_observed_kind(witness.kind())
        .with_locality(witness.owner()));
    }
    if witness.owner() != validation.owner() {
        return Err(PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::StaleGeneration,
            lease.protected_bytes(),
        )
        .with_locality(witness.owner()));
    }
    Ok(())
}
