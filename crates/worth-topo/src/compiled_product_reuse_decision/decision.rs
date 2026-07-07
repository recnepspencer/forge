use super::counters::TopologyDerivedReuseDecisionCounters;
use super::denial::TopologyDerivedRebuildDenial;
use super::posture::TopologyDerivedReuseDecisionPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReuseDecision {
    posture: TopologyDerivedReuseDecisionPosture,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial: Option<TopologyDerivedRebuildDenial>,
    compiled_product_identity_digest: Option<String>,
    equivalence_policy_identity_digest: Option<String>,
    selected_equivalence_family_identity: Option<String>,
    selected_equivalence_basis_identity_digest: Option<String>,
    selected_compatibility_basis_identity_digest: Option<String>,
    selected_reuse_basis_identity_digest: Option<String>,
    counters: TopologyDerivedReuseDecisionCounters,
    comparison_supported: bool,
    unsupported_comparison_reason: Option<String>,
}

impl TopologyDerivedReuseDecision {
    pub(crate) fn new(
        posture: TopologyDerivedReuseDecisionPosture,
        reuse_decision_identity_digest: Option<String>,
        rebuild_denial: Option<TopologyDerivedRebuildDenial>,
        compiled_product_identity_digest: Option<String>,
        equivalence_policy_identity_digest: Option<String>,
        selected_equivalence_family_identity: Option<String>,
        selected_equivalence_basis_identity_digest: Option<String>,
        selected_compatibility_basis_identity_digest: Option<String>,
        selected_reuse_basis_identity_digest: Option<String>,
        counters: TopologyDerivedReuseDecisionCounters,
        comparison_supported: bool,
        unsupported_comparison_reason: Option<String>,
    ) -> Self {
        Self {
            posture,
            reuse_decision_identity_digest,
            rebuild_denial,
            compiled_product_identity_digest,
            equivalence_policy_identity_digest,
            selected_equivalence_family_identity,
            selected_equivalence_basis_identity_digest,
            selected_compatibility_basis_identity_digest,
            selected_reuse_basis_identity_digest,
            counters,
            comparison_supported,
            unsupported_comparison_reason,
        }
    }

    pub const fn posture(&self) -> TopologyDerivedReuseDecisionPosture {
        self.posture
    }

    #[cfg(test)]
    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial(&self) -> Option<&TopologyDerivedRebuildDenial> {
        self.rebuild_denial.as_ref()
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

    pub const fn counters(&self) -> &TopologyDerivedReuseDecisionCounters {
        &self.counters
    }

    pub const fn comparison_supported(&self) -> bool {
        self.comparison_supported
    }

    pub fn unsupported_comparison_reason(&self) -> Option<&str> {
        self.unsupported_comparison_reason.as_deref()
    }
}
