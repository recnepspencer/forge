use super::worth_ui_source_token_stream::WorthUiSourceTokenStream;

use crate::source::{
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiSourceModuleId, WorthUiSourceSpan,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

pub(super) fn parse_block_body_tokens(
    module_id: &WorthUiSourceModuleId,
    stream: &mut WorthUiSourceTokenStream,
    left_brace_span: &WorthUiSourceSpan,
) -> Result<(Vec<WorthUiSourceToken>, WorthUiSourceToken), WorthUiParseDiagnostic> {
    let mut depth = 1usize;
    let mut body_tokens = Vec::new();

    while let Some(token) = stream.next() {
        match token.kind() {
            WorthUiSourceTokenKind::LeftBrace => {
                depth += 1;
                body_tokens.push(token);
            }
            WorthUiSourceTokenKind::RightBrace => {
                depth -= 1;
                if depth == 0 {
                    return Ok((body_tokens, token));
                }
                body_tokens.push(token);
            }
            _ => body_tokens.push(token),
        }
    }

    Err(WorthUiParseDiagnostic::new(
        WorthUiParseDiagnosticCode::UnterminatedBlock,
        "block declaration reached end of module without a closing '}'",
        WorthUiSourceSpan::new(
            module_id.clone(),
            left_brace_span.start_byte(),
            left_brace_span.end_byte(),
        ),
    ))
}

pub(super) fn recover_module_root(stream: &mut WorthUiSourceTokenStream) {
    while let Some(token) = stream.peek() {
        if matches!(token.kind(), WorthUiSourceTokenKind::Semicolon) {
            let _ = stream.next();
            break;
        }
        if matches!(
            token.kind(),
            WorthUiSourceTokenKind::KeywordImport
                | WorthUiSourceTokenKind::KeywordApp
                | WorthUiSourceTokenKind::KeywordWorkspace
                | WorthUiSourceTokenKind::KeywordPage
                | WorthUiSourceTokenKind::KeywordRuntime
                | WorthUiSourceTokenKind::KeywordLayout
                | WorthUiSourceTokenKind::KeywordContent
                | WorthUiSourceTokenKind::KeywordAppearance
                | WorthUiSourceTokenKind::KeywordComponent
                | WorthUiSourceTokenKind::KeywordSurface
                | WorthUiSourceTokenKind::KeywordBinding
                | WorthUiSourceTokenKind::KeywordToken
        ) {
            break;
        }
        let _ = stream.next();
    }
}
