use std::collections::{BTreeMap, BTreeSet};

use crate::tests::domains::fintech::world::{
    FinancialLocalityDefinition, FinancialLocalityFormula, LocalityScope, LocalitySemanticOutputId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FreshFinancialLocalityRecompute {
    baseline: BTreeMap<LocalitySemanticOutputId, i64>,
    shocked: BTreeMap<LocalitySemanticOutputId, i64>,
}

impl FreshFinancialLocalityRecompute {
    pub(in crate::tests::domains::fintech) fn run(
        definition: &FinancialLocalityDefinition,
    ) -> Self {
        let mutation = definition.mutation();
        let mut baseline = BTreeMap::new();
        let mut shocked = BTreeMap::new();
        for output in definition.outputs() {
            let baseline_value = evaluate_formula(output, &baseline, false);
            baseline.insert(output.id, baseline_value);
            let shocked_value = match output.formula {
                FinancialLocalityFormula::MarketSource {
                    baseline_value,
                    mutation_delta,
                } if output.id == mutation.producer => baseline_value
                    .checked_add(mutation_delta)
                    .expect("fresh locality source shock overflow"),
                FinancialLocalityFormula::MarketSource { baseline_value, .. } => baseline_value,
                FinancialLocalityFormula::StableControl { retained_value } => retained_value,
                FinancialLocalityFormula::LinearDependency {
                    multiplier_micros,
                    basis_value,
                } => {
                    let inputs = output
                        .dependencies
                        .iter()
                        .map(|dependency| {
                            if dependency.producer == mutation.producer
                                && (dependency.aspect != mutation.aspect
                                    || !scopes_overlap(dependency.edge_scope, mutation.scope))
                            {
                                baseline[&dependency.producer]
                            } else {
                                shocked[&dependency.producer]
                            }
                        })
                        .sum();
                    apply_linear(inputs, multiplier_micros, basis_value)
                }
            };
            shocked.insert(output.id, shocked_value);
        }
        Self { baseline, shocked }
    }

    pub(in crate::tests::domains::fintech) fn baseline_values(
        &self,
    ) -> &BTreeMap<LocalitySemanticOutputId, i64> {
        &self.baseline
    }

    pub(in crate::tests::domains::fintech) fn shocked_values(
        &self,
    ) -> &BTreeMap<LocalitySemanticOutputId, i64> {
        &self.shocked
    }

    pub(in crate::tests::domains::fintech) fn changed_outputs(
        &self,
    ) -> BTreeSet<LocalitySemanticOutputId> {
        self.baseline
            .iter()
            .filter_map(|(output, before)| (self.shocked[output] != *before).then_some(*output))
            .collect()
    }
}

fn evaluate_formula(
    output: &crate::tests::domains::fintech::world::FinancialLocalityOutput,
    values: &BTreeMap<LocalitySemanticOutputId, i64>,
    shocked: bool,
) -> i64 {
    debug_assert!(!shocked, "baseline evaluator receives baseline posture");
    match output.formula {
        FinancialLocalityFormula::MarketSource { baseline_value, .. } => baseline_value,
        FinancialLocalityFormula::StableControl { retained_value } => retained_value,
        FinancialLocalityFormula::LinearDependency {
            multiplier_micros,
            basis_value,
        } => {
            let inputs = output
                .dependencies
                .iter()
                .map(|dependency| values[&dependency.producer])
                .sum();
            apply_linear(inputs, multiplier_micros, basis_value)
        }
    }
}

fn apply_linear(inputs: i64, multiplier_micros: i64, basis_value: i64) -> i64 {
    inputs
        .checked_mul(multiplier_micros)
        .and_then(|value| value.checked_div(1_000_000))
        .and_then(|value| value.checked_add(basis_value))
        .expect("fresh locality financial formula overflow")
}

fn scopes_overlap(left: Option<LocalityScope>, right: Option<LocalityScope>) -> bool {
    match (left, right) {
        (None, _) | (_, None) => true,
        (Some(left), Some(right)) if left.region != right.region => false,
        (Some(left), Some(right)) => {
            left.detail.is_none() || right.detail.is_none() || left.detail == right.detail
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::world::{FinancialWorldDefinition, SparseFanoutAxis};

    #[test]
    fn fresh_locality_recompute_is_driven_only_by_financial_definition_and_scope() {
        let definition = FinancialWorldDefinition::sparse_book_fanout(
            41,
            64,
            SparseFanoutAxis::RejectedDescendants,
        );
        let locality = definition.locality().unwrap();
        let fresh = FreshFinancialLocalityRecompute::run(locality);

        assert_eq!(fresh.baseline_values().len(), 64);
        assert_eq!(fresh.shocked_values().len(), 64);
        assert!(fresh.changed_outputs().len() < 64);
    }
}
