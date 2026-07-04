use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProductCounters;
use crate::workload_platform::selected_equivalence_family::SpatialSelectedEquivalenceFamilyIdentity;

use super::counters::EvidenceLookupReuseDecisionCounters;
use super::denial::EvidenceLookupIndexRebuildDenial;
use super::posture::EvidenceLookupReuseDecisionPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexReuseDecision {
    posture: EvidenceLookupReuseDecisionPosture,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_equivalence_family_identity: SpatialSelectedEquivalenceFamilyIdentity,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial: Option<EvidenceLookupIndexRebuildDenial>,
    counters: EvidenceLookupReuseDecisionCounters,
}

impl EvidenceLookupIndexReuseDecision {
    pub(crate) fn new(
        posture: EvidenceLookupReuseDecisionPosture,
        compiled_product_identity_digest: String,
        equivalence_policy_identity_digest: String,
        selected_equivalence_family_identity: SpatialSelectedEquivalenceFamilyIdentity,
        selected_equivalence_basis_identity_digest: String,
        selected_compatibility_basis_identity_digest: String,
        selected_reuse_basis_identity_digest: String,
        reuse_decision_identity_digest: Option<String>,
        rebuild_denial: Option<EvidenceLookupIndexRebuildDenial>,
        counters: EvidenceLookupReuseDecisionCounters,
    ) -> Self {
        Self {
            posture,
            compiled_product_identity_digest,
            equivalence_policy_identity_digest,
            selected_equivalence_family_identity,
            selected_equivalence_basis_identity_digest,
            selected_compatibility_basis_identity_digest,
            selected_reuse_basis_identity_digest,
            reuse_decision_identity_digest,
            rebuild_denial,
            counters,
        }
    }

    pub const fn posture(&self) -> EvidenceLookupReuseDecisionPosture {
        self.posture
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
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

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial(&self) -> Option<&EvidenceLookupIndexRebuildDenial> {
        self.rebuild_denial.as_ref()
    }

    pub const fn counters(&self) -> &EvidenceLookupReuseDecisionCounters {
        &self.counters
    }

    pub const fn product_counters(&self) -> &EvidenceLookupIndexProductCounters {
        self.counters.product_counters()
    }
}
