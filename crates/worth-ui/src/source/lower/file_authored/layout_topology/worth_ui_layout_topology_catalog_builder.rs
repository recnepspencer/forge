use crate::source::{
    WorthUiAuthoringEntryReport, WorthUiLayoutTopologyCatalog, WorthUiLayoutTopologyDiagnostic,
    WorthUiLayoutTopologyDiagnosticCode, WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport,
    WorthUiPageLayoutTopology, WorthUiParsedPageDeclaration, WorthUiParsedSourceDeclaration,
    WorthUiParsedSourcePackage, WorthUiSourceToken, WorthUiSourceTokenKind,
};

use super::super::authoring_entry::WorthUiAuthoringSymbolTable;
use super::worth_ui_layout_topology_parser::parse_layout_topology;

pub(crate) fn validate_layout_topology_tokens(
    tokens: &[WorthUiSourceToken],
    layout_locus: &str,
) -> Result<WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport> {
    parse_layout_topology(tokens, layout_locus)
}

pub(crate) fn build_layout_topology_catalog(
    parsed_package: &WorthUiParsedSourcePackage,
) -> Result<WorthUiLayoutTopologyCatalog, WorthUiLayoutTopologyReport> {
    let table = WorthUiAuthoringSymbolTable::build(parsed_package).map_err(map_authoring_report)?;
    let mut pages = Vec::new();

    for module_id in parsed_package.module_ids() {
        let module = parsed_package
            .module(module_id)
            .expect("parsed package should contain every canonical module");
        for declaration in module.declarations() {
            let WorthUiParsedSourceDeclaration::Page(page) = declaration else {
                continue;
            };
            let (layout_name, layout_tokens) = page_layout_target(page, &table)?;
            let root = parse_layout_topology(&layout_tokens, layout_name.as_str())?;
            pages.push(WorthUiPageLayoutTopology::new(
                page.name_text(),
                layout_name,
                root,
                !page.template_parameters().is_empty(),
            ));
        }
    }

    Ok(WorthUiLayoutTopologyCatalog::new(pages))
}

fn page_layout_target(
    page: &WorthUiParsedPageDeclaration,
    table: &WorthUiAuthoringSymbolTable<'_>,
) -> Result<(String, Vec<WorthUiSourceToken>), WorthUiLayoutTopologyReport> {
    let tokens = page.body().tokens();
    let mut index = 0usize;

    while index < tokens.len() {
        if !matches!(tokens[index].kind(), WorthUiSourceTokenKind::KeywordLayout) {
            index += 1;
            continue;
        }

        return match tokens.get(index + 1).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Identifier(layout_name)) => table
                .layouts()
                .get(layout_name.as_str())
                .map(|layout| {
                    (
                        layout_name.clone(),
                        layout.declaration.body().tokens().to_vec(),
                    )
                })
                .ok_or_else(|| {
                    WorthUiLayoutTopologyReport::new(vec![WorthUiLayoutTopologyDiagnostic::new(
                        WorthUiLayoutTopologyDiagnosticCode::UnknownLayoutReference,
                        page.name_text(),
                        format!(
                            "page '{}' references unknown layout '{}'",
                            page.name_text(),
                            layout_name
                        ),
                    )])
                }),
            Some(WorthUiSourceTokenKind::LeftBrace) => {
                extract_inline_layout_tokens(tokens, index + 1).map(|inline_tokens| {
                    (format!("{}.inline_layout", page.name_text()), inline_tokens)
                })
            }
            _ => Err(WorthUiLayoutTopologyReport::new(vec![
                WorthUiLayoutTopologyDiagnostic::new(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidPageLayoutReference,
                    page.name_text(),
                    format!(
                        "page '{}' layout section requires a named reference or inline block",
                        page.name_text()
                    ),
                ),
            ])),
        };
    }

    Err(WorthUiLayoutTopologyReport::new(vec![
        WorthUiLayoutTopologyDiagnostic::new(
            WorthUiLayoutTopologyDiagnosticCode::InvalidPageLayoutReference,
            page.name_text(),
            format!(
                "page '{}' does not declare a layout section",
                page.name_text()
            ),
        ),
    ]))
}

fn extract_inline_layout_tokens(
    tokens: &[WorthUiSourceToken],
    left_brace_index: usize,
) -> Result<Vec<WorthUiSourceToken>, WorthUiLayoutTopologyReport> {
    let mut depth = 0usize;
    let mut index = left_brace_index;

    while index < tokens.len() {
        match tokens[index].kind() {
            WorthUiSourceTokenKind::LeftBrace => depth += 1,
            WorthUiSourceTokenKind::RightBrace => {
                depth -= 1;
                if depth == 0 {
                    return Ok(tokens[left_brace_index + 1..index].to_vec());
                }
            }
            _ => {}
        }
        index += 1;
    }

    Err(WorthUiLayoutTopologyReport::new(vec![
        WorthUiLayoutTopologyDiagnostic::new(
            WorthUiLayoutTopologyDiagnosticCode::InvalidPageLayoutReference,
            "inline-layout",
            "inline layout block requires a closing '}'",
        ),
    ]))
}

fn map_authoring_report(report: WorthUiAuthoringEntryReport) -> WorthUiLayoutTopologyReport {
    let message = report
        .diagnostics()
        .first()
        .map(|diagnostic| diagnostic.message().to_owned())
        .unwrap_or_else(|| {
            "layout topology build requires valid authoring declarations".to_owned()
        });
    WorthUiLayoutTopologyReport::new(vec![WorthUiLayoutTopologyDiagnostic::new(
        WorthUiLayoutTopologyDiagnosticCode::InvalidPageLayoutReference,
        "authoring-entry",
        message,
    )])
}
