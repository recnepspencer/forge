use super::super::super::locality_scale::{LocalityLane, LocalityScaleTuple};
use super::super::{
    FinancialLocalityAction, FinancialLocalityActionTrace, FinancialLocalityDefinition,
    FinancialLocalityFormula, FinancialLocalityMutation, FinancialLocalityOutput,
    FinancialLocalitySubscription, FinancialLocalityTraceIdentity, LocalityEconomicOwner,
    LocalityFactorPublication, LocalityGenerationContract, LocalityMarketFactor,
    LocalityOutputRole, LocalitySemanticOutputId,
};

pub(super) struct ConvergentScale {
    pub(super) producer_permutations: u8,
    pub(super) duplicate_admissions: u8,
    pub(super) canonical_seeds: u16,
}

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    dimensions: ConvergentScale,
    lane: LocalityLane,
) -> FinancialLocalityDefinition {
    assert_eq!(dimensions.producer_permutations, 24);
    assert!(dimensions.canonical_seeds > 0);
    let factors = [
        LocalityMarketFactor::Quote,
        LocalityMarketFactor::FxSpot,
        LocalityMarketFactor::Curve,
        LocalityMarketFactor::Volatility,
    ];
    let mut outputs = factors
        .into_iter()
        .enumerate()
        .map(|(ordinal, factor)| factor_source(seed, ordinal as u32, factor))
        .collect::<Vec<_>>();
    outputs.push(portfolio_target(&factors));
    let mutations = factors
        .into_iter()
        .enumerate()
        .map(|(ordinal, factor)| FinancialLocalityMutation {
            producer: LocalitySemanticOutputId::new(ordinal as u32),
            aspect: factor.aspect(),
            scope: None,
            admission_generation: 2,
            publication_order: ordinal as u32,
        })
        .collect::<Vec<_>>();
    let action_traces = producer_permutation_traces(&mutations, dimensions.duplicate_admissions);
    FinancialLocalityDefinition::generated(
        seed,
        scale,
        outputs,
        LocalityGenerationContract::traced(action_traces, lane),
    )
}

fn producer_permutation_traces(
    mutations: &[FinancialLocalityMutation],
    duplicate_admissions: u8,
) -> Vec<FinancialLocalityActionTrace> {
    let mut traces = Vec::with_capacity(24);
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    let order = [a, b, c, d];
                    if order
                        .iter()
                        .copied()
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        != 4
                    {
                        continue;
                    }
                    traces.push(permutation_trace(
                        traces.len() as u8,
                        order.map(|index| mutations[index]),
                        duplicate_admissions,
                    ));
                }
            }
        }
    }
    traces
}

fn permutation_trace(
    ordinal: u8,
    mutations: [FinancialLocalityMutation; 4],
    duplicate_admissions: u8,
) -> FinancialLocalityActionTrace {
    let mut actions = mutations
        .into_iter()
        .enumerate()
        .map(|(publication_order, mut mutation)| {
            mutation.publication_order = publication_order as u32;
            FinancialLocalityAction::CommitFactor(mutation)
        })
        .collect::<Vec<_>>();
    actions.extend((1..=duplicate_admissions).map(|retry_ordinal| {
        FinancialLocalityAction::RetryAdmission {
            target: LocalitySemanticOutputId::new(4),
            retry_ordinal,
        }
    }));
    FinancialLocalityActionTrace::new(
        FinancialLocalityTraceIdentity::ProducerPermutation(ordinal),
        actions,
    )
}

fn factor_source(seed: u64, ordinal: u32, factor: LocalityMarketFactor) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(ordinal),
        owner: LocalityEconomicOwner::MarketDataFeed(ordinal),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::one(factor),
            baseline_value: 10_000 + seed as i64 + i64::from(ordinal) * 1_000,
            mutation_delta: 100 + i64::from(ordinal),
        },
        subscriptions: Vec::new(),
    }
}

fn portfolio_target(factors: &[LocalityMarketFactor; 4]) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(4),
        owner: LocalityEconomicOwner::BookRisk(0),
        role: LocalityOutputRole::BookAggregate,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: 500,
        },
        subscriptions: factors
            .iter()
            .enumerate()
            .map(|(ordinal, factor)| {
                FinancialLocalitySubscription::unscoped(
                    LocalitySemanticOutputId::new(ordinal as u32),
                    factor.aspect(),
                )
            })
            .collect(),
    }
}
