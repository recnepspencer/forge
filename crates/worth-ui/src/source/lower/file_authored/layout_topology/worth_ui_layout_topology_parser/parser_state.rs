use crate::source::{
    WorthUiLayoutTopologyDiagnostic, WorthUiLayoutTopologyDiagnosticCode, WorthUiSourceToken,
    WorthUiSourceTokenKind,
};

pub(super) struct WorthUiLayoutTopologyParser<'a> {
    pub(super) tokens: &'a [WorthUiSourceToken],
    pub(super) index: usize,
    pub(super) layout_locus: &'a str,
    diagnostics: Vec<WorthUiLayoutTopologyDiagnostic>,
}

impl<'a> WorthUiLayoutTopologyParser<'a> {
    pub(super) fn new(tokens: &'a [WorthUiSourceToken], layout_locus: &'a str) -> Self {
        Self {
            tokens,
            index: 0,
            layout_locus,
            diagnostics: Vec::new(),
        }
    }

    pub(super) fn has_trailing_tokens(&self) -> bool {
        self.index < self.tokens.len()
    }

    pub(super) fn into_diagnostics(self) -> Vec<WorthUiLayoutTopologyDiagnostic> {
        self.diagnostics
    }

    pub(super) fn push(
        &mut self,
        code: WorthUiLayoutTopologyDiagnosticCode,
        message: impl Into<String>,
    ) {
        self.diagnostics.push(WorthUiLayoutTopologyDiagnostic::new(
            code,
            self.layout_locus,
            message,
        ));
    }

    pub(super) fn push_diagnostic(&mut self, diagnostic: WorthUiLayoutTopologyDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub(super) fn peek_identifier(&self) -> Option<&str> {
        match self.tokens.get(self.index).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::Identifier(text)) => Some(text.as_str()),
            _ => None,
        }
    }

    pub(super) fn advance_identifier(&mut self) -> Option<String> {
        let identifier = self.peek_identifier()?.to_owned();
        self.index += 1;
        Some(identifier)
    }

    pub(super) fn advance_number(&mut self) -> Option<u32> {
        match self.tokens.get(self.index).map(WorthUiSourceToken::kind) {
            Some(WorthUiSourceTokenKind::NumberLiteral(value)) => {
                self.index += 1;
                Some(*value)
            }
            _ => {
                self.push(
                    WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                    "expected a numeric literal",
                );
                None
            }
        }
    }

    pub(super) fn expect_named_field(&mut self, field_name: &str) {
        if self.advance_identifier().as_deref() != Some(field_name) {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                format!("clamp sizing requires a '{field_name}:' field"),
            );
        }
        if !matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::Colon)
        ) {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                format!("clamp sizing requires a ':' after '{field_name}'"),
            );
        } else {
            self.index += 1;
        }
    }

    pub(super) fn expect_left_paren(&mut self) {
        if self.is_left_paren() {
            self.index += 1;
        } else {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                "sizing expression requires '('",
            );
        }
    }

    pub(super) fn expect_right_paren(&mut self) {
        if self.is_right_paren() {
            self.index += 1;
        } else {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                "sizing expression requires ')'",
            );
        }
    }

    pub(super) fn expect_comma(&mut self) {
        if matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::Comma)
        ) {
            self.index += 1;
        } else {
            self.push(
                WorthUiLayoutTopologyDiagnosticCode::InvalidLayoutSizing,
                "sizing expression requires ','",
            );
        }
    }

    pub(super) fn expect_left_brace(
        &mut self,
        code: WorthUiLayoutTopologyDiagnosticCode,
        message: &str,
    ) {
        if self.is_at_left_brace() {
            self.index += 1;
        } else {
            self.push(code, message);
        }
    }

    pub(super) fn expect_right_brace(
        &mut self,
        code: WorthUiLayoutTopologyDiagnosticCode,
        message: &str,
    ) {
        if self.is_at_right_brace() {
            self.index += 1;
        } else {
            self.push(code, message);
        }
    }

    pub(super) fn is_at_left_brace(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::LeftBrace)
        )
    }

    pub(super) fn is_at_right_brace(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::RightBrace)
        )
    }

    pub(super) fn is_left_paren(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::LeftParen)
        )
    }

    pub(super) fn is_right_paren(&self) -> bool {
        matches!(
            self.tokens.get(self.index).map(WorthUiSourceToken::kind),
            Some(WorthUiSourceTokenKind::RightParen)
        )
    }

    pub(super) fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }
}
