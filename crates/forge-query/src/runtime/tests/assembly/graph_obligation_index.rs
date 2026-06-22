use super::super::support::*;

#[test]
fn query_builder_assembles_graph_obligation_index_from_catalog() {
    let registration = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", "schema", "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind_id(77),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
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
        ForgeQueryGraphObligationIndex::from_catalog(
            runtime.graph_obligation_registration_catalog()
        )
        .index_digest()
    );
}

#[test]
fn runtime_selects_graph_obligations_through_assembled_index() {
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let runtime = complete_backend_from_parts_builder()
        .graph_obligation(
            ForgeQueryGraphObligationRegistration::schema_contract_validator(
                ForgeQueryGraphObligationRuleIdentity::new(
                    "test.graph-obligation-index",
                    "relation-kind",
                    "v1",
                )
                .unwrap(),
                ForgeQueryGraphTouchSelector::relation_kind_id(77),
                world,
            ),
        )
        .graph_obligation(ForgeQueryGraphObligationRegistration::blocking_invariant(
            ForgeQueryGraphObligationRuleIdentity::new(
                "test.graph-obligation-index",
                "collection",
                "v1",
            )
            .unwrap(),
            ForgeQueryGraphTouchSelector::relation_kind("topology.edge").unwrap(),
            world,
        ))
        .build_backend_from_parts()
        .build()
        .expect("query runtime should assemble selectable graph obligation index");
    let descriptor = symbolic_relation_retirement_descriptor();

    let selection = runtime.select_graph_obligations_for_touch(
        &descriptor,
        &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
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
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let left = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", "schema", "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind_id(77),
        world,
    );
    let right = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", "blocking", "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.edge").unwrap(),
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

fn symbolic_relation_retirement_descriptor() -> ForgeQueryGraphTouchDescriptor {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection("topology.edge")
            .unwrap(),
        touched_aspect_paths: vec!["weight".to_string()],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 0);
    let step = ForgeQueryGraphCompositionProgramStep::new(
        0,
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        Some("edge".to_string()),
    )
    .with_relation_kind_id(forge_relational::facade::identity::KindId(77));
    let program = ForgeQueryGraphCompositionProgram::new(vec![step], &breadth);
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap()
}
