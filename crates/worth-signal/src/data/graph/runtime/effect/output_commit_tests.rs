use super::*;
use crate::data::comparator::{DefaultComparatorPolicyResolver, VersionComparatorResolver};
use crate::data::output::OutputChange;
use crate::facade::mark_dirty;
use crate::tests::support::{
    evaluate, evaluate_with_resolver, version_ab, GraphDependencyBatchExt, ASPECT_A,
};

struct CountingOutputResolver {
    calls: usize,
    decision: bool,
}

impl VersionComparatorResolver for CountingOutputResolver {
    fn resolve(
        &mut self,
        _key: &str,
        _aspect: crate::facade::Aspect,
        _cached: u64,
        _current: u64,
    ) -> Result<bool, SignalError> {
        self.calls += 1;
        Ok(self.decision)
    }
}

#[test]
fn every_prepublication_seam_leaves_semantic_state_untouched() {
    let seams = [
        OutputCommitPreparationSeam::SemanticDecision,
        OutputCommitPreparationSeam::ProducedDelta,
        OutputCommitPreparationSeam::DirectCauseAdmission,
        OutputCommitPreparationSeam::PacketPrevalidation,
    ];
    for seam in seams {
        assert_prepublication_failure_is_atomic(seam);
    }
}

#[test]
fn mutable_output_resolver_is_consulted_once_by_canonical_commit() {
    let mut graph = SignalGraph::new();
    let producer = graph
        .node()
        .produces_aspects(ASPECT_A)
        .output_equivalence(OutputEquivalencePolicy::Custom {
            key: "stateful-output".to_string(),
        })
        .build();
    let mut resolver = CountingOutputResolver {
        calls: 0,
        decision: true,
    };
    evaluate_with_resolver(
        &mut graph,
        producer,
        &mut |_node, _graph| Ok(version_ab(1, 0)),
        &mut resolver,
    )
    .unwrap();
    resolver.calls = 0;
    resolver.decision = false;
    mark_dirty(&mut graph, producer, ASPECT_A).unwrap();
    evaluate_with_resolver(
        &mut graph,
        producer,
        &mut |_node, _graph| Ok(version_ab(2, 0)),
        &mut resolver,
    )
    .unwrap();

    assert_eq!(resolver.calls, 1);
    assert_eq!(
        graph.node_aspect_version(producer).unwrap(),
        version_ab(1, 0)
    );
}

#[test]
fn output_tolerance_cannot_suppress_a_nodes_first_committed_truth() {
    let mut graph = SignalGraph::new();
    let producer = graph
        .node()
        .produces_aspects(ASPECT_A)
        .output_equivalence(OutputEquivalencePolicy::AspectVersionTolerance { epsilon: 5 })
        .build();

    evaluate(&mut graph, producer, &mut |_node, _graph| {
        Ok(version_ab(1, 0))
    })
    .unwrap();

    assert_eq!(
        graph.node_aspect_version(producer).unwrap(),
        version_ab(1, 0)
    );
    assert!(graph.node_runtime_artifact_state_present(producer).unwrap());
}

fn assert_prepublication_failure_is_atomic(seam: OutputCommitPreparationSeam) {
    let mut graph = SignalGraph::new();
    let producer = graph.node().produces_aspects(ASPECT_A).build();
    let consumer = graph.node().build();
    graph
        .append_dependency(consumer, producer, ASPECT_A)
        .unwrap();
    evaluate(&mut graph, producer, &mut |_node, _graph| {
        Ok(version_ab(1, 0))
    })
    .unwrap();
    evaluate(&mut graph, consumer, &mut |_node, _graph| {
        Ok(version_ab(10, 0))
    })
    .unwrap();
    mark_dirty(&mut graph, producer, ASPECT_A).unwrap();

    let before_version = graph.node_aspect_version(producer).unwrap();
    let before_producer_state = graph.get_state(producer).unwrap();
    let before_consumer_state = graph.get_state(consumer).unwrap();
    let before_snapshot = graph.get_dep_snapshot(producer).unwrap().clone();
    let before_causes = graph.pending_causes(consumer).unwrap().to_vec();
    let before_ordinal = graph.cause_sets.output_commit_ordinal_for_test();
    let before_observation = graph.observe().explain(producer).unwrap().output_change;

    let mut effect = super::super::tests::test_effect_with_labels(Vec::new());
    effect.operational.node = producer;
    effect.operational.aspect_version = version_ab(2, 0);
    effect.operational.output_change = OutputChange::Replaced;
    let apply = graph
        .build_apply_commit_packet(effect, OutputEquivalencePolicy::ExactAspectVersion, false)
        .unwrap();
    let error = graph
        .prepare_output_commit_packet_with_probe(
            apply,
            &mut DefaultComparatorPolicyResolver::default(),
            |candidate| {
                if candidate == seam {
                    return Err(SignalError::internal("injected prepublication failure"));
                }
                Ok(())
            },
        )
        .expect_err("injected seam must reject preparation");

    assert!(error
        .to_string()
        .contains("injected prepublication failure"));
    assert_eq!(graph.node_aspect_version(producer).unwrap(), before_version);
    assert_eq!(graph.get_state(producer).unwrap(), before_producer_state);
    assert_eq!(graph.get_state(consumer).unwrap(), before_consumer_state);
    assert_eq!(graph.get_dep_snapshot(producer).unwrap(), &before_snapshot);
    assert_eq!(graph.pending_causes(consumer).unwrap(), before_causes);
    assert_eq!(
        graph.cause_sets.output_commit_ordinal_for_test(),
        before_ordinal
    );
    assert_eq!(
        graph.observe().explain(producer).unwrap().output_change,
        before_observation
    );
}
