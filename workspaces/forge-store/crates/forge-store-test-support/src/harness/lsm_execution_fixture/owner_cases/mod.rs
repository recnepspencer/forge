mod execution;
mod execution_world;
mod membership;
mod open;
mod world;

use forge_store_layout_indexes::LsmExecutionOwnerCaseObservation;
use forge_store_lsm_authority::LsmMembershipOwnerCaseObservation;

#[derive(Debug)]
pub struct LsmOwnerCaseObservations {
    membership: Vec<LsmMembershipOwnerCaseObservation>,
    execution: Vec<LsmExecutionOwnerCaseObservation>,
}

impl LsmOwnerCaseObservations {
    pub fn membership(&self) -> impl Iterator<Item = LsmMembershipOwnerCaseObservation> + '_ {
        self.membership.iter().copied()
    }

    pub fn execution(&self) -> impl Iterator<Item = LsmExecutionOwnerCaseObservation> + '_ {
        self.execution.iter().copied()
    }
}

pub fn observe_lsm_owner_cases() -> LsmOwnerCaseObservations {
    LsmOwnerCaseObservations {
        membership: open::observe()
            .into_iter()
            .chain(membership::observe())
            .collect(),
        execution: execution::observe(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use forge_store_lsm_authority::{lsm_membership_owner_case_inventory, LsmMembershipOperation};

    use super::observe_lsm_owner_cases;

    #[test]
    fn implemented_membership_operations_equal_their_owner_inventories() {
        let observed = observe_lsm_owner_cases()
            .membership()
            .map(|case| case.id())
            .collect::<BTreeSet<_>>();
        let implemented = [
            LsmMembershipOperation::Open,
            LsmMembershipOperation::PersistRecord,
            LsmMembershipOperation::SelectCompaction,
            LsmMembershipOperation::ReplaceMembership,
            LsmMembershipOperation::LookupPublishedReplacement,
        ];
        let declared = lsm_membership_owner_case_inventory()
            .filter(|case| implemented.contains(&case.id().operation()))
            .map(|case| case.id())
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, declared);
    }

    #[test]
    fn layout_execution_operations_equal_their_owner_inventories() {
        let observed = observe_lsm_owner_cases()
            .execution()
            .map(|case| case.id())
            .collect::<BTreeSet<_>>();
        let declared = forge_store_layout_indexes::lsm_execution_owner_case_inventory()
            .map(|case| case.id())
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, declared);
    }
}
