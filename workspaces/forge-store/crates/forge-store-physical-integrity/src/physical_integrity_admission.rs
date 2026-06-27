use crate::{
    execute_declared_checksum, ChecksumAlgorithmClaim, ChecksumAlgorithmId,
    ChecksumAlgorithmMismatchDenial, ChecksumScopeDeclaration, DeclaredPhysicalChecksum,
    IntegrityCheckedFrame, IntegrityCheckedPage, IntegrityInspectionLease,
    PhysicalIntegrityAdmissionRequest, PreDecodeAdmissionCounters, PreDecodePhysicalDenial,
    PreDecodePhysicalDenialKind, S3ChecksumDeclarationAdmission,
};
use forge_store_physical_format::{
    PhysicalFrameKind, PhysicalHeaderDecodeWitness, PhysicalHeaderKind, PhysicalPageKind,
    PhysicalReferenceValidationWitness,
};

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
        declaration: S3ChecksumDeclarationAdmission,
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
            .admit_for_s3_entry(self.lease.entry_witness());
        self.with_checksum_declaration(declaration)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityAdmission<'lease> {
    lease: IntegrityInspectionLease<'lease>,
    declaration: S3ChecksumDeclarationAdmission,
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
        let PhysicalIntegrityAdmissionRequest::Page {
            cell,
            header_witness,
            expected_kind,
            expected_checksum,
        } = request
        else {
            return Err(PreDecodePhysicalDenial::new(
                PreDecodePhysicalDenialKind::PhysicalHeaderDenied,
                self.lease.protected_bytes(),
            ));
        };
        reject_page_witness(cell.owner(), header_witness, expected_kind, self.lease)?;
        reject_truncated_page(self.lease, header_witness)?;
        let checksum = require_matching_checksum(
            self.lease,
            self.declaration
                .declaration()
                .coverage_basis()
                .algorithm_id(),
            header_witness,
            expected_checksum,
        )?;
        Ok(IntegrityCheckedPage::new(
            self.lease.protected_bytes(),
            header_witness,
            checksum,
            PreDecodeAdmissionCounters::admitted(self.lease.protected_bytes().len_bytes() as u64),
            self.declaration.declaration().coverage_basis().clone(),
        ))
    }

    pub fn admit_frame(
        &self,
        request: PhysicalIntegrityAdmissionRequest,
    ) -> Result<IntegrityCheckedFrame<'lease>, PreDecodePhysicalDenial> {
        let PhysicalIntegrityAdmissionRequest::Frame {
            validation,
            header_witness,
            expected_kind,
            expected_checksum,
        } = request
        else {
            return Err(PreDecodePhysicalDenial::new(
                PreDecodePhysicalDenialKind::PhysicalHeaderDenied,
                self.lease.protected_bytes(),
            ));
        };
        reject_frame_witness(validation, header_witness, expected_kind, self.lease)?;
        reject_truncated_frame(self.lease, header_witness)?;
        let checksum = require_matching_checksum(
            self.lease,
            self.declaration
                .declaration()
                .coverage_basis()
                .algorithm_id(),
            header_witness,
            expected_checksum,
        )?;
        Ok(IntegrityCheckedFrame::new(
            self.lease.protected_bytes(),
            header_witness,
            checksum,
            PreDecodeAdmissionCounters::admitted(self.lease.protected_bytes().len_bytes() as u64),
            self.declaration.declaration().coverage_basis().clone(),
        ))
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
    if lease.protected_bytes().len_bytes() < witness.payload_length() as usize {
        return Err(PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::TruncatedPhysicalPage,
            lease.protected_bytes(),
        )
        .with_locality(witness.owner()));
    }
    Ok(())
}

fn reject_truncated_frame(
    lease: IntegrityInspectionLease<'_>,
    witness: PhysicalHeaderDecodeWitness,
) -> Result<(), PreDecodePhysicalDenial> {
    if lease.protected_bytes().len_bytes() < witness.payload_length() as usize {
        return Err(PreDecodePhysicalDenial::new(
            PreDecodePhysicalDenialKind::TruncatedPhysicalFrame,
            lease.protected_bytes(),
        )
        .with_locality(witness.owner()));
    }
    Ok(())
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
    expected_owner: forge_store_physical_format::PhysicalGenerationOwner,
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
