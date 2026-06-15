use crate::source::{
    validate_layout_topology_tokens, WorthUiParsedPageDeclaration, WorthUiSourceToken,
    WorthUiSourceTokenKind,
};

use super::{
    worth_ui_authoring_symbol_table::WorthUiAuthoringSymbolTable, WorthUiAuthoringEntryDiagnostic,
    WorthUiAuthoringEntryDiagnosticCode,
};

pub(crate) fn validate_page_sections(
    page: &WorthUiParsedPageDeclaration,
    table: &WorthUiAuthoringSymbolTable<'_>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    let mut runtime = None;
    let mut layout = None;
    let mut content = None;
    let mut appearance = None;
    let mut index = 0usize;
    let tokens = page.body().tokens();

    while index < tokens.len() {
        let (kind, next) = match tokens[index].kind() {
            WorthUiSourceTokenKind::KeywordRuntime => ("runtime", index + 1),
            WorthUiSourceTokenKind::KeywordLayout => ("layout", index + 1),
            WorthUiSourceTokenKind::KeywordContent => ("content", index + 1),
            WorthUiSourceTokenKind::KeywordAppearance => ("appearance", index + 1),
            _ => {
                index += 1;
                continue;
            }
        };

        let section_target = match tokens.get(next).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Identifier(name)) => {
                index = next + 1;
                PageSectionTarget::Named(name.clone())
            }
            Some(WorthUiSourceTokenKind::LeftBrace) => {
                let (inline_tokens, next_index) = extract_balanced_block(tokens, next);
                index = next_index;
                PageSectionTarget::Inline(inline_tokens)
            }
            _ => {
                diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                    WorthUiAuthoringEntryDiagnosticCode::DuplicatePageSection,
                    format!(
                        "page '{}' section '{kind}' requires a named reference or inline block",
                        page.name_text()
                    ),
                    tokens[index].span().clone(),
                ));
                index += 1;
                continue;
            }
        };

        let slot = match kind {
            "runtime" => &mut runtime,
            "layout" => &mut layout,
            "content" => &mut content,
            "appearance" => &mut appearance,
            _ => unreachable!(),
        };
        if let Some(previous) = slot.replace(section_target.clone()) {
            diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                if previous != section_target {
                    WorthUiAuthoringEntryDiagnosticCode::MixedPageSectionDefinition
                } else {
                    WorthUiAuthoringEntryDiagnosticCode::DuplicatePageSection
                },
                format!(
                    "page '{}' declares section '{kind}' more than once",
                    page.name_text()
                ),
                page.span().clone(),
            ));
        }
    }

    require_page_section(page, "runtime", runtime.as_ref(), diagnostics);
    require_page_section(page, "layout", layout.as_ref(), diagnostics);
    require_page_section(page, "content", content.as_ref(), diagnostics);

    validate_named_target(
        runtime.as_ref(),
        |name| table.runtimes().contains_key(name),
        page,
        diagnostics,
        WorthUiAuthoringEntryDiagnosticCode::UnknownRuntimeReference,
        "runtime",
    );
    validate_named_target(
        layout.as_ref(),
        |name| table.layouts().contains_key(name),
        page,
        diagnostics,
        WorthUiAuthoringEntryDiagnosticCode::UnknownLayoutReference,
        "layout",
    );
    validate_layout_topology(page, layout.as_ref(), table, diagnostics);
    validate_named_target(
        content.as_ref(),
        |name| table.contents().contains_key(name),
        page,
        diagnostics,
        WorthUiAuthoringEntryDiagnosticCode::UnknownContentReference,
        "content",
    );
    validate_named_target(
        appearance.as_ref(),
        |name| table.appearances().contains_key(name),
        page,
        diagnostics,
        WorthUiAuthoringEntryDiagnosticCode::UnknownAppearanceReference,
        "appearance",
    );
}

fn require_page_section(
    page: &WorthUiParsedPageDeclaration,
    section_name: &str,
    section: Option<&PageSectionTarget>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    if section.is_none() {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::MissingPageSection,
            format!(
                "page '{}' requires a {section_name} section",
                page.name_text()
            ),
            page.span().clone(),
        ));
    }
}

fn validate_named_target(
    target: Option<&PageSectionTarget>,
    exists: impl Fn(&str) -> bool,
    page: &WorthUiParsedPageDeclaration,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
    code: WorthUiAuthoringEntryDiagnosticCode,
    label: &str,
) {
    let Some(PageSectionTarget::Named(name)) = target else {
        return;
    };
    if !exists(name.as_str()) {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            code,
            format!(
                "page '{}' references unknown {label} '{}'",
                page.name_text(),
                name
            ),
            page.span().clone(),
        ));
    }
}

fn extract_balanced_block(
    tokens: &[WorthUiSourceToken],
    mut index: usize,
) -> (Vec<WorthUiSourceToken>, usize) {
    let mut depth = 0usize;
    let block_start = index + 1;
    while index < tokens.len() {
        match tokens[index].kind() {
            WorthUiSourceTokenKind::LeftBrace => depth += 1,
            WorthUiSourceTokenKind::RightBrace => {
                depth -= 1;
                if depth == 0 {
                    return (tokens[block_start..index].to_vec(), index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }
    (Vec::new(), index)
}

fn validate_layout_topology(
    page: &WorthUiParsedPageDeclaration,
    target: Option<&PageSectionTarget>,
    table: &WorthUiAuthoringSymbolTable<'_>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    let report = match target {
        Some(PageSectionTarget::Named(name)) => table.layouts().get(name.as_str()).map(|layout| {
            validate_layout_topology_tokens(layout.declaration.body().tokens(), name.as_str())
        }),
        Some(PageSectionTarget::Inline(tokens)) => Some(validate_layout_topology_tokens(
            tokens.as_slice(),
            format!("{}.inline_layout", page.name_text()).as_str(),
        )),
        None => None,
    };

    let Some(Err(report)) = report else {
        return;
    };
    let message = report
        .diagnostics()
        .first()
        .map(|diagnostic| diagnostic.message().to_owned())
        .unwrap_or_else(|| "layout declaration is structurally invalid".to_owned());
    diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
        WorthUiAuthoringEntryDiagnosticCode::InvalidLayoutTopology,
        format!("page '{}' layout is invalid: {message}", page.name_text()),
        page.span().clone(),
    ));
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PageSectionTarget {
    Named(String),
    Inline(Vec<WorthUiSourceToken>),
}
