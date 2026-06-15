use crate::source::{
    WorthUiAuthoringEntryDiagnosticCode, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiSourcePackageLoader, WorthUiSourceParser,
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
        content OverviewContent {}
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent {}
        runtime ProductDetailRuntime {}
        layout ProductDetailLayout { column { row height fill { slot main } } }
        content ProductDetailContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect("authoring hierarchy should lower");

    let module = lowered
        .module(&lowered.module_ids()[0])
        .expect("artifact input module should exist");
    assert!(
        module.nodes().is_empty(),
        "phase 1 authoring entry should preserve the existing IR families without inventing page/runtime nodes"
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
            content {}
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
        content ProductsContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect("extracted authoring should lower");

    assert!(inline.equivalent_shape(&extracted));
}

#[test]
fn unknown_workspace_reference_is_rejected_at_authoring_entry_boundary() {
    let report = lower_source(
        r#"
        app ShopifyAdminApp {
            theme ShopifyAdminTheme
            workspace MissingWorkspace
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
        content ProductsContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("unknown workspace should fail authoring entry");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::UnknownWorkspaceReference]
    );
}

#[test]
fn unknown_theme_reference_is_rejected_at_authoring_entry_boundary() {
    let report = lower_source(
        r#"
        app ShopifyAdminApp {
            theme MissingTheme
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
        content ProductsContent {}
        "#,
    )
    .expect_err("unknown theme should fail authoring entry");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::UnknownThemeReference]
    );
}

#[test]
fn page_missing_required_sections_is_rejected_before_snapshot_resolution() {
    let report = lower_source(
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
        }
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("missing content section should fail authoring entry");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::MissingPageSection]
    );
}

#[test]
fn dynamic_page_signature_must_match_declared_template_parameters() {
    let report = lower_source(
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
            pages [OverviewPage]
            dynamic_pages [ProductDetailPage(product_slug: ProductSlug)]
        }
        page OverviewPage {
            runtime OverviewRuntime
            layout OverviewLayout
            content OverviewContent
        }
        page ProductDetailPage(product_id: ProductId) {
            runtime ProductDetailRuntime
            layout ProductDetailLayout
            content ProductDetailContent
        }
        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent {}
        runtime ProductDetailRuntime {}
        layout ProductDetailLayout { column { row height fill { slot main } } }
        content ProductDetailContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("mismatched dynamic page signature should fail authoring entry");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::DynamicPageSignatureMismatch]
    );
}

#[test]
fn static_pages_list_cannot_reference_template_page_forms() {
    let report = lower_source(
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
            pages [ProductDetailPage(product_id: ProductId)]
        }
        page ProductDetailPage(product_id: ProductId) {
            runtime ProductDetailRuntime
            layout ProductDetailLayout
            content ProductDetailContent
        }
        runtime ProductDetailRuntime {}
        layout ProductDetailLayout { column { row height fill { slot main } } }
        content ProductDetailContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("static page list should reject template page references");

    assert_eq!(
        diagnostic_codes(&report),
        vec![
            WorthUiAuthoringEntryDiagnosticCode::StaticPageCannotDeclareSignature,
            WorthUiAuthoringEntryDiagnosticCode::StaticPageReferencesTemplate
        ]
    );
}

#[test]
fn dynamic_pages_list_requires_typed_signature() {
    let report = lower_source(
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
            pages [OverviewPage]
            dynamic_pages [ProductDetailPage]
        }
        page OverviewPage {
            runtime OverviewRuntime
            layout OverviewLayout
            content OverviewContent
        }
        page ProductDetailPage(product_id: ProductId) {
            runtime ProductDetailRuntime
            layout ProductDetailLayout
            content ProductDetailContent
        }
        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent {}
        runtime ProductDetailRuntime {}
        layout ProductDetailLayout { column { row height fill { slot main } } }
        content ProductDetailContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("dynamic page list without a signature should fail authoring entry");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::DynamicPageRequiresSignature]
    );
}

#[test]
fn unowned_page_is_rejected_at_authoring_entry_boundary() {
    let report = lower_source(
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
            pages [OverviewPage]
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
        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent {}
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("unowned page should fail authoring entry");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::UnownedPageDeclaration]
    );
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

fn diagnostic_codes(
    report: &crate::source::WorthUiAuthoringEntryReport,
) -> Vec<WorthUiAuthoringEntryDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}
