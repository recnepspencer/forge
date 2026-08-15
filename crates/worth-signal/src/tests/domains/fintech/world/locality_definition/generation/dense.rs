use super::super::super::locality_scale::{DensityRatio, LocalityLane, LocalityScaleTuple};
use super::super::{
    FinancialAspect, FinancialLocalityDefinition, FinancialLocalityFormula,
    FinancialLocalityMutation, FinancialLocalityOutput, FinancialLocalitySubscription,
    LocalityEconomicOwner, LocalityFactorPublication, LocalityGenerationContract,
    LocalityMarketFactor, LocalityOutputRole, LocalitySemanticOutputId,
};

pub(super) struct DenseScale {
    pub(super) total_outputs: u32,
    pub(super) affected_ratio: DensityRatio,
}

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    dimensions: DenseScale,
    lane: LocalityLane,
) -> FinancialLocalityDefinition {
    let DenseScale {
        total_outputs,
        affected_ratio,
    } = dimensions;
    assert!(total_outputs >= 100);
    let affected_total = affected_count(total_outputs, affected_ratio);
    let source = LocalitySemanticOutputId::new(0);
    let mut outputs = vec![market_close_source(seed, source)];
    for ordinal in 1..total_outputs {
        outputs.push(market_close_output(
            ordinal,
            source,
            ordinal < affected_total,
        ));
    }
    FinancialLocalityDefinition::generated(
        seed,
        scale,
        outputs,
        LocalityGenerationContract::direct(
            FinancialLocalityMutation {
                producer: source,
                aspect: FinancialAspect::Price,
                scope: None,
                admission_generation: 2,
                publication_order: 0,
            },
            lane,
        ),
    )
}

fn affected_count(total: u32, ratio: DensityRatio) -> u32 {
    match ratio {
        DensityRatio::OneInOneHundred => total / 100,
        DensityRatio::OneInFour => total / 4,
        DensityRatio::FourInFive => total / 5 * 4,
    }
}

fn market_close_source(seed: u64, id: LocalitySemanticOutputId) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id,
        owner: LocalityEconomicOwner::MarketDataFeed(0),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::two(
                LocalityMarketFactor::Quote,
                LocalityMarketFactor::Curve,
            ),
            baseline_value: 1_000_000 + seed as i64,
            mutation_delta: 10_000,
        },
        subscriptions: Vec::new(),
    }
}

fn market_close_output(
    ordinal: u32,
    source: LocalitySemanticOutputId,
    affected: bool,
) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(ordinal),
        owner: LocalityEconomicOwner::Position(ordinal),
        role: LocalityOutputRole::PositionValuation,
        formula: if affected {
            FinancialLocalityFormula::LinearDependency {
                multiplier_micros: 1_000_000,
                basis_value: i64::from(ordinal),
            }
        } else {
            FinancialLocalityFormula::StableControl {
                retained_value: 2_000_000 + i64::from(ordinal),
            }
        },
        subscriptions: vec![FinancialLocalitySubscription::unscoped(
            source,
            if affected {
                FinancialAspect::Price
            } else {
                FinancialAspect::Curve
            },
        )],
    }
}
