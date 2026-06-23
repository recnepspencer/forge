struct CanonicalSourceFixture<'a> {
    theme_name: &'a str,
    collection_surface: &'a str,
    topbar: &'a str,
    collection_row_height: &'a str,
    root_modifiers: &'a str,
    workspace_pages: &'a str,
    extra_declarations: &'a str,
}

impl Default for CanonicalSourceFixture<'_> {
    fn default() -> Self {
        Self {
            theme_name: "ShopifyAdminTheme",
            collection_surface: "validation.surface.products.collection",
            topbar: "AdminTopbar",
            collection_row_height: "fill",
            root_modifiers: "",
            workspace_pages: "ProductsPage",
            extra_declarations: "",
        }
    }
}

pub(crate) fn source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        collection_surface,
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn reordered_source_text(collection_surface: &str) -> String {
    format!(
        r#"
        runtime ProductsRuntime {{}}
        appearance ShopifyAdminTheme {{}}

        content ProductsContent {{
            collection -> {collection_surface}
        }}

        layout ProductsLayout {{
            column {{
                row height fill scroll_owner {{ slot collection }}
            }}
        }}

        page ProductsPage {{
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }}

        workspace AdminWorkspace {{
            shell {{
                topbar AdminTopbar
                rail AdminPrimaryRail
                page_host AdminPageHost
                inspector AdminInspectorDock
                status AdminStatusBar
                overlays [CommandPaletteOverlay]
                toasts AdminToastCenter
            }}
            pages [ProductsPage]
        }}

        app ShopifyAdminApp {{
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }}
        "#
    )
}

pub(crate) fn layout_changed_source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        collection_surface,
        collection_row_height: "fit",
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn layout_gap_changed_source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        collection_surface,
        root_modifiers: " gap(24)",
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn layout_padding_changed_source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        collection_surface,
        root_modifiers: " padding(18)",
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn shell_reassigned_source_text() -> String {
    canonical_source(CanonicalSourceFixture {
        topbar: "AlternateTopbar",
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn appearance_recipe_renamed_source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        theme_name: "ShopifyAdminThemeAlt",
        collection_surface,
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn runtime_binding_added_source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        collection_surface,
        extra_declarations: "binding workspace.view_binding.selection {}",
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn page_added_source_text(collection_surface: &str) -> String {
    canonical_source(CanonicalSourceFixture {
        collection_surface,
        workspace_pages: "ProductsPage, OrdersPage",
        extra_declarations: r#"
        page OrdersPage {
            runtime OrdersRuntime
            layout OrdersLayout
            content OrdersContent
        }

        runtime OrdersRuntime {}

        layout OrdersLayout {
            column {
                row height fill scroll_owner { slot orders }
            }
        }

        content OrdersContent {
            orders -> validation.surface.orders.collection
        }
        "#,
        ..CanonicalSourceFixture::default()
    })
}

pub(crate) fn mixed_content_and_appearance_source_text() -> String {
    canonical_source(CanonicalSourceFixture {
        theme_name: "ShopifyAdminThemeAlt",
        collection_surface: "validation.surface.orders.collection",
        ..CanonicalSourceFixture::default()
    })
}

fn canonical_source(fixture: CanonicalSourceFixture<'_>) -> String {
    format!(
        r#"
        app ShopifyAdminApp {{
            theme {theme_name}
            workspace AdminWorkspace
        }}

        workspace AdminWorkspace {{
            shell {{
                topbar {topbar}
                rail AdminPrimaryRail
                page_host AdminPageHost
                inspector AdminInspectorDock
                status AdminStatusBar
                overlays [CommandPaletteOverlay]
                toasts AdminToastCenter
            }}
            pages [{workspace_pages}]
        }}

        page ProductsPage {{
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }}

        runtime ProductsRuntime {{}}
        appearance {theme_name} {{}}

        layout ProductsLayout {{
            column{root_modifiers} {{
                row height {collection_row_height} scroll_owner {{ slot collection }}
            }}
        }}

        content ProductsContent {{
            collection -> {collection_surface}
        }}

        {extra_declarations}
        "#,
        theme_name = fixture.theme_name,
        topbar = fixture.topbar,
        workspace_pages = fixture.workspace_pages,
        collection_row_height = fixture.collection_row_height,
        root_modifiers = fixture.root_modifiers,
        collection_surface = fixture.collection_surface,
        extra_declarations = fixture.extra_declarations,
    )
}
