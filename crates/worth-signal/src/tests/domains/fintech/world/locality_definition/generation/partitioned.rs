use std::collections::BTreeSet;

use super::super::super::locality_scale::LocalityScaleTuple;
use super::super::{
    FinancialAspect, FinancialLocalityDefinition, FinancialLocalityDependency,
    FinancialLocalityFormula, FinancialLocalityMutation, FinancialLocalityOutput,
    LocalityEconomicOwner, LocalityOutputRole, LocalityScope, LocalitySemanticOutputId,
};

pub(super) struct PartitionScale {
    pub(super) regions: u16,
    pub(super) matching_memberships: u16,
    pub(super) instruments_per_matching_region: u16,
}

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    dimensions: PartitionScale,
) -> FinancialLocalityDefinition {
    assert!(dimensions.regions > 0);
    assert!(
        dimensions.matching_memberships > 0
            && dimensions.matching_memberships <= dimensions.regions
    );
    assert!(dimensions.instruments_per_matching_region > 0);
    let source = LocalitySemanticOutputId::new(0);
    let mut outputs = vec![source_output(seed, source)];
    let mut ordinal = 1_u32;
    append_matching_curve_membership(
        &mut outputs,
        &mut ordinal,
        source,
        dimensions.instruments_per_matching_region,
    );
    for membership in 1..dimensions.matching_memberships {
        append_queried_rejecting_membership(&mut outputs, &mut ordinal, source, membership);
    }
    for region in 1..dimensions.regions {
        append_disjoint_curve_region(&mut outputs, &mut ordinal, source, region);
    }
    FinancialLocalityDefinition {
        seed,
        scale,
        outputs,
        mutation: FinancialLocalityMutation {
            producer: source,
            aspect: FinancialAspect::Curve,
            scope: Some(LocalityScope::detail(0, 0)),
        },
    }
}

fn source_output(seed: u64, source: LocalitySemanticOutputId) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: source,
        owner: LocalityEconomicOwner::MarketDataFeed(0),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            baseline_value: 50_000 + seed as i64,
            mutation_delta: 100,
        },
        produced_aspects: BTreeSet::from([FinancialAspect::Curve]),
        dependencies: Vec::new(),
        expected_for_mutation: true,
        unchanged_output_stop: false,
    }
}

fn append_matching_curve_membership(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    source: LocalitySemanticOutputId,
    instrument_count: u16,
) {
    let membership = LocalitySemanticOutputId::new(*ordinal);
    let scope = LocalityScope::detail(0, 0);
    outputs.push(FinancialLocalityOutput {
        id: membership,
        owner: LocalityEconomicOwner::DeskRisk(0),
        role: LocalityOutputRole::BookAggregate,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: 10,
        },
        produced_aspects: BTreeSet::from([FinancialAspect::Price]),
        dependencies: vec![FinancialLocalityDependency::scoped(
            source,
            FinancialAspect::Curve,
            scope,
            scope,
        )],
        expected_for_mutation: true,
        unchanged_output_stop: false,
    });
    *ordinal += 1;
    for instrument in 0..instrument_count {
        append_partition_instrument(
            outputs,
            ordinal,
            InstrumentDeclaration {
                membership,
                region: 0,
                instrument,
                expected_for_mutation: true,
            },
        );
    }
}

fn append_queried_rejecting_membership(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    source: LocalitySemanticOutputId,
    membership: u16,
) {
    let queried_scope = LocalityScope::detail(0, 0);
    let rejected_contract_scope = LocalityScope::detail(0, membership);
    outputs.push(FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(*ordinal),
        owner: LocalityEconomicOwner::AuditControl(*ordinal),
        role: LocalityOutputRole::AuditCheck,
        formula: FinancialLocalityFormula::StableControl {
            retained_value: 60_000 + i64::from(membership),
        },
        produced_aspects: BTreeSet::from([FinancialAspect::Alert]),
        dependencies: vec![FinancialLocalityDependency::scoped(
            source,
            FinancialAspect::Curve,
            queried_scope,
            rejected_contract_scope,
        )],
        expected_for_mutation: false,
        unchanged_output_stop: false,
    });
    *ordinal += 1;
}

fn append_disjoint_curve_region(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    source: LocalitySemanticOutputId,
    region: u16,
) {
    let membership = LocalitySemanticOutputId::new(*ordinal);
    let scope = LocalityScope::detail(region, 0);
    outputs.push(FinancialLocalityOutput {
        id: membership,
        owner: LocalityEconomicOwner::DeskRisk(region),
        role: LocalityOutputRole::BookAggregate,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: i64::from(region) * 10,
        },
        produced_aspects: BTreeSet::from([FinancialAspect::Price]),
        dependencies: vec![FinancialLocalityDependency::scoped(
            source,
            FinancialAspect::Curve,
            scope,
            scope,
        )],
        expected_for_mutation: false,
        unchanged_output_stop: false,
    });
    *ordinal += 1;
    append_partition_instrument(
        outputs,
        ordinal,
        InstrumentDeclaration {
            membership,
            region,
            instrument: 0,
            expected_for_mutation: false,
        },
    );
}

struct InstrumentDeclaration {
    membership: LocalitySemanticOutputId,
    region: u16,
    instrument: u16,
    expected_for_mutation: bool,
}

fn append_partition_instrument(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    declaration: InstrumentDeclaration,
) {
    outputs.push(FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(*ordinal),
        owner: LocalityEconomicOwner::Position(*ordinal),
        role: LocalityOutputRole::PositionRisk,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: 500
                + i64::from(declaration.region) * 100
                + i64::from(declaration.instrument),
        },
        produced_aspects: BTreeSet::from([FinancialAspect::Risk]),
        dependencies: vec![FinancialLocalityDependency::unscoped(
            declaration.membership,
            FinancialAspect::Price,
        )],
        expected_for_mutation: declaration.expected_for_mutation,
        unchanged_output_stop: false,
    });
    *ordinal += 1;
}
