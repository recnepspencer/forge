use super::*;

#[test]
fn one_dependency_can_fan_out_across_declared_signal_partitions() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let node_capability = |node| {
        let TransitionOutcome::Success(capability) = graph.admit_installed_node(node) else {
            panic!("installed Signal node capability")
        };
        capability
    };
    let declaration = |node, partition| {
        BridgeSignalAspectTargetDeclaration::allocate(
            BridgeAspectRegistrationId::from_stable_name("profile-name"),
            worth_signal::facade::PartitionToken::new(partition),
            node_capability(node),
        )
    };
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![
                declaration(first, "bridge-a"),
                declaration(second, "bridge-b"),
            ],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("partition-distinct Signal fan-out should remain one correspondence")
    };
    assert_eq!(correspondence.target_count(), 2);
    assert_eq!(
        correspondence
            .targets()
            .map(|target| target.partition().clone())
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([
            worth_signal::facade::PartitionToken::new("bridge-a"),
            worth_signal::facade::PartitionToken::new("bridge-b"),
        ]),
    );
}

#[test]
fn inherited_detail_loss_red_control_scales_seed_work_with_sibling_targets() {
    for width in [1, 4, 16] {
        let mut graph = SignalGraph::new();
        let targets = (0..width)
            .map(|_| {
                let node = graph.node().build();
                target(&graph, node)
            })
            .collect::<Vec<_>>();
        let runtime = runtime(
            exact_mapping(),
            vec![registration(dependency("query:one"), targets)],
        );
        let TransitionOutcome::Success(correspondence) =
            runtime.install_semantic_correspondence(dependency("query:one"), &graph)
        else {
            panic!("the inherited correspondence must install")
        };
        let TransitionOutcome::Success(receipt) = runtime
            .deliver_installed_correspondence_envelope(
                &correspondence,
                &mut graph,
                &field_change_envelope(),
            )
        else {
            panic!("the inherited coarse correspondence must deliver")
        };

        assert_eq!(receipt.truth_targets_admitted(), 1);
        assert_eq!(receipt.signal_seeds_emitted(), width);
        assert_eq!(receipt.node_fan_out(), width);
        assert_eq!(receipt.slots_touched(), width);
    }
}

#[test]
fn declared_source_widening_requires_the_exact_registered_cause() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime_with_source_widening(
        exact_mapping(),
        BridgeAspectChangeWideningCause::FieldToWholeAspect,
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("declared source widening correspondence")
    };
    assert_eq!(
        correspondence
            .admission_counters()
            .field_to_whole_source_admissions(),
        1
    );
    assert!(runtime
        .deliver_installed_correspondence_envelope(
            &correspondence,
            &mut graph,
            &field_change_envelope_with_precision(BridgeAspectChangePrecision::DeclaredWidening),
        )
        .is_success());

    let mut wrong_graph = SignalGraph::new();
    let wrong_node = wrong_graph.node().build();
    let wrong = runtime_with_source_widening(
        exact_mapping(),
        BridgeAspectChangeWideningCause::OpaquePayloadToWholeAspect,
        vec![registration(
            dependency("query:one"),
            vec![target(&wrong_graph, wrong_node)],
        )],
    );
    let TransitionOutcome::Success(wrong_correspondence) =
        wrong.install_semantic_correspondence(dependency("query:one"), &wrong_graph)
    else {
        panic!("cause-specific source policy correspondence")
    };
    assert!(matches!(
        wrong.deliver_installed_correspondence_envelope(
            &wrong_correspondence,
            &mut wrong_graph,
            &field_change_envelope_with_precision(BridgeAspectChangePrecision::DeclaredWidening),
        ),
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == crate::facade::BridgeCorrespondenceDenialKind::MappingSemanticMismatch
    ));
}

#[test]
fn declared_field_to_whole_widening_reaches_a_sibling_field_dependency() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let sibling =
        super::semantic_dependencies::dependency_with_projection_field("query:one", "status");
    let runtime = runtime_with_source_widening(
        sibling_field_mapping(),
        BridgeAspectChangeWideningCause::FieldToWholeAspect,
        vec![registration(sibling.clone(), vec![target(&graph, node)])],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(sibling, &graph)
    else {
        panic!("declared whole-aspect breadth must install for a sibling field")
    };
    let TransitionOutcome::Success(receipt) = runtime.deliver_installed_correspondence_envelope(
        &correspondence,
        &mut graph,
        &field_change_envelope_with_precision(BridgeAspectChangePrecision::DeclaredWidening),
    ) else {
        panic!("declared whole-aspect breadth must reach the sibling dependency")
    };
    assert_eq!(receipt.truth_targets_admitted(), 1);
    assert_eq!(receipt.signal_seeds_emitted(), 1);
}

#[test]
fn retained_mapping_widening_classes_have_one_exact_admission_counter() {
    for (mapping, slice, policy) in [
        (
            exact_mapping(),
            SubscriptionSliceKind::SignalAspect,
            SliceWideningPolicy::RegisteredAspectCoarseWidening,
        ),
        (
            mapping(
                MappingSelector::exact("relational-record:entity:0:1:1"),
                TruthPatchTargetSelector::any(),
            ),
            SubscriptionSliceKind::RegisteredCoarseWidening,
            SliceWideningPolicy::RegisteredSurfaceCoarseWidening,
        ),
        (
            mapping(
                MappingSelector::any(),
                TruthPatchTargetSelector::entity_field(FieldKey::new("name").unwrap()),
            ),
            SubscriptionSliceKind::SignalPartition,
            SliceWideningPolicy::RegisteredPartitionWidening,
        ),
    ] {
        let registration = BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::admit_bridge_owned("profile-name"),
            mapping.truth_scope().clone(),
            mapping.snapshot_read_contract().clone(),
            TruthDeltaSurfaceKind::EntityField,
            slice,
            policy,
        );
        let mut graph = SignalGraph::new();
        let node = graph.node().build();
        let semantic_dependency = if policy == SliceWideningPolicy::RegisteredPartitionWidening {
            dependency("query:partition")
        } else {
            dependency("query:one")
        };
        let runtime = runtime_with_aspect_mapping(
            mapping,
            registration,
            vec![super::registration(
                semantic_dependency.clone(),
                vec![target(&graph, node)],
            )],
        );
        let TransitionOutcome::Success(correspondence) =
            runtime.install_semantic_correspondence(semantic_dependency, &graph)
        else {
            panic!("declared {policy:?} must admit its exact widening class");
        };
        let counters = correspondence.admission_counters();
        assert_eq!(counters.widened_matches(), 1);
        assert_eq!(counters.exact_matches(), 0);
        assert_eq!(
            [
                counters.entity_widened_matches(),
                counters.aspect_widened_matches(),
                counters.surface_widened_matches(),
                counters.partition_widened_matches(),
            ],
            match policy {
                SliceWideningPolicy::RegisteredEntityCoarseWidening => [1, 0, 0, 0],
                SliceWideningPolicy::RegisteredAspectCoarseWidening => [0, 1, 0, 0],
                SliceWideningPolicy::RegisteredSurfaceCoarseWidening => [0, 0, 1, 0],
                SliceWideningPolicy::RegisteredPartitionWidening => [0, 0, 0, 1],
                SliceWideningPolicy::Disallow => unreachable!(),
            }
        );
    }
}

#[test]
fn widening_labels_cannot_reclassify_an_exact_or_nonpartition_dependency() {
    for (slice, policy) in [
        (
            SubscriptionSliceKind::RegisteredCoarseWidening,
            SliceWideningPolicy::RegisteredEntityCoarseWidening,
        ),
        (
            SubscriptionSliceKind::SignalPartition,
            SliceWideningPolicy::RegisteredPartitionWidening,
        ),
    ] {
        let mapping = exact_mapping();
        let registration = BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::admit_bridge_owned("profile-name"),
            mapping.truth_scope().clone(),
            mapping.snapshot_read_contract().clone(),
            TruthDeltaSurfaceKind::EntityField,
            slice,
            policy,
        );
        let mut graph = SignalGraph::new();
        let node = graph.node().build();
        let runtime = runtime_with_aspect_mapping(
            mapping,
            registration,
            vec![super::registration(
                dependency("query:one"),
                vec![target(&graph, node)],
            )],
        );
        assert!(matches!(
            runtime.install_semantic_correspondence(dependency("query:one"), &graph),
            TransitionOutcome::Denied(denial)
                if denial.kind() == crate::facade::BridgeCorrespondenceDenialKind::MappingSemanticMismatch
                    && denial.counters().widened_matches() == 0
        ));
    }
}
