use std::collections::BTreeMap;

use crate::data::aspect::{Aspect, AspectMask, SignalAspectLoweringOwner};
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::conditional_execution::{
    SignalConditionalArtifactReuse, SignalConditionalCondition,
    SignalConditionalContractDefinition, SignalConditionalVersionComparator,
};
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::node::EvaluationCondition;
use crate::data::output_equivalence::OutputEquivalencePolicy;

use super::super::{
    FinancialAspect, FinancialConsumerRole, FinancialSemanticHandles, FinancialWorldDefinition,
    InstrumentId, MarketFactorKey,
};
use super::{ConsumerSemanticHandle, FactorSourceHandle, PositionSemanticHandles};

pub(super) fn build_semantic_topology(
    graph: &mut SignalGraph,
    definition: &FinancialWorldDefinition,
) -> Result<FinancialSemanticHandles, SignalError> {
    let mut factors = BTreeMap::new();
    for factor in definition.market().factors() {
        let aspect = factor_signal_aspect(definition, factor);
        let output_equivalence = match definition.factor_output_equivalence(factor) {
            super::super::FinancialOutputEquivalencePolicy::Exact => {
                OutputEquivalencePolicy::ExactAspectVersion
            }
            super::super::FinancialOutputEquivalencePolicy::Tolerance { epsilon } => {
                OutputEquivalencePolicy::AspectVersionTolerance { epsilon }
            }
        };
        let node = graph
            .node()
            .reads_aspects(AspectMask::EMPTY)
            .produces_aspects(AspectMask::from_aspect(aspect))
            .partitioned_output()
            .output_equivalence(output_equivalence)
            .build();
        factors.insert(factor, FactorSourceHandle(node));
    }

    let mut positions = BTreeMap::new();
    for position in definition.positions() {
        let reads = position
            .subscriptions
            .iter()
            .fold(AspectMask::EMPTY, |mask, subscription| {
                mask | AspectMask::from_aspect(factor_signal_aspect(
                    definition,
                    subscription.factor,
                ))
            });
        let valuation = graph
            .node()
            .reads_aspects(reads)
            .produces_aspects(AspectMask::from_aspect(super::super::super::aspects::PRICE))
            .build();
        let risk = graph
            .node()
            .reads_aspects(AspectMask::from_aspect(super::super::super::aspects::PRICE))
            .produces_aspects(AspectMask::from_aspect(super::super::super::aspects::RISK))
            .build();
        graph.set_dependencies(
            valuation,
            position.subscriptions.iter().map(|subscription| {
                let source = factors[&subscription.factor].0;
                DependencyEdge::partition_detail(
                    source,
                    factor_signal_aspect(definition, subscription.factor),
                    subscription.partition,
                    subscription.detail,
                )
            }),
        )?;
        graph.set_dependencies(
            risk,
            [DependencyEdge::new(
                valuation,
                super::super::super::aspects::PRICE,
            )],
        )?;
        positions.insert(
            position.instrument,
            PositionSemanticHandles { valuation, risk },
        );
    }

    let consumers = build_consumers(graph, definition, &positions)?;
    Ok(FinancialSemanticHandles {
        factors,
        positions,
        consumers,
    })
}

fn build_consumers(
    graph: &mut SignalGraph,
    definition: &FinancialWorldDefinition,
    positions: &BTreeMap<InstrumentId, PositionSemanticHandles>,
) -> Result<BTreeMap<FinancialConsumerRole, ConsumerSemanticHandle>, SignalError> {
    let mut consumers = BTreeMap::new();
    let installed_owner = definition
        .consumers()
        .iter()
        .any(|consumer| {
            matches!(
                consumer.comparator,
                super::super::FinancialComparatorPolicy::InstalledTolerance { .. }
            )
        })
        .then(SignalAspectLoweringOwner::fresh);
    if let Some(owner) = installed_owner.as_ref() {
        graph.claim_aspect_lowering_owner(owner).map_err(|denial| {
            SignalError::invalid_input(format!(
                "financial comparator lowering owner was denied: {denial:?}"
            ))
        })?;
    }
    for declaration in definition.consumers() {
        let builder = graph
            .node()
            .reads_aspects(AspectMask::from_aspect(signal_aspect(
                declaration.dependency_aspect,
            )))
            .produces_aspects(AspectMask::from_aspect(super::super::super::aspects::ALERT));
        let builder = match declaration.condition {
            super::super::FinancialConditionPolicy::AspectFilter(aspect) => builder.condition(
                EvaluationCondition::AspectFilter(AspectMask::from_aspect(signal_aspect(aspect))),
            ),
            super::super::FinancialConditionPolicy::DeltaThreshold(threshold) => {
                builder.condition(EvaluationCondition::DeltaThreshold(threshold as f64))
            }
        };
        let node = match declaration.comparator {
            super::super::FinancialComparatorPolicy::Exact => builder
                .dependency_comparator(VersionComparatorPolicy::Exact)
                .build(),
            super::super::FinancialComparatorPolicy::Tolerance { epsilon } => builder
                .dependency_comparator(VersionComparatorPolicy::Tolerance { epsilon })
                .build(),
            super::super::FinancialComparatorPolicy::InstalledTolerance { .. } => builder.build(),
        };
        graph.set_dependencies(
            node,
            [DependencyEdge::new(
                positions[&declaration.position].risk,
                signal_aspect(declaration.dependency_aspect),
            )],
        )?;
        if matches!(
            declaration.comparator,
            super::super::FinancialComparatorPolicy::InstalledTolerance { .. }
        ) {
            install_runtime_comparator(
                graph,
                installed_owner
                    .as_ref()
                    .expect("installed financial comparator must have a lowering owner"),
                node,
            )?;
        }
        consumers.insert(declaration.role, ConsumerSemanticHandle(node));
    }
    Ok(consumers)
}

fn install_runtime_comparator(
    graph: &mut SignalGraph,
    owner: &SignalAspectLoweringOwner,
    node: crate::data::handle::NodeId,
) -> Result<(), SignalError> {
    let worth_proof::TransitionOutcome::Success(capability) = graph.admit_installed_node(node)
    else {
        return Err(SignalError::invalid_input(
            "financial installed comparator node was not admitted",
        ));
    };
    graph
        .install_conditional_contract(
            owner,
            capability,
            SignalConditionalContractDefinition {
                condition: SignalConditionalCondition::AspectFilter(AspectMask::from_aspect(
                    super::super::super::aspects::RISK,
                )),
                dependency_aspects: AspectMask::from_aspect(super::super::super::aspects::RISK),
                trigger_aspects: AspectMask::from_aspect(super::super::super::aspects::RISK),
                dependency_comparator: SignalConditionalVersionComparator::RuntimeResolved,
                output_comparator: SignalConditionalVersionComparator::Exact,
                artifact_reuse: SignalConditionalArtifactReuse::NotReusable,
            },
        )
        .map_err(|denial| {
            SignalError::invalid_input(format!(
                "financial installed comparator contract was denied: {denial:?}"
            ))
        })?;
    Ok(())
}

pub(super) const fn factor_aspect(factor: MarketFactorKey) -> FinancialAspect {
    match factor {
        MarketFactorKey::Quote(_) | MarketFactorKey::FxSpot(_) => FinancialAspect::Price,
        MarketFactorKey::Curve(_) => FinancialAspect::Curve,
        MarketFactorKey::Volatility(_) => FinancialAspect::Volatility,
    }
}

pub(in crate::tests::domains::fintech) fn factor_signal_aspect(
    definition: &FinancialWorldDefinition,
    factor: MarketFactorKey,
) -> Aspect {
    if definition.uses_producer_local_factor_slots() {
        super::super::super::aspects::PRICE
    } else {
        signal_aspect(factor_aspect(factor))
    }
}

pub(in crate::tests::domains::fintech) const fn signal_aspect(aspect: FinancialAspect) -> Aspect {
    match aspect {
        FinancialAspect::Price => super::super::super::aspects::PRICE,
        FinancialAspect::Curve => super::super::super::aspects::CURVE,
        FinancialAspect::Volatility => super::super::super::aspects::VOL,
        FinancialAspect::Risk => super::super::super::aspects::RISK,
        FinancialAspect::Alert => super::super::super::aspects::ALERT,
    }
}
