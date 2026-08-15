use super::super::super::locality_scale::{LocalityLane, LocalityScaleTuple, SparseFanoutAxis};
use super::super::{
    FinancialAspect, FinancialLocalityDefinition, FinancialLocalityFormula,
    FinancialLocalityMutation, FinancialLocalityOutput, FinancialLocalitySubscription,
    LocalityEconomicOwner, LocalityFactorPublication, LocalityGenerationContract,
    LocalityMarketFactor, LocalityOutputRole, LocalityScope, LocalitySemanticOutputId,
    RELEVANT_CHAIN_OUTPUTS,
};

pub(super) struct SparseScale {
    pub(super) total_outputs: u32,
    pub(super) axis: SparseFanoutAxis,
}

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    dimensions: SparseScale,
    lane: LocalityLane,
) -> FinancialLocalityDefinition {
    let SparseScale {
        total_outputs,
        axis,
    } = dimensions;
    assert!(total_outputs >= RELEVANT_CHAIN_OUTPUTS);
    let source = LocalitySemanticOutputId::new(0);
    let mut outputs = vec![source_output(seed, source)];
    append_relevant_chain(&mut outputs);
    let mutation_scope = match axis {
        SparseFanoutAxis::IndexDisjoint => {
            append_index_disjoint(&mut outputs, source, total_outputs);
            None
        }
        SparseFanoutAxis::QueriedRejecting => {
            append_queried_rejecting(&mut outputs, source, total_outputs);
            Some(LocalityScope::detail(0, 0))
        }
        SparseFanoutAxis::RejectedDescendants => {
            append_rejected_descendants(&mut outputs, source, total_outputs);
            None
        }
    };
    FinancialLocalityDefinition::generated(
        seed,
        scale,
        outputs,
        LocalityGenerationContract::direct(
            FinancialLocalityMutation {
                producer: source,
                aspect: FinancialAspect::Price,
                scope: mutation_scope,
                admission_generation: 2,
                publication_order: 0,
            },
            lane,
        ),
    )
}

fn source_output(seed: u64, source: LocalitySemanticOutputId) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: source,
        owner: LocalityEconomicOwner::MarketDataFeed(0),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::two(
                LocalityMarketFactor::Quote,
                LocalityMarketFactor::Curve,
            ),
            baseline_value: 1_000_000 + seed as i64,
            mutation_delta: 25_000,
        },
        subscriptions: Vec::new(),
    }
}

fn append_relevant_chain(outputs: &mut Vec<FinancialLocalityOutput>) {
    for ordinal in 1..RELEVANT_CHAIN_OUTPUTS {
        outputs.push(FinancialLocalityOutput {
            id: LocalitySemanticOutputId::new(ordinal),
            owner: chain_owner(ordinal),
            role: chain_role(ordinal),
            formula: FinancialLocalityFormula::LinearDependency {
                multiplier_micros: 1_000_000,
                basis_value: i64::from(ordinal) * 100,
            },
            subscriptions: vec![FinancialLocalitySubscription::unscoped(
                LocalitySemanticOutputId::new(ordinal - 1),
                chain_published_aspect(ordinal - 1),
            )],
        });
    }
}

fn append_index_disjoint(
    outputs: &mut Vec<FinancialLocalityOutput>,
    source: LocalitySemanticOutputId,
    total_outputs: u32,
) {
    for ordinal in RELEVANT_CHAIN_OUTPUTS..total_outputs {
        outputs.push(FinancialLocalityOutput {
            id: LocalitySemanticOutputId::new(ordinal),
            owner: LocalityEconomicOwner::AuditControl(ordinal),
            role: LocalityOutputRole::AuditCheck,
            formula: FinancialLocalityFormula::StableControl {
                retained_value: 10_000 + i64::from(ordinal),
            },
            subscriptions: vec![FinancialLocalitySubscription::unscoped(
                source,
                FinancialAspect::Curve,
            )],
        });
    }
}

fn append_queried_rejecting(
    outputs: &mut Vec<FinancialLocalityOutput>,
    source: LocalitySemanticOutputId,
    total_outputs: u32,
) {
    for ordinal in RELEVANT_CHAIN_OUTPUTS..total_outputs {
        let queried_scope = LocalityScope::detail(0, 0);
        let rejected_detail = ((ordinal - RELEVANT_CHAIN_OUTPUTS) % u32::from(u16::MAX) + 1) as u16;
        let rejected_contract_scope = LocalityScope::detail(0, rejected_detail);
        outputs.push(FinancialLocalityOutput {
            id: LocalitySemanticOutputId::new(ordinal),
            owner: LocalityEconomicOwner::AuditControl(ordinal),
            role: LocalityOutputRole::AuditCheck,
            formula: FinancialLocalityFormula::StableControl {
                retained_value: 20_000 + i64::from(ordinal),
            },
            subscriptions: vec![FinancialLocalitySubscription::scoped(
                source,
                FinancialAspect::Price,
                queried_scope,
                rejected_contract_scope,
            )],
        });
    }
}

fn append_rejected_descendants(
    outputs: &mut Vec<FinancialLocalityOutput>,
    source: LocalitySemanticOutputId,
    total_outputs: u32,
) {
    if total_outputs == RELEVANT_CHAIN_OUTPUTS {
        return;
    }
    let stop = LocalitySemanticOutputId::new(RELEVANT_CHAIN_OUTPUTS);
    outputs.push(FinancialLocalityOutput {
        id: stop,
        owner: LocalityEconomicOwner::AuditControl(RELEVANT_CHAIN_OUTPUTS),
        role: LocalityOutputRole::AuditCheck,
        formula: FinancialLocalityFormula::StableControl {
            retained_value: 30_000,
        },
        subscriptions: vec![FinancialLocalitySubscription::unscoped(
            source,
            FinancialAspect::Price,
        )],
    });
    append_descendant_reports(outputs, stop, total_outputs);
}

fn append_descendant_reports(
    outputs: &mut Vec<FinancialLocalityOutput>,
    stop: LocalitySemanticOutputId,
    total_outputs: u32,
) {
    for ordinal in (RELEVANT_CHAIN_OUTPUTS + 1)..total_outputs {
        outputs.push(FinancialLocalityOutput {
            id: LocalitySemanticOutputId::new(ordinal),
            owner: LocalityEconomicOwner::RegulatoryReport(ordinal),
            role: LocalityOutputRole::RegulatoryReport,
            formula: FinancialLocalityFormula::LinearDependency {
                multiplier_micros: 1_000_000,
                basis_value: i64::from(ordinal),
            },
            subscriptions: vec![FinancialLocalitySubscription::unscoped(
                stop,
                FinancialAspect::Alert,
            )],
        });
    }
}

fn chain_owner(ordinal: u32) -> LocalityEconomicOwner {
    match ordinal % 5 {
        0 => LocalityEconomicOwner::Position(ordinal),
        1 => LocalityEconomicOwner::BookRisk(ordinal as u16),
        2 => LocalityEconomicOwner::DeskRisk(ordinal as u16),
        3 => LocalityEconomicOwner::AuditControl(ordinal),
        _ => LocalityEconomicOwner::RegulatoryReport(ordinal),
    }
}

fn chain_role(ordinal: u32) -> LocalityOutputRole {
    match ordinal % 5 {
        0 => LocalityOutputRole::PositionValuation,
        1 => LocalityOutputRole::BookAggregate,
        2 => LocalityOutputRole::DeskAggregate,
        3 => LocalityOutputRole::AuditCheck,
        _ => LocalityOutputRole::RegulatoryReport,
    }
}

fn chain_published_aspect(ordinal: u32) -> FinancialAspect {
    if ordinal == 0 {
        return FinancialAspect::Price;
    }
    match chain_role(ordinal) {
        LocalityOutputRole::PositionValuation | LocalityOutputRole::MarketQuote => {
            FinancialAspect::Price
        }
        LocalityOutputRole::PositionRisk
        | LocalityOutputRole::BookAggregate
        | LocalityOutputRole::DeskAggregate => FinancialAspect::Risk,
        LocalityOutputRole::AuditCheck | LocalityOutputRole::RegulatoryReport => {
            FinancialAspect::Alert
        }
    }
}
