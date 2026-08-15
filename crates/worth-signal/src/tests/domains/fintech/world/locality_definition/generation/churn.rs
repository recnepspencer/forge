use super::super::super::locality_scale::{LocalityLane, LocalityScaleTuple};
use super::super::{
    FinancialAspect, FinancialLocalityAction, FinancialLocalityActionTrace,
    FinancialLocalityDefinition, FinancialLocalityFormula, FinancialLocalityMutation,
    FinancialLocalityOutput, FinancialLocalityStagedWork, FinancialLocalitySubscription,
    FinancialLocalityTopologyChange, FinancialLocalityTraceIdentity, FinancialStructuralMutation,
    LocalityEconomicOwner, LocalityFactorPublication, LocalityGenerationContract,
    LocalityMarketFactor, LocalityOutputRole, LocalitySemanticOutputId,
};

pub(super) struct ChurnScale {
    pub(super) rounds: u16,
    pub(super) canonical_seeds: u16,
}

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    dimensions: ChurnScale,
    lane: LocalityLane,
) -> FinancialLocalityDefinition {
    let ChurnScale {
        rounds,
        canonical_seeds,
    } = dimensions;
    assert!(rounds > 0 && canonical_seeds > 0);
    let old_factor = LocalitySemanticOutputId::new(0);
    let new_factor = LocalitySemanticOutputId::new(1);
    let valuation = LocalitySemanticOutputId::new(2);
    let risk = LocalitySemanticOutputId::new(3);
    let outputs = vec![
        factor(seed, old_factor, 0),
        factor(seed, new_factor, 1),
        valuation_output(valuation, old_factor),
        risk_output(risk, valuation),
    ];
    FinancialLocalityDefinition::generated(
        seed,
        scale,
        outputs,
        LocalityGenerationContract::traced(
            vec![churn_trace(rounds, old_factor, new_factor, valuation, risk)],
            lane,
        ),
    )
}

fn churn_trace(
    rounds: u16,
    old_factor: LocalitySemanticOutputId,
    new_factor: LocalitySemanticOutputId,
    target: LocalitySemanticOutputId,
    risk: LocalitySemanticOutputId,
) -> FinancialLocalityActionTrace {
    let mut actions = Vec::with_capacity(usize::from(rounds) * 8);
    let mut topology_ordinal = 0_u64;
    let mut dependency_revision = 1_u64;
    let mut owner = LocalityEconomicOwner::Position(target.ordinal());
    let mut factor = old_factor;
    for round in 0..rounds {
        actions.push(FinancialLocalityAction::CommitFactor(
            FinancialLocalityMutation {
                producer: factor,
                aspect: FinancialAspect::Price,
                scope: None,
                admission_generation: u64::from(round) + 2,
                publication_order: u32::from(round) * 2,
            },
        ));
        let stale = FinancialLocalityStagedWork {
            target,
            dependency_revision,
            readiness_epoch: 1,
        };
        actions.push(FinancialLocalityAction::StagePreRewireWork {
            round,
            binding: stale,
        });
        let next_factor = if factor == old_factor {
            new_factor
        } else {
            old_factor
        };
        let next_owner = LocalityEconomicOwner::Position(1_000 + u32::from(round));
        topology_ordinal += 1;
        dependency_revision += 1;
        actions.push(FinancialLocalityAction::AcceptedOwnerMove {
            round,
            change: FinancialLocalityTopologyChange {
                target,
                before_owner: owner,
                after_owner: next_owner,
                before_subscription: price_subscription(factor),
                after_subscription: price_subscription(next_factor),
                structural: structural(target, topology_ordinal, dependency_revision),
            },
        });
        actions.push(FinancialLocalityAction::RejectStaleWork {
            round,
            stale,
            current_dependency_revision: dependency_revision,
        });
        topology_ordinal += 1;
        dependency_revision += 1;
        actions.push(FinancialLocalityAction::AcceptedDependencyRemoval {
            round,
            owner: next_owner,
            removed_subscription: price_subscription(next_factor),
            structural: structural(target, topology_ordinal, dependency_revision),
        });
        topology_ordinal += 1;
        dependency_revision += 1;
        actions.push(FinancialLocalityAction::AcceptedDependencyRecreation {
            round,
            owner: next_owner,
            subscription: price_subscription(next_factor),
            structural: structural(target, topology_ordinal, dependency_revision),
        });
        actions.push(FinancialLocalityAction::RejectedCycle {
            round,
            target,
            attempted_subscription: FinancialLocalitySubscription::unscoped(
                risk,
                FinancialAspect::Risk,
            ),
            attempted_topology_ordinal: topology_ordinal + 1,
            retained_dependency_revision: dependency_revision,
        });
        actions.push(FinancialLocalityAction::CommitFactor(
            FinancialLocalityMutation {
                producer: next_factor,
                aspect: FinancialAspect::Price,
                scope: None,
                admission_generation: u64::from(round) + 2,
                publication_order: u32::from(round) * 2 + 1,
            },
        ));
        factor = next_factor;
        owner = next_owner;
    }
    FinancialLocalityActionTrace::new(FinancialLocalityTraceIdentity::PortfolioChurn, actions)
}

fn price_subscription(producer: LocalitySemanticOutputId) -> FinancialLocalitySubscription {
    FinancialLocalitySubscription::unscoped(producer, FinancialAspect::Price)
}

fn structural(
    target: LocalitySemanticOutputId,
    topology_mutation_ordinal: u64,
    resulting_dependency_revision: u64,
) -> FinancialStructuralMutation {
    FinancialStructuralMutation {
        target,
        topology_mutation_ordinal,
        resulting_dependency_revision,
    }
}

fn factor(seed: u64, id: LocalitySemanticOutputId, offset: u32) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id,
        owner: LocalityEconomicOwner::MarketDataFeed(offset),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::one(if offset == 0 {
                LocalityMarketFactor::Quote
            } else {
                LocalityMarketFactor::FxSpot
            }),
            baseline_value: 5_000 + seed as i64 + i64::from(offset) * 100,
            mutation_delta: 50,
        },
        subscriptions: Vec::new(),
    }
}

fn valuation_output(
    id: LocalitySemanticOutputId,
    producer: LocalitySemanticOutputId,
) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id,
        owner: LocalityEconomicOwner::Position(id.ordinal()),
        role: LocalityOutputRole::PositionValuation,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: 100,
        },
        subscriptions: vec![FinancialLocalitySubscription::unscoped(
            producer,
            FinancialAspect::Price,
        )],
    }
}

fn risk_output(
    id: LocalitySemanticOutputId,
    valuation: LocalitySemanticOutputId,
) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id,
        owner: LocalityEconomicOwner::BookRisk(0),
        role: LocalityOutputRole::PositionRisk,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: 200,
        },
        subscriptions: vec![FinancialLocalitySubscription::unscoped(
            valuation,
            FinancialAspect::Price,
        )],
    }
}
