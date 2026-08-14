use std::collections::BTreeSet;

use super::super::super::locality_scale::{LocalityScaleTuple, SparseFanoutAxis};
use super::super::{
    FinancialAspect, FinancialLocalityDefinition, FinancialLocalityDependency,
    FinancialLocalityFormula, FinancialLocalityMutation, FinancialLocalityOutput,
    LocalityEconomicOwner, LocalityOutputRole, LocalityScope, LocalitySemanticOutputId,
    RELEVANT_CHAIN_OUTPUTS,
};

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    total_outputs: u32,
    axis: SparseFanoutAxis,
) -> FinancialLocalityDefinition {
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
    FinancialLocalityDefinition {
        seed,
        scale,
        outputs,
        mutation: FinancialLocalityMutation {
            producer: source,
            aspect: FinancialAspect::Price,
            scope: mutation_scope,
        },
    }
}

fn source_output(seed: u64, source: LocalitySemanticOutputId) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: source,
        owner: LocalityEconomicOwner::MarketDataFeed(0),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            baseline_value: 1_000_000 + seed as i64,
            mutation_delta: 25_000,
        },
        produced_aspects: BTreeSet::from([FinancialAspect::Price, FinancialAspect::Curve]),
        dependencies: Vec::new(),
        expected_for_mutation: true,
        unchanged_output_stop: false,
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
            produced_aspects: BTreeSet::from([FinancialAspect::Price]),
            dependencies: vec![FinancialLocalityDependency::unscoped(
                LocalitySemanticOutputId::new(ordinal - 1),
                FinancialAspect::Price,
            )],
            expected_for_mutation: true,
            unchanged_output_stop: false,
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
            produced_aspects: BTreeSet::from([FinancialAspect::Alert]),
            dependencies: vec![FinancialLocalityDependency::unscoped(
                source,
                FinancialAspect::Curve,
            )],
            expected_for_mutation: false,
            unchanged_output_stop: false,
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
            produced_aspects: BTreeSet::from([FinancialAspect::Alert]),
            dependencies: vec![FinancialLocalityDependency::scoped(
                source,
                FinancialAspect::Price,
                queried_scope,
                rejected_contract_scope,
            )],
            expected_for_mutation: false,
            unchanged_output_stop: false,
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
        produced_aspects: BTreeSet::from([FinancialAspect::Alert]),
        dependencies: vec![FinancialLocalityDependency::unscoped(
            source,
            FinancialAspect::Price,
        )],
        expected_for_mutation: true,
        unchanged_output_stop: true,
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
            produced_aspects: BTreeSet::from([FinancialAspect::Alert]),
            dependencies: vec![FinancialLocalityDependency::unscoped(
                stop,
                FinancialAspect::Alert,
            )],
            expected_for_mutation: false,
            unchanged_output_stop: false,
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
