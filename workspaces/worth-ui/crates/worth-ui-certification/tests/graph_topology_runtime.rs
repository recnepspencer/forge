use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
#[path = "fixtures/graph_topology_test_support.rs"]
mod graph_topology_test_support;

use graph_topology_test_support::{
    artifact_from_compiler_provenance, artifact_from_file_provenance, diagnostic_surface_spec,
    extra_root_page_spec, graph_node_identity, local_composition_spec, mosaic_spec, page_set_spec,
    region_spec, root_page_artifact, slotted_control_spec,
};
use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::MosaicSizingContractId;
use worth_ui::facade::graph::{
    UiGraphContainmentClaim, UiGraphParentResolutionClaim, UiGraphTopologyLocalDenial,
};

#[test]
fn public_freeze_materializes_parent_child_slot_topology_as_graph_truth() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-topology.slot",
            )
            .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let control = artifact_from_file_provenance(&app, "app/graph_topology.wui", 0);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph_node_identity(graph, control);
    let root_topology = graph
        .inspection()
        .inspect_topology_node(root_page_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let control_topology = graph
        .inspection()
        .inspect_topology_node(control_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let control_node = graph
        .inspection()
        .inspect_graph_node(control_id)
        .expect("graph node inspection should resolve admitted node")
        .value();

    assert_eq!(root_topology.parent_node_identity(), None);
    assert_eq!(
        root_topology.containment_claim(),
        &UiGraphContainmentClaim::RootPage
    );
    assert_eq!(
        root_topology.parent_resolution_claim(),
        &UiGraphParentResolutionClaim::RootPage
    );
    assert_eq!(
        root_topology
            .page_membership()
            .expect("root page should own page membership")
            .page_node_identity(),
        root_page_id
    );

    assert_eq!(control_topology.parent_node_identity(), Some(root_page_id));
    assert_eq!(
        control_topology.parent_resolution_claim(),
        &UiGraphParentResolutionClaim::ContainedByRootPage
    );
    assert_eq!(
        control_topology.containment_claim(),
        &UiGraphContainmentClaim::Control {
            control_name: "save".into(),
        }
    );
    assert_eq!(
        control_topology
            .slot_topology()
            .expect("slotted control should publish slot topology")
            .slot_name(),
        "footer"
    );
    assert_eq!(
        control_topology
            .page_membership()
            .expect("control should inherit page membership from root page")
            .page_node_identity(),
        root_page_id
    );
    assert_eq!(format!("{:?}", control_node.operator_kind()), "Stack");

    assert!(graph
        .lookup()
        .child_nodes(root_page_id)
        .value()
        .contains(&control_id));
    assert!(graph
        .lookup()
        .slot_occupants(root_page_id, "footer")
        .value()
        .contains(&control_id));
}

#[test]
fn public_freeze_exposes_explicit_region_and_mosaic_membership_indexes() {
    let fixture = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.certification.graph-topology.membership",
    )
    .with_semantic_artifact_spec(region_spec())
    .with_semantic_artifact_spec(mosaic_spec());
    let region_provenance = fixture.admitted_provenance_for("workflow_editor.region.sidebar");
    let mosaic_provenance = fixture.admitted_provenance_for("workflow_editor.mosaic.workspace");
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(fixture)
        .freeze()
        .expect("application preparation should succeed");
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let region = artifact_from_compiler_provenance(&app, &region_provenance);
    let mosaic = artifact_from_compiler_provenance(&app, &mosaic_provenance);
    let root_page_id = graph_node_identity(graph, root_page);
    let region_id = graph_node_identity(graph, region);
    let mosaic_id = graph_node_identity(graph, mosaic);
    let region_topology = graph
        .inspection()
        .inspect_topology_node(region_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let mosaic_topology = graph
        .inspection()
        .inspect_topology_node(mosaic_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();

    assert_eq!(
        region_topology.containment_claim(),
        &UiGraphContainmentClaim::Region {
            region_name: "sidebar".into(),
        }
    );
    assert_eq!(
        mosaic_topology.containment_claim(),
        &UiGraphContainmentClaim::Mosaic {
            mosaic_name: "workspace".into(),
            sizing_contract_id: Some(
                MosaicSizingContractId::new("workspace.sizing.main")
                    .expect("expected sizing contract id should be valid"),
            ),
        }
    );
    assert_eq!(
        region_topology
            .region_membership()
            .expect("region declaration should materialize region membership")
            .region_name(),
        "sidebar"
    );
    assert_eq!(
        mosaic_topology
            .mosaic_membership()
            .expect("mosaic declaration should materialize mosaic membership")
            .mosaic_name(),
        "workspace"
    );
    assert!(graph
        .lookup()
        .region_members("sidebar")
        .value()
        .contains(&region_id));
    assert!(graph
        .lookup()
        .mosaic_members("workspace")
        .value()
        .contains(&mosaic_id));
    assert!(graph
        .lookup()
        .page_members(root_page_id)
        .value()
        .contains(&region_id));
    assert!(graph
        .lookup()
        .page_members(root_page_id)
        .value()
        .contains(&mosaic_id));
}

#[test]
fn topology_indexes_locate_nodes_while_attachment_posture_stays_on_node_truth() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-topology.attachment",
            )
            .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph
        .lookup()
        .slot_occupants(root_page_id, "footer")
        .value()
        .first()
        .copied()
        .expect("slot lookup should resolve the control node through graph indexes");
    let control = graph
        .inspection()
        .inspect_graph_node(control_id)
        .expect("graph node identity from topology index should resolve node truth")
        .value();

    assert!(control.attachment_posture().query_binding_attached());
    assert!(control.attachment_posture().service_usage_attached());
}

#[test]
fn graph_topology_keeps_root_contained_claims_explicit_without_generic_membership_tags() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-topology.containment-claims",
            )
            .with_semantic_artifact_spec(page_set_spec())
            .with_semantic_artifact_spec(local_composition_spec())
            .with_semantic_artifact_spec(diagnostic_surface_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let page_set = artifact_from_file_provenance(&app, "app/graph_topology_claims.wui", 0);
    let local_composition = artifact_from_file_provenance(&app, "app/graph_topology_claims.wui", 1);
    let diagnostic_surface =
        artifact_from_file_provenance(&app, "app/graph_topology_claims.wui", 2);
    let root_page_id = graph_node_identity(graph, root_page);
    let page_set_topology = graph
        .inspection()
        .inspect_topology_node(graph_node_identity(graph, page_set))
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let local_composition_topology = graph
        .inspection()
        .inspect_topology_node(graph_node_identity(graph, local_composition))
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let diagnostic_surface_topology = graph
        .inspection()
        .inspect_topology_node(graph_node_identity(graph, diagnostic_surface))
        .expect("topology inspection should resolve admitted node topology")
        .value();

    for topology in [
        &page_set_topology,
        &local_composition_topology,
        &diagnostic_surface_topology,
    ] {
        assert_eq!(topology.parent_node_identity(), Some(root_page_id));
        assert_eq!(
            topology.parent_resolution_claim(),
            &UiGraphParentResolutionClaim::ContainedByRootPage
        );
        assert_eq!(
            topology
                .page_membership()
                .expect("root-contained graph node should have explicit page membership")
                .page_node_identity(),
            root_page_id
        );
    }

    assert_eq!(
        page_set_topology.containment_claim(),
        &UiGraphContainmentClaim::PageSet {
            page_set_name: "shell".into(),
        }
    );
    assert_eq!(
        local_composition_topology.containment_claim(),
        &UiGraphContainmentClaim::LocalComposition {
            local_composition_name: "inspector".into(),
        }
    );
    assert_eq!(
        diagnostic_surface_topology.containment_claim(),
        &UiGraphContainmentClaim::DiagnosticSurface {
            diagnostic_surface_name: "lint".into(),
        }
    );
}

#[test]
fn freeze_returns_typed_denial_when_topology_has_multiple_root_pages() {
    let denial = match WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-topology.root-denial",
            )
            .with_semantic_artifact_spec(extra_root_page_spec())
            .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze()
    {
        Ok(_) => panic!("ambiguous root topology must deny application preparation"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphCommit
    );
    let WorthUiApplicationPreparationDenial::GraphCommit(denial) = denial else {
        panic!("expected graph-commit denial");
    };
    assert_eq!(denial.local_denials().len(), 3);
    assert!(denial.local_denials().iter().all(|local| {
        local.topology_denial()
            == Some(&UiGraphTopologyLocalDenial::RootPageCardinality {
                observed_root_pages: 2,
            })
    }));
}
