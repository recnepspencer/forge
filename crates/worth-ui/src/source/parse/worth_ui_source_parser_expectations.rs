use super::worth_ui_source_token_stream::WorthUiSourceTokenStream;
use crate::source::{
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiSourceModuleId, WorthUiSourceSpan,
    WorthUiSourceToken, WorthUiSourceTokenKind,
};

pub(super) fn expect_identifier_token(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
    message: &str,
) -> Result<WorthUiSourceToken, WorthUiParseDiagnostic> {
    match stream.peek() {
        Some(token) if matches!(token.kind(), WorthUiSourceTokenKind::Identifier(_)) => {
            Ok(stream.next().expect("peeked token should exist"))
        }
        Some(token) => Err(WorthUiParseDiagnostic::new(
            WorthUiParseDiagnosticCode::MissingIdentifier,
            message,
            token.span().clone(),
        )),
        None => Err(unexpected_end_diagnostic(
            module_id,
            source_length,
            message,
            WorthUiParseDiagnosticCode::MissingIdentifier,
        )),
    }
}

pub(super) fn expect_string_literal_token(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
    message: &str,
) -> Result<WorthUiSourceToken, WorthUiParseDiagnostic> {
    match stream.peek() {
        Some(token) if matches!(token.kind(), WorthUiSourceTokenKind::StringLiteral(_)) => {
            Ok(stream.next().expect("peeked token should exist"))
        }
        Some(token) => Err(WorthUiParseDiagnostic::new(
            WorthUiParseDiagnosticCode::MissingStringLiteral,
            message,
            token.span().clone(),
        )),
        None => Err(unexpected_end_diagnostic(
            module_id,
            source_length,
            message,
            WorthUiParseDiagnosticCode::MissingStringLiteral,
        )),
    }
}

pub(super) fn expect_punctuation_token(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    stream: &mut WorthUiSourceTokenStream,
    expectation: TokenExpectation,
    message: &str,
) -> Result<WorthUiSourceToken, WorthUiParseDiagnostic> {
    match stream.peek() {
        Some(token) if expectation.matches(token.kind()) => {
            Ok(stream.next().expect("peeked token should exist"))
        }
        Some(token) => Err(WorthUiParseDiagnostic::new(
            expectation.missing_code(),
            message,
            token.span().clone(),
        )),
        None => Err(unexpected_end_diagnostic(
            module_id,
            source_length,
            message,
            expectation.missing_code(),
        )),
    }
}

pub(super) fn unexpected_end_diagnostic(
    module_id: &WorthUiSourceModuleId,
    source_length: usize,
    message: &str,
    code: WorthUiParseDiagnosticCode,
) -> WorthUiParseDiagnostic {
    WorthUiParseDiagnostic::new(
        code,
        message,
        WorthUiSourceSpan::new(module_id.clone(), source_length, source_length),
    )
}

pub(super) fn unexpected_token_diagnostic(
    token: WorthUiSourceToken,
    message: &str,
) -> WorthUiParseDiagnostic {
    WorthUiParseDiagnostic::new(
        WorthUiParseDiagnosticCode::UnexpectedToken,
        message,
        token.span().clone(),
    )
}

pub(super) fn span_from_bounds(
    start: &WorthUiSourceSpan,
    end: &WorthUiSourceSpan,
) -> WorthUiSourceSpan {
    WorthUiSourceSpan::new(
        start.module_id().clone(),
        start.start_byte(),
        end.end_byte(),
    )
}

pub(super) fn token_identifier_text(token: &WorthUiSourceToken) -> String {
    match token.kind() {
        WorthUiSourceTokenKind::Identifier(text) => text.clone(),
        _ => unreachable!("identifier token text requested from non-identifier token"),
    }
}

pub(super) fn token_string_literal_text(token: &WorthUiSourceToken) -> String {
    match token.kind() {
        WorthUiSourceTokenKind::StringLiteral(text) => text.clone(),
        _ => unreachable!("string literal text requested from non-string token"),
    }
}

#[derive(Clone, Copy)]
pub(super) enum TokenExpectation {
    LeftBrace,
    LeftParen,
    RightParen,
    Semicolon,
    Equals,
    Colon,
}

impl TokenExpectation {
    fn matches(self, token_kind: &WorthUiSourceTokenKind) -> bool {
        matches!(
            (self, token_kind),
            (Self::LeftBrace, WorthUiSourceTokenKind::LeftBrace)
                | (Self::LeftParen, WorthUiSourceTokenKind::LeftParen)
                | (Self::RightParen, WorthUiSourceTokenKind::RightParen)
                | (Self::Semicolon, WorthUiSourceTokenKind::Semicolon)
                | (Self::Equals, WorthUiSourceTokenKind::Equals)
                | (Self::Colon, WorthUiSourceTokenKind::Colon)
        )
    }

    fn missing_code(self) -> WorthUiParseDiagnosticCode {
        match self {
            Self::LeftBrace => WorthUiParseDiagnosticCode::MissingBlockStart,
            Self::LeftParen | Self::RightParen | Self::Colon => {
                WorthUiParseDiagnosticCode::UnexpectedToken
            }
            Self::Semicolon => WorthUiParseDiagnosticCode::MissingSemicolon,
            Self::Equals => WorthUiParseDiagnosticCode::MissingEquals,
        }
    }
}
