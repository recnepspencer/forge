use crate::source::{
    WorthUiArtifactInputBlockNode, WorthUiArtifactInputBodyAtom, WorthUiArtifactInputImportNode,
    WorthUiArtifactInputNode, WorthUiArtifactInputProvenance, WorthUiArtifactInputReference,
    WorthUiArtifactInputSemanticArtifactNode, WorthUiArtifactInputTokenNode,
    WorthUiDslCompileDiagnostic, WorthUiDslCompileDiagnosticCode, WorthUiDslCompileStopClass,
    WorthUiDslSourceSpan, WorthUiParsedBlockBody, WorthUiParsedSourceDeclaration,
    WorthUiSourceTokenKind,
};

pub(crate) fn lower_parsed_source_declaration(
    declaration: &WorthUiParsedSourceDeclaration,
    declaration_index: usize,
) -> Result<WorthUiArtifactInputNode, WorthUiDslCompileDiagnostic> {
    Ok(match declaration {
        WorthUiParsedSourceDeclaration::Import(import_declaration) => {
            WorthUiArtifactInputNode::Import(lower_import_declaration(
                import_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::Component(block_declaration) => {
            WorthUiArtifactInputNode::Component(lower_parsed_block_declaration(
                block_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::Control(block_declaration) => {
            WorthUiArtifactInputNode::Component(lower_parsed_block_declaration(
                block_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::Intent(block_declaration) => {
            lower_intent_declaration(block_declaration, declaration_index)?
        }
        WorthUiParsedSourceDeclaration::Surface(block_declaration) => {
            WorthUiArtifactInputNode::Surface(lower_parsed_block_declaration(
                block_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::Binding(block_declaration) => {
            WorthUiArtifactInputNode::Binding(lower_parsed_block_declaration(
                block_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::QueryScalar(block_declaration) => {
            WorthUiArtifactInputNode::QueryScalar(lower_parsed_block_declaration(
                block_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::QueryCollection(block_declaration) => {
            WorthUiArtifactInputNode::QueryCollection(lower_parsed_block_declaration(
                block_declaration,
                declaration_index,
            ))
        }
        WorthUiParsedSourceDeclaration::Token(token_declaration) => {
            WorthUiArtifactInputNode::Token(lower_token_declaration(
                token_declaration,
                declaration_index,
            ))
        }
    })
}

fn lower_import_declaration(
    declaration: &crate::source::WorthUiParsedImportDeclaration,
    declaration_index: usize,
) -> WorthUiArtifactInputImportNode {
    WorthUiArtifactInputImportNode::new(
        WorthUiArtifactInputReference::new(declaration.path_text()),
        WorthUiArtifactInputProvenance::parsed_source(
            declaration.span().clone(),
            None,
            declaration_index,
        ),
    )
}

fn lower_intent_declaration(
    declaration: &crate::source::WorthUiParsedBlockDeclaration,
    declaration_index: usize,
) -> Result<WorthUiArtifactInputNode, WorthUiDslCompileDiagnostic> {
    let provenance = WorthUiArtifactInputProvenance::parsed_source(
        declaration.span().clone(),
        None,
        declaration_index,
    );
    let body = lower_parsed_block_body(declaration.body());
    let semantic =
        crate::WorthUiIntentDeclarationSpec::parse_file_authored(declaration.name_text(), &body)
            .map_err(|error| intent_diagnostic(declaration, error))?
            .into_semantic_declaration();
    Ok(WorthUiArtifactInputNode::SemanticArtifact(
        WorthUiArtifactInputSemanticArtifactNode::new(semantic, provenance),
    ))
}

fn lower_token_declaration(
    declaration: &crate::source::WorthUiParsedTokenDeclaration,
    declaration_index: usize,
) -> WorthUiArtifactInputTokenNode {
    WorthUiArtifactInputTokenNode::new(
        declaration.name_text(),
        None,
        declaration.value_text(),
        WorthUiArtifactInputProvenance::parsed_source(
            declaration.span().clone(),
            Some(declaration.value_span().clone()),
            declaration_index,
        ),
    )
}

fn intent_diagnostic(
    declaration: &crate::source::WorthUiParsedBlockDeclaration,
    error: crate::WorthUiIntentDeclarationParseError,
) -> WorthUiDslCompileDiagnostic {
    let span = declaration.span();
    WorthUiDslCompileDiagnostic::new(
        WorthUiDslCompileDiagnosticCode::InvalidIntentDeclaration,
        WorthUiDslCompileStopClass::LanguageLegality,
        error.detail(),
        Some(span.module_id().as_str().to_owned()),
        Some(WorthUiDslSourceSpan::new(
            span.module_id().as_str(),
            span.start_byte(),
            span.end_byte(),
        )),
    )
}

fn lower_parsed_block_declaration(
    block_declaration: &crate::source::WorthUiParsedBlockDeclaration,
    declaration_index: usize,
) -> WorthUiArtifactInputBlockNode {
    WorthUiArtifactInputBlockNode::new(
        block_declaration.name_text(),
        None,
        lower_parsed_block_body(block_declaration.body()),
        WorthUiArtifactInputProvenance::parsed_source(
            block_declaration.span().clone(),
            None,
            declaration_index,
        ),
    )
}

fn lower_parsed_block_body(body: &WorthUiParsedBlockBody) -> Vec<WorthUiArtifactInputBodyAtom> {
    body.tokens()
        .iter()
        .map(lower_token_kind_to_body_atom)
        .collect()
}

fn lower_token_kind_to_body_atom(
    token_kind: &WorthUiSourceTokenKind,
) -> WorthUiArtifactInputBodyAtom {
    match token_kind {
        WorthUiSourceTokenKind::Identifier(text) => {
            WorthUiArtifactInputBodyAtom::Identifier(text.clone())
        }
        WorthUiSourceTokenKind::StringLiteral(text) => {
            WorthUiArtifactInputBodyAtom::StringLiteral(text.clone())
        }
        WorthUiSourceTokenKind::KeywordImport => WorthUiArtifactInputBodyAtom::KeywordImport,
        WorthUiSourceTokenKind::KeywordComponent => WorthUiArtifactInputBodyAtom::KeywordComponent,
        WorthUiSourceTokenKind::KeywordControl => WorthUiArtifactInputBodyAtom::KeywordControl,
        WorthUiSourceTokenKind::KeywordIntent => WorthUiArtifactInputBodyAtom::KeywordIntent,
        WorthUiSourceTokenKind::KeywordSurface => WorthUiArtifactInputBodyAtom::KeywordSurface,
        WorthUiSourceTokenKind::KeywordBinding => WorthUiArtifactInputBodyAtom::KeywordBinding,
        WorthUiSourceTokenKind::KeywordQueryScalar => {
            WorthUiArtifactInputBodyAtom::KeywordQueryScalar
        }
        WorthUiSourceTokenKind::KeywordQueryCollection => {
            WorthUiArtifactInputBodyAtom::KeywordQueryCollection
        }
        WorthUiSourceTokenKind::KeywordToken => WorthUiArtifactInputBodyAtom::KeywordToken,
        WorthUiSourceTokenKind::LeftBrace => WorthUiArtifactInputBodyAtom::LeftBrace,
        WorthUiSourceTokenKind::RightBrace => WorthUiArtifactInputBodyAtom::RightBrace,
        WorthUiSourceTokenKind::Semicolon => WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiSourceTokenKind::Equals => WorthUiArtifactInputBodyAtom::Equals,
    }
}
