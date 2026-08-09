use std::collections::BTreeSet;
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphAspectConsumerKind, UiGraphAspectPublisherKind, UiGraphLookupCostClass,
    UiGraphLookupFamily, UiGraphMountEligibilityIdentity, UiGraphNodeIdentity,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

#[test]
fn aspect_indexes_are_many_to_many_and_graph_owned() {
    let app = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.graph-aspects")
                .with_semantic_artifact_spec(first_publishing_control_spec())
                .with_semantic_artifact_spec(second_publishing_control_spec())
                .with_semantic_artifact_spec(first_consuming_region_spec())
                .with_semantic_artifact_spec(second_consuming_region_spec())
                .with_semantic_artifact_spec(competing_publishing_control_spec())
                .with_semantic_artifact_spec(competing_consuming_region_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let graph = app.graph();
    let first_publisher = artifact_from_file_provenance(&app, "app/graph_aspect_runtime.wui", 0);
    let second_publisher = artifact_from_file_provenance(&app, "app/graph_aspect_runtime.wui", 1);
    let first_consumer = artifact_from_file_provenance(&app, "app/graph_aspect_runtime.wui", 2);
    let second_consumer = artifact_from_file_provenance(&app, "app/graph_aspect_runtime.wui", 3);
    let competing_publisher =
        artifact_from_file_provenance(&app, "app/graph_aspect_runtime.wui", 4);
    let competing_consumer = artifact_from_file_provenance(&app, "app/graph_aspect_runtime.wui", 5);
    let text_aspect = first_publisher
        .aspect_contract()
        .expect("publishing declaration should admit aspect contract")
        .published()
        .aspects()
        .first()
        .cloned()
        .expect("publishing declaration should publish one aspect");
    let competing_aspect = competing_publisher
        .aspect_contract()
        .expect("competing publisher should admit aspect contract")
        .published()
        .aspects()
        .first()
        .cloned()
        .expect("competing publisher should publish one competing aspect");
    let published = graph.lookup().published_aspect(&text_aspect);
    let consumed = graph.lookup().consumed_aspect(&text_aspect);
    let competing_published = graph.lookup().published_aspect(&competing_aspect);
    let competing_consumed = graph.lookup().consumed_aspect(&competing_aspect);
    let publisher_node_ids = BTreeSet::from([
        graph_node_identity(graph, first_publisher),
        graph_node_identity(graph, second_publisher),
    ]);
    let publisher_receipt_ids = publisher_node_ids
        .iter()
        .map(|node_id| mount_eligibility_identity(graph, *node_id))
        .collect::<BTreeSet<_>>();
    let consumer_node_ids = BTreeSet::from([
        graph_node_identity(graph, first_consumer),
        graph_node_identity(graph, second_consumer),
    ]);
    let consumer_receipt_ids = consumer_node_ids
        .iter()
        .map(|node_id| mount_eligibility_identity(graph, *node_id))
        .collect::<BTreeSet<_>>();
    let competing_publisher_node_id = graph_node_identity(graph, competing_publisher);
    let competing_publisher_receipt_id =
        mount_eligibility_identity(graph, competing_publisher_node_id);
    let competing_consumer_node_id = graph_node_identity(graph, competing_consumer);
    let competing_consumer_receipt_id =
        mount_eligibility_identity(graph, competing_consumer_node_id);

    assert_eq!(
        published.receipt().family(),
        UiGraphLookupFamily::PublishedAspect
    );
    assert_eq!(
        published.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert_eq!(
        consumed.receipt().family(),
        UiGraphLookupFamily::ConsumedAspect
    );
    assert_eq!(
        consumed.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );

    let published_graph_nodes = publisher_graph_nodes(published.value());
    let published_receipts = publisher_receipts(published.value());

    assert_eq!(published.value().len(), 4);
    assert_eq!(published_graph_nodes, publisher_node_ids);
    assert_eq!(published_receipts, publisher_receipt_ids);
    assert!(!published_graph_nodes.contains(&competing_publisher_node_id));
    assert!(!published_receipts.contains(&competing_publisher_receipt_id));

    let consumed_graph_nodes = consumer_graph_nodes(consumed.value());
    let consumed_receipts = consumer_receipts(consumed.value());

    assert_eq!(consumed.value().len(), 4);
    assert_eq!(consumed_graph_nodes, consumer_node_ids);
    assert_eq!(consumed_receipts, consumer_receipt_ids);
    assert!(!consumed_graph_nodes.contains(&competing_consumer_node_id));
    assert!(!consumed_receipts.contains(&competing_consumer_receipt_id));

    assert_eq!(
        competing_published.receipt().family(),
        UiGraphLookupFamily::PublishedAspect
    );
    assert_eq!(
        competing_published.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert_eq!(
        publisher_graph_nodes(competing_published.value()),
        BTreeSet::from([competing_publisher_node_id])
    );
    assert_eq!(
        publisher_receipts(competing_published.value()),
        BTreeSet::from([competing_publisher_receipt_id])
    );
    assert!(!publisher_graph_nodes(competing_published.value())
        .contains(&graph_node_identity(graph, first_publisher)));

    assert_eq!(
        competing_consumed.receipt().family(),
        UiGraphLookupFamily::ConsumedAspect
    );
    assert_eq!(
        competing_consumed.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert_eq!(
        consumer_graph_nodes(competing_consumed.value()),
        BTreeSet::from([competing_consumer_node_id])
    );
    assert_eq!(
        consumer_receipts(competing_consumed.value()),
        BTreeSet::from([competing_consumer_receipt_id])
    );
    assert!(!consumer_graph_nodes(competing_consumed.value())
        .contains(&graph_node_identity(graph, first_consumer)));
}

fn publisher_graph_nodes(
    publishers: &[worth_ui::facade::graph::UiGraphAspectPublisher],
) -> BTreeSet<UiGraphNodeIdentity> {
    publishers
        .iter()
        .filter_map(|publisher| match publisher.kind() {
            UiGraphAspectPublisherKind::GraphNode(node_id) => Some(node_id),
            UiGraphAspectPublisherKind::MountEligibilitySlot(_)
            | UiGraphAspectPublisherKind::FutureReceipt => None,
        })
        .collect()
}

fn publisher_receipts(
    publishers: &[worth_ui::facade::graph::UiGraphAspectPublisher],
) -> BTreeSet<UiGraphMountEligibilityIdentity> {
    publishers
        .iter()
        .filter_map(|publisher| match publisher.kind() {
            UiGraphAspectPublisherKind::MountEligibilitySlot(receipt_id) => Some(receipt_id),
            UiGraphAspectPublisherKind::GraphNode(_)
            | UiGraphAspectPublisherKind::FutureReceipt => None,
        })
        .collect()
}

fn consumer_graph_nodes(
    consumers: &[worth_ui::facade::graph::UiGraphAspectConsumer],
) -> BTreeSet<UiGraphNodeIdentity> {
    consumers
        .iter()
        .filter_map(|consumer| match consumer.kind() {
            UiGraphAspectConsumerKind::GraphNode(node_id) => Some(node_id),
            UiGraphAspectConsumerKind::MountEligibilitySlot(_) => None,
        })
        .collect()
}

fn consumer_receipts(
    consumers: &[worth_ui::facade::graph::UiGraphAspectConsumer],
) -> BTreeSet<UiGraphMountEligibilityIdentity> {
    consumers
        .iter()
        .filter_map(|consumer| match consumer.kind() {
            UiGraphAspectConsumerKind::MountEligibilitySlot(receipt_id) => Some(receipt_id),
            UiGraphAspectConsumerKind::GraphNode(_) => None,
        })
        .collect()
}

fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn mount_eligibility_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    graph_node_identity: UiGraphNodeIdentity,
) -> UiGraphMountEligibilityIdentity {
    graph
        .lookup()
        .mount_eligibility_slot_for_node(graph_node_identity)
        .expect("mount eligibility slot should resolve for graph node")
        .value()
        .mount_eligibility_identity()
}

fn artifact_from_file_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .unwrap_or_else(|| {
            panic!("expected declaration artifact for {module_path}#{declaration_index}")
        })
}

fn first_publishing_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.publisher"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_aspect_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:publish"))
    .with_published_aspect(UiDslAspectName::new("content.text"))
}

fn second_publishing_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.publisher_secondary"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_aspect_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:publish-secondary"))
    .with_published_aspect(UiDslAspectName::new("content.text"))
}

fn first_consuming_region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_aspect_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("region:consumer"))
    .with_consumed_aspect(UiDslAspectName::new("content.text"))
}

fn second_consuming_region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.consumer_secondary"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_aspect_runtime.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("region:consumer-secondary"))
    .with_consumed_aspect(UiDslAspectName::new("content.text"))
}

fn competing_publishing_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.icon_publisher"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_aspect_runtime.wui", 4),
    )
    .with_structural_token(UiDslStructuralToken::new("control:publish-icon"))
    .with_published_aspect(UiDslAspectName::new("appearance.background"))
}

fn competing_consuming_region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.icon_consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_aspect_runtime.wui", 5),
    )
    .with_structural_token(UiDslStructuralToken::new("region:consume-icon"))
    .with_consumed_aspect(UiDslAspectName::new("appearance.background"))
}
