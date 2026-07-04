use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProductCounters;
use crate::workload_platform::selected_equivalence_family::SpatialSelectedEquivalenceFamilyIdentity;

use super::mismatch_locus::EvidenceLookupReuseMismatchLocus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexRebuildDenial {
    denial_identity_digest: String,
    mismatch_loci: Vec<EvidenceLookupReuseMismatchLocus>,
    selected_equivalence_family_identity: SpatialSelectedEquivalenceFamilyIdentity,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    counters: EvidenceLookupIndexProductCounters,
}

impl EvidenceLookupIndexRebuildDenial {
    pub(crate) fn new(
        denial_identity_digest: String,
        mismatch_loci: Vec<EvidenceLookupReuseMismatchLocus>,
        selected_equivalence_family_identity: SpatialSelectedEquivalenceFamilyIdentity,
        selected_equivalence_basis_identity_digest: String,
        selected_compatibility_basis_identity_digest: String,
        selected_reuse_basis_identity_digest: String,
        counters: EvidenceLookupIndexProductCounters,
    ) -> Self {
        Self {
            denial_identity_digest,
            mismatch_loci,
            selected_equivalence_family_identity,
            selected_equivalence_basis_identity_digest,
            selected_compatibility_basis_identity_digest,
            selected_reuse_basis_identity_digest,
            counters,
        }
    }

    pub fn denial_identity_digest(&self) -> &str {
        &self.denial_identity_digest
    }

    pub fn mismatch_loci(&self) -> &[EvidenceLookupReuseMismatchLocus] {
        &self.mismatch_loci
    }

    pub const fn selected_equivalence_family_identity(
        &self,
    ) -> SpatialSelectedEquivalenceFamilyIdentity {
        self.selected_equivalence_family_identity
    }

    pub fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub const fn counters(&self) -> &EvidenceLookupIndexProductCounters {
        &self.counters
    }
}
