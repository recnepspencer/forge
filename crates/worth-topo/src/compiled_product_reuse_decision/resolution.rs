use super::decision::TopologyDerivedReuseDecision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReuseResolution {
    decision: TopologyDerivedReuseDecision,
    authority_identity_match: bool,
    branch_identity_match: bool,
    invalidation_target_match: bool,
    materialized_topology_digest_match: bool,
    interpreted_topology_digest_match: bool,
    derived_validation_digest_match: bool,
    equivalent_derived_meaning: bool,
}

impl TopologyDerivedReuseResolution {
    pub(crate) fn new(
        decision: TopologyDerivedReuseDecision,
        authority_identity_match: bool,
        branch_identity_match: bool,
        invalidation_target_match: bool,
        materialized_topology_digest_match: bool,
        interpreted_topology_digest_match: bool,
        derived_validation_digest_match: bool,
        equivalent_derived_meaning: bool,
    ) -> Self {
        Self {
            decision,
            authority_identity_match,
            branch_identity_match,
            invalidation_target_match,
            materialized_topology_digest_match,
            interpreted_topology_digest_match,
            derived_validation_digest_match,
            equivalent_derived_meaning,
        }
    }

    pub const fn decision(&self) -> &TopologyDerivedReuseDecision {
        &self.decision
    }

    pub const fn authority_identity_match(&self) -> bool {
        self.authority_identity_match
    }

    pub const fn branch_identity_match(&self) -> bool {
        self.branch_identity_match
    }

    pub const fn invalidation_target_match(&self) -> bool {
        self.invalidation_target_match
    }

    pub const fn materialized_topology_digest_match(&self) -> bool {
        self.materialized_topology_digest_match
    }

    pub const fn interpreted_topology_digest_match(&self) -> bool {
        self.interpreted_topology_digest_match
    }

    pub const fn derived_validation_digest_match(&self) -> bool {
        self.derived_validation_digest_match
    }

    pub const fn equivalent_derived_meaning(&self) -> bool {
        self.equivalent_derived_meaning
    }
}
