use crate::{
    IntegrityEntryBasis, IntegrityEntryDenial, IntegrityEntryDenialKind, IntegrityEntryRequest,
    IntegrityEntryWitness, IntegrityInspectionLease,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryAdmission;

impl IntegrityEntryAdmission {
    pub fn admit<'runtime, 'lease>(
        request: IntegrityEntryRequest<'runtime, 'lease>,
    ) -> Result<IntegrityInspectionLease<'runtime, 'lease>, IntegrityEntryDenial> {
        require_matching_store_authority(&request)?;
        require_verification_coverage(&request)?;
        let basis = IntegrityEntryBasis::from_store_authority(
            request.protected_view_ref().basis(),
            request.verification().runtime_identity(),
            request.verification().bytes(),
        );
        let (protected_view, verification) = request.into_parts();
        Ok(IntegrityInspectionLease::new(
            protected_view,
            verification,
            IntegrityEntryWitness::mint(basis),
        ))
    }
}

fn require_matching_store_authority(
    request: &IntegrityEntryRequest<'_, '_>,
) -> Result<(), IntegrityEntryDenial> {
    let chunk = request.protected_view_ref().basis();
    let verification = request.verification();
    if chunk.store_identity() != verification.store_identity() {
        return Err(IntegrityEntryDenial::new(
            IntegrityEntryDenialKind::VerificationStoreMismatch,
        ));
    }
    if chunk.store_generation() != verification.store_generation() {
        return Err(IntegrityEntryDenial::new(
            IntegrityEntryDenialKind::VerificationGenerationMismatch,
        ));
    }
    Ok(())
}

fn require_verification_coverage(
    request: &IntegrityEntryRequest<'_, '_>,
) -> Result<(), IntegrityEntryDenial> {
    let protected_bytes = request.protected_view_ref().len_bytes() as u64;
    let allocation_bytes = request.verification().bytes();
    if allocation_bytes < protected_bytes {
        return Err(IntegrityEntryDenial::new(
            IntegrityEntryDenialKind::VerificationAllocationTooSmall {
                protected_bytes,
                allocation_bytes,
            },
        ));
    }
    Ok(())
}
