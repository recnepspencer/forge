use crate::source::{
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiSourcePackageLoader, WorthUiSourceParser,
};

#[test]
fn authoring_entry_accepts_workspace_page_hierarchy_with_typed_dynamic_page() {
    let lowered = lower_source(
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
            pages [OverviewPage, ProductsPage]
            dynamic_pages [ProductDetailPage(product_id: ProductId)]
        }

        page OverviewPage {
            runtime OverviewRuntime
            layout OverviewLayout
            content OverviewContent
        }

        page ProductsPage {
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }

        page ProductDetailPage(product_id: ProductId) {
            runtime ProductDetailRuntime
            layout ProductDetailLayout
            content ProductDetailContent
        }

        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent { main -> OverviewSurface }
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent { main -> ProductsSurface }
        runtime ProductDetailRuntime {}
        layout ProductDetailLayout { column { row height fill { slot main } } }
        content ProductDetailContent { main -> ProductDetailSurface }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect("authoring hierarchy should lower");

    let module = lowered
        .module(&lowered.module_ids()[0])
        .expect("artifact input module should exist");
    assert_eq!(
        module.nodes().len(),
        3,
        "page authoring should lower owned pages into canonical page artifact input nodes"
    );
    assert!(
        module
            .nodes()
            .iter()
            .all(|node| matches!(node, crate::source::WorthUiArtifactInputNode::Page(_))),
        "authoring-only runtime/layout/content declarations should compose into page nodes"
    );
}

#[test]
fn inline_and_extracted_authoring_forms_lower_to_equivalent_artifact_input() {
    let inline = lower_source(
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
            runtime {}
            layout { column { row height fill { slot main } } }
            content { main -> ProductsSurface }
        }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect("inline authoring should lower");
    let extracted = lower_source(
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
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent { main -> ProductsSurface }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect("extracted authoring should lower");

    assert!(inline.equivalent_shape(&extracted));
}

fn lower_source(
    source_text: &str,
) -> Result<crate::source::WorthUiArtifactInput, crate::source::WorthUiAuthoringEntryReport> {
    let source_package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", source_text)
        .compile()
        .expect("source package should compile");
    let parsed_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source package should parse");
    WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_package)
}
