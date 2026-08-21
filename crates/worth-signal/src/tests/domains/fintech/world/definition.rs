use std::collections::BTreeMap;

use super::super::{regimes::MarketRegime, scales::FintechScale};
use super::locality_scale::{LocalityScaleTuple, SparseFanoutAxis};
use super::{
    CurveBucket, FinancialAspect, FinancialComparatorPolicy, FinancialLocalityDefinition,
    FinancialMarketInputs, FinancialOutputEquivalencePolicy, FinancialPosition, InstrumentId,
    MarketFactorKey, PositionKind,
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
struct PortfolioFinancialWorldDefinition {
    seed: u64,
    fixture_scale: FintechScale,
    fixture_regime: MarketRegime,
    market: FinancialMarketInputs,
    positions: Vec<FinancialPosition>,
    consumers: Vec<FinancialConsumerDeclaration>,
    factor_output_equivalence: BTreeMap<MarketFactorKey, FinancialOutputEquivalencePolicy>,
    producer_local_factor_slots: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FinancialWorldDefinitionKind {
    Portfolio(PortfolioFinancialWorldDefinition),
    Locality(FinancialLocalityDefinition),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FinancialWorldDefinition {
    kind: FinancialWorldDefinitionKind,
}

impl FinancialWorldDefinition {
    pub(crate) fn deterministic(seed: u64) -> Self {
        let positions = vec![
            FinancialPosition::rates_bond(),
            FinancialPosition::fx_forward(),
            FinancialPosition::variance_swap(),
        ];
        let fx_position = positions[1].instrument;
        Self {
            kind: FinancialWorldDefinitionKind::Portfolio(PortfolioFinancialWorldDefinition {
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
                        condition: FinancialConditionPolicy::AspectFilter(
                            FinancialAspect::Volatility,
                        ),
                        comparator: FinancialComparatorPolicy::Tolerance { epsilon: 0 },
                    },
                ],
                factor_output_equivalence: BTreeMap::new(),
                producer_local_factor_slots: false,
            }),
        }
    }

    pub(in crate::tests::domains::fintech) fn sparse_book_fanout(
        seed: u64,
        total_outputs: u32,
        axis: SparseFanoutAxis,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::SparseBookFanout {
                    total_outputs,
                    axis,
                },
            )),
        }
    }

    pub(crate) fn partitioned_curve_universe(
        seed: u64,
        regions: u16,
        matching_memberships: u16,
        instruments_per_matching_region: u16,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::PartitionedCurveUniverse {
                    regions,
                    matching_memberships,
                    instruments_per_matching_region,
                },
            )),
        }
    }

    pub(in crate::tests::domains::fintech) fn locality_case(
        seed: u64,
        case: super::LocalityCaseContract,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(
                FinancialLocalityDefinition::generate_case(seed, case),
            ),
        }
    }

    pub(in crate::tests::domains::fintech) fn convergent_factor_batch(
        seed: u64,
        duplicate_admissions: u8,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::ConvergentFactorBatch {
                    producer_permutations: 24,
                    duplicate_admissions,
                    canonical_seeds: 1,
                },
            )),
        }
    }

    pub(crate) fn dense_market_close(
        seed: u64,
        total_outputs: u32,
        affected_ratio: super::DensityRatio,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::DenseMarketClose {
                    total_outputs,
                    affected_ratio,
                },
            )),
        }
    }

    pub(in crate::tests::domains::fintech) fn runtime_fixture(
        scale: FintechScale,
        regime: MarketRegime,
        seed: u64,
    ) -> Self {
        let mut definition = Self::deterministic(seed);
        let portfolio = definition.portfolio_mut();
        portfolio.fixture_scale = scale;
        portfolio.fixture_regime = regime;
        definition
    }

    pub(in crate::tests::domains::fintech) fn comparator_courtroom(seed: u64) -> Self {
        let mut definition = Self::deterministic(seed);
        let portfolio = definition.portfolio_mut();
        let position = portfolio.positions[1].instrument;
        portfolio.consumers.extend([
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
        let portfolio = definition.portfolio_mut();
        let position = portfolio.positions[1].instrument;
        portfolio.consumers = vec![FinancialConsumerDeclaration {
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
        let portfolio = definition.portfolio_mut();
        let position = portfolio.positions[1].instrument;
        portfolio.consumers = vec![FinancialConsumerDeclaration {
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
        definition.portfolio_mut().producer_local_factor_slots = true;
        definition
    }

    pub(in crate::tests::domains::fintech) const fn seed(&self) -> u64 {
        match &self.kind {
            FinancialWorldDefinitionKind::Portfolio(portfolio) => portfolio.seed,
            FinancialWorldDefinitionKind::Locality(locality) => locality.seed(),
        }
    }

    pub(in crate::tests::domains::fintech) fn fixture_scale(&self) -> FintechScale {
        self.portfolio().fixture_scale
    }

    pub(in crate::tests::domains::fintech) fn fixture_regime(&self) -> MarketRegime {
        self.portfolio().fixture_regime
    }

    pub(in crate::tests::domains::fintech) fn market(&self) -> &FinancialMarketInputs {
        &self.portfolio().market
    }

    pub(in crate::tests::domains::fintech) fn first_market_factor(&self) -> MarketFactorKey {
        self.market()
            .factors()
            .next()
            .expect("financial world must expose a market factor")
    }

    pub(in crate::tests::domains::fintech) fn positions(&self) -> &[FinancialPosition] {
        &self.portfolio().positions
    }

    pub(in crate::tests::domains::fintech) fn consumers(&self) -> &[FinancialConsumerDeclaration] {
        &self.portfolio().consumers
    }

    pub(in crate::tests::domains::fintech) fn locality(
        &self,
    ) -> Option<&FinancialLocalityDefinition> {
        match &self.kind {
            FinancialWorldDefinitionKind::Portfolio(_) => None,
            FinancialWorldDefinitionKind::Locality(locality) => Some(locality),
        }
    }

    pub(in crate::tests::domains::fintech) fn factor_output_equivalence(
        &self,
        factor: MarketFactorKey,
    ) -> FinancialOutputEquivalencePolicy {
        self.portfolio()
            .factor_output_equivalence
            .get(&factor)
            .copied()
            .unwrap_or(FinancialOutputEquivalencePolicy::Exact)
    }

    pub(in crate::tests::domains::fintech) fn factor_output_equivalence_policies(
        &self,
    ) -> impl Iterator<Item = FinancialOutputEquivalencePolicy> + '_ {
        self.market()
            .factors()
            .map(|factor| self.factor_output_equivalence(factor))
    }

    pub(super) fn uses_producer_local_factor_slots(&self) -> bool {
        self.portfolio().producer_local_factor_slots
    }

    pub(in crate::tests::domains::fintech) fn position(
        &self,
        instrument: InstrumentId,
    ) -> &FinancialPosition {
        self.positions()
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
        let portfolio = changed.portfolio_mut();
        portfolio.market = portfolio.market.with_factor_delta(factor, delta);
        changed
    }

    pub(crate) fn with_first_market_factor_delta(&self, delta: i64) -> Self {
        self.with_market_factor_delta(self.first_market_factor(), delta)
    }

    pub(in crate::tests::domains::fintech) fn with_factor_output_tolerance(
        mut self,
        factor: MarketFactorKey,
        epsilon: u64,
    ) -> Self {
        self.portfolio_mut().factor_output_equivalence.insert(
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
        let portfolio = changed.portfolio_mut();
        let position = portfolio
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

    fn portfolio(&self) -> &PortfolioFinancialWorldDefinition {
        match &self.kind {
            FinancialWorldDefinitionKind::Portfolio(portfolio) => portfolio,
            FinancialWorldDefinitionKind::Locality(_) => {
                panic!("portfolio operation used with a locality courtroom")
            }
        }
    }

    fn portfolio_mut(&mut self) -> &mut PortfolioFinancialWorldDefinition {
        match &mut self.kind {
            FinancialWorldDefinitionKind::Portfolio(portfolio) => portfolio,
            FinancialWorldDefinitionKind::Locality(_) => {
                panic!("portfolio mutation used with a locality courtroom")
            }
        }
    }
}
