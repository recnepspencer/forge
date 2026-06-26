use crate::source::{
    WorthUiArtifactInputBlockNode, WorthUiArtifactInputBodyAtom, WorthUiArtifactInputImportNode,
    WorthUiArtifactInputNode, WorthUiArtifactInputProvenance, WorthUiArtifactInputReference,
    WorthUiArtifactInputTokenNode, WorthUiParsedBlockBody, WorthUiParsedSourceDeclaration,
    WorthUiSourceTokenKind,
};

pub(crate) fn lower_parsed_source_declaration(
    declaration: &WorthUiParsedSourceDeclaration,
) -> WorthUiArtifactInputNode {
    match declaration {
        WorthUiParsedSourceDeclaration::Import(import_declaration) => {
            WorthUiArtifactInputNode::Import(WorthUiArtifactInputImportNode::new(
                WorthUiArtifactInputReference::new(import_declaration.path_text()),
                WorthUiArtifactInputProvenance::parsed_source(
                    import_declaration.span().clone(),
                    None,
                ),
            ))
        }
        WorthUiParsedSourceDeclaration::Component(block_declaration) => {
            WorthUiArtifactInputNode::Component(lower_parsed_block_declaration(block_declaration))
        }
        WorthUiParsedSourceDeclaration::Surface(block_declaration) => {
            WorthUiArtifactInputNode::Surface(lower_parsed_block_declaration(block_declaration))
        }
        WorthUiParsedSourceDeclaration::Binding(block_declaration) => {
            WorthUiArtifactInputNode::Binding(lower_parsed_block_declaration(block_declaration))
        }
        WorthUiParsedSourceDeclaration::Token(token_declaration) => {
            WorthUiArtifactInputNode::Token(WorthUiArtifactInputTokenNode::new(
                token_declaration.name_text(),
                None,
                token_declaration.value_text(),
                WorthUiArtifactInputProvenance::parsed_source(
                    token_declaration.span().clone(),
                    Some(token_declaration.value_span().clone()),
                ),
            ))
        }
    }
}

fn lower_parsed_block_declaration(
    block_declaration: &crate::source::WorthUiParsedBlockDeclaration,
) -> WorthUiArtifactInputBlockNode {
    WorthUiArtifactInputBlockNode::new(
        block_declaration.name_text(),
        None,
        lower_parsed_block_body(block_declaration.body()),
        WorthUiArtifactInputProvenance::parsed_source(block_declaration.span().clone(), None),
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
        WorthUiSourceTokenKind::KeywordSurface => WorthUiArtifactInputBodyAtom::KeywordSurface,
        WorthUiSourceTokenKind::KeywordBinding => WorthUiArtifactInputBodyAtom::KeywordBinding,
        WorthUiSourceTokenKind::KeywordToken => WorthUiArtifactInputBodyAtom::KeywordToken,
        WorthUiSourceTokenKind::LeftBrace => WorthUiArtifactInputBodyAtom::LeftBrace,
        WorthUiSourceTokenKind::RightBrace => WorthUiArtifactInputBodyAtom::RightBrace,
        WorthUiSourceTokenKind::Semicolon => WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiSourceTokenKind::Equals => WorthUiArtifactInputBodyAtom::Equals,
    }
}
