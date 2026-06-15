use super::worth_ui_source_block_parser::{parse_block_body_tokens, recover_module_root};
use super::worth_ui_source_parser_expectations::{
    expect_identifier_token, expect_punctuation_token, expect_string_literal_token,
    span_from_bounds, token_identifier_text, token_string_literal_text,
    unexpected_token_diagnostic, TokenExpectation,
};
use super::worth_ui_source_token_stream::WorthUiSourceTokenStream;

use crate::source::{
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiParsedAuthoringDeclaration,
    WorthUiParsedBlockBody, WorthUiParsedBlockDeclaration, WorthUiParsedImportDeclaration,
    WorthUiParsedPageDeclaration, WorthUiParsedSourceDeclaration, WorthUiParsedTemplateParameter,
    WorthUiParsedTokenDeclaration, WorthUiSourceModuleId, WorthUiSourceSpan, WorthUiSourceToken,
    WorthUiSourceTokenKind,
};

pub(super) fn parse_module_declarations(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    tokens: Vec<WorthUiSourceToken>,
) -> Result<Vec<WorthUiParsedSourceDeclaration>, Vec<WorthUiParseDiagnostic>> {
    let mut stream = WorthUiSourceTokenStream::new(tokens);
    let mut declarations = Vec::new();
    let mut diagnostics = Vec::new();

    while !stream.is_eof() {
        match parse_next_declaration(module_id, source_length, &mut stream) {
            Ok(declaration) => declarations.push(declaration),
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                recover_module_root(&mut stream);
            }
        }
    }

    if diagnostics.is_empty() {
        Ok(declarations)
    } else {
        Err(diagnostics)
    }
}

fn parse_next_declaration(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
) -> Result<WorthUiParsedSourceDeclaration, WorthUiParseDiagnostic> {
    match stream.peek().map(WorthUiSourceToken::kind) {
        Some(WorthUiSourceTokenKind::KeywordImport) => {
            parse_import_declaration(module_id, source_length, stream)
        }
        Some(WorthUiSourceTokenKind::KeywordApp) => {
            parse_authoring_declaration(module_id, source_length, stream, AuthoringKind::App)
        }
        Some(WorthUiSourceTokenKind::KeywordWorkspace) => {
            parse_authoring_declaration(module_id, source_length, stream, AuthoringKind::Workspace)
        }
        Some(WorthUiSourceTokenKind::KeywordPage) => {
            parse_page_declaration(module_id, source_length, stream)
        }
        Some(WorthUiSourceTokenKind::KeywordRuntime) => {
            parse_authoring_declaration(module_id, source_length, stream, AuthoringKind::Runtime)
        }
        Some(WorthUiSourceTokenKind::KeywordLayout) => {
            parse_authoring_declaration(module_id, source_length, stream, AuthoringKind::Layout)
        }
        Some(WorthUiSourceTokenKind::KeywordContent) => {
            parse_authoring_declaration(module_id, source_length, stream, AuthoringKind::Content)
        }
        Some(WorthUiSourceTokenKind::KeywordAppearance) => {
            parse_authoring_declaration(module_id, source_length, stream, AuthoringKind::Appearance)
        }
        Some(WorthUiSourceTokenKind::KeywordComponent) => {
            parse_block_declaration(module_id, source_length, stream, BlockKind::Component)
        }
        Some(WorthUiSourceTokenKind::KeywordSurface) => {
            parse_block_declaration(module_id, source_length, stream, BlockKind::Surface)
        }
        Some(WorthUiSourceTokenKind::KeywordBinding) => {
            parse_block_declaration(module_id, source_length, stream, BlockKind::Binding)
        }
        Some(WorthUiSourceTokenKind::KeywordToken) => {
            parse_token_declaration(module_id, source_length, stream)
        }
        Some(_) => Err(unexpected_token_diagnostic(
            stream.next().expect("peeked token should exist"),
            "expected a top-level declaration keyword",
        )),
        None => unreachable!("declaration parse should not run at eof"),
    }
}

fn parse_import_declaration(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
) -> Result<WorthUiParsedSourceDeclaration, WorthUiParseDiagnostic> {
    let import_keyword = stream.next().expect("import token should exist");
    let import_path_token = expect_string_literal_token(
        module_id,
        source_length,
        stream,
        "import declaration requires a quoted module path",
    )?;
    let semicolon = expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::Semicolon,
        "import declaration must terminate with ';'",
    )?;

    Ok(WorthUiParsedSourceDeclaration::Import(
        WorthUiParsedImportDeclaration::new(
            token_string_literal_text(&import_path_token),
            span_from_bounds(import_keyword.span(), semicolon.span()),
        ),
    ))
}

fn parse_block_declaration(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
    block_kind: BlockKind,
) -> Result<WorthUiParsedSourceDeclaration, WorthUiParseDiagnostic> {
    let keyword = stream.next().expect("block keyword token should exist");
    let name_token = expect_identifier_token(
        module_id,
        source_length,
        stream,
        "named block declaration requires an identifier",
    )?;
    let left_brace = expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::LeftBrace,
        "named block declaration requires '{'",
    )?;
    let (body_tokens, right_brace) = parse_block_body_tokens(module_id, stream, left_brace.span())?;

    let declaration = WorthUiParsedBlockDeclaration::new(
        token_identifier_text(&name_token),
        span_from_bounds(keyword.span(), right_brace.span()),
        WorthUiParsedBlockBody::new(
            span_from_bounds(left_brace.span(), right_brace.span()),
            body_tokens,
        ),
    );

    Ok(match block_kind {
        BlockKind::Component => WorthUiParsedSourceDeclaration::Component(declaration),
        BlockKind::Surface => WorthUiParsedSourceDeclaration::Surface(declaration),
        BlockKind::Binding => WorthUiParsedSourceDeclaration::Binding(declaration),
    })
}

fn parse_authoring_declaration(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
    authoring_kind: AuthoringKind,
) -> Result<WorthUiParsedSourceDeclaration, WorthUiParseDiagnostic> {
    let keyword = stream.next().expect("authoring keyword token should exist");
    let name_token = expect_identifier_token(
        module_id,
        source_length,
        stream,
        "named authoring declaration requires an identifier",
    )?;
    let left_brace = expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::LeftBrace,
        "named authoring declaration requires '{'",
    )?;
    let (body_tokens, right_brace) = parse_block_body_tokens(module_id, stream, left_brace.span())?;
    let declaration = WorthUiParsedAuthoringDeclaration::new(
        token_identifier_text(&name_token),
        span_from_bounds(keyword.span(), right_brace.span()),
        WorthUiParsedBlockBody::new(
            span_from_bounds(left_brace.span(), right_brace.span()),
            body_tokens,
        ),
    );

    Ok(match authoring_kind {
        AuthoringKind::App => WorthUiParsedSourceDeclaration::App(declaration),
        AuthoringKind::Workspace => WorthUiParsedSourceDeclaration::Workspace(declaration),
        AuthoringKind::Runtime => WorthUiParsedSourceDeclaration::Runtime(declaration),
        AuthoringKind::Layout => WorthUiParsedSourceDeclaration::Layout(declaration),
        AuthoringKind::Content => WorthUiParsedSourceDeclaration::Content(declaration),
        AuthoringKind::Appearance => WorthUiParsedSourceDeclaration::Appearance(declaration),
    })
}

fn parse_page_declaration(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
) -> Result<WorthUiParsedSourceDeclaration, WorthUiParseDiagnostic> {
    let keyword = stream.next().expect("page keyword token should exist");
    let name_token = expect_identifier_token(
        module_id,
        source_length,
        stream,
        "page declaration requires an identifier",
    )?;
    let template_parameters = if matches!(
        stream.peek().map(WorthUiSourceToken::kind),
        Some(WorthUiSourceTokenKind::LeftParen)
    ) {
        parse_template_parameters(module_id, source_length, stream)?
    } else {
        Vec::new()
    };
    let left_brace = expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::LeftBrace,
        "page declaration requires '{'",
    )?;
    let (body_tokens, right_brace) = parse_block_body_tokens(module_id, stream, left_brace.span())?;

    Ok(WorthUiParsedSourceDeclaration::Page(
        WorthUiParsedPageDeclaration::new(
            token_identifier_text(&name_token),
            template_parameters,
            span_from_bounds(keyword.span(), right_brace.span()),
            WorthUiParsedBlockBody::new(
                span_from_bounds(left_brace.span(), right_brace.span()),
                body_tokens,
            ),
        ),
    ))
}

fn parse_template_parameters(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
) -> Result<Vec<WorthUiParsedTemplateParameter>, WorthUiParseDiagnostic> {
    let left_paren = expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::LeftParen,
        "page template parameter list requires '('",
    )?;
    let mut parameters = Vec::new();

    loop {
        if matches!(
            stream.peek().map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::RightParen)
        ) {
            let _ = expect_punctuation_token(
                module_id,
                source_length,
                stream,
                TokenExpectation::RightParen,
                "page template parameter list requires ')'",
            )?;
            break;
        }

        let parameter_name = expect_identifier_token(
            module_id,
            source_length,
            stream,
            "page template parameter requires a parameter name",
        )?;
        let _ = expect_punctuation_token(
            module_id,
            source_length,
            stream,
            TokenExpectation::Colon,
            "page template parameter requires ':' before the type",
        )?;
        let parameter_type = expect_identifier_token(
            module_id,
            source_length,
            stream,
            "page template parameter requires a type identifier",
        )?;
        parameters.push(WorthUiParsedTemplateParameter::new(
            token_identifier_text(&parameter_name),
            token_identifier_text(&parameter_type),
            span_from_bounds(parameter_name.span(), parameter_type.span()),
        ));

        match stream.peek().map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Comma) => {
                let _ = stream.next();
            }
            Some(WorthUiSourceTokenKind::RightParen) => {
                let _ = expect_punctuation_token(
                    module_id,
                    source_length,
                    stream,
                    TokenExpectation::RightParen,
                    "page template parameter list requires ')'",
                )?;
                break;
            }
            Some(_) => {
                return Err(unexpected_token_diagnostic(
                    stream.next().expect("peeked token should exist"),
                    "page template parameter list requires ',' or ')'",
                ));
            }
            None => {
                return Err(WorthUiParseDiagnostic::new(
                    WorthUiParseDiagnosticCode::UnexpectedToken,
                    "page template parameter list reached end of module without ')'",
                    WorthUiSourceSpan::new(
                        module_id.clone(),
                        left_paren.span().start_byte(),
                        left_paren.span().end_byte(),
                    ),
                ));
            }
        }
    }

    Ok(parameters)
}

fn parse_token_declaration(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
) -> Result<WorthUiParsedSourceDeclaration, WorthUiParseDiagnostic> {
    let token_keyword = stream.next().expect("token keyword token should exist");
    let name_token = expect_identifier_token(
        module_id,
        source_length,
        stream,
        "token declaration requires an identifier",
    )?;
    expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::Equals,
        "token declaration requires '=' before its value",
    )?;
    let value_token = expect_string_literal_token(
        module_id,
        source_length,
        stream,
        "token declaration requires a quoted string value",
    )?;
    let semicolon = expect_punctuation_token(
        module_id,
        source_length,
        stream,
        TokenExpectation::Semicolon,
        "token declaration must terminate with ';'",
    )?;

    Ok(WorthUiParsedSourceDeclaration::Token(
        WorthUiParsedTokenDeclaration::new(
            token_identifier_text(&name_token),
            token_string_literal_text(&value_token),
            span_from_bounds(token_keyword.span(), semicolon.span()),
            value_token.span().clone(),
        ),
    ))
}

#[derive(Clone, Copy)]
enum BlockKind {
    Component,
    Surface,
    Binding,
}

#[derive(Clone, Copy)]
enum AuthoringKind {
    App,
    Workspace,
    Runtime,
    Layout,
    Content,
    Appearance,
}
