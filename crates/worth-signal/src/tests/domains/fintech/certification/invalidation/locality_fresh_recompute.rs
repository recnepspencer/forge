use std::collections::{BTreeMap, BTreeSet};

use crate::tests::domains::fintech::world::{
    FinancialLocalityAction, FinancialLocalityActionTrace, FinancialLocalityDefinition,
    FinancialLocalityFormula, FinancialLocalityMutation, FinancialLocalityScenario,
    LocalitySemanticOutputId,
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
        Self::run_for_trace(definition, &definition.action_traces()[0])
    }

    pub(in crate::tests::domains::fintech) fn run_for_trace(
        definition: &FinancialLocalityDefinition,
        trace: &FinancialLocalityActionTrace,
    ) -> Self {
        let baseline = baseline_values(definition);
        let mut outputs = definition.outputs().to_vec();
        let mut shocked = baseline.clone();
        let mut publications = Vec::new();
        let mut current_commit_group = BTreeSet::new();
        let coalesce_commit_group =
            definition.scenario() != FinancialLocalityScenario::PortfolioDependencyChurn;
        for action in trace.actions() {
            match *action {
                FinancialLocalityAction::CommitFactor(mutation) => {
                    if !coalesce_commit_group || current_commit_group.insert(mutation.producer) {
                        apply_publication(&outputs, &mut shocked, mutation);
                    }
                    publications.push(mutation);
                    recompute_dependencies(&outputs, &baseline, &mut shocked, &publications);
                }
                FinancialLocalityAction::AcceptedOwnerMove { change, .. } => {
                    current_commit_group.clear();
                    let output = &mut outputs[change.target.ordinal() as usize];
                    assert_eq!(output.owner, change.before_owner);
                    assert_eq!(output.subscriptions, [change.before_subscription]);
                    output.owner = change.after_owner;
                    output.subscriptions = vec![change.after_subscription];
                    recompute_dependencies(&outputs, &baseline, &mut shocked, &publications);
                }
                FinancialLocalityAction::AcceptedDependencyRemoval {
                    removed_subscription,
                    structural,
                    ..
                } => {
                    current_commit_group.clear();
                    let output = &mut outputs[structural.target.ordinal() as usize];
                    assert_eq!(output.subscriptions, [removed_subscription]);
                    output.subscriptions.clear();
                    recompute_dependencies(&outputs, &baseline, &mut shocked, &publications);
                }
                FinancialLocalityAction::AcceptedDependencyRecreation {
                    subscription,
                    structural,
                    ..
                } => {
                    current_commit_group.clear();
                    let output = &mut outputs[structural.target.ordinal() as usize];
                    assert!(output.subscriptions.is_empty());
                    output.subscriptions.push(subscription);
                    recompute_dependencies(&outputs, &baseline, &mut shocked, &publications);
                }
                _ => {
                    current_commit_group.clear();
                }
            }
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

fn baseline_values(
    definition: &FinancialLocalityDefinition,
) -> BTreeMap<LocalitySemanticOutputId, i64> {
    let mut baseline = BTreeMap::new();
    for output in definition.outputs() {
        baseline.insert(output.id, evaluate_formula(output, &baseline, false));
    }
    baseline
}

fn apply_publication(
    outputs: &[crate::tests::domains::fintech::world::FinancialLocalityOutput],
    values: &mut BTreeMap<LocalitySemanticOutputId, i64>,
    mutation: FinancialLocalityMutation,
) {
    let output = &outputs[mutation.producer.ordinal() as usize];
    let FinancialLocalityFormula::MarketSource { mutation_delta, .. } = output.formula else {
        panic!("fresh locality publication target is not a market source");
    };
    values.entry(mutation.producer).and_modify(|value| {
        *value = value
            .checked_add(mutation_delta)
            .expect("fresh locality publication overflow")
    });
}

fn recompute_dependencies(
    outputs: &[crate::tests::domains::fintech::world::FinancialLocalityOutput],
    baseline: &BTreeMap<LocalitySemanticOutputId, i64>,
    values: &mut BTreeMap<LocalitySemanticOutputId, i64>,
    publications: &[FinancialLocalityMutation],
) {
    for output in outputs {
        let value = match output.formula {
            FinancialLocalityFormula::MarketSource { .. } => continue,
            FinancialLocalityFormula::StableControl { retained_value } => retained_value,
            FinancialLocalityFormula::LinearDependency {
                multiplier_micros,
                basis_value,
            } => apply_linear(
                output
                    .subscriptions
                    .iter()
                    .map(|subscription| {
                        let source_was_published = publications
                            .iter()
                            .any(|mutation| mutation.producer == subscription.upstream);
                        let subscription_was_touched = publications.iter().any(|mutation| {
                            mutation.producer == subscription.upstream
                                && mutation.aspect == subscription.input_aspect
                                && scopes_overlap(mutation.scope, subscription.edge_scope)
                        });
                        if source_was_published && !subscription_was_touched {
                            baseline[&subscription.upstream]
                        } else {
                            values[&subscription.upstream]
                        }
                    })
                    .sum(),
                multiplier_micros,
                basis_value,
            ),
        };
        values.insert(output.id, value);
    }
}

fn scopes_overlap(
    left: Option<crate::tests::domains::fintech::world::LocalityScope>,
    right: Option<crate::tests::domains::fintech::world::LocalityScope>,
) -> bool {
    match (left, right) {
        (None, _) | (_, None) => true,
        (Some(left), Some(right)) if left.region != right.region => false,
        (Some(left), Some(right)) => {
            left.detail.is_none() || right.detail.is_none() || left.detail == right.detail
        }
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
                .subscriptions
                .iter()
                .map(|subscription| values[&subscription.upstream])
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
