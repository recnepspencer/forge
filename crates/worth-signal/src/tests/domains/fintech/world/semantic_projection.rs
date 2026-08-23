use std::collections::BTreeMap;

use super::{
    FinancialAmount, FinancialAspect, FinancialConsumerRole, FinancialMarketInputs,
    FinancialPosition, InstrumentId, MarketFactorKey, PositionFinancialResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SemanticOutputKey {
    Factor(MarketFactorKey),
    Valuation(InstrumentId),
    Risk(InstrumentId),
    Consumer(FinancialConsumerRole),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct ProjectedSemanticOutput {
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) revision: u64,
    pub(in crate::tests::domains::fintech) canonical_financial_value: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FinancialEconomicSnapshot {
    factors: BTreeMap<MarketFactorKey, i64>,
    positions: BTreeMap<InstrumentId, PositionFinancialResult>,
    consumers: BTreeMap<FinancialConsumerRole, FinancialAmount>,
}

impl FinancialEconomicSnapshot {
    pub(in crate::tests::domains::fintech) fn from_results(
        market: &FinancialMarketInputs,
        positions: &[FinancialPosition],
        results: BTreeMap<InstrumentId, PositionFinancialResult>,
        consumers: impl IntoIterator<Item = (FinancialConsumerRole, InstrumentId)>,
    ) -> Self {
        let factors = market
            .factors()
            .map(|factor| (factor, market.value(factor).scaled()))
            .collect();
        let consumers = consumers
            .into_iter()
            .map(|(role, instrument)| {
                let result = results
                    .get(&instrument)
                    .expect("consumer position must have a financial result");
                (role, result.risk)
            })
            .collect();
        debug_assert_eq!(positions.len(), results.len());
        Self {
            factors,
            positions: results,
            consumers,
        }
    }

    pub(in crate::tests::domains::fintech) fn semantic_values(
        &self,
    ) -> impl Iterator<Item = (SemanticOutputKey, FinancialAspect, i64)> + '_ {
        self.factors
            .iter()
            .map(|(factor, value)| {
                (
                    SemanticOutputKey::Factor(*factor),
                    factor_output_aspect(*factor),
                    *value,
                )
            })
            .chain(self.positions.iter().flat_map(|(instrument, result)| {
                [
                    (
                        SemanticOutputKey::Valuation(*instrument),
                        FinancialAspect::Price,
                        result.value.micros(),
                    ),
                    (
                        SemanticOutputKey::Risk(*instrument),
                        FinancialAspect::Risk,
                        result.risk.micros(),
                    ),
                ]
            }))
            .chain(self.consumers.iter().map(|(role, value)| {
                (
                    SemanticOutputKey::Consumer(*role),
                    FinancialAspect::Alert,
                    value.micros(),
                )
            }))
    }

    pub(crate) fn semantic_value_map(&self) -> BTreeMap<SemanticOutputKey, i64> {
        self.semantic_values()
            .map(|(key, _, value)| (key, value))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FinancialSemanticProjection {
    outputs: BTreeMap<SemanticOutputKey, ProjectedSemanticOutput>,
}

impl FinancialSemanticProjection {
    pub(in crate::tests::domains::fintech) fn initial(
        snapshot: &FinancialEconomicSnapshot,
    ) -> Self {
        Self {
            outputs: snapshot
                .semantic_values()
                .map(|(key, aspect, canonical_financial_value)| {
                    (
                        key,
                        ProjectedSemanticOutput {
                            aspect,
                            revision: 1,
                            canonical_financial_value,
                        },
                    )
                })
                .collect(),
        }
    }

    pub(in crate::tests::domains::fintech) fn advance(
        &self,
        snapshot: &FinancialEconomicSnapshot,
    ) -> Self {
        let outputs = snapshot
            .semantic_values()
            .map(|(key, aspect, canonical_financial_value)| {
                let previous = self
                    .outputs
                    .get(&key)
                    .expect("advanced projection must retain its semantic keys");
                assert_eq!(previous.aspect, aspect, "semantic aspect changed in place");
                let revision = if previous.canonical_financial_value == canonical_financial_value {
                    previous.revision
                } else {
                    previous
                        .revision
                        .checked_add(1)
                        .expect("financial semantic revision overflow")
                };
                (
                    key,
                    ProjectedSemanticOutput {
                        aspect,
                        revision,
                        canonical_financial_value,
                    },
                )
            })
            .collect();
        Self { outputs }
    }

    pub(in crate::tests::domains::fintech) fn output(
        &self,
        key: SemanticOutputKey,
    ) -> ProjectedSemanticOutput {
        self.outputs[&key]
    }

    pub(in crate::tests::domains::fintech) fn iter(
        &self,
    ) -> impl Iterator<Item = (SemanticOutputKey, ProjectedSemanticOutput)> + '_ {
        self.outputs.iter().map(|(key, output)| (*key, *output))
    }
}

fn factor_output_aspect(factor: MarketFactorKey) -> FinancialAspect {
    match factor {
        MarketFactorKey::Quote(_) | MarketFactorKey::FxSpot(_) => FinancialAspect::Price,
        MarketFactorKey::Curve(_) => FinancialAspect::Curve,
        MarketFactorKey::Volatility(_) => FinancialAspect::Volatility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::certification::invalidation::FreshFinancialRecompute;
    use crate::tests::domains::fintech::world::{
        FinancialWorldDefinition, FxPair, MarketFactorKey,
    };

    #[test]
    fn semantic_projection_advances_only_changed_economic_results() {
        let base = FinancialWorldDefinition::deterministic(41);
        let shocked =
            base.with_market_factor_delta(MarketFactorKey::FxSpot(FxPair::EurUsd), 20_000);
        let before = FreshFinancialRecompute::run(&base).economic_snapshot();
        let after = FreshFinancialRecompute::run(&shocked).economic_snapshot();
        let initial = FinancialSemanticProjection::initial(&before);
        let advanced = initial.advance(&after);
        let fx = crate::tests::domains::fintech::world::InstrumentId("EURUSD-1Y-FWD");
        let bond = crate::tests::domains::fintech::world::InstrumentId("UST-2Y-ZERO");

        assert_eq!(
            advanced.output(SemanticOutputKey::Risk(fx)).revision,
            initial.output(SemanticOutputKey::Risk(fx)).revision + 1
        );
        assert_eq!(
            advanced.output(SemanticOutputKey::Risk(bond)).revision,
            initial.output(SemanticOutputKey::Risk(bond)).revision
        );
    }
}
