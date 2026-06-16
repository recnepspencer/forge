use crate::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence,
    MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfaceDescriptor, SurfaceId,
    SurfaceKind, SurfacePlacementClass, SurfaceStateClass, WorthUi, WorthUiApp,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInput, WorthUiArtifactInputResolver, WorthUiArtifactNode,
    WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiMosaicRegionFacts, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiSourcePackageLoader, WorthUiSourceParser, WorthUiStructuralLegalityLowerer,
};

#[test]
fn source_authored_page_slotting_survives_canonical_artifact_assembly() {
    let artifact_input = lower_source(
        r#"
        app ShopifyAdminApp {
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }
        workspace AdminWorkspace {
            shell {
                topbar AdminTopbar
                rail AdminRail
                page_host AdminPageHost
                inspector AdminInspector
                status AdminStatus
                overlays [CommandPaletteOverlay]
                toasts AdminToasts
            }
            pages [ProductsPage]
        }
        page ProductsPage {
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout {
            column {
                row height fill { slot main }
                row height fit { slot status }
            }
        }
        content ProductsContent {
            main -> validation.surface.products.collection
            status -> validation.surface.products.status
        }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect("source-authored slotting should lower to artifact input");
    let artifact = canonical_artifact_from_input(artifact_input);
    let page = artifact_page(&artifact, "ProductsPage");

    assert_eq!(
        mounted_surface_ids(page.structure().root_regions()),
        vec![
            "validation.surface.products.collection".to_owned(),
            "validation.surface.products.status".to_owned()
        ],
        "source-authored content slots must survive as canonical mosaic mounts"
    );
}

#[test]
fn content_slot_declaration_reorder_preserves_canonical_page_identity() {
    let canonical = canonical_artifact_from_input(
        lower_source(products_page_source(
            r#"
            main -> validation.surface.products.collection
            status -> validation.surface.products.status
            "#,
        ))
        .expect("canonical content order should lower"),
    );
    let reordered = canonical_artifact_from_input(
        lower_source(products_page_source(
            r#"
            status -> validation.surface.products.status
            main -> validation.surface.products.collection
            "#,
        ))
        .expect("reordered content slots should lower"),
    );

    let canonical_page = artifact_page(&canonical, "ProductsPage");
    let reordered_page = artifact_page(&reordered, "ProductsPage");
    assert_eq!(
        canonical_page.identity_seed(),
        reordered_page.identity_seed(),
        "slot declaration order must not become page identity folklore"
    );
    assert_eq!(
        mounted_surface_ids(canonical_page.structure().root_regions()),
        mounted_surface_ids(reordered_page.structure().root_regions()),
        "canonical mount ordering must come from layout topology, not content declaration order"
    );
}

fn products_page_source(content_assignments: &str) -> String {
    format!(
        r#"
        app ShopifyAdminApp {{
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }}
        workspace AdminWorkspace {{
            shell {{
                topbar AdminTopbar
                rail AdminRail
                page_host AdminPageHost
                inspector AdminInspector
                status AdminStatus
                overlays [CommandPaletteOverlay]
                toasts AdminToasts
            }}
            pages [ProductsPage]
        }}
        page ProductsPage {{
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }}
        runtime ProductsRuntime {{}}
        layout ProductsLayout {{
            column {{
                row height fill {{ slot main }}
                row height fit {{ slot status }}
            }}
        }}
        content ProductsContent {{
            {content_assignments}
        }}
        appearance ShopifyAdminTheme {{}}
        "#
    )
}

fn lower_source(
    source_text: impl AsRef<str>,
) -> Result<WorthUiArtifactInput, crate::source::WorthUiAuthoringEntryReport> {
    let source_package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", source_text.as_ref())
        .compile()
        .expect("source package should compile");
    let parsed_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source package should parse");
    WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_package)
}

fn canonical_artifact_from_input(artifact_input: WorthUiArtifactInput) -> WorthUiArtifact {
    let app = page_artifact_test_app();
    let snapshot = app.capabilities();
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("page artifact input should resolve against registered surfaces");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("page slot structure should satisfy registered mosaic rules");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("page slot structure should bind without query-local workarounds");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("page structure should produce stable identity seeds")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("page structure should assemble as canonical artifact")
}

fn page_artifact_test_app() -> WorthUiApp {
    WorthUi::app()
        .register_component(validation_component())
        .register_mosaic_region_kind(page_region(
            "worth.ui.layout.column",
            MosaicRegionRole::stack(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(page_region(
            "worth.ui.layout.row",
            MosaicRegionRole::split(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(page_region(
            "worth.ui.layout.slot",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
        .register_surface(validation_surface("validation.surface.products.collection"))
        .register_surface(validation_surface("validation.surface.products.status"))
        .freeze()
}

fn validation_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.sample").unwrap(),
        ComponentPropSchema::named("validation.sample.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn page_region(
    id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(MosaicRegionKindId::new(id).unwrap(), role)
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}

fn validation_surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new("validation.component.sample").unwrap(),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn artifact_page<'a>(
    artifact: &'a WorthUiArtifact,
    page_name: &str,
) -> &'a crate::source::WorthUiArtifactPageNode {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Page(page) if page.name_text() == page_name => Some(page),
            _ => None,
        })
        .expect("canonical artifact should contain source-authored page node")
}

fn mounted_surface_ids(regions: &[WorthUiMosaicRegionFacts]) -> Vec<String> {
    let mut surface_ids = Vec::new();
    collect_mounted_surface_ids(regions, &mut surface_ids);
    surface_ids
}

fn collect_mounted_surface_ids(
    regions: &[WorthUiMosaicRegionFacts],
    surface_ids: &mut Vec<String>,
) {
    for region in regions {
        for mount in region.mounts() {
            surface_ids.push(mount.surface().id().as_str().to_owned());
        }
        collect_mounted_surface_ids(region.child_regions(), surface_ids);
    }
}
