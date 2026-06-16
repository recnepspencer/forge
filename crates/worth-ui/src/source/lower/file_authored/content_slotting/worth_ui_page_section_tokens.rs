use crate::source::{
    WorthUiAuthoringEntryDiagnosticCode, WorthUiAuthoringEntryReport, WorthUiParsedPageDeclaration,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

use super::super::authoring_entry::WorthUiAuthoringSymbolTable;
use super::worth_ui_content_slotting_diagnostic::page_content_slotting_diagnostic;

#[derive(Clone, Copy)]
pub(super) enum PageSectionKind {
    Layout,
    Content,
}

pub(super) fn page_section_tokens(
    page: &WorthUiParsedPageDeclaration,
    table: &WorthUiAuthoringSymbolTable<'_>,
    kind: PageSectionKind,
) -> Result<Vec<WorthUiSourceToken>, WorthUiAuthoringEntryReport> {
    let tokens = page.body().tokens();
    let mut index = 0usize;
    while index < tokens.len() {
        if !matches_section(tokens[index].kind(), kind) {
            index += 1;
            continue;
        }
        return match tokens.get(index + 1).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Identifier(name)) => {
                named_section_tokens(table, kind, name)
            }
            Some(WorthUiSourceTokenKind::LeftBrace) => Ok(extract_inline_tokens(tokens, index + 1)),
            _ => Err(WorthUiAuthoringEntryReport::new(vec![
                page_content_slotting_diagnostic(
                    WorthUiAuthoringEntryDiagnosticCode::MissingPageSection,
                    page,
                    format!("page '{}' has an invalid section target", page.name_text()),
                ),
            ])),
        };
    }
    Err(WorthUiAuthoringEntryReport::new(vec![
        page_content_slotting_diagnostic(
            WorthUiAuthoringEntryDiagnosticCode::MissingPageSection,
            page,
            format!("page '{}' is missing a required section", page.name_text()),
        ),
    ]))
}

fn named_section_tokens(
    table: &WorthUiAuthoringSymbolTable<'_>,
    kind: PageSectionKind,
    name: &str,
) -> Result<Vec<WorthUiSourceToken>, WorthUiAuthoringEntryReport> {
    let declaration = match kind {
        PageSectionKind::Layout => table.layouts().get(name),
        PageSectionKind::Content => table.contents().get(name),
    }
    .expect("authoring validation should reject unknown section references");
    Ok(declaration.declaration.body().tokens().to_vec())
}

fn extract_inline_tokens(
    tokens: &[WorthUiSourceToken],
    left_brace_index: usize,
) -> Vec<WorthUiSourceToken> {
    let mut depth = 0usize;
    for index in left_brace_index..tokens.len() {
        match tokens[index].kind() {
            WorthUiSourceTokenKind::LeftBrace => depth += 1,
            WorthUiSourceTokenKind::RightBrace => {
                depth -= 1;
                if depth == 0 {
                    return tokens[left_brace_index + 1..index].to_vec();
                }
            }
            _ => {}
        }
    }
    Vec::new()
}

fn matches_section(token: &WorthUiSourceTokenKind, kind: PageSectionKind) -> bool {
    matches!(
        (token, kind),
        (
            WorthUiSourceTokenKind::KeywordLayout,
            PageSectionKind::Layout
        ) | (
            WorthUiSourceTokenKind::KeywordContent,
            PageSectionKind::Content
        )
    )
}
