use std::collections::BTreeSet;

use super::{FinancialAspect, FinancialLocalityFormula, FinancialLocalityOutput};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityAdmissionPolicy {
    Always,
    ChangedSubscribedAspect(BTreeSet<FinancialAspect>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityComparisonPolicy {
    ExactEconomicRevision,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityOutputPolicy {
    ExactEconomicRevision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityExecutionPolicy {
    pub(in crate::tests::domains::fintech) admission: FinancialLocalityAdmissionPolicy,
    pub(in crate::tests::domains::fintech) dependency_comparison: FinancialLocalityComparisonPolicy,
    pub(in crate::tests::domains::fintech) output_equivalence: FinancialLocalityOutputPolicy,
}

impl FinancialLocalityOutput {
    pub(in crate::tests::domains::fintech) fn execution_policy(
        &self,
    ) -> FinancialLocalityExecutionPolicy {
        let admission = match self.formula {
            FinancialLocalityFormula::MarketSource { .. } => {
                FinancialLocalityAdmissionPolicy::Always
            }
            FinancialLocalityFormula::LinearDependency { .. }
            | FinancialLocalityFormula::StableControl { .. } => {
                FinancialLocalityAdmissionPolicy::ChangedSubscribedAspect(
                    self.subscriptions
                        .iter()
                        .map(|subscription| subscription.input_aspect)
                        .collect(),
                )
            }
        };
        FinancialLocalityExecutionPolicy {
            admission,
            dependency_comparison: FinancialLocalityComparisonPolicy::ExactEconomicRevision,
            output_equivalence: FinancialLocalityOutputPolicy::ExactEconomicRevision,
        }
    }
}
