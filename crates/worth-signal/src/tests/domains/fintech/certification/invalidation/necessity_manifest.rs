use std::collections::{BTreeMap, BTreeSet};

use crate::tests::domains::fintech::world::{
    FinancialAspect, FinancialComparatorPolicy, FinancialConditionPolicy, FinancialConsumerRole,
    FinancialOutputEquivalencePolicy, FinancialWorldDefinition, MarketFactorKey, PositionKind,
    SemanticOutputKey,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialNecessityReason {
    AuthoritativeFactorMutation {
        partition: &'static str,
        detail: &'static str,
        input_aspect: FinancialAspect,
    },
    PositionAspectTranslation {
        input: FinancialAspect,
        output: FinancialAspect,
    },
    ConsumerAdmission {
        dependency: FinancialAspect,
        condition: FinancialConditionPolicy,
        comparator: FinancialComparatorPolicy,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialNecessityEntry {
    pub(in crate::tests::domains::fintech) work: SemanticOutputKey,
    pub(in crate::tests::domains::fintech) reason: FinancialNecessityReason,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialNecessityManifest {
    mutation: MarketFactorKey,
    entries: Vec<FinancialNecessityEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialNecessityEvidence {
    required_work: BTreeSet<SemanticOutputKey>,
    baseline_values: BTreeMap<SemanticOutputKey, i64>,
}

impl FinancialNecessityEvidence {
    pub(in crate::tests::domains::fintech) fn for_mutation(
        definition: &FinancialWorldDefinition,
        mutation: MarketFactorKey,
    ) -> Self {
        Self {
            required_work: FinancialNecessityManifest::derive(definition, mutation).required_work(),
            baseline_values: baseline_values(definition),
        }
    }

    pub(in crate::tests::domains::fintech) fn for_revision_delta(
        definition: &FinancialWorldDefinition,
        mutation: MarketFactorKey,
        revision_delta: u64,
    ) -> Self {
        Self {
            required_work: FinancialNecessityManifest::derive_for_revision_delta(
                definition,
                mutation,
                revision_delta,
            )
            .required_work(),
            baseline_values: baseline_values(definition),
        }
    }

    pub(in crate::tests::domains::fintech) fn for_mutations(
        definition: &FinancialWorldDefinition,
        mutations: impl IntoIterator<Item = MarketFactorKey>,
    ) -> Self {
        Self {
            required_work: mutations
                .into_iter()
                .flat_map(|mutation| {
                    FinancialNecessityManifest::derive(definition, mutation).required_work()
                })
                .collect(),
            baseline_values: baseline_values(definition),
        }
    }

    pub(in crate::tests::domains::fintech) fn for_dependency_rewire(
        baseline: &FinancialWorldDefinition,
        before: &FinancialWorldDefinition,
        after: &FinancialWorldDefinition,
        cause_factor: MarketFactorKey,
        instrument: super::super::super::world::InstrumentId,
    ) -> Self {
        let mut required_work =
            FinancialNecessityManifest::derive_dependency_rewire(before, after, instrument);
        required_work.insert(SemanticOutputKey::Factor(cause_factor));
        Self {
            required_work,
            baseline_values: baseline_values(baseline),
        }
    }

    pub(super) fn required_work(&self) -> &BTreeSet<SemanticOutputKey> {
        &self.required_work
    }

    pub(super) fn expected_committed_values(
        &self,
        fresh: &super::FreshFinancialRecompute,
    ) -> BTreeMap<SemanticOutputKey, i64> {
        let mut expected = self.baseline_values.clone();
        let fresh_values = fresh.economic_snapshot().semantic_value_map();
        for key in &self.required_work {
            if let Some(value) = fresh_values.get(key) {
                expected.insert(*key, *value);
            }
        }
        expected
    }
}

fn baseline_values(definition: &FinancialWorldDefinition) -> BTreeMap<SemanticOutputKey, i64> {
    super::FreshFinancialRecompute::run(definition)
        .economic_snapshot()
        .semantic_value_map()
}

impl FinancialNecessityManifest {
    pub(in crate::tests::domains::fintech) fn derive(
        definition: &FinancialWorldDefinition,
        mutation: MarketFactorKey,
    ) -> Self {
        let mut entries = vec![FinancialNecessityEntry {
            work: SemanticOutputKey::Factor(mutation),
            reason: FinancialNecessityReason::AuthoritativeFactorMutation {
                partition: mutation.partition().0,
                detail: mutation.partition().1,
                input_aspect: factor_aspect(mutation),
            },
        }];
        for position in definition.positions() {
            let Some(subscription) = position
                .subscriptions
                .iter()
                .find(|subscription| subscription.factor == mutation)
            else {
                continue;
            };
            entries.push(FinancialNecessityEntry {
                work: SemanticOutputKey::Valuation(position.instrument),
                reason: FinancialNecessityReason::PositionAspectTranslation {
                    input: subscription.input_aspect,
                    output: FinancialAspect::Price,
                },
            });
            entries.push(FinancialNecessityEntry {
                work: SemanticOutputKey::Risk(position.instrument),
                reason: FinancialNecessityReason::PositionAspectTranslation {
                    input: FinancialAspect::Price,
                    output: FinancialAspect::Risk,
                },
            });
            if !mutation_changes_risk(&position.kind, mutation) {
                continue;
            }
            for consumer in definition
                .consumers()
                .iter()
                .filter(|consumer| consumer.position == position.instrument)
            {
                if !condition_can_admit(consumer.condition, consumer.dependency_aspect) {
                    continue;
                }
                entries.push(FinancialNecessityEntry {
                    work: SemanticOutputKey::Consumer(consumer.role),
                    reason: FinancialNecessityReason::ConsumerAdmission {
                        dependency: consumer.dependency_aspect,
                        condition: consumer.condition,
                        comparator: consumer.comparator,
                    },
                });
            }
        }
        Self { mutation, entries }
    }

    pub(in crate::tests::domains::fintech) fn derive_for_revision_delta(
        definition: &FinancialWorldDefinition,
        mutation: MarketFactorKey,
        revision_delta: u64,
    ) -> Self {
        let mut manifest = Self::derive(definition, mutation);
        manifest.entries.retain(|entry| {
            matches!(entry.work, SemanticOutputKey::Factor(_))
                || factor_change_propagates(definition, mutation, revision_delta)
                    && consumer_change_is_meaningful(entry, revision_delta)
        });
        manifest
    }

    pub(in crate::tests::domains::fintech) fn derive_dependency_rewire(
        before: &FinancialWorldDefinition,
        after: &FinancialWorldDefinition,
        instrument: super::super::super::world::InstrumentId,
    ) -> BTreeSet<SemanticOutputKey> {
        let before_result = super::super::super::world::reference_position_result(
            before.market(),
            before.position(instrument),
        );
        let after_result = super::super::super::world::reference_position_result(
            after.market(),
            after.position(instrument),
        );
        let mut required = BTreeSet::from([
            SemanticOutputKey::Valuation(instrument),
            SemanticOutputKey::Risk(instrument),
        ]);
        if before_result.risk != after_result.risk {
            required.extend(
                after
                    .consumers()
                    .iter()
                    .filter(|consumer| consumer.position == instrument)
                    .filter(|consumer| {
                        condition_can_admit(consumer.condition, consumer.dependency_aspect)
                    })
                    .map(|consumer| SemanticOutputKey::Consumer(consumer.role)),
            );
        }
        required
    }

    pub(in crate::tests::domains::fintech) const fn mutation(&self) -> MarketFactorKey {
        self.mutation
    }

    pub(in crate::tests::domains::fintech) fn entries(&self) -> &[FinancialNecessityEntry] {
        &self.entries
    }

    pub(in crate::tests::domains::fintech) fn required_work(&self) -> BTreeSet<SemanticOutputKey> {
        self.entries.iter().map(|entry| entry.work).collect()
    }
}

fn mutation_changes_risk(kind: &PositionKind, mutation: MarketFactorKey) -> bool {
    match kind {
        PositionKind::ZeroCouponBond { quote, curve, .. } => {
            matches!(
                mutation,
                MarketFactorKey::Quote(candidate) if candidate == *quote
            ) || matches!(
                mutation,
                MarketFactorKey::Curve(candidate) if candidate == *curve
            )
        }
        PositionKind::FxForward {
            pair,
            domestic_curve,
            foreign_curve,
            ..
        } => {
            matches!(mutation, MarketFactorKey::FxSpot(candidate) if candidate == *pair)
                || matches!(mutation, MarketFactorKey::Curve(candidate) if candidate == *domestic_curve || candidate == *foreign_curve)
        }
        PositionKind::VarianceSwap { volatility, .. } => matches!(
            mutation,
            MarketFactorKey::Volatility(candidate) if candidate == *volatility
        ),
    }
}

fn condition_can_admit(condition: FinancialConditionPolicy, dependency: FinancialAspect) -> bool {
    match condition {
        FinancialConditionPolicy::AspectFilter(filter) => filter == dependency,
        FinancialConditionPolicy::DeltaThreshold(_) => true,
    }
}

fn factor_change_propagates(
    definition: &FinancialWorldDefinition,
    mutation: MarketFactorKey,
    revision_delta: u64,
) -> bool {
    match definition.factor_output_equivalence(mutation) {
        FinancialOutputEquivalencePolicy::Exact => revision_delta != 0,
        FinancialOutputEquivalencePolicy::Tolerance { epsilon } => revision_delta > epsilon,
    }
}

fn consumer_change_is_meaningful(entry: &FinancialNecessityEntry, revision_delta: u64) -> bool {
    let FinancialNecessityReason::ConsumerAdmission {
        condition,
        comparator,
        ..
    } = entry.reason
    else {
        return true;
    };
    if matches!(
        condition,
        FinancialConditionPolicy::DeltaThreshold(threshold) if revision_delta <= threshold
    ) {
        return false;
    }
    match comparator {
        FinancialComparatorPolicy::Exact => revision_delta != 0,
        FinancialComparatorPolicy::Tolerance { epsilon }
        | FinancialComparatorPolicy::InstalledTolerance { epsilon } => revision_delta > epsilon,
    }
}

fn factor_aspect(factor: MarketFactorKey) -> FinancialAspect {
    match factor {
        MarketFactorKey::Quote(_) | MarketFactorKey::FxSpot(_) => FinancialAspect::Price,
        MarketFactorKey::Curve(_) => FinancialAspect::Curve,
        MarketFactorKey::Volatility(_) => FinancialAspect::Volatility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::world::{FxPair, InstrumentId};

    #[test]
    fn necessity_manifest_uses_financial_subscriptions_not_runtime_reachability() {
        let definition = FinancialWorldDefinition::deterministic(41);
        let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
        let manifest = FinancialNecessityManifest::derive(&definition, factor);
        let fx = InstrumentId("EURUSD-1Y-FWD");

        assert_eq!(manifest.mutation(), factor);
        assert_eq!(
            manifest.required_work(),
            BTreeSet::from([
                SemanticOutputKey::Factor(factor),
                SemanticOutputKey::Valuation(fx),
                SemanticOutputKey::Risk(fx),
                SemanticOutputKey::Consumer(FinancialConsumerRole::RiskMatched),
            ])
        );
        assert!(!manifest
            .required_work()
            .contains(&SemanticOutputKey::Consumer(
                FinancialConsumerRole::RiskUnmatched
            )));
        assert!(manifest.entries().iter().any(|entry| matches!(
            entry.reason,
            FinancialNecessityReason::AuthoritativeFactorMutation {
                partition: "fx",
                detail: "eur-usd",
                ..
            }
        )));
    }
}
