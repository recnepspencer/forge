use std::collections::BTreeSet;

use crate::source::{WorthUiSourceToken, WorthUiSourceTokenKind};

use super::{
    worth_ui_authoring_symbol_table::WorthUiNamedAuthoringDecl, WorthUiAuthoringEntryDiagnostic,
    WorthUiAuthoringEntryDiagnosticCode,
};

pub(crate) fn validate_workspace_shell(
    workspace: WorthUiNamedAuthoringDecl<'_>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    let tokens = workspace.declaration.body().tokens();
    let mut shell_blocks = shell_block_ranges(tokens);
    let Some((start, end)) = shell_blocks.next() else {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::MissingWorkspaceShell,
            format!(
                "workspace '{}' must declare exactly one shell block",
                workspace.declaration.name_text()
            ),
            workspace.declaration.span().clone(),
        ));
        return;
    };

    for duplicate_start in shell_blocks {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::DuplicateWorkspaceShell,
            format!(
                "workspace '{}' cannot declare more than one shell block",
                workspace.declaration.name_text()
            ),
            tokens[duplicate_start.0].span().clone(),
        ));
    }

    validate_shell_block_entries(
        workspace.declaration.name_text(),
        workspace.declaration.span().clone(),
        &tokens[start + 2..end],
        diagnostics,
    );
}

fn shell_block_ranges(tokens: &[WorthUiSourceToken]) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut ranges = Vec::new();
    let mut index = 0usize;

    while index + 1 < tokens.len() {
        if is_shell_keyword(tokens[index].kind())
            && matches!(tokens[index + 1].kind(), WorthUiSourceTokenKind::LeftBrace)
        {
            if let Some(end) = matching_brace_index(tokens, index + 1) {
                ranges.push((index, end));
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }

    ranges.into_iter()
}

fn validate_shell_block_entries(
    workspace_name: &str,
    workspace_span: crate::source::WorthUiSourceSpan,
    tokens: &[WorthUiSourceToken],
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    let mut seen_slots = BTreeSet::new();
    let mut index = 0usize;

    while index < tokens.len() {
        let Some(slot) = ShellSlot::from_token_kind(tokens[index].kind()) else {
            diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                WorthUiAuthoringEntryDiagnosticCode::InvalidWorkspaceShellEntry,
                format!(
                    "workspace '{workspace_name}' shell block only supports topbar, rail, page_host, inspector, status, overlays, and toasts entries"
                ),
                tokens[index].span().clone(),
            ));
            return;
        };

        if !seen_slots.insert(slot) {
            diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                WorthUiAuthoringEntryDiagnosticCode::DuplicateWorkspaceShellSlot,
                format!(
                    "workspace '{workspace_name}' shell block cannot declare '{}' more than once",
                    slot.label()
                ),
                tokens[index].span().clone(),
            ));
        }

        index = validate_shell_slot_value(workspace_name, slot, tokens, index + 1, diagnostics);
    }

    for slot in ShellSlot::required_slots() {
        if !seen_slots.contains(slot) {
            diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                WorthUiAuthoringEntryDiagnosticCode::MissingWorkspaceShellSlot,
                format!(
                    "workspace '{workspace_name}' shell block requires a '{}' entry",
                    slot.label()
                ),
                tokens
                    .first()
                    .map(|token| token.span().clone())
                    .unwrap_or_else(|| workspace_span.clone()),
            ));
        }
    }
}

fn validate_shell_slot_value(
    workspace_name: &str,
    slot: ShellSlot,
    tokens: &[WorthUiSourceToken],
    index: usize,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> usize {
    match slot {
        ShellSlot::Overlays => validate_overlay_list(workspace_name, tokens, index, diagnostics),
        _ => validate_named_shell_target(workspace_name, slot, tokens, index, diagnostics),
    }
}

fn validate_named_shell_target(
    workspace_name: &str,
    slot: ShellSlot,
    tokens: &[WorthUiSourceToken],
    index: usize,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> usize {
    let Some(token) = tokens.get(index) else {
        diagnostics.push(invalid_shell_entry(
            workspace_name,
            format!("shell '{}' entry requires a named target", slot.label()),
            tokens.last(),
        ));
        return tokens.len();
    };

    if !matches!(token.kind(), WorthUiSourceTokenKind::Identifier(_)) {
        diagnostics.push(invalid_shell_entry(
            workspace_name,
            format!("shell '{}' entry requires a named target", slot.label()),
            Some(token),
        ));
    }
    index + 1
}

fn validate_overlay_list(
    workspace_name: &str,
    tokens: &[WorthUiSourceToken],
    mut index: usize,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> usize {
    let Some(open) = tokens.get(index) else {
        diagnostics.push(invalid_shell_entry(
            workspace_name,
            "shell 'overlays' entry requires a bracketed overlay list".to_owned(),
            tokens.last(),
        ));
        return tokens.len();
    };
    if !matches!(open.kind(), WorthUiSourceTokenKind::LeftBracket) {
        diagnostics.push(invalid_shell_entry(
            workspace_name,
            "shell 'overlays' entry requires a bracketed overlay list".to_owned(),
            Some(open),
        ));
        return index + 1;
    }
    index += 1;

    while let Some(token) = tokens.get(index) {
        match token.kind() {
            WorthUiSourceTokenKind::RightBracket => return index + 1,
            WorthUiSourceTokenKind::Comma => index += 1,
            WorthUiSourceTokenKind::Identifier(_) => index += 1,
            _ => {
                diagnostics.push(invalid_shell_entry(
                    workspace_name,
                    "shell 'overlays' entry requires only named overlay targets".to_owned(),
                    Some(token),
                ));
                return index + 1;
            }
        }
    }

    diagnostics.push(invalid_shell_entry(
        workspace_name,
        "shell 'overlays' entry requires a closing ']'".to_owned(),
        tokens.last(),
    ));
    tokens.len()
}

fn invalid_shell_entry(
    workspace_name: &str,
    detail: String,
    token: Option<&WorthUiSourceToken>,
) -> WorthUiAuthoringEntryDiagnostic {
    WorthUiAuthoringEntryDiagnostic::new(
        WorthUiAuthoringEntryDiagnosticCode::InvalidWorkspaceShellEntry,
        format!("workspace '{workspace_name}' {detail}"),
        token
            .expect("workspace shell diagnostics require a source span")
            .span()
            .clone(),
    )
}

fn is_shell_keyword(token_kind: &WorthUiSourceTokenKind) -> bool {
    matches!(token_kind, WorthUiSourceTokenKind::Identifier(text) if text == "shell")
}

fn matching_brace_index(tokens: &[WorthUiSourceToken], open_brace_index: usize) -> Option<usize> {
    let mut depth = 0usize;

    for (offset, token) in tokens.iter().enumerate().skip(open_brace_index) {
        match token.kind() {
            WorthUiSourceTokenKind::LeftBrace => depth += 1,
            WorthUiSourceTokenKind::RightBrace => {
                depth -= 1;
                if depth == 0 {
                    return Some(offset);
                }
            }
            _ => {}
        }
    }

    None
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum ShellSlot {
    Topbar,
    Rail,
    PageHost,
    Inspector,
    Status,
    Overlays,
    Toasts,
}

impl ShellSlot {
    fn from_token_kind(token_kind: &WorthUiSourceTokenKind) -> Option<Self> {
        match token_kind {
            WorthUiSourceTokenKind::Identifier(text) if text == "topbar" => Some(Self::Topbar),
            WorthUiSourceTokenKind::Identifier(text) if text == "rail" => Some(Self::Rail),
            WorthUiSourceTokenKind::Identifier(text) if text == "page_host" => Some(Self::PageHost),
            WorthUiSourceTokenKind::Identifier(text) if text == "inspector" => {
                Some(Self::Inspector)
            }
            WorthUiSourceTokenKind::Identifier(text) if text == "status" => Some(Self::Status),
            WorthUiSourceTokenKind::Identifier(text) if text == "overlays" => Some(Self::Overlays),
            WorthUiSourceTokenKind::Identifier(text) if text == "toasts" => Some(Self::Toasts),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Topbar => "topbar",
            Self::Rail => "rail",
            Self::PageHost => "page_host",
            Self::Inspector => "inspector",
            Self::Status => "status",
            Self::Overlays => "overlays",
            Self::Toasts => "toasts",
        }
    }

    fn required_slots() -> &'static [Self] {
        &[
            Self::Topbar,
            Self::Rail,
            Self::PageHost,
            Self::Inspector,
            Self::Status,
            Self::Toasts,
        ]
    }
}
