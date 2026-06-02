use forge_query::facade::{
    ForgeQueryContributionComposedClassification, ForgeQueryGroupedAtomicity,
    ForgeQueryGroupedIntent, ForgeQueryOrdinaryOutcome, ForgeQuerySupportContributionAuthoring,
};

use crate::topology_operators::{
    topology_grouped_operator_neighborhood, topology_operator_contribution_workflow,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyOperatorContributionIntent, TopologyOperatorWorkflowHandleExt,
};

use super::current_head_handle::current_head_handle;

#[test]
fn grouped_operator_lane_retains_query_group_meaning_and_support() {
    let handle = current_head_handle();
    let input = topology_grouped_operator_neighborhood(
        TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
            "phase3.grouped.loop.a",
            "phase3.grouped.relation.a",
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                1,
                1,
            ),
        ),
    )
    .with_member(TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        "phase3.grouped.loop.b",
        "phase3.grouped.relation.b",
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            2,
            1,
        ),
    ))
    .with_atomicity(ForgeQueryGroupedAtomicity::Atomic)
    .with_grouping_intent(ForgeQueryGroupedIntent::Authoritative)
    .with_shared_rationale("phase 3 topology operator grouped lane");

    let declaration = handle
        .declare_topology_grouped_operator(input)
        .expect("grouped topology operator should declare");
    assert_eq!(declaration.members().len(), 2);
    assert_eq!(
        declaration.declaration_family_key(),
        "topology.create_inner_loop_on_existing_face"
    );

    let support = handle.topology_grouped_operator_support(&declaration);
    assert!(!support.statuses().is_empty());

    let outcome = handle.orchestrate_topology_grouped_operator_outcome(declaration);
    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(grouped) => {
            assert_eq!(grouped.member_envelopes().len(), 2);
        }
        _ => panic!("expected bound grouped topology operator outcome"),
    }
}

#[test]
fn grouped_contribution_lane_keeps_shared_neighborhood_support_visible() {
    let handle = current_head_handle();
    let input = topology_grouped_operator_neighborhood(
        TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
            "phase3.grouped.contrib.loop.a",
            "phase3.grouped.contrib.relation.a",
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                3,
                1,
            ),
        ),
    )
    .with_member(TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        "phase3.grouped.contrib.loop.b",
        "phase3.grouped.contrib.relation.b",
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            4,
            1,
        ),
    ))
    .with_atomicity(ForgeQueryGroupedAtomicity::Atomic)
    .with_grouping_intent(ForgeQueryGroupedIntent::Authoritative)
    .with_shared_rationale("phase 3 grouped contribution topology lane")
    .with_shared_support_contribution(
        ForgeQuerySupportContributionAuthoring::declaration_traceability(
            "topology.traceability.grouped_inner_loop",
            "grouped topology authoring keeps shared traceability attached",
        ),
    );

    let grouped = match handle.grouped_topology_operator_contributions_checked(input) {
        Ok(grouped) => grouped,
        Err(_) => panic!("grouped topology contribution lane should admit shared support"),
    };

    assert_eq!(grouped.declaration().members().len(), 2);
    assert_eq!(grouped.members().len(), 2);
    assert_eq!(
        grouped.members()[0].1.intent_results()[0].semantic_code(),
        "topology.traceability.grouped_inner_loop"
    );
}

#[test]
fn contribution_composed_lane_keeps_topology_named_query_product_story() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.contribution",
        schema::facade::platform::entities::TopologyEntityKind::Vertex,
    );

    let composed = match handle.orchestrate_topology_operator_with_contributions(
        topology_operator_contribution_workflow(declaration).with_contribution(
            TopologyOperatorContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "topology.traceability.create_topology_entity",
                    "topology operator declaration stays traceable on the query lane",
                ),
            ),
        ),
    ) {
        Ok(composed) => composed,
        Err(_) => panic!("topology contribution-composed lane should admit traceability support"),
    };

    assert_eq!(
        composed.classification(),
        ForgeQueryContributionComposedClassification::FullyAdmitted
    );
    assert_eq!(
        composed.envelope().declaration_family_key(),
        "topology.create_topology_entity"
    );
    assert_eq!(composed.contributions().len(), 3);
    let semantic_codes = composed
        .intent_results()
        .iter()
        .map(|result| result.semantic_code())
        .collect::<Vec<_>>();
    assert!(semantic_codes.contains(
        &"topology.create_topology_entity.naming_row.create_topology_entity.edited_entity_names.preserved"
    ));
    assert!(semantic_codes.contains(
        &"topology.create_topology_entity.derived_fallback_policy.allow_explicit_fallback"
    ));
    assert!(semantic_codes.contains(&"topology.traceability.create_topology_entity"));
}

#[test]
fn contribution_composed_lane_retains_one_query_naming_row_per_declared_mutation_row() {
    let handle = current_head_handle();
    let declaration = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        "phase4.operator.contribution.inner_loop",
        "phase4.operator.contribution.inner_loop.relation",
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            9,
            1,
        ),
    );
    let expected_naming_row_count = declaration
        .clone()
        .declared_mutation_sequence()
        .naming_report()
        .rows
        .len();

    let proof = handle.orchestrate_topology_operator_with_contributions_proof(
        topology_operator_contribution_workflow(declaration),
    );
    let retained_naming_row_count = proof
        .intent_results()
        .iter()
        .filter(|result| result.semantic_code().contains(".naming_row."))
        .count();

    assert_eq!(retained_naming_row_count, expected_naming_row_count);
}

#[test]
fn contribution_composed_partial_proof_retains_denied_intent_context() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.contribution.denial",
        schema::facade::platform::entities::TopologyEntityKind::Loop,
    );

    let proof = handle.orchestrate_topology_operator_with_contributions_proof(
        topology_operator_contribution_workflow(declaration).with_contribution(
            TopologyOperatorContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "topology.traceability.denial",
                    "",
                ),
            ),
        ),
    );

    assert_eq!(proof.intent_results().len(), 3);
    assert!(proof.declaration().envelope_digest().is_some());
    assert_eq!(
        proof.intent_results()[0].semantic_code(),
        "topology.create_topology_entity.naming_row.create_topology_entity.edited_entity_names.preserved"
    );
    assert_eq!(
        proof.intent_results()[1].semantic_code(),
        "topology.create_topology_entity.derived_fallback_policy.allow_explicit_fallback"
    );

    assert_eq!(
        proof.composition_classification(),
        Some(ForgeQueryContributionComposedClassification::PartiallyAdmitted)
    );
    assert_eq!(
        proof.intent_results()[2].semantic_code(),
        "topology.traceability.denial"
    );
    assert!(handle
        .recover_topology_operator_contribution_proof(proof)
        .is_none());
}
