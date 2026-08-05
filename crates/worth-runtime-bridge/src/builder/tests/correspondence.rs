use std::sync::Arc;

use worth_foundational::facade::{
    AbsenceLaw, AspectBinding, AspectContract, AspectContractRevision, AspectEvolutionPolicy,
    AspectIdentity, AspectKey, AspectLocator, AspectMask, AuthoritativeAspectChangeKind,
    CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement, LocatorAuthority,
    ScalarAspectType, StructAspectShape,
};
use worth_proof::TransitionOutcome;
use worth_signal::facade::{Aspect, SignalGraph, MAX_ASPECTS};

use super::support::{TestSink, TestSource};
use crate::facade::{
    BridgeAspectChangePrecision, BridgeAspectChangeWideningCause, BridgeAspectRegistration,
    BridgeAspectRegistrationId, BridgeAuthoritativeSourceProvenance, BridgeBuildErrorKind,
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgeMappingId, BridgeMappingRegistration, BridgeProducerMetadata,
    BridgeSemanticAspectChange, BridgeSemanticCorrespondenceRegistration,
    BridgeSemanticDependencyCandidate, BridgeSemanticLocality, BridgeSignalAspectTargetDeclaration,
    CoarseRoutingMode, MappingSelector, RuntimeBridge, RuntimeBridgeBuilder,
    SignalInvalidationScope, SliceWideningPolicy, SnapshotReadContract, SubscriptionSliceKind,
    TruthDeltaSurfaceKind, TruthPatchScope, TruthPatchTargetSelector,
};
use crate::relational_identity::RelationalBridgeRecordIdentityParts;
use crate::truth_identity_fixtures::{truth_branch, truth_commit, truth_patch, truth_snapshot};

#[test]
fn foreign_runtime_and_cloned_graph_require_distinct_recovery_paths() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let installed_runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        installed_runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("installed correspondence");
    };
    let aspect = correspondence.targets().next().unwrap().aspect();
    let original_before = graph.node_aspect_version(node).unwrap().get(aspect);

    let foreign_runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    assert!(matches!(
        foreign_runtime.deliver_installed_correspondence_envelope(
            &correspondence,
            &mut graph,
            &field_change_envelope(),
        ),
        TransitionOutcome::Stale(_)
    ));
    assert_eq!(
        graph.node_aspect_version(node).unwrap().get(aspect),
        original_before
    );

    let mut cloned_graph = graph.clone();
    let clone_before = cloned_graph.node_aspect_version(node).unwrap().get(aspect);
    assert!(matches!(
        installed_runtime.deliver_installed_correspondence_envelope(
            &correspondence,
            &mut cloned_graph,
            &field_change_envelope(),
        ),
        TransitionOutcome::RebindRequired(_)
    ));
    assert_eq!(
        cloned_graph.node_aspect_version(node).unwrap().get(aspect),
        clone_before
    );
}

#[test]
fn allocation_exhaustion_is_typed_and_leaves_existing_slots_intact() {
    let graph = SignalGraph::new();
    let mut graph = graph;
    let node = graph.node().build();
    let registrations = (0..MAX_ASPECTS)
        .map(|slot| {
            registration(
                dependency(&format!("query:{slot}")),
                vec![target(&graph, node)],
            )
        })
        .chain(std::iter::once(registration(
            dependency("query:overflow"),
            vec![target(&graph, node)],
        )))
        .collect();
    let runtime = runtime(exact_mapping(), registrations);
    for slot in 0..MAX_ASPECTS {
        let correspondence =
            runtime.install_semantic_correspondence(dependency(&format!("query:{slot}")), &graph);
        assert!(matches!(correspondence, TransitionOutcome::Success(_)));
    }
    let overflow = runtime.install_semantic_correspondence(dependency("query:overflow"), &graph);
    let TransitionOutcome::Denied(denial) = overflow else {
        panic!("slot overflow must deny");
    };
    assert_eq!(
        denial.kind(),
        crate::facade::BridgeCorrespondenceDenialKind::CapacityExhausted
    );
    assert_eq!(denial.counters().capacity_denials(), 1);
    assert_eq!(denial.counters().allocation_keys_examined(), MAX_ASPECTS);
    assert_eq!(denial.counters().targets_admitted(), 0);
    let rebuild = runtime
        .rebuild_correspondence_allocation_index()
        .expect("capacity denial must leave rebuildable installed allocations");
    assert_eq!(rebuild.authoritative_allocation_records(), MAX_ASPECTS);
    assert_eq!(rebuild.rebuilt_allocation_keys(), MAX_ASPECTS);
    assert!(rebuild.exact_index_parity());
}

#[test]
fn equal_numeric_slots_are_scoped_by_node_and_graph_basis() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let exact = |node| exact_target(&graph, node, Aspect::new(0));
    let runtime = runtime(
        exact_mapping(),
        vec![
            registration(dependency("query:first"), vec![exact(first)]),
            registration(dependency("query:second"), vec![exact(second)]),
        ],
    );
    assert!(runtime
        .install_semantic_correspondence(dependency("query:first"), &graph)
        .is_success());
    assert!(runtime
        .install_semantic_correspondence(dependency("query:second"), &graph)
        .is_success());
}

#[test]
fn derived_only_signal_slot_cannot_be_promoted_into_truth_correspondence() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let derived_only = Aspect::new(7);
    assert!(graph
        .admit_installed_aspect(node, derived_only)
        .is_success());
    let derived_before = graph.node_aspect_version(node).unwrap().get(derived_only);

    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("truth correspondence should install independently");
    };
    assert!(correspondence
        .targets()
        .all(|target| target.aspect() != derived_only));
    assert!(runtime
        .deliver_installed_correspondence_envelope(
            &correspondence,
            &mut graph,
            &field_change_envelope(),
        )
        .is_success());
    assert_eq!(
        graph.node_aspect_version(node).unwrap().get(derived_only),
        derived_before
    );
}

#[test]
fn many_to_one_requires_an_observable_declared_widening() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let widened_runtime = runtime(
        widened_mapping(),
        vec![
            registration(
                dependency("query:first"),
                vec![exact_target(&graph, node, Aspect::new(0))],
            ),
            registration(
                dependency("query:second"),
                vec![exact_target(&graph, node, Aspect::new(0))],
            ),
        ],
    );
    let TransitionOutcome::Success(first_widened) =
        widened_runtime.install_semantic_correspondence(dependency("query:first"), &graph)
    else {
        panic!("declared widening should admit first source");
    };
    let TransitionOutcome::Success(second_widened) =
        widened_runtime.install_semantic_correspondence(dependency("query:second"), &graph)
    else {
        panic!("declared widening should admit shared source");
    };
    assert_eq!(first_widened.admission_counters().widened_matches(), 1);
    assert_eq!(second_widened.admission_counters().widened_matches(), 1);
    assert!(matches!(
        widened_runtime.deliver_installed_correspondence_envelope(
            &first_widened,
            &mut graph,
            &field_change_envelope(),
        ),
        TransitionOutcome::RebindRequired(
            crate::facade::BridgeCorrespondenceRebindRequired::AllocationSourceSet
        )
    ));
    let TransitionOutcome::Success(readmitted_first) =
        widened_runtime.install_semantic_correspondence(dependency("query:first"), &graph)
    else {
        panic!("the prior owner can readmit against the complete shared source set");
    };
    assert_eq!(
        readmitted_first
            .targets()
            .next()
            .unwrap()
            .allocation_sources()
            .len(),
        2
    );

    let exact_runtime = runtime(
        exact_mapping(),
        vec![
            registration(
                dependency("query:first"),
                vec![exact_target(&graph, node, Aspect::new(0))],
            ),
            registration(
                dependency("query:second"),
                vec![exact_target(&graph, node, Aspect::new(0))],
            ),
        ],
    );
    assert!(exact_runtime
        .install_semantic_correspondence(dependency("query:first"), &graph)
        .is_success());
    assert!(matches!(
        exact_runtime.install_semantic_correspondence(dependency("query:second"), &graph),
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == crate::facade::BridgeCorrespondenceDenialKind::SharedSlotRequiresDeclaredWidening
    ));
}

#[test]
fn duplicate_fan_out_denies_atomically_without_allocation_residue() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let duplicate = exact_target(&graph, node, Aspect::new(0));
    let denial = BridgeSemanticCorrespondenceRegistration::new(
        dependency("query:one"),
        vec![duplicate.clone(), duplicate],
    )
    .expect_err("duplicate target set must deny before runtime construction");
    assert_eq!(
        denial.kind(),
        crate::facade::BridgeCorrespondenceDenialKind::DuplicateTarget
    );
    assert_eq!(denial.counters().targets_admitted(), 0);
}

#[test]
fn widened_source_cannot_flow_through_exact_correspondence() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("exact correspondence");
    };
    let aspect = correspondence.targets().next().unwrap().aspect();
    let before = graph.node_aspect_version(node).unwrap().get(aspect);
    let TransitionOutcome::Denied(denial) = runtime.deliver_installed_correspondence_envelope(
        &correspondence,
        &mut graph,
        &field_change_envelope_with_precision(BridgeAspectChangePrecision::DeclaredWidening),
    ) else {
        panic!("widened source must not satisfy exact correspondence");
    };
    assert_eq!(
        denial.kind(),
        crate::facade::BridgeCorrespondenceDenialKind::MappingSemanticMismatch
    );
    assert_eq!(denial.counters().correspondence_lookups(), 1);
    assert_eq!(denial.counters().truth_targets_admitted(), 0);
    assert_eq!(denial.counters().signal_seeds_emitted(), 0);
    assert_eq!(graph.node_aspect_version(node).unwrap().get(aspect), before);
}

mod atomic_batch;
mod conditional_compatibility;
mod delivery_cost;
mod graph_cohesion;
mod outcome_topology;
mod partition_and_precision;
mod registration_drift;
mod semantic_dependencies;
mod semantic_fixture;
mod semantic_invalidation;
mod slot_search;
mod source_contact;

use semantic_fixture::*;
