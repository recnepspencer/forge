use crate::source::{
    WorthUiAuthoringEntryDiagnosticCode, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiSourcePackageLoader, WorthUiSourceParser,
};

#[test]
fn page_content_must_fill_every_layout_slot_once() {
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
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout {
            column {
                row height fill { slot main }
                row height fit { slot status }
            }
        }
        content ProductsContent { main -> ProductsSurface }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("missing slot content must fail before snapshot resolution");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::MissingContentSlotAssignment]
    );
}

#[test]
fn page_content_rejects_duplicate_slot_assignments() {
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
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent {
            main -> ProductsSurface
            main -> DuplicateSurface
        }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("duplicate content slots must fail");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::DuplicateContentSlotAssignment]
    );
}

#[test]
fn page_layout_rejects_duplicate_slot_names_before_mount_construction() {
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
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout {
            column {
                row height fill { slot main }
                row height fit { slot main }
            }
        }
        content ProductsContent { main -> ProductsSurface }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("duplicate layout slot names would make content slotting ambiguous");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::InvalidLayoutTopology]
    );
}

#[test]
fn page_content_rejects_unknown_slot_assignments() {
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
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent {
            main -> ProductsSurface
            phantom -> PhantomSurface
        }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("unknown content slots must fail");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::UnknownContentSlotAssignment]
    );
}

#[test]
fn page_content_rejects_malformed_slot_assignment_syntax() {
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
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent { main ProductsSurface }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("content assignment syntax must use arrow form");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::InvalidContentSlotAssignment]
    );
}

#[test]
fn page_content_rejects_layout_modifier_smuggling() {
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
            content ProductsContent
        }
        runtime ProductsRuntime {}
        layout ProductsLayout { column { row height fill { slot main } } }
        content ProductsContent { main -> ProductsSurface height fit scroll_owner }
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("content must not accept layout modifiers as content facts");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::InvalidContentSlotAssignment]
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
