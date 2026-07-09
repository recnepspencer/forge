use crate::placement::admission::{
    basis::BlobPlacementReachabilityBasis, BlobPlacementAdmissionDenial,
    BlobPlacementCounterSnapshot, BlobPlacementIntent,
};

pub(crate) fn verify_readiness_basis_match(
    basis: &BlobPlacementReachabilityBasis,
    intent: &BlobPlacementIntent,
) -> Result<(), BlobPlacementAdmissionDenial> {
    if !basis.admits_readiness(intent.readiness()) {
        return Err(
            BlobPlacementAdmissionDenial::PlacementReadinessBasisMismatch {
                counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
            },
        );
    }
    Ok(())
}
