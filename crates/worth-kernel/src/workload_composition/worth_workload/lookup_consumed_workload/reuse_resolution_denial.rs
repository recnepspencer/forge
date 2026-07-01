use worth_spatial::facade::evidence_lookup_index_product::{
    EvidenceLookupIndexProductCounters, EvidenceLookupIndexRebuildDenial,
    EvidenceLookupReuseMismatchLocus, SpatialSelectedEquivalenceFamilyIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupConsumedWorkloadReuseResolutionDenied {
    denial_identity_digest: String,
    mismatch_loci: Vec<EvidenceLookupReuseMismatchLocus>,
    selected_equivalence_family_identity: SpatialSelectedEquivalenceFamilyIdentity,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    counters: EvidenceLookupIndexProductCounters,
}

impl LookupConsumedWorkloadReuseResolutionDenied {
    pub fn from_spatial_denial(denial: &EvidenceLookupIndexRebuildDenial) -> Self {
        Self {
            denial_identity_digest: denial.denial_identity_digest().to_string(),
            mismatch_loci: denial.mismatch_loci().to_vec(),
            selected_equivalence_family_identity: denial.selected_equivalence_family_identity(),
            selected_equivalence_basis_identity_digest: denial
                .selected_equivalence_basis_identity_digest()
                .to_string(),
            selected_compatibility_basis_identity_digest: denial
                .selected_compatibility_basis_identity_digest()
                .to_string(),
            selected_reuse_basis_identity_digest: denial
                .selected_reuse_basis_identity_digest()
                .to_string(),
            counters: denial.counters().clone(),
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

    pub fn human_reason(&self) -> String {
        let mismatch_loci = self
            .mismatch_loci
            .iter()
            .map(mismatch_locus_name)
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "lookup-consumed workload composition rejects denied reuse resolution {} at [{}]",
            self.denial_identity_digest, mismatch_loci
        )
    }
}

fn mismatch_locus_name(locus: &EvidenceLookupReuseMismatchLocus) -> &'static str {
    match locus {
        EvidenceLookupReuseMismatchLocus::SpatialTouchAuthorityDigest => {
            "spatial_touch_authority_digest"
        }
        EvidenceLookupReuseMismatchLocus::StageReceiptDigest => "stage_receipt_digest",
        EvidenceLookupReuseMismatchLocus::EvidenceLedgerBasisDigest => {
            "evidence_ledger_basis_digest"
        }
        EvidenceLookupReuseMismatchLocus::TopologySupportDigest => "topology_support_digest",
        EvidenceLookupReuseMismatchLocus::QuerySupportDigest => "query_support_digest",
        EvidenceLookupReuseMismatchLocus::EquivalencePolicyIdentity => {
            "equivalence_policy_identity"
        }
        EvidenceLookupReuseMismatchLocus::SelectedEquivalenceFamilyIdentity => {
            "selected_equivalence_family_identity"
        }
        EvidenceLookupReuseMismatchLocus::SelectedEquivalenceBasisIdentity => {
            "selected_equivalence_basis_identity"
        }
        EvidenceLookupReuseMismatchLocus::SelectedCompatibilityBasisIdentity => {
            "selected_compatibility_basis_identity"
        }
        EvidenceLookupReuseMismatchLocus::SelectedReuseBasisIdentity => {
            "selected_reuse_basis_identity"
        }
    }
}
