use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphContainmentClaim, UiGraphInstantiationLocalDenialKind, UiGraphInstantiationPlan,
    UiGraphNodeIdentity, UiGraphParentResolutionClaim, UiGraphTopologyLocalDenial,
};
use worth_ui::facade::registry::MosaicSizingContractId;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn topology_rows_and_indexes_agree_for_every_admitted_non_root_family() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.authority")
                .with_semantic_artifact_spec(slotted_control_spec())
                .with_semantic_artifact_spec(region_spec())
                .with_semantic_artifact_spec(mosaic_spec())
                .with_semantic_artifact_spec(page_set_spec())
                .with_semantic_artifact_spec(local_composition_spec())
                .with_semantic_artifact_spec(diagnostic_surface_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let graph = app.graph();
    let root_page_id = graph_node_identity(graph, root_page_artifact(&app));
    let cases = [
        family_case(
            graph_node_identity(
                graph,
                artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 0),
            ),
            UiGraphContainmentClaim::Control {
                control_name: "save".into(),
            },
            Some("footer"),
            None,
            None,
        ),
        family_case(
            graph_node_identity(
                graph,
                artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 1),
            ),
            UiGraphContainmentClaim::Region {
                region_name: "sidebar".into(),
            },
            None,
            Some("sidebar"),
            None,
        ),
        family_case(
            graph_node_identity(
                graph,
                artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 2),
            ),
            UiGraphContainmentClaim::Mosaic {
                mosaic_name: "workspace".into(),
                sizing_contract_id: Some(
                    MosaicSizingContractId::new("workspace.sizing.main")
                        .expect("expected sizing contract id should be valid"),
                ),
            },
            None,
            None,
            Some("workspace"),
        ),
        family_case(
            graph_node_identity(
                graph,
                artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 3),
            ),
            UiGraphContainmentClaim::PageSet {
                page_set_name: "shell".into(),
            },
            None,
            None,
            None,
        ),
        family_case(
            graph_node_identity(
                graph,
                artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 4),
            ),
            UiGraphContainmentClaim::LocalComposition {
                local_composition_name: "inspector".into(),
            },
            None,
            None,
            None,
        ),
        family_case(
            graph_node_identity(
                graph,
                artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 5),
            ),
            UiGraphContainmentClaim::DiagnosticSurface {
                diagnostic_surface_name: "lint".into(),
            },
            None,
            None,
            None,
        ),
    ];

    for case in cases {
        let topology = graph
            .inspection()
            .inspect_topology_node(case.node_identity)
            .expect("all admitted topology families should materialize graph-owned node truth");
        let topology = topology.value();
        assert_eq!(topology.containment_claim(), &case.containment_claim);
        assert_eq!(
            topology.parent_resolution_claim(),
            &UiGraphParentResolutionClaim::ContainedByRootPage
        );
        assert_eq!(topology.parent_node_identity(), Some(root_page_id));
        assert_eq!(
            topology
                .page_membership()
                .expect(
                    "all non-root admitted topology families should have explicit page membership"
                )
                .page_node_identity(),
            root_page_id
        );
        assert_eq!(
            topology.slot_topology().map(|slot| slot.slot_name()),
            case.slot_name
        );
        assert_eq!(
            topology
                .region_membership()
                .map(|membership| membership.region_name()),
            case.region_name
        );
        assert_eq!(
            topology
                .mosaic_membership()
                .map(|membership| membership.mosaic_name()),
            case.mosaic_name
        );
        assert!(graph
            .lookup()
            .child_nodes(root_page_id)
            .value()
            .contains(&case.node_identity));
        assert!(graph
            .lookup()
            .page_members(root_page_id)
            .value()
            .contains(&case.node_identity));
        if let Some(slot_name) = case.slot_name {
            assert!(graph
                .lookup()
                .slot_occupants(root_page_id, slot_name)
                .value()
                .contains(&case.node_identity));
        }
        if let Some(region_name) = case.region_name {
            assert!(graph
                .lookup()
                .region_members(region_name)
                .value()
                .contains(&case.node_identity));
        }
        if let Some(mosaic_name) = case.mosaic_name {
            assert!(graph
                .lookup()
                .mosaic_members(mosaic_name)
                .value()
                .contains(&case.node_identity));
        }
    }
}

#[test]
fn admit_handoffs_localizes_zero_root_topology_as_typed_boundary_denial() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.zero-root-denial")
                .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let control_handoff =
        artifact_from_file_provenance(&app, "app/graph_topology_authority.wui", 0)
            .graph_handoff()
            .expect("control declaration should lower to a sealed graph handoff");
    let plan =
        UiGraphInstantiationPlan::admit_handoffs(std::slice::from_ref(&control_handoff), &[])
            .expect(
                "zero-root topology should deny locally inside graph instantiation plan admission",
            );

    assert!(plan.node_entries().is_empty());
    assert_eq!(plan.local_denials().len(), 1);
    let denial = &plan.local_denials()[0];
    assert_eq!(denial.declaration_identity(), control_handoff.identity());
    assert!(matches!(
        denial.kind(),
        UiGraphInstantiationLocalDenialKind::Topology(
            UiGraphTopologyLocalDenial::RootPageCardinality {
                observed_root_pages: 0
            }
        )
    ));
}

#[test]
fn public_freeze_returns_typed_root_cardinality_denial() {
    let denial = match WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.root-denial")
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

#[derive(Clone)]
struct FamilyCase {
    node_identity: UiGraphNodeIdentity,
    containment_claim: UiGraphContainmentClaim,
    slot_name: Option<&'static str>,
    region_name: Option<&'static str>,
    mosaic_name: Option<&'static str>,
}

fn family_case(
    node_identity: UiGraphNodeIdentity,
    containment_claim: UiGraphContainmentClaim,
    slot_name: Option<&'static str>,
    region_name: Option<&'static str>,
    mosaic_name: Option<&'static str>,
) -> FamilyCase {
    FamilyCase {
        node_identity,
        containment_claim,
        slot_name,
        region_name,
        mosaic_name,
    }
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

fn root_page_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| {
                    handoff.role()
                        == worth_ui::facade::declaration::UiDeclarationStructuralRole::Page
                })
                .unwrap_or(false)
        })
        .expect("bootstrap root page artifact should exist")
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
            panic!("expected declaration artifact for {module_path}#{declaration_index} on freeze path")
        })
}

fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
    .with_structural_token(UiDslStructuralToken::new(
        "mosaic-sizing:workspace.sizing.main",
    ))
}

fn page_set_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page_set.shell"),
        UiDslSemanticFamily::PageSet,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("page-set:shell"))
}

fn local_composition_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
        UiDslSemanticFamily::LocalComposition,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 4),
    )
    .with_structural_token(UiDslStructuralToken::new("local-composition:inspector"))
}

fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 5),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

fn extra_root_page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.authored_root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/graph_topology_root_denial.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}
