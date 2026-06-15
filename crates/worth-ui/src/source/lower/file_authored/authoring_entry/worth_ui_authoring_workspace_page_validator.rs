use std::collections::BTreeSet;

use crate::source::{
    WorthUiParsedPageDeclaration, WorthUiParsedTemplateParameter, WorthUiSourceSpan,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

use super::{
    worth_ui_authoring_symbol_table::WorthUiAuthoringSymbolTable, WorthUiAuthoringEntryDiagnostic,
    WorthUiAuthoringEntryDiagnosticCode,
};

pub(crate) fn validate_workspace_page_ownership(
    table: &WorthUiAuthoringSymbolTable<'_>,
    workspace_span: &WorthUiSourceSpan,
    workspace_name: &str,
    tokens: &[WorthUiSourceToken],
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> BTreeSet<String> {
    let mut referenced_pages = BTreeSet::new();
    let mut index = 0usize;

    while index < tokens.len() {
        let Some(list_kind) = WorkspacePageListKind::from_token_kind(tokens[index].kind()) else {
            index += 1;
            continue;
        };
        index = validate_workspace_page_list(
            table,
            workspace_span,
            workspace_name,
            tokens,
            index + 1,
            list_kind,
            &mut referenced_pages,
            diagnostics,
        );
    }

    referenced_pages
}

fn validate_workspace_page_list(
    table: &WorthUiAuthoringSymbolTable<'_>,
    workspace_span: &WorthUiSourceSpan,
    workspace_name: &str,
    tokens: &[WorthUiSourceToken],
    mut index: usize,
    list_kind: WorkspacePageListKind,
    referenced_pages: &mut BTreeSet<String>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> usize {
    if !matches!(
        tokens.get(index).map(WorthUiSourceToken::kind),
        Some(WorthUiSourceTokenKind::LeftBracket)
    ) {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::UnknownPageReference,
            format!(
                "workspace '{workspace_name}' {} declaration requires '['",
                list_kind.label()
            ),
            workspace_span.clone(),
        ));
        return tokens.len();
    }
    index += 1;

    while index < tokens.len() {
        match tokens[index].kind() {
            WorthUiSourceTokenKind::RightBracket => return index + 1,
            WorthUiSourceTokenKind::Comma => index += 1,
            WorthUiSourceTokenKind::Identifier(page_name) => {
                index = validate_workspace_page_reference(
                    table,
                    tokens,
                    index,
                    list_kind,
                    page_name.clone(),
                    referenced_pages,
                    diagnostics,
                );
            }
            _ => {
                diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
                    WorthUiAuthoringEntryDiagnosticCode::UnknownPageReference,
                    format!(
                        "workspace '{}' {} list requires named page references",
                        workspace_name,
                        list_kind.label()
                    ),
                    tokens[index].span().clone(),
                ));
                return tokens.len();
            }
        }
    }

    diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
        WorthUiAuthoringEntryDiagnosticCode::UnknownPageReference,
        format!(
            "workspace '{}' {} declaration requires a closing ']'",
            workspace_name,
            list_kind.label()
        ),
        workspace_span.clone(),
    ));
    tokens.len()
}

fn validate_workspace_page_reference(
    table: &WorthUiAuthoringSymbolTable<'_>,
    tokens: &[WorthUiSourceToken],
    identifier_index: usize,
    list_kind: WorkspacePageListKind,
    page_name: String,
    referenced_pages: &mut BTreeSet<String>,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> usize {
    referenced_pages.insert(page_name.clone());
    let signature = parse_page_reference_signature(tokens, identifier_index + 1, diagnostics);
    let next_index = signature.next_index;

    let Some(page_declaration) = table
        .pages()
        .get(page_name.as_str())
        .map(|page| page.declaration)
    else {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::UnknownPageReference,
            format!("workspace references unknown page '{page_name}'"),
            tokens[identifier_index].span().clone(),
        ));
        return next_index;
    };

    match list_kind {
        WorkspacePageListKind::Pages => validate_static_page_reference(
            page_declaration,
            &page_name,
            signature.signature,
            &tokens[identifier_index],
            diagnostics,
        ),
        WorkspacePageListKind::DynamicPages => validate_dynamic_page_reference(
            page_declaration,
            &page_name,
            signature,
            &tokens[identifier_index],
            diagnostics,
        ),
    }

    next_index
}

fn validate_static_page_reference(
    page_declaration: &WorthUiParsedPageDeclaration,
    page_name: &str,
    signature: Option<Vec<PageReferenceParameter>>,
    identifier_token: &WorthUiSourceToken,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    if signature.is_some() {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::StaticPageCannotDeclareSignature,
            format!("static pages list cannot declare a typed signature for '{page_name}'"),
            identifier_token.span().clone(),
        ));
    }

    if !page_declaration.template_parameters().is_empty() {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::StaticPageReferencesTemplate,
            format!("static pages list cannot reference template page '{page_name}'"),
            identifier_token.span().clone(),
        ));
    }
}

fn validate_dynamic_page_reference(
    page_declaration: &WorthUiParsedPageDeclaration,
    page_name: &str,
    signature_result: PageReferenceSignature,
    identifier_token: &WorthUiSourceToken,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) {
    let Some(signature) = signature_result.signature else {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::DynamicPageRequiresSignature,
            format!("dynamic pages list must declare a typed signature for '{page_name}'"),
            identifier_token.span().clone(),
        ));
        return;
    };

    if !signature_result.is_valid {
        return;
    }

    if !template_signature_matches(page_declaration.template_parameters(), &signature) {
        diagnostics.push(WorthUiAuthoringEntryDiagnostic::new(
            WorthUiAuthoringEntryDiagnosticCode::DynamicPageSignatureMismatch,
            format!(
                "dynamic page signature for '{page_name}' must match the declared page template parameters"
            ),
            identifier_token.span().clone(),
        ));
    }
}

fn template_signature_matches(
    template_parameters: &[WorthUiParsedTemplateParameter],
    signature: &[PageReferenceParameter],
) -> bool {
    template_parameters.len() == signature.len()
        && template_parameters.iter().zip(signature.iter()).all(
            |(template_parameter, signature_parameter)| {
                template_parameter.name_text() == signature_parameter.name_text
                    && template_parameter.type_text() == signature_parameter.type_text
            },
        )
}

fn parse_page_reference_signature(
    tokens: &[WorthUiSourceToken],
    start_index: usize,
    diagnostics: &mut Vec<WorthUiAuthoringEntryDiagnostic>,
) -> PageReferenceSignature {
    if !matches!(
        tokens.get(start_index).map(WorthUiSourceToken::kind),
        Some(WorthUiSourceTokenKind::LeftParen)
    ) {
        return PageReferenceSignature::without_signature(start_index);
    }

    let mut parameters = Vec::new();
    let mut index = start_index + 1;

    while index < tokens.len() {
        match tokens[index].kind() {
            WorthUiSourceTokenKind::RightParen => {
                return PageReferenceSignature::with_signature(parameters, index + 1);
            }
            WorthUiSourceTokenKind::Comma => index += 1,
            WorthUiSourceTokenKind::Identifier(parameter_name) => {
                if !matches!(
                    tokens.get(index + 1).map(WorthUiSourceToken::kind),
                    Some(WorthUiSourceTokenKind::Colon)
                ) {
                    diagnostics.push(invalid_dynamic_page_signature(tokens[index].span().clone()));
                    return PageReferenceSignature::invalid(parameters, index + 1);
                }
                let Some(WorthUiSourceTokenKind::Identifier(parameter_type)) =
                    tokens.get(index + 2).map(WorthUiSourceToken::kind)
                else {
                    diagnostics.push(invalid_dynamic_page_signature(tokens[index].span().clone()));
                    return PageReferenceSignature::invalid(parameters, index + 2);
                };
                parameters.push(PageReferenceParameter {
                    name_text: parameter_name.clone(),
                    type_text: parameter_type.clone(),
                });
                index += 3;
            }
            _ => {
                diagnostics.push(invalid_dynamic_page_signature(tokens[index].span().clone()));
                return PageReferenceSignature::invalid(parameters, index + 1);
            }
        }
    }

    diagnostics.push(invalid_dynamic_page_signature(
        tokens[start_index].span().clone(),
    ));
    PageReferenceSignature::invalid(parameters, tokens.len())
}

fn invalid_dynamic_page_signature(span: WorthUiSourceSpan) -> WorthUiAuthoringEntryDiagnostic {
    WorthUiAuthoringEntryDiagnostic::new(
        WorthUiAuthoringEntryDiagnosticCode::InvalidDynamicPageSignature,
        "dynamic page references must use '(parameter_name: ParameterType, ...)' authoring",
        span,
    )
}

#[derive(Clone, Copy)]
enum WorkspacePageListKind {
    Pages,
    DynamicPages,
}

impl WorkspacePageListKind {
    fn from_token_kind(token_kind: &WorthUiSourceTokenKind) -> Option<Self> {
        match token_kind {
            WorthUiSourceTokenKind::Identifier(text) if text == "pages" => Some(Self::Pages),
            WorthUiSourceTokenKind::Identifier(text) if text == "dynamic_pages" => {
                Some(Self::DynamicPages)
            }
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Pages => "pages",
            Self::DynamicPages => "dynamic_pages",
        }
    }
}

struct PageReferenceSignature {
    signature: Option<Vec<PageReferenceParameter>>,
    next_index: usize,
    is_valid: bool,
}

impl PageReferenceSignature {
    fn without_signature(next_index: usize) -> Self {
        Self {
            signature: None,
            next_index,
            is_valid: true,
        }
    }

    fn with_signature(signature: Vec<PageReferenceParameter>, next_index: usize) -> Self {
        Self {
            signature: Some(signature),
            next_index,
            is_valid: true,
        }
    }

    fn invalid(signature: Vec<PageReferenceParameter>, next_index: usize) -> Self {
        Self {
            signature: Some(signature),
            next_index,
            is_valid: false,
        }
    }
}

struct PageReferenceParameter {
    name_text: String,
    type_text: String,
}
