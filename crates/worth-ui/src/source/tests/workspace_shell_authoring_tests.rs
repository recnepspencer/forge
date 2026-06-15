use crate::source::{
    WorthUiAuthoringEntryDiagnosticCode, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiSourcePackageLoader, WorthUiSourceParser,
};

#[test]
fn workspace_shell_is_required_for_authoring_hierarchy() {
    let report = lower_source(
        r#"
        app ShopifyAdminApp {
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }

        workspace AdminWorkspace {
            pages [OverviewPage]
        }

        page OverviewPage {
            runtime OverviewRuntime
            layout OverviewLayout
            content OverviewContent
        }

        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("workspace without shell must fail authoring entry validation");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::MissingWorkspaceShell]
    );
}

#[test]
fn workspace_shell_requires_each_structural_slot_once() {
    let report = lower_source(
        r#"
        app ShopifyAdminApp {
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }

        workspace AdminWorkspace {
            shell {
                topbar AdminTopbar
                topbar DuplicateTopbar
                rail AdminRail
                page_host AdminPageHost
                inspector AdminInspector
                overlays [CommandPaletteOverlay]
            }
            pages [OverviewPage]
        }

        page OverviewPage {
            runtime OverviewRuntime
            layout OverviewLayout
            content OverviewContent
        }

        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("duplicate or missing shell slots must fail authoring entry validation");

    assert_eq!(
        diagnostic_codes(&report),
        vec![
            WorthUiAuthoringEntryDiagnosticCode::MissingWorkspaceShellSlot,
            WorthUiAuthoringEntryDiagnosticCode::MissingWorkspaceShellSlot,
            WorthUiAuthoringEntryDiagnosticCode::DuplicateWorkspaceShellSlot,
        ]
    );
}

#[test]
fn workspace_shell_overlay_entry_requires_bracketed_named_targets() {
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
                overlays CommandPaletteOverlay
                toasts AdminToasts
            }
            pages [OverviewPage]
        }

        page OverviewPage {
            runtime OverviewRuntime
            layout OverviewLayout
            content OverviewContent
        }

        runtime OverviewRuntime {}
        layout OverviewLayout { column { row height fill { slot main } } }
        content OverviewContent {}
        appearance ShopifyAdminTheme {}
        "#,
    )
    .expect_err("overlay shell entry must remain bracketed");

    assert_eq!(
        diagnostic_codes(&report),
        vec![WorthUiAuthoringEntryDiagnosticCode::InvalidWorkspaceShellEntry]
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
