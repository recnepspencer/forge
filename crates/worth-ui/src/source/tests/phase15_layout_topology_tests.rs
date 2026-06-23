use crate::source::{
    build_layout_topology_catalog, WorthUiLayoutAxis, WorthUiLayoutSizingSpec,
    WorthUiLayoutSizingValue, WorthUiSourcePackageLoader, WorthUiSourceParser,
};

#[test]
fn products_layout_topology_preserves_nested_sizing_and_scroll_structure() {
    let parsed_package = parse_source(
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
            column gap(18) padding(20) {
                row height fit { slot toolbar }

                row height fill gap(16) {
                    column width clamp(min: rail.md, preferred: share(2), max: rail.xl) scroll_owner {
                        slot filters
                    }

                    column width fill {
                        row height ratio(3, 5) scroll_owner {
                            slot collection
                        }

                        row height share(2) scroll_owner {
                            slot activity
                        }
                    }

                    column width clamp(min: inspector.md, preferred: share(2), max: inspector.xl) resizable restore {
                        slot inspector
                    }
                }

                row height fit { slot status }
            }
        }
        "#,
    );

    let catalog =
        build_layout_topology_catalog(&parsed_package).expect("phase 3 sample should build");
    let products_page = catalog
        .page("ProductsPage")
        .expect("products page layout should exist");
    let root = products_page.root();

    assert_eq!(root.axis(), &WorthUiLayoutAxis::Column);
    assert_eq!(root.children().len(), 3);
    assert_eq!(root.gap(), Some(&WorthUiLayoutSizingValue::Number(18)));
    assert_eq!(root.padding(), Some(&WorthUiLayoutSizingValue::Number(20)));

    let center_row = root.children()[1]
        .as_region()
        .expect("middle child should be a layout region");
    assert_eq!(center_row.sizing(), Some(&WorthUiLayoutSizingSpec::Fill));
    assert_eq!(center_row.children().len(), 3);
    assert_eq!(
        center_row.gap(),
        Some(&WorthUiLayoutSizingValue::Number(16))
    );

    let filters = center_row.children()[0]
        .as_region()
        .expect("filters column should be a region");
    assert!(filters.scroll_owner());
    assert_eq!(
        filters.sizing(),
        Some(&WorthUiLayoutSizingSpec::Clamp {
            min: WorthUiLayoutSizingValue::NamedToken("rail.md".to_owned()),
            preferred: Box::new(WorthUiLayoutSizingSpec::Share(2)),
            max: WorthUiLayoutSizingValue::NamedToken("rail.xl".to_owned()),
        })
    );

    let collection_column = center_row.children()[1]
        .as_region()
        .expect("collection column should be a region");
    assert_eq!(collection_column.axis(), &WorthUiLayoutAxis::Column);
    assert_eq!(collection_column.children().len(), 2);
    assert_eq!(
        collection_column.children()[0]
            .as_region()
            .expect("collection slot region should exist")
            .sizing(),
        Some(&WorthUiLayoutSizingSpec::Ratio {
            numerator: 3,
            denominator: 5,
        })
    );

    let inspector = center_row.children()[2]
        .as_region()
        .expect("inspector column should be a region");
    assert!(inspector.resizable());
    assert!(inspector.restorable());
}

#[test]
fn layout_topology_catalog_is_equivalent_under_module_reorder() {
    const MAIN_MODULE: &str = r#"
        app AdminApp {
            theme AdminTheme
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

            pages [ProductsPage, OrdersPage]
        }
        "#;
    const PRODUCTS_MODULE: &str = r#"
        page ProductsPage {
            title "Products"
            runtime ProductsRuntime
            layout SharedLayout
            content ProductsContent
        }

        runtime ProductsRuntime {}
        content ProductsContent {}
        "#;
    const ORDERS_MODULE: &str = r#"
        page OrdersPage {
            title "Orders"
            runtime OrdersRuntime
            layout OrdersLayout
            content OrdersContent
        }

        runtime OrdersRuntime {}
        content OrdersContent {}
        "#;
    const LAYOUTS_MODULE: &str = r#"
        layout SharedLayout {
            column {
                row height fit { slot toolbar }
                row height fill scroll_owner { slot collection }
            }
        }

        layout OrdersLayout {
            row {
                column width share(2) { slot queue }
                column width fill { slot inspector }
            }
        }

        appearance AdminTheme {}
        "#;

    let catalog_a = topology_catalog_from_modules([
        ("app/main.wui", MAIN_MODULE),
        ("app/pages/products.wui", PRODUCTS_MODULE),
        ("app/pages/orders.wui", ORDERS_MODULE),
        ("app/layouts/main.wui", LAYOUTS_MODULE),
    ]);
    let catalog_b = topology_catalog_from_modules([
        ("app/layouts/main.wui", LAYOUTS_MODULE),
        ("app/pages/orders.wui", ORDERS_MODULE),
        ("app/pages/products.wui", PRODUCTS_MODULE),
        ("app/main.wui", MAIN_MODULE),
    ]);

    assert_eq!(catalog_a, catalog_b);
}

fn parse_source(module_path: &str, source_text: &str) -> crate::source::WorthUiParsedSourcePackage {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(module_path, source_text)
        .compile()
        .expect("source package should compile");
    WorthUiSourceParser::parse_package(&package).expect("source package should parse")
}

fn topology_catalog_from_modules<'a>(
    modules: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> crate::source::WorthUiLayoutTopologyCatalog {
    let mut loader = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace");
    for (path, source_text) in modules {
        loader = loader.register_module_with_source(path, source_text);
    }
    let package = loader.compile().expect("module set should compile");
    let parsed = WorthUiSourceParser::parse_package(&package).expect("module set should parse");
    build_layout_topology_catalog(&parsed).expect("layout topology should build")
}

trait LayoutChildTestExt {
    fn as_region(&self) -> Option<&crate::source::WorthUiLayoutTopologyNode>;
}

impl LayoutChildTestExt for crate::source::WorthUiLayoutTopologyChild {
    fn as_region(&self) -> Option<&crate::source::WorthUiLayoutTopologyNode> {
        match self {
            crate::source::WorthUiLayoutTopologyChild::Region(node) => Some(node),
            crate::source::WorthUiLayoutTopologyChild::Slot(_) => None,
        }
    }
}
