#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceFamilyProposal {
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    requirements: u8,
    fact_references: u16,
    mounted_work_references: u16,
    conflict_policy: super::super::UiServiceProposalConflictPolicy,
}

impl UiServiceFamilyProposal {
    pub(in crate::runtime) const fn portal(
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            family: crate::capability::UiRuntimeServiceFamily::Portal,
            scope,
            requirements: 1,
            fact_references: 0,
            mounted_work_references: 1,
            conflict_policy: super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        }
    }

    pub(in crate::runtime) const fn focus(
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            family: crate::capability::UiRuntimeServiceFamily::Focus,
            scope,
            requirements: 1,
            fact_references: 1,
            mounted_work_references: 0,
            conflict_policy: super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        }
    }

    pub(in crate::runtime) const fn motion(
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            family: crate::capability::UiRuntimeServiceFamily::Motion,
            scope,
            requirements: 1,
            fact_references: 1,
            mounted_work_references: 0,
            conflict_policy: super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        }
    }

    #[cfg(test)]
    pub(in crate::runtime) fn recorded_fixture(
        family: crate::capability::UiRuntimeServiceFamily,
        scope: u64,
        requirements: u8,
        fact_references: u16,
        mounted_work_references: u16,
    ) -> Self {
        Self {
            family,
            scope: super::super::UiServiceProposalOccupancyScopeIdentity::for_test(scope),
            requirements,
            fact_references,
            mounted_work_references,
            conflict_policy: super::super::UiServiceProposalConflictPolicy::RejectOccupied,
        }
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn with_conflict_policy(
        mut self,
        conflict_policy: super::super::UiServiceProposalConflictPolicy,
    ) -> Self {
        self.conflict_policy = conflict_policy;
        self
    }

    pub(in crate::runtime) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        self.family
    }

    pub(in crate::runtime) const fn scope(
        self,
    ) -> super::super::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn requirements(self) -> u8 {
        self.requirements
    }

    pub(in crate::runtime) const fn fact_references(self) -> u16 {
        self.fact_references
    }

    pub(in crate::runtime) const fn mounted_work_references(self) -> u16 {
        self.mounted_work_references
    }

    pub(in crate::runtime) const fn conflict_policy(
        self,
    ) -> super::super::UiServiceProposalConflictPolicy {
        self.conflict_policy
    }
}

#[cfg(test)]
mod tests {
    use super::UiServiceFamilyProposal;

    #[test]
    fn recorded_family_proposal_is_typed_and_contains_no_family_payload() {
        let proposal = UiServiceFamilyProposal::recorded_fixture(
            crate::capability::UiRuntimeServiceFamily::Portal,
            7,
            2,
            3,
            4,
        );
        assert_eq!(
            proposal.family(),
            crate::capability::UiRuntimeServiceFamily::Portal
        );
        assert_eq!(
            proposal.scope(),
            crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity::for_test(7)
        );
        assert_eq!(proposal.requirements(), 2);
        assert_eq!(proposal.fact_references(), 3);
        assert_eq!(proposal.mounted_work_references(), 4);
    }
}
