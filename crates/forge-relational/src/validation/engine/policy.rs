use crate::validation::data::InvariantCostClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantExecutionPolicy {
    AllowAll,
    MaxCost(InvariantCostClass),
}

impl InvariantExecutionPolicy {
    pub fn allows(self, cost: InvariantCostClass) -> bool {
        match self {
            Self::AllowAll => true,
            Self::MaxCost(limit) => rank(cost) <= rank(limit),
        }
    }
}

const fn rank(cost: InvariantCostClass) -> u8 {
    match cost {
        InvariantCostClass::Constant => 0,
        InvariantCostClass::TargetedScan => 1,
        InvariantCostClass::FullScan => 2,
        InvariantCostClass::HarnessHeavy => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantExecutionPolicy;
    use crate::validation::data::InvariantCostClass;

    #[test]
    fn max_cost_policy_filters_more_expensive_rules() {
        let policy = InvariantExecutionPolicy::MaxCost(InvariantCostClass::TargetedScan);
        assert!(policy.allows(InvariantCostClass::Constant));
        assert!(policy.allows(InvariantCostClass::TargetedScan));
        assert!(!policy.allows(InvariantCostClass::FullScan));
        assert!(!policy.allows(InvariantCostClass::HarnessHeavy));
    }
}
