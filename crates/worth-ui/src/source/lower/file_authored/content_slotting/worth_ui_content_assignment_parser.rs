use std::collections::BTreeMap;

use crate::source::{
    WorthUiAuthoringEntryDiagnosticCode, WorthUiAuthoringEntryReport, WorthUiParsedPageDeclaration,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

use super::worth_ui_content_slotting_diagnostic::page_content_slotting_diagnostic;

pub(super) fn parse_content_assignments(
    page: &WorthUiParsedPageDeclaration,
    tokens: &[WorthUiSourceToken],
) -> Result<BTreeMap<String, String>, WorthUiAuthoringEntryReport> {
    let mut assignments = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut index = 0usize;
    while index < tokens.len() {
        match parse_assignment(tokens, index) {
            Ok((slot, surface, next_index)) => {
                if assignments.insert(slot.clone(), surface).is_some() {
                    diagnostics.push(page_content_slotting_diagnostic(
                        WorthUiAuthoringEntryDiagnosticCode::DuplicateContentSlotAssignment,
                        page,
                        format!(
                            "page '{}' content fills slot '{slot}' more than once",
                            page.name_text()
                        ),
                    ));
                }
                index = next_index;
            }
            Err(next_index) => {
                diagnostics.push(page_content_slotting_diagnostic(
                    WorthUiAuthoringEntryDiagnosticCode::InvalidContentSlotAssignment,
                    page,
                    format!(
                        "page '{}' content entries must use 'slot -> SurfaceName'",
                        page.name_text()
                    ),
                ));
                index = next_index.max(index + 1);
            }
        }
    }

    if diagnostics.is_empty() {
        Ok(assignments)
    } else {
        Err(WorthUiAuthoringEntryReport::new(diagnostics))
    }
}

fn parse_assignment(
    tokens: &[WorthUiSourceToken],
    index: usize,
) -> Result<(String, String, usize), usize> {
    let Some(WorthUiSourceTokenKind::Identifier(slot)) =
        tokens.get(index).map(WorthUiSourceToken::kind)
    else {
        return Err(skip_to_next_entry(tokens, index));
    };
    if !matches!(
        tokens.get(index + 1).map(WorthUiSourceToken::kind),
        Some(WorthUiSourceTokenKind::Arrow)
    ) {
        return Err(skip_to_next_entry(tokens, index + 1));
    }
    let Some(WorthUiSourceTokenKind::Identifier(surface)) =
        tokens.get(index + 2).map(WorthUiSourceToken::kind)
    else {
        return Err(skip_to_next_entry(tokens, index + 2));
    };
    let mut next_index = index + 3;
    if matches!(
        tokens.get(next_index).map(WorthUiSourceToken::kind),
        Some(WorthUiSourceTokenKind::Semicolon | WorthUiSourceTokenKind::Comma)
    ) {
        next_index += 1;
    }
    Ok((slot.clone(), surface.clone(), next_index))
}

fn skip_to_next_entry(tokens: &[WorthUiSourceToken], mut index: usize) -> usize {
    while index < tokens.len()
        && !matches!(
            tokens[index].kind(),
            WorthUiSourceTokenKind::Semicolon | WorthUiSourceTokenKind::Comma
        )
    {
        index += 1;
    }
    (index + 1).min(tokens.len())
}
