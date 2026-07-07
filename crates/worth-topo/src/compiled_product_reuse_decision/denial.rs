use super::counters::TopologyDerivedReuseDecisionCounters;
use super::mismatch_locus::TopologyDerivedReuseMismatchLocus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedRebuildDenial {
    denial_identity_digest: String,
    mismatch_loci: Vec<TopologyDerivedReuseMismatchLocus>,
    compiled_product_identity_digest: Option<String>,
    equivalence_policy_identity_digest: Option<String>,
    selected_equivalence_family_identity: Option<String>,
    selected_equivalence_basis_identity_digest: Option<String>,
    selected_compatibility_basis_identity_digest: Option<String>,
    selected_reuse_basis_identity_digest: Option<String>,
    counters: TopologyDerivedReuseDecisionCounters,
}

impl TopologyDerivedRebuildDenial {
    pub(crate) fn new(
        denial_identity_digest: String,
        mismatch_loci: Vec<TopologyDerivedReuseMismatchLocus>,
        compiled_product_identity_digest: Option<String>,
        equivalence_policy_identity_digest: Option<String>,
        selected_equivalence_family_identity: Option<String>,
        selected_equivalence_basis_identity_digest: Option<String>,
        selected_compatibility_basis_identity_digest: Option<String>,
        selected_reuse_basis_identity_digest: Option<String>,
        counters: TopologyDerivedReuseDecisionCounters,
    ) -> Self {
        Self {
            denial_identity_digest,
            mismatch_loci,
            compiled_product_identity_digest,
            equivalence_policy_identity_digest,
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

    pub fn mismatch_loci(&self) -> &[TopologyDerivedReuseMismatchLocus] {
        &self.mismatch_loci
    }

    #[cfg(test)]
    pub fn compiled_product_identity_digest(&self) -> Option<&str> {
        self.compiled_product_identity_digest.as_deref()
    }

    #[cfg(test)]
    pub fn equivalence_policy_identity_digest(&self) -> Option<&str> {
        self.equivalence_policy_identity_digest.as_deref()
    }

    #[cfg(test)]
    pub fn selected_equivalence_family_identity(&self) -> Option<&str> {
        self.selected_equivalence_family_identity.as_deref()
    }

    #[cfg(test)]
    pub fn selected_equivalence_basis_identity_digest(&self) -> Option<&str> {
        self.selected_equivalence_basis_identity_digest.as_deref()
    }

    #[cfg(test)]
    pub fn selected_compatibility_basis_identity_digest(&self) -> Option<&str> {
        self.selected_compatibility_basis_identity_digest.as_deref()
    }

    #[cfg(test)]
    pub fn selected_reuse_basis_identity_digest(&self) -> Option<&str> {
        self.selected_reuse_basis_identity_digest.as_deref()
    }

    #[cfg(test)]
    pub const fn counters(&self) -> &TopologyDerivedReuseDecisionCounters {
        &self.counters
    }
}
