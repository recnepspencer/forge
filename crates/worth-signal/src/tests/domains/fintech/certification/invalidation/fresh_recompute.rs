use std::collections::BTreeMap;

use crate::tests::domains::fintech::world::{
    reference_position_result, FinancialEconomicSnapshot, FinancialWorldDefinition, InstrumentId,
    PositionFinancialResult,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FreshFinancialRecompute {
    seed: u64,
    position_results: BTreeMap<InstrumentId, PositionFinancialResult>,
    economic_snapshot: FinancialEconomicSnapshot,
}

impl FreshFinancialRecompute {
    pub(crate) fn run(definition: &FinancialWorldDefinition) -> Self {
        let position_results = definition
            .positions()
            .iter()
            .map(|position| {
                (
                    position.instrument,
                    reference_position_result(definition.market(), position),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let economic_snapshot = FinancialEconomicSnapshot::from_results(
            definition.market(),
            definition.positions(),
            position_results.clone(),
            definition
                .consumers()
                .iter()
                .map(|consumer| (consumer.role, consumer.position)),
        );
        Self {
            seed: definition.seed(),
            position_results,
            economic_snapshot,
        }
    }

    pub(in crate::tests::domains::fintech) const fn seed(&self) -> u64 {
        self.seed
    }

    pub(in crate::tests::domains::fintech) fn result(
        &self,
        instrument: InstrumentId,
    ) -> PositionFinancialResult {
        self.position_results[&instrument]
    }

    pub(crate) fn economic_snapshot(&self) -> FinancialEconomicSnapshot {
        self.economic_snapshot.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::world::{FxPair, MarketFactorKey};

    #[test]
    fn fresh_recompute_reads_only_authoritative_financial_definition() {
        let base = FinancialWorldDefinition::deterministic(41);
        let shocked =
            base.with_market_factor_delta(MarketFactorKey::FxSpot(FxPair::EurUsd), 20_000);
        let before = FreshFinancialRecompute::run(&base);
        let after = FreshFinancialRecompute::run(&shocked);
        let fx = InstrumentId("EURUSD-1Y-FWD");
        let bond = InstrumentId("UST-2Y-ZERO");

        assert_eq!(before.seed(), 41);
        assert_ne!(before.result(fx), after.result(fx));
        assert_eq!(before.result(bond), after.result(bond));
    }
}
