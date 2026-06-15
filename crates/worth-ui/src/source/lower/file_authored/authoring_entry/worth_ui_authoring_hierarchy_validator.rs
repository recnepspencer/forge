use std::collections::BTreeSet;

use crate::source::{
    WorthUiParsedAuthoringDeclaration, WorthUiSourceSpan, WorthUiSourceToken,
    WorthUiSourceTokenKind,
};

use super::{
    worth_ui_authoring_symbol_table::{WorthUiAuthoringSymbolTable, WorthUiNamedAuthoringDecl},
    worth_ui_authoring_workspace_page_validator::validate_workspace_page_ownership,
    worth_ui_authoring_workspace_shell_validator::validate_workspace_shell,
    WorthUiAuthoringEntryDiagnostic, WorthUiAuthoringEntryDiagnosticCode,
};

pub(crate) fn validate_authoring_hierarchy(
    table: &WorthUiAuthoringSymbolTable<'_>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> Option<BTreeSet<String>> {
    let app = require_app_declaration(table, diagnostics)?;
    let app_references = parse_app_references(app, diagnostics)?;
    let workspace = resolve_workspace(table, app, app_references.workspace_name(), diagnostics)?;
    validate_workspace_shell(workspace, diagnostics);
    let referenced_pages = validate_workspace_page_ownership(
        table,
        workspace.declaration.span(),
        app_references.workspace_name(),
        workspace.declaration.body().tokens(),
        diagnostics,
    );
    validate_workspace_ownership(table, app_references.workspace_name(), diagnostics);
    validate_theme_reference(table, app, app_references.theme_name(), diagnostics);
    Some(referenced_pages)
}

fn require_app_declaration<'a>(
    table: &'a WorthUiAuthoringSymbolTable<'a>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> Option<&'a WorthUiParsedAuthoringDeclaration> {
    let Some(app) = table.app_declaration() else {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::MissingAppDeclaration,
            "authoring hierarchy requires an app declaration when workspace or page declarations are present",
            first_authoring_span(table).clone(),
        ));
        return None;
    };
    Some(app)
}

fn first_authoring_span<'a>(table: &'a WorthUiAuthoringSymbolTable<'a>) -> &'a WorthUiSourceSpan {
    table
        .app_declaration()
        .map(WorthUiParsedAuthoringDeclaration::span)
        .or_else(|| {
            table
                .workspaces()
                .values()
                .next()
                .map(|decl| decl.declaration.span())
        })
        .or_else(|| {
            table
                .pages()
                .values()
                .next()
                .map(|decl| decl.declaration.span())
        })
        .expect("authoring root validation only runs when authoring roots exist")
}

fn parse_app_references(
    app: &WorthUiParsedAuthoringDeclaration,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> Option<AppAuthoringReferences> {
    let workspace_name = parse_named_app_reference(app, AppReferenceKind::Workspace, diagnostics)?;
    let theme_name = parse_named_app_reference(app, AppReferenceKind::Theme, diagnostics)?;
    Some(AppAuthoringReferences {
        workspace_name,
        theme_name,
    })
}

fn parse_named_app_reference(
    app: &WorthUiParsedAuthoringDeclaration,
    reference_kind: AppReferenceKind,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> Option<String> {
    let mut value = None;

    for window in app.body().tokens().windows(2) {
        if reference_kind.matches_leading_token(window[0].kind())
            && matches!(window[1].kind(), WorthUiSourceTokenKind::Identifier(_))
        {
            let name = identifier_text(&window[1]);
            if value.replace(name.to_owned()).is_some() {
                diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                    reference_kind.duplicate_code(),
                    format!(
                        "app must declare exactly one {} reference",
                        reference_kind.label()
                    ),
                    app.span().clone(),
                ));
            }
        }
    }

    if value.is_none() {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            reference_kind.missing_code(),
            format!(
                "app declaration requires a {} reference",
                reference_kind.label()
            ),
            app.span().clone(),
        ));
    }

    value
}

fn resolve_workspace<'a>(
    table: &'a WorthUiAuthoringSymbolTable<'a>,
    app: &WorthUiParsedAuthoringDeclaration,
    workspace_name: &str,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> Option<WorthUiNamedAuthoringDecl<'a>> {
    let Some(workspace) = table.workspaces().get(workspace_name).copied() else {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::UnknownWorkspaceReference,
            format!("app references unknown workspace '{workspace_name}'"),
            app.span().clone(),
        ));
        return None;
    };
    Some(workspace)
}

fn validate_workspace_ownership(
    table: &WorthUiAuthoringSymbolTable<'_>,
    workspace_name: &str,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    for (name, declaration) in table.workspaces() {
        if *name != workspace_name {
            diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                WorthUiAuthoringEntryDiagnosticCode::UnownedWorkspaceDeclaration,
                format!("workspace '{name}' is not owned by the declared app"),
                declaration.declaration.span().clone(),
            ));
        }
    }
}

fn validate_theme_reference(
    table: &WorthUiAuthoringSymbolTable<'_>,
    app: &WorthUiParsedAuthoringDeclaration,
    theme_name: &str,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    if !table.appearances().contains_key(theme_name) {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::UnknownThemeReference,
            format!("app references unknown theme '{theme_name}'"),
            app.span().clone(),
        ));
    }
}

fn identifier_text(token: &WorthUiSourceToken) -> &str {
    match token.kind() {
        WorthUiSourceTokenKind::Identifier(text) => text,
        _ => unreachable!("identifier text requested from non-identifier token"),
    }
}

struct AppAuthoringReferences {
    workspace_name: String,
    theme_name: String,
}

impl AppAuthoringReferences {
    fn workspace_name(&self) -> &str {
        self.workspace_name.as_str()
    }

    fn theme_name(&self) -> &str {
        self.theme_name.as_str()
    }
}

#[derive(Clone, Copy)]
enum AppReferenceKind {
    Workspace,
    Theme,
}

impl AppReferenceKind {
    fn label(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Theme => "theme",
        }
    }

    fn matches_leading_token(self, token_kind: &WorthUiSourceTokenKind) -> bool {
        match self {
            Self::Workspace => matches!(token_kind, WorthUiSourceTokenKind::KeywordWorkspace),
            Self::Theme => {
                matches!(token_kind, WorthUiSourceTokenKind::Identifier(text) if text == "theme")
            }
        }
    }

    fn missing_code(self) -> WorthUiAuthoringEntryDiagnosticCode {
        match self {
            Self::Workspace => WorthUiAuthoringEntryDiagnosticCode::MissingWorkspaceReference,
            Self::Theme => WorthUiAuthoringEntryDiagnosticCode::MissingThemeReference,
        }
    }

    fn duplicate_code(self) -> WorthUiAuthoringEntryDiagnosticCode {
        match self {
            Self::Workspace => WorthUiAuthoringEntryDiagnosticCode::DuplicateWorkspaceReference,
            Self::Theme => WorthUiAuthoringEntryDiagnosticCode::DuplicateThemeReference,
        }
    }
}
