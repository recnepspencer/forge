use crate::data::aspect::AspectMask;
use crate::data::error::SignalError;
use crate::facade::{DiagnosticsTier, NodeState, SignalBranchHandle, SignalSnapshotV1};

use super::compiler::topology::{factor_signal_aspect, signal_aspect};
use super::compiler::CompiledFinancialWorld;
use super::{
    FinancialAspect, FinancialSemanticProjection, FinancialWorldDefinition, SemanticOutputKey,
};
use crate::tests::domains::fintech::certification::invalidation::FreshFinancialRecompute;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialScenarioIdentity {
    QuoteToRiskAspectTranslation,
    HeterogeneousConsumerComparators,
    ToleranceSuppressedRepricing,
    ProducerLocalFactorSlotCollision,
    PartitionedCurveBucketBump,
    GatedRepricingRelease,
    InstrumentDependencyRewire,
    BranchShockRestoreReplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialComparatorProfile {
    Exact,
    ExactToleranceAndInstalledTolerance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialScaleTuple {
    pub(in crate::tests::domains::fintech) factors: usize,
    pub(in crate::tests::domains::fintech) positions: usize,
    pub(in crate::tests::domains::fintech) books: usize,
    pub(in crate::tests::domains::fintech) desks: usize,
}

impl FinancialScaleTuple {
    pub(in crate::tests::domains::fintech) fn from_definition(
        definition: &FinancialWorldDefinition,
    ) -> Self {
        Self {
            factors: definition.market().factors().count(),
            positions: definition.positions().len(),
            books: definition
                .positions()
                .iter()
                .map(|position| position.book)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            desks: definition
                .positions()
                .iter()
                .map(|position| position.desk)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialPolicyTuple {
    pub(in crate::tests::domains::fintech) consumer_comparators: FinancialComparatorProfile,
    pub(in crate::tests::domains::fintech) producer_output_equivalence:
        super::FinancialOutputEquivalencePolicy,
    pub(in crate::tests::domains::fintech) diagnostics: DiagnosticsTier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialReproductionTuple {
    pub(in crate::tests::domains::fintech) scenario: FinancialScenarioIdentity,
    pub(in crate::tests::domains::fintech) seed: u64,
    pub(in crate::tests::domains::fintech) scale: FinancialScaleTuple,
    pub(in crate::tests::domains::fintech) policy: FinancialPolicyTuple,
    pub(in crate::tests::domains::fintech) mutation_step: u32,
    pub(in crate::tests::domains::fintech) economic_delta: i64,
}

pub(in crate::tests::domains::fintech) struct CausallyCompleteFinancialBaseline {
    compiled: CompiledFinancialWorld,
    fresh: FreshFinancialRecompute,
    reproduction: FinancialReproductionTuple,
    branch: SignalBranchHandle,
    checkpoint: SignalSnapshotV1,
    _seal: BaselineSeal,
}

struct BaselineSeal;

pub(super) fn seal_financial_baseline(
    mut compiled: CompiledFinancialWorld,
) -> Result<CausallyCompleteFinancialBaseline, SignalError> {
    let fresh = FreshFinancialRecompute::run(compiled.definition());
    if fresh.economic_snapshot() != *compiled.economic_snapshot() {
        return Err(SignalError::internal(
            "runtime financial formulas disagree with independent fresh recompute",
        ));
    }
    verify_contracts_and_snapshots(&compiled)?;
    verify_projected_truth(&compiled, compiled.projection())?;
    let branch = compiled.runtime_mut().observe().current_branch();
    let checkpoint = compiled.runtime_mut().capture_snapshot()?;
    let definition = compiled.definition();
    let scale = FinancialScaleTuple::from_definition(definition);
    let reproduction = FinancialReproductionTuple {
        scenario: FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        seed: definition.seed(),
        scale,
        policy: FinancialPolicyTuple {
            consumer_comparators: FinancialComparatorProfile::Exact,
            producer_output_equivalence: super::FinancialOutputEquivalencePolicy::Exact,
            diagnostics: DiagnosticsTier::Development,
        },
        mutation_step: 1,
        economic_delta: 0,
    };
    Ok(CausallyCompleteFinancialBaseline {
        compiled,
        fresh,
        reproduction,
        branch,
        checkpoint,
        _seal: BaselineSeal,
    })
}

impl CausallyCompleteFinancialBaseline {
    pub(in crate::tests::domains::fintech) fn compiled(&self) -> &CompiledFinancialWorld {
        &self.compiled
    }

    pub(in crate::tests::domains::fintech) fn into_compiled(self) -> CompiledFinancialWorld {
        self.compiled
    }

    pub(in crate::tests::domains::fintech) fn fresh(&self) -> &FreshFinancialRecompute {
        &self.fresh
    }

    pub(in crate::tests::domains::fintech) const fn reproduction(
        &self,
    ) -> FinancialReproductionTuple {
        self.reproduction
    }

    pub(in crate::tests::domains::fintech) fn branch(&self) -> &SignalBranchHandle {
        &self.branch
    }
}

fn verify_contracts_and_snapshots(compiled: &CompiledFinancialWorld) -> Result<(), SignalError> {
    for (key, projected) in compiled.projection().iter() {
        let node = compiled.handles().node_for(key);
        if compiled.graph().get_state(node)? != NodeState::Clean {
            return Err(SignalError::internal(format!(
                "financial baseline node {node} was not clean"
            )));
        }
        let contract = compiled.graph().get_contract(node)?;
        let expected_produces = AspectMask::from_aspect(match key {
            SemanticOutputKey::Factor(factor) => {
                factor_signal_aspect(compiled.definition(), factor)
            }
            _ => signal_aspect(projected.aspect),
        });
        if contract.semantics.produces != expected_produces {
            return Err(SignalError::internal(format!(
                "financial baseline node {node} lacks an exact output contract"
            )));
        }
        let expected_reads = expected_reads(compiled.definition(), key);
        if contract.semantics.reads != expected_reads {
            return Err(SignalError::internal(format!(
                "financial baseline node {node} lacks an exact input contract"
            )));
        }
    }
    for node in compiled.handles().derived_nodes() {
        if compiled
            .graph()
            .get_dep_snapshot(node)?
            .entries()
            .is_empty()
        {
            return Err(SignalError::internal(format!(
                "financial baseline node {node} lacks an established dependency snapshot"
            )));
        }
    }
    Ok(())
}

pub(super) fn verify_projected_truth(
    compiled: &CompiledFinancialWorld,
    projection: &FinancialSemanticProjection,
) -> Result<(), SignalError> {
    for (key, projected) in projection.iter() {
        let aspect = match key {
            SemanticOutputKey::Factor(factor) => {
                factor_signal_aspect(compiled.definition(), factor)
            }
            _ => signal_aspect(projected.aspect),
        };
        let actual = compiled.node_version(key)?.get(aspect);
        if actual != projected.revision {
            return Err(SignalError::internal(format!(
                "financial projection mismatch for {key:?}: expected {}, got {actual}",
                projected.revision
            )));
        }
    }
    Ok(())
}

pub(super) fn verify_projected_work(
    compiled: &CompiledFinancialWorld,
    work: &std::collections::BTreeSet<SemanticOutputKey>,
) -> Result<(), SignalError> {
    for key in work {
        let projected = compiled.projection().output(*key);
        let aspect = match key {
            SemanticOutputKey::Factor(factor) => {
                factor_signal_aspect(compiled.definition(), *factor)
            }
            _ => signal_aspect(projected.aspect),
        };
        let actual = compiled.node_version(*key)?.get(aspect);
        if actual != projected.revision {
            return Err(SignalError::internal(format!(
                "financial projection mismatch for {key:?}: expected {}, got {actual}",
                projected.revision
            )));
        }
    }
    Ok(())
}

fn expected_reads(definition: &FinancialWorldDefinition, key: SemanticOutputKey) -> AspectMask {
    match key {
        SemanticOutputKey::Factor(_) => AspectMask::EMPTY,
        SemanticOutputKey::Valuation(instrument) => definition
            .position(instrument)
            .subscriptions
            .iter()
            .fold(AspectMask::EMPTY, |mask, subscription| {
                mask | AspectMask::from_aspect(factor_signal_aspect(
                    definition,
                    subscription.factor,
                ))
            }),
        SemanticOutputKey::Risk(_) => {
            AspectMask::from_aspect(signal_aspect(FinancialAspect::Price))
        }
        SemanticOutputKey::Consumer(_) => {
            AspectMask::from_aspect(signal_aspect(FinancialAspect::Risk))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::world::{
        compile_financial_world, FinancialWorldDefinition, InstrumentId,
    };

    #[test]
    fn compiler_seals_a_reproducible_causally_complete_financial_baseline() {
        let baseline = compile_financial_world(FinancialWorldDefinition::deterministic(41))
            .expect("authentic financial baseline should seal");
        let reproduction = baseline.reproduction();

        assert_eq!(reproduction.seed, 41);
        assert_eq!(reproduction.scale.positions, 3);
        assert!(reproduction.scale.factors >= 5);
        assert_eq!(reproduction.mutation_step, 1);
        assert_eq!(baseline.fresh().seed(), 41);
        assert!(!baseline.checkpoint.meta.core_storage_profile.is_empty());
        assert_eq!(
            baseline
                .compiled()
                .node_state(SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")))
                .unwrap(),
            NodeState::Clean
        );
        let _branch = baseline.branch();
    }
}
