use forge_store_physical_backend::AdmittedBackendCapabilityWitness;

use crate::BlobChunkReachabilityProofSet;

use crate::placement::admission::{
    basis::BlobPlacementReachabilityBasis,
    receipt_construction::construct_admitted_placement,
    types::AdmittedBlobPlacement,
    verification::{verify_class_backend_capability, verify_readiness_basis_match},
    BlobPlacementAdmissionDenial, BlobPlacementIntent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementAdmissionAuthority {
    backend: AdmittedBackendCapabilityWitness,
}

impl BlobPlacementAdmissionAuthority {
    pub const fn from_admitted_backend(backend: AdmittedBackendCapabilityWitness) -> Self {
        Self { backend }
    }

    pub fn admit(
        &self,
        reachability: &BlobChunkReachabilityProofSet,
        intent: BlobPlacementIntent,
    ) -> Result<AdmittedBlobPlacement, BlobPlacementAdmissionDenial> {
        let basis = BlobPlacementReachabilityBasis::from_reachability(reachability);
        verify_readiness_basis_match(&basis, &intent)?;
        let counters = verify_class_backend_capability(&self.backend, &intent, &basis)?;
        Ok(construct_admitted_placement(
            basis,
            reachability,
            intent,
            counters,
        ))
    }
}
