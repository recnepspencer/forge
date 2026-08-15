use super::super::super::locality_scale::{LocalityLane, LocalityScaleTuple};
use super::super::{
    FinancialAspect, FinancialLocalityAction, FinancialLocalityActionTrace,
    FinancialLocalityDefinition, FinancialLocalityFormula, FinancialLocalityMutation,
    FinancialLocalityOutput, FinancialLocalitySubscription, FinancialLocalityTraceIdentity,
    LocalityEconomicOwner, LocalityFactorPublication, LocalityGenerationContract,
    LocalityMarketFactor, LocalityOutputRole, LocalityScope, LocalitySemanticOutputId,
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
    lane: LocalityLane,
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
    append_whole_partition_detail_twin(&mut outputs, &mut ordinal, source);
    let correlated_source = append_correlated_scope_twin(&mut outputs, &mut ordinal, seed);
    FinancialLocalityDefinition::generated(
        seed,
        scale,
        outputs,
        LocalityGenerationContract::traced(
            partition_action_traces(source, correlated_source),
            lane,
        ),
    )
}

fn append_whole_partition_detail_twin(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    source: LocalitySemanticOutputId,
) {
    let scope = LocalityScope::detail(0, 1);
    outputs.push(FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(*ordinal),
        owner: LocalityEconomicOwner::AuditControl(*ordinal),
        role: LocalityOutputRole::AuditCheck,
        formula: FinancialLocalityFormula::StableControl {
            retained_value: 65_000 + i64::from(*ordinal),
        },
        subscriptions: vec![FinancialLocalitySubscription::scoped(
            source,
            FinancialAspect::Curve,
            scope,
            scope,
        )],
    });
    *ordinal += 1;
}

fn partition_action_traces(
    source: LocalitySemanticOutputId,
    correlated_source: LocalitySemanticOutputId,
) -> Vec<FinancialLocalityActionTrace> {
    vec![
        factor_trace(
            FinancialLocalityTraceIdentity::PrimaryMutation,
            vec![mutation(MutationDeclaration {
                producer: source,
                aspect: FinancialAspect::Curve,
                scope: LocalityScope::detail(0, 0),
                admission_generation: 2,
                publication_order: 0,
            })],
        ),
        factor_trace(
            FinancialLocalityTraceIdentity::PartitionWholeRegion,
            vec![mutation(MutationDeclaration {
                producer: source,
                aspect: FinancialAspect::Curve,
                scope: LocalityScope::partition(0),
                admission_generation: 2,
                publication_order: 0,
            })],
        ),
        factor_trace(
            FinancialLocalityTraceIdentity::PartitionCorrelatedScopes,
            vec![
                mutation(MutationDeclaration {
                    producer: correlated_source,
                    aspect: FinancialAspect::Price,
                    scope: LocalityScope::detail(500, 1),
                    admission_generation: 2,
                    publication_order: 0,
                }),
                mutation(MutationDeclaration {
                    producer: correlated_source,
                    aspect: FinancialAspect::Volatility,
                    scope: LocalityScope::detail(501, 2),
                    admission_generation: 3,
                    publication_order: 1,
                }),
            ],
        ),
    ]
}

fn factor_trace(
    identity: FinancialLocalityTraceIdentity,
    mutations: Vec<FinancialLocalityMutation>,
) -> FinancialLocalityActionTrace {
    FinancialLocalityActionTrace::new(
        identity,
        mutations
            .into_iter()
            .map(FinancialLocalityAction::CommitFactor)
            .collect(),
    )
}

struct MutationDeclaration {
    producer: LocalitySemanticOutputId,
    aspect: FinancialAspect,
    scope: LocalityScope,
    admission_generation: u64,
    publication_order: u32,
}

fn mutation(declaration: MutationDeclaration) -> FinancialLocalityMutation {
    FinancialLocalityMutation {
        producer: declaration.producer,
        aspect: declaration.aspect,
        scope: Some(declaration.scope),
        admission_generation: declaration.admission_generation,
        publication_order: declaration.publication_order,
    }
}

fn source_output(seed: u64, source: LocalitySemanticOutputId) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: source,
        owner: LocalityEconomicOwner::MarketDataFeed(0),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::one(LocalityMarketFactor::Curve),
            baseline_value: 50_000 + seed as i64,
            mutation_delta: 100,
        },
        subscriptions: Vec::new(),
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
        subscriptions: vec![FinancialLocalitySubscription::scoped(
            source,
            FinancialAspect::Curve,
            scope,
            scope,
        )],
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
        subscriptions: vec![FinancialLocalitySubscription::scoped(
            source,
            FinancialAspect::Curve,
            queried_scope,
            rejected_contract_scope,
        )],
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
        subscriptions: vec![FinancialLocalitySubscription::scoped(
            source,
            FinancialAspect::Curve,
            scope,
            scope,
        )],
    });
    *ordinal += 1;
    append_partition_instrument(
        outputs,
        ordinal,
        InstrumentDeclaration {
            membership,
            region,
            instrument: 0,
        },
    );
}

fn append_correlated_scope_twin(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    seed: u64,
) -> LocalitySemanticOutputId {
    let source = LocalitySemanticOutputId::new(*ordinal);
    outputs.push(FinancialLocalityOutput {
        id: source,
        owner: LocalityEconomicOwner::MarketDataFeed(*ordinal),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::two(
                LocalityMarketFactor::Quote,
                LocalityMarketFactor::Volatility,
            ),
            baseline_value: 70_000 + seed as i64,
            mutation_delta: 125,
        },
        subscriptions: Vec::new(),
    });
    *ordinal += 1;
    append_scope_probe(
        outputs,
        ordinal,
        ScopeProbeDeclaration {
            source,
            aspect: FinancialAspect::Price,
            scope: LocalityScope::detail(500, 1),
            role: LocalityOutputRole::PositionValuation,
        },
    );
    append_scope_probe(
        outputs,
        ordinal,
        ScopeProbeDeclaration {
            source,
            aspect: FinancialAspect::Volatility,
            scope: LocalityScope::detail(501, 2),
            role: LocalityOutputRole::PositionRisk,
        },
    );
    source
}

struct ScopeProbeDeclaration {
    source: LocalitySemanticOutputId,
    aspect: FinancialAspect,
    scope: LocalityScope,
    role: LocalityOutputRole,
}

fn append_scope_probe(
    outputs: &mut Vec<FinancialLocalityOutput>,
    ordinal: &mut u32,
    declaration: ScopeProbeDeclaration,
) {
    outputs.push(FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(*ordinal),
        owner: LocalityEconomicOwner::Position(*ordinal),
        role: declaration.role,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: i64::from(*ordinal),
        },
        subscriptions: vec![FinancialLocalitySubscription::scoped(
            declaration.source,
            declaration.aspect,
            declaration.scope,
            declaration.scope,
        )],
    });
    *ordinal += 1;
}

struct InstrumentDeclaration {
    membership: LocalitySemanticOutputId,
    region: u16,
    instrument: u16,
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
        subscriptions: vec![FinancialLocalitySubscription::unscoped(
            declaration.membership,
            FinancialAspect::Risk,
        )],
    });
    *ordinal += 1;
}
