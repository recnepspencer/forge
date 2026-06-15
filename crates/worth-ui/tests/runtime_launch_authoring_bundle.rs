use worth_ui::facade::{WorthUi, WorthUiRuntimeSourceModule};

#[test]
fn prepare_authoring_for_returns_layout_topology_from_same_source_package_as_runtime_launch() {
    let app = WorthUi::app().freeze();
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
            r#"
            app ShopifyAdminApp {
                theme ShopifyAdminTheme
                workspace AdminWorkspace
            }

            workspace AdminWorkspace {
                shell {
                    topbar AdminTopbar
                    rail AdminPrimaryRail
                    page_host AdminPageHost
                    inspector AdminInspectorDock
                    status AdminStatusBar
                    overlays [CommandPaletteOverlay]
                    toasts AdminToastCenter
                }

                pages [ProductsPage]
            }

            page ProductsPage {
                title "Products"
                runtime ProductsRuntime
                layout ProductsLayout
                content ProductsContent
            }

            runtime ProductsRuntime {}
            content ProductsContent {}
            appearance ShopifyAdminTheme {}

            layout ProductsLayout {
                column {
                    row height fit { slot toolbar }
                    row height fill scroll_owner { slot collection }
                }
            }
            "#,
        ))
        .prepare_authoring_for(&app)
        .expect("authoring bundle should prepare");

    let products_page = prepared
        .layout_topology()
        .page("ProductsPage")
        .expect("prepared authoring should expose page topology");
    assert_eq!(products_page.layout_name(), "ProductsLayout");
    assert!(!products_page.dynamic_template());

    let runtime = app
        .launch_runtime(prepared.into_runtime_launch())
        .expect("prepared authoring bundle should still launch a runtime");
    assert_ne!(runtime.inspect_active().artifact_digest(), 0);
    assert_ne!(runtime.inspect_active().active_plan_digest(), 0);
}
