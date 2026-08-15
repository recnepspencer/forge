use super::super::super::locality_scale::{LocalityLane, LocalityScaleTuple, RestorePosture};
use super::super::{
    FinancialAspect, FinancialLocalityAction, FinancialLocalityActionTrace,
    FinancialLocalityDefinition, FinancialLocalityFormula, FinancialLocalityMutation,
    FinancialLocalityOutput, FinancialLocalitySourceObligation, FinancialLocalitySubscription,
    FinancialLocalityTraceIdentity, LocalityEconomicOwner, LocalityFactorPublication,
    LocalityGenerationContract, LocalityMarketFactor, LocalityOutputRole, LocalitySemanticOutputId,
};

pub(super) struct RestoreScale {
    pub(super) posture: RestorePosture,
    pub(super) total_outputs: u32,
    pub(super) canonical_seeds: u16,
}

pub(super) fn generate(
    seed: u64,
    scale: LocalityScaleTuple,
    dimensions: RestoreScale,
    lane: LocalityLane,
) -> FinancialLocalityDefinition {
    let RestoreScale {
        posture,
        total_outputs,
        canonical_seeds,
    } = dimensions;
    assert!(canonical_seeds > 0);
    let (outputs, mutations) = match posture {
        RestorePosture::Narrow => narrow(seed),
        RestorePosture::Convergent => convergent(seed),
        RestorePosture::DenseFourInFive => dense(seed, total_outputs),
    };
    FinancialLocalityDefinition::generated(
        seed,
        scale,
        outputs,
        LocalityGenerationContract::traced(vec![restore_trace(mutations)], lane),
    )
}

fn restore_trace(mutations: Vec<FinancialLocalityMutation>) -> FinancialLocalityActionTrace {
    let retained = mutations[0];
    let mut actions = mutations
        .into_iter()
        .map(FinancialLocalityAction::CommitFactor)
        .collect::<Vec<_>>();
    actions.push(FinancialLocalityAction::StageSourceRecompute {
        obligation: FinancialLocalitySourceObligation {
            source: retained.producer,
            aspect: retained.aspect,
            scope: retained.scope,
            admission_generation: retained.admission_generation + 1,
            dependency_revision: 0,
        },
    });
    actions.extend([
        FinancialLocalityAction::CaptureBranch { branch_ordinal: 1 },
        FinancialLocalityAction::CaptureCheckpoint {
            checkpoint_ordinal: 1,
        },
        FinancialLocalityAction::DestroyDerivedState {
            destruction_ordinal: 1,
        },
        FinancialLocalityAction::ReadmitFreshRuntime { runtime_epoch: 2 },
        FinancialLocalityAction::ReplayCanonicalTrace { replay_ordinal: 1 },
        FinancialLocalityAction::DeterministicRerun { rerun_ordinal: 1 },
    ]);
    FinancialLocalityActionTrace::new(FinancialLocalityTraceIdentity::BranchRestoreReplay, actions)
}

fn narrow(seed: u64) -> (Vec<FinancialLocalityOutput>, Vec<FinancialLocalityMutation>) {
    let source = source(seed, 0, FinancialAspect::Price);
    let outputs = vec![source, dependent(1, 0, FinancialAspect::Price)];
    (outputs, vec![mutation(0, FinancialAspect::Price, 1)])
}

fn convergent(seed: u64) -> (Vec<FinancialLocalityOutput>, Vec<FinancialLocalityMutation>) {
    let aspects = [
        FinancialAspect::Price,
        FinancialAspect::Price,
        FinancialAspect::Curve,
        FinancialAspect::Volatility,
    ];
    let mut outputs = aspects
        .into_iter()
        .enumerate()
        .map(|(ordinal, aspect)| source(seed, ordinal as u32, aspect))
        .collect::<Vec<_>>();
    outputs.push(FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(4),
        owner: LocalityEconomicOwner::BookRisk(0),
        role: LocalityOutputRole::BookAggregate,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: 50,
        },
        subscriptions: aspects
            .into_iter()
            .enumerate()
            .map(|(ordinal, aspect)| {
                FinancialLocalitySubscription::unscoped(
                    LocalitySemanticOutputId::new(ordinal as u32),
                    aspect,
                )
            })
            .collect(),
    });
    let mutations = aspects
        .into_iter()
        .enumerate()
        .map(|(ordinal, aspect)| mutation(ordinal as u32, aspect, ordinal as u64 + 1))
        .collect();
    (outputs, mutations)
}

fn dense(
    seed: u64,
    total_outputs: u32,
) -> (Vec<FinancialLocalityOutput>, Vec<FinancialLocalityMutation>) {
    assert!(total_outputs >= 5 && total_outputs % 5 == 0);
    let mut outputs = vec![source(seed, 0, FinancialAspect::Price)];
    let affected = total_outputs / 5 * 4;
    for ordinal in 1..total_outputs {
        outputs.push(if ordinal < affected {
            dependent(ordinal, 0, FinancialAspect::Price)
        } else {
            stable_dependent(ordinal, 0)
        });
    }
    (outputs, vec![mutation(0, FinancialAspect::Price, 1)])
}

fn source(seed: u64, ordinal: u32, aspect: FinancialAspect) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(ordinal),
        owner: LocalityEconomicOwner::MarketDataFeed(ordinal),
        role: LocalityOutputRole::MarketQuote,
        formula: FinancialLocalityFormula::MarketSource {
            publication: LocalityFactorPublication::one(factor_for_aspect(aspect, ordinal)),
            baseline_value: 7_000 + seed as i64 + i64::from(ordinal) * 100,
            mutation_delta: 75,
        },
        subscriptions: Vec::new(),
    }
}

fn dependent(ordinal: u32, producer: u32, aspect: FinancialAspect) -> FinancialLocalityOutput {
    FinancialLocalityOutput {
        id: LocalitySemanticOutputId::new(ordinal),
        owner: LocalityEconomicOwner::Position(ordinal),
        role: LocalityOutputRole::PositionRisk,
        formula: FinancialLocalityFormula::LinearDependency {
            multiplier_micros: 1_000_000,
            basis_value: i64::from(ordinal),
        },
        subscriptions: vec![FinancialLocalitySubscription::unscoped(
            LocalitySemanticOutputId::new(producer),
            aspect,
        )],
    }
}

fn factor_for_aspect(aspect: FinancialAspect, ordinal: u32) -> LocalityMarketFactor {
    match aspect {
        FinancialAspect::Price if ordinal == 1 => LocalityMarketFactor::FxSpot,
        FinancialAspect::Price => LocalityMarketFactor::Quote,
        FinancialAspect::Curve => LocalityMarketFactor::Curve,
        FinancialAspect::Volatility => LocalityMarketFactor::Volatility,
        FinancialAspect::Risk | FinancialAspect::Alert => {
            panic!("risk and alert cannot be primary market factors")
        }
    }
}

fn stable_dependent(ordinal: u32, producer: u32) -> FinancialLocalityOutput {
    let mut output = dependent(ordinal, producer, FinancialAspect::Price);
    output.formula = FinancialLocalityFormula::StableControl {
        retained_value: 9_000 + i64::from(ordinal),
    };
    output
}

fn mutation(
    producer: u32,
    aspect: FinancialAspect,
    publication_ordinal: u64,
) -> FinancialLocalityMutation {
    FinancialLocalityMutation {
        producer: LocalitySemanticOutputId::new(producer),
        aspect,
        scope: None,
        admission_generation: 2,
        publication_order: publication_ordinal.saturating_sub(1) as u32,
    }
}
