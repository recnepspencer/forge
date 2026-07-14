use super::super::support::*;

#[test]
fn query_builder_assembles_graph_obligation_index_from_catalog() {
    let registration = WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", "schema", "v1")
            .unwrap(),
        WorthQueryGraphTouchSelector::relation_kind_id(77),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let runtime = complete_backend_from_parts_builder()
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("query runtime should assemble graph obligation index");

    assert_eq!(runtime.graph_obligation_index().registration_count(), 1);
    assert_eq!(runtime.graph_obligation_index().bucket_count(), 1);
    assert_eq!(runtime.graph_obligation_index().support_rows().len(), 6);
    assert_eq!(
        runtime
            .graph_obligation_index()
            .complexity_contracts()
            .len(),
        2
    );
    assert_eq!(
        runtime.graph_obligation_index().index_digest(),
        WorthQueryGraphObligationIndex::from_catalog(
            runtime.graph_obligation_registration_catalog()
        )
        .index_digest()
    );
}

#[test]
fn runtime_selects_graph_obligations_through_assembled_index() {
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let runtime = complete_backend_from_parts_builder()
        .graph_obligation(
            WorthQueryGraphObligationRegistration::schema_contract_validator(
                WorthQueryGraphObligationRuleIdentity::new(
                    "test.graph-obligation-index",
                    "relation-kind",
                    "v1",
                )
                .unwrap(),
                WorthQueryGraphTouchSelector::relation_kind_id(77),
                world,
            ),
        )
        .graph_obligation(WorthQueryGraphObligationRegistration::blocking_invariant(
            WorthQueryGraphObligationRuleIdentity::new(
                "test.graph-obligation-index",
                "collection",
                "v1",
            )
            .unwrap(),
            WorthQueryGraphTouchSelector::relation_kind("topology.edge").unwrap(),
            world,
        ))
        .build_backend_from_parts()
        .build()
        .expect("query runtime should assemble selectable graph obligation index");
    let descriptor = symbolic_relation_retirement_descriptor();

    let selection = runtime.select_graph_obligations_for_touch(
        &descriptor,
        &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );
    let names = selection
        .matched_registrations()
        .iter()
        .map(|registration| registration.rule_identity().name())
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(
        selection.index_digest(),
        runtime.graph_obligation_index().index_digest()
    );
    assert_eq!(selection.matched_obligation_count(), 2);
    assert_eq!(
        names,
        std::collections::BTreeSet::from(["collection", "relation-kind"])
    );
    assert_eq!(
        selection.counters().attempted_bucket_lookup_count(),
        selection.counters().touch_lookup_key_count()
            * selection.counters().operating_world_lookup_key_count()
    );
    assert_eq!(selection.counters().registration_full_scan_count(), 0);
}

#[test]
fn runtime_graph_obligation_index_digest_is_order_independent() {
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let left = WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", "schema", "v1")
            .unwrap(),
        WorthQueryGraphTouchSelector::relation_kind_id(77),
        world,
    );
    let right = WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", "blocking", "v1")
            .unwrap(),
        WorthQueryGraphTouchSelector::relation_kind("topology.edge").unwrap(),
        world,
    );

    let first = complete_backend_from_parts_builder()
        .graph_obligation(left.clone())
        .graph_obligation(right.clone())
        .build_backend_from_parts()
        .build()
        .unwrap();
    let second = complete_backend_from_parts_builder()
        .graph_obligation(right)
        .graph_obligation(left)
        .build_backend_from_parts()
        .build()
        .unwrap();

    assert_eq!(
        first.graph_obligation_index().index_digest(),
        second.graph_obligation_index().index_digest()
    );
}

fn symbolic_relation_retirement_descriptor() -> WorthQueryGraphTouchDescriptor {
    let command = WorthQueryWriteCommand::DeleteSymbolicAspects {
        reference: WorthQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection("topology.edge")
            .unwrap(),
        touched_aspects: vec![test_aspect_touch("weight")],
        metadata: WorthQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = WorthQueryGraphCompositionBreadth::new(1, 0, 0);
    let step = WorthQueryGraphCompositionProgramStep::new(
        0,
        WorthQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        Some(
            crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
                "graph-composition-test",
                "topology.edge",
            ),
        ),
        Some("edge".to_string()),
    )
    .with_relation_kind_id(worth_relational::facade::identity::KindId(77));
    let program = WorthQueryGraphCompositionProgram::new(vec![step], &breadth);
    WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap()
}
