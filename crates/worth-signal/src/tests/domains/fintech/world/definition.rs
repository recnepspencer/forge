use super::super::{regimes::MarketRegime, scales::FintechScale};
use super::{
    CurveBucket, FinancialAspect, FinancialComparatorPolicy, FinancialMarketInputs,
    FinancialOutputEquivalencePolicy, FinancialPosition, InstrumentId, MarketFactorKey,
    PositionKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialConsumerRole {
    RiskMatched,
    RiskUnmatched,
    RiskTolerance,
    RiskInstalled,
    RiskThreshold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialConditionPolicy {
    AspectFilter(FinancialAspect),
    DeltaThreshold(u64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialConsumerDeclaration {
    pub(in crate::tests::domains::fintech) role: FinancialConsumerRole,
    pub(in crate::tests::domains::fintech) position: InstrumentId,
    pub(in crate::tests::domains::fintech) dependency_aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) condition: FinancialConditionPolicy,
    pub(in crate::tests::domains::fintech) comparator: FinancialComparatorPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialWorldDefinition {
    seed: u64,
    fixture_scale: FintechScale,
    fixture_regime: MarketRegime,
    market: FinancialMarketInputs,
    positions: Vec<FinancialPosition>,
    consumers: Vec<FinancialConsumerDeclaration>,
    factor_output_equivalence:
        std::collections::BTreeMap<MarketFactorKey, FinancialOutputEquivalencePolicy>,
    producer_local_factor_slots: bool,
}

impl FinancialWorldDefinition {
    pub(in crate::tests::domains::fintech) fn deterministic(seed: u64) -> Self {
        let positions = vec![
            FinancialPosition::rates_bond(),
            FinancialPosition::fx_forward(),
            FinancialPosition::variance_swap(),
        ];
        let fx_position = positions[1].instrument;
        Self {
            seed,
            fixture_scale: FintechScale::smoke(),
            fixture_regime: MarketRegime::Calm,
            market: FinancialMarketInputs::deterministic(seed),
            positions,
            consumers: vec![
                FinancialConsumerDeclaration {
                    role: FinancialConsumerRole::RiskMatched,
                    position: fx_position,
                    dependency_aspect: FinancialAspect::Risk,
                    condition: FinancialConditionPolicy::AspectFilter(FinancialAspect::Risk),
                    comparator: FinancialComparatorPolicy::Exact,
                },
                FinancialConsumerDeclaration {
                    role: FinancialConsumerRole::RiskUnmatched,
                    position: fx_position,
                    dependency_aspect: FinancialAspect::Risk,
                    condition: FinancialConditionPolicy::AspectFilter(FinancialAspect::Volatility),
                    comparator: FinancialComparatorPolicy::Tolerance { epsilon: 0 },
                },
            ],
            factor_output_equivalence: std::collections::BTreeMap::new(),
            producer_local_factor_slots: false,
        }
    }

    pub(in crate::tests::domains::fintech) fn runtime_fixture(
        scale: FintechScale,
        regime: MarketRegime,
        seed: u64,
    ) -> Self {
        let mut definition = Self::deterministic(seed);
        definition.fixture_scale = scale;
        definition.fixture_regime = regime;
        definition
    }

    pub(in crate::tests::domains::fintech) fn comparator_courtroom(seed: u64) -> Self {
        let mut definition = Self::deterministic(seed);
        let position = definition.positions[1].instrument;
        definition.consumers.extend([
            FinancialConsumerDeclaration {
                role: FinancialConsumerRole::RiskTolerance,
                position,
                dependency_aspect: FinancialAspect::Risk,
                condition: FinancialConditionPolicy::AspectFilter(FinancialAspect::Risk),
                comparator: FinancialComparatorPolicy::Tolerance { epsilon: 5 },
            },
            FinancialConsumerDeclaration {
                role: FinancialConsumerRole::RiskInstalled,
                position,
                dependency_aspect: FinancialAspect::Risk,
                condition: FinancialConditionPolicy::AspectFilter(FinancialAspect::Risk),
                comparator: FinancialComparatorPolicy::InstalledTolerance { epsilon: 5 },
            },
        ]);
        definition
    }

    pub(in crate::tests::domains::fintech) fn gated_courtroom(seed: u64) -> Self {
        let mut definition = Self::deterministic(seed);
        let position = definition.positions[1].instrument;
        definition.consumers = vec![FinancialConsumerDeclaration {
            role: FinancialConsumerRole::RiskThreshold,
            position,
            dependency_aspect: FinancialAspect::Risk,
            condition: FinancialConditionPolicy::DeltaThreshold(2),
            comparator: FinancialComparatorPolicy::Exact,
        }];
        definition
    }

    pub(in crate::tests::domains::fintech) fn partition_courtroom(seed: u64) -> Self {
        let mut definition = Self::deterministic(seed);
        let position = definition.positions[1].instrument;
        definition.consumers = vec![FinancialConsumerDeclaration {
            role: FinancialConsumerRole::RiskThreshold,
            position,
            dependency_aspect: FinancialAspect::Risk,
            condition: FinancialConditionPolicy::DeltaThreshold(0),
            comparator: FinancialComparatorPolicy::Exact,
        }];
        definition
    }

    pub(in crate::tests::domains::fintech) fn producer_local_slot_courtroom(seed: u64) -> Self {
        let mut definition = Self::deterministic(seed);
        definition.producer_local_factor_slots = true;
        definition
    }

    pub(in crate::tests::domains::fintech) const fn seed(&self) -> u64 {
        self.seed
    }

    pub(in crate::tests::domains::fintech) const fn fixture_scale(&self) -> FintechScale {
        self.fixture_scale
    }

    pub(in crate::tests::domains::fintech) const fn fixture_regime(&self) -> MarketRegime {
        self.fixture_regime
    }

    pub(in crate::tests::domains::fintech) fn market(&self) -> &FinancialMarketInputs {
        &self.market
    }

    pub(in crate::tests::domains::fintech) fn positions(&self) -> &[FinancialPosition] {
        &self.positions
    }

    pub(in crate::tests::domains::fintech) fn consumers(&self) -> &[FinancialConsumerDeclaration] {
        &self.consumers
    }

    pub(in crate::tests::domains::fintech) fn factor_output_equivalence(
        &self,
        factor: MarketFactorKey,
    ) -> FinancialOutputEquivalencePolicy {
        self.factor_output_equivalence
            .get(&factor)
            .copied()
            .unwrap_or(FinancialOutputEquivalencePolicy::Exact)
    }

    pub(in crate::tests::domains::fintech) fn factor_output_equivalence_policies(
        &self,
    ) -> impl Iterator<Item = FinancialOutputEquivalencePolicy> + '_ {
        self.market
            .factors()
            .map(|factor| self.factor_output_equivalence(factor))
    }

    pub(super) const fn uses_producer_local_factor_slots(&self) -> bool {
        self.producer_local_factor_slots
    }

    pub(in crate::tests::domains::fintech) fn position(
        &self,
        instrument: InstrumentId,
    ) -> &FinancialPosition {
        self.positions
            .iter()
            .find(|position| position.instrument == instrument)
            .expect("financial definition must own the requested position")
    }

    pub(in crate::tests::domains::fintech) fn with_market_factor_delta(
        &self,
        factor: MarketFactorKey,
        delta: i64,
    ) -> Self {
        let mut changed = self.clone();
        changed.market = self.market.with_factor_delta(factor, delta);
        changed
    }

    pub(in crate::tests::domains::fintech) fn with_factor_output_tolerance(
        mut self,
        factor: MarketFactorKey,
        epsilon: u64,
    ) -> Self {
        self.factor_output_equivalence.insert(
            factor,
            FinancialOutputEquivalencePolicy::Tolerance { epsilon },
        );
        self
    }

    pub(in crate::tests::domains::fintech) fn with_fx_forward_domestic_curve(
        &self,
        instrument: InstrumentId,
        next_curve: CurveBucket,
    ) -> Self {
        let mut changed = self.clone();
        let position = changed
            .positions
            .iter_mut()
            .find(|position| position.instrument == instrument)
            .expect("financial definition must own the rewired instrument");
        let PositionKind::FxForward { domestic_curve, .. } = &mut position.kind else {
            panic!("domestic-curve rewire requires an FX forward")
        };
        let previous = *domestic_curve;
        *domestic_curve = next_curve;
        let subscription = position
            .subscriptions
            .iter_mut()
            .find(|subscription| subscription.factor == MarketFactorKey::Curve(previous))
            .expect("FX forward must retain its domestic-curve subscription");
        subscription.factor = MarketFactorKey::Curve(next_curve);
        let (partition, detail) = subscription.factor.partition();
        subscription.partition = partition;
        subscription.detail = detail;
        changed
    }
}
