use crate::source::WorthUiArtifactInputBodyAtom;

use super::{WorthUiStructuralLanguageDiagnosticCode, WorthUiStructuralParseFailure};

pub(super) struct StructuralCursor<'a> {
    atoms: &'a [WorthUiArtifactInputBodyAtom],
    cursor: usize,
}

impl<'a> StructuralCursor<'a> {
    pub(super) fn new(atoms: &'a [WorthUiArtifactInputBodyAtom]) -> Self {
        Self { atoms, cursor: 0 }
    }

    pub(super) fn expect_keyword(
        &mut self,
        expected: &str,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        match self.consume_identifier().as_deref() {
            Some(actual) if actual == expected => Ok(()),
            Some(actual) => Err(WorthUiStructuralParseFailure {
                code: WorthUiStructuralLanguageDiagnosticCode::InvalidStructuralSyntax,
                authored_text: actual.to_owned(),
                structural_locus: locus.to_owned(),
            }),
            None => Err(self.syntax_failure(locus)),
        }
    }

    pub(super) fn expect_identifier(
        &mut self,
        locus: &str,
    ) -> Result<String, WorthUiStructuralParseFailure> {
        self.consume_identifier()
            .ok_or_else(|| self.syntax_failure(locus))
    }

    pub(super) fn expect_left_brace(
        &mut self,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        self.expect_atom(locus, WorthUiArtifactInputBodyAtom::LeftBrace)
    }

    pub(super) fn expect_right_brace(
        &mut self,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        self.expect_atom(locus, WorthUiArtifactInputBodyAtom::RightBrace)
    }

    pub(super) fn expect_semicolon(
        &mut self,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        self.expect_atom(locus, WorthUiArtifactInputBodyAtom::Semicolon)
    }

    pub(super) fn peek_identifier(&self) -> Option<String> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::Identifier(text)) => Some(text.clone()),
            _ => None,
        }
    }

    pub(super) fn consume_identifier(&mut self) -> Option<String> {
        let result = self.peek_identifier();
        if result.is_some() {
            self.cursor += 1;
        }
        result
    }

    pub(super) fn consume_optional_semicolon(&mut self) {
        if self.peek_semicolon() {
            self.cursor += 1;
        }
    }

    pub(super) fn peek_right_brace(&self) -> bool {
        matches!(
            self.atoms.get(self.cursor),
            Some(WorthUiArtifactInputBodyAtom::RightBrace)
        )
    }

    pub(super) fn peek_semicolon(&self) -> bool {
        matches!(
            self.atoms.get(self.cursor),
            Some(WorthUiArtifactInputBodyAtom::Semicolon)
        )
    }

    pub(super) fn is_eof(&self) -> bool {
        self.cursor >= self.atoms.len()
    }

    pub(super) fn syntax_failure(&self, locus: &str) -> WorthUiStructuralParseFailure {
        WorthUiStructuralParseFailure {
            code: WorthUiStructuralLanguageDiagnosticCode::InvalidStructuralSyntax,
            authored_text: self
                .peek_identifier()
                .unwrap_or_else(|| "unexpected_eof".to_owned()),
            structural_locus: locus.to_owned(),
        }
    }

    fn expect_atom(
        &mut self,
        locus: &str,
        expected: WorthUiArtifactInputBodyAtom,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        if self.atoms.get(self.cursor) == Some(&expected) {
            self.cursor += 1;
            Ok(())
        } else {
            Err(self.syntax_failure(locus))
        }
    }
}
