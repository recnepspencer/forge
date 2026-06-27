use crate::source::WorthUiArtifactInputBodyAtom;

use super::worth_ui_structural_legality_diagnostic::WorthUiStructuralLegalityDiagnosticCode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiAuthoredStructuralBody {
    root_regions: Vec<WorthUiAuthoredRegion>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiAuthoredRegion {
    pub(crate) region_id_text: String,
    pub(crate) sizing_contract_id_text: Option<String>,
    pub(crate) state_slot_id_text: Option<String>,
    pub(crate) child_regions: Vec<WorthUiAuthoredRegion>,
    pub(crate) mounts: Vec<WorthUiAuthoredMount>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiAuthoredMount {
    pub(crate) surface_id_text: String,
    pub(crate) placement_policy_id_text: Option<String>,
    pub(crate) state_slot_id_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiStructuralParseFailure {
    pub(crate) code: WorthUiStructuralLegalityDiagnosticCode,
    pub(crate) authored_text: String,
    pub(crate) structural_locus: String,
}

pub(crate) struct WorthUiStructuralBodyParser;

impl WorthUiStructuralBodyParser {
    pub(crate) fn parse(
        body_atoms: &[WorthUiArtifactInputBodyAtom],
    ) -> Result<WorthUiAuthoredStructuralBody, WorthUiStructuralParseFailure> {
        let mut parser = StructuralParser::new(body_atoms);
        let root_regions = parser.parse_root_regions()?;
        if !parser.is_eof() {
            return Err(parser.syntax_failure("trailing_tokens"));
        }
        Ok(WorthUiAuthoredStructuralBody { root_regions })
    }
}

impl WorthUiAuthoredStructuralBody {
    pub(crate) fn root_regions(&self) -> &[WorthUiAuthoredRegion] {
        &self.root_regions
    }
}

struct StructuralParser<'a> {
    atoms: &'a [WorthUiArtifactInputBodyAtom],
    cursor: usize,
}

impl<'a> StructuralParser<'a> {
    fn new(atoms: &'a [WorthUiArtifactInputBodyAtom]) -> Self {
        Self { atoms, cursor: 0 }
    }

    fn parse_root_regions(
        &mut self,
    ) -> Result<Vec<WorthUiAuthoredRegion>, WorthUiStructuralParseFailure> {
        let mut root_regions = Vec::new();
        while !self.is_eof() {
            if !matches!(self.peek_identifier().as_deref(), Some("region")) {
                return Err(WorthUiStructuralParseFailure {
                    code: WorthUiStructuralLegalityDiagnosticCode::IllegalRootStructuralStatement,
                    authored_text: self
                        .peek_identifier()
                        .unwrap_or_else(|| "unexpected_eof".to_owned()),
                    structural_locus: "root".to_owned(),
                });
            }
            root_regions.push(self.parse_region("root")?);
        }
        Ok(root_regions)
    }

    fn parse_region(
        &mut self,
        locus: &str,
    ) -> Result<WorthUiAuthoredRegion, WorthUiStructuralParseFailure> {
        self.expect_keyword("region", locus)?;
        let region_id_text = self.expect_identifier(locus)?;
        self.expect_left_brace(locus)?;

        let mut sizing_contract_id_text = None;
        let mut state_slot_id_text = None;
        let mut child_regions = Vec::new();
        let mut mounts = Vec::new();

        while !self.peek_right_brace() {
            match self.peek_identifier().as_deref() {
                Some("region") => {
                    let child_locus = format!("{locus}/region:{region_id_text}");
                    child_regions.push(self.parse_region(&child_locus)?);
                }
                Some("mount") => {
                    let mount_locus = format!("{locus}/region:{region_id_text}");
                    mounts.push(self.parse_mount(&mount_locus)?);
                }
                Some("sizing") => {
                    self.consume_identifier();
                    let sizing_id = self.expect_identifier(locus)?;
                    self.expect_semicolon(locus)?;
                    if sizing_contract_id_text.replace(sizing_id.clone()).is_some() {
                        return Err(WorthUiStructuralParseFailure {
                            code: WorthUiStructuralLegalityDiagnosticCode::DuplicateRegionSizingDeclaration,
                            authored_text: sizing_id,
                            structural_locus: format!("{locus}/region:{region_id_text}"),
                        });
                    }
                }
                Some("state") => {
                    self.consume_identifier();
                    let state_id = self.expect_identifier(locus)?;
                    self.expect_semicolon(locus)?;
                    if state_slot_id_text.replace(state_id.clone()).is_some() {
                        return Err(WorthUiStructuralParseFailure {
                            code: WorthUiStructuralLegalityDiagnosticCode::DuplicateRegionStateDeclaration,
                            authored_text: state_id,
                            structural_locus: format!("{locus}/region:{region_id_text}"),
                        });
                    }
                }
                Some(other) => {
                    return Err(WorthUiStructuralParseFailure {
                        code: WorthUiStructuralLegalityDiagnosticCode::InvalidStructuralSyntax,
                        authored_text: other.to_owned(),
                        structural_locus: locus.to_owned(),
                    });
                }
                None => return Err(self.syntax_failure(locus)),
            }
        }

        self.expect_right_brace(locus)?;
        Ok(WorthUiAuthoredRegion {
            region_id_text,
            sizing_contract_id_text,
            state_slot_id_text,
            child_regions,
            mounts,
        })
    }

    fn parse_mount(
        &mut self,
        locus: &str,
    ) -> Result<WorthUiAuthoredMount, WorthUiStructuralParseFailure> {
        self.expect_keyword("mount", locus)?;
        let surface_id_text = self.expect_identifier(locus)?;
        let mut placement_policy_id_text = None;
        let mut state_slot_id_text = None;

        while !self.peek_semicolon() {
            match self.peek_identifier().as_deref() {
                Some("placement") => {
                    self.consume_identifier();
                    let placement_id = self.expect_identifier(locus)?;
                    if placement_policy_id_text
                        .replace(placement_id.clone())
                        .is_some()
                    {
                        return Err(WorthUiStructuralParseFailure {
                            code: WorthUiStructuralLegalityDiagnosticCode::DuplicateMountPlacementDeclaration,
                            authored_text: placement_id,
                            structural_locus: format!("{locus}/mount:{surface_id_text}"),
                        });
                    }
                }
                Some("state") => {
                    self.consume_identifier();
                    let state_id = self.expect_identifier(locus)?;
                    if state_slot_id_text.replace(state_id.clone()).is_some() {
                        return Err(WorthUiStructuralParseFailure {
                            code: WorthUiStructuralLegalityDiagnosticCode::DuplicateMountStateDeclaration,
                            authored_text: state_id,
                            structural_locus: format!("{locus}/mount:{surface_id_text}"),
                        });
                    }
                }
                Some(other) => {
                    return Err(WorthUiStructuralParseFailure {
                        code: WorthUiStructuralLegalityDiagnosticCode::InvalidStructuralSyntax,
                        authored_text: other.to_owned(),
                        structural_locus: locus.to_owned(),
                    });
                }
                None => return Err(self.syntax_failure(locus)),
            }
        }

        self.expect_semicolon(locus)?;
        Ok(WorthUiAuthoredMount {
            surface_id_text,
            placement_policy_id_text,
            state_slot_id_text,
        })
    }

    fn expect_keyword(
        &mut self,
        expected: &str,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        match self.consume_identifier().as_deref() {
            Some(actual) if actual == expected => Ok(()),
            Some(actual) => Err(WorthUiStructuralParseFailure {
                code: WorthUiStructuralLegalityDiagnosticCode::InvalidStructuralSyntax,
                authored_text: actual.to_owned(),
                structural_locus: locus.to_owned(),
            }),
            None => Err(self.syntax_failure(locus)),
        }
    }

    fn expect_identifier(&mut self, locus: &str) -> Result<String, WorthUiStructuralParseFailure> {
        self.consume_identifier()
            .ok_or_else(|| self.syntax_failure(locus))
    }

    fn expect_left_brace(&mut self, locus: &str) -> Result<(), WorthUiStructuralParseFailure> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::LeftBrace) => {
                self.cursor += 1;
                Ok(())
            }
            _ => Err(self.syntax_failure(locus)),
        }
    }

    fn expect_right_brace(&mut self, locus: &str) -> Result<(), WorthUiStructuralParseFailure> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::RightBrace) => {
                self.cursor += 1;
                Ok(())
            }
            _ => Err(self.syntax_failure(locus)),
        }
    }

    fn expect_semicolon(&mut self, locus: &str) -> Result<(), WorthUiStructuralParseFailure> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::Semicolon) => {
                self.cursor += 1;
                Ok(())
            }
            _ => Err(self.syntax_failure(locus)),
        }
    }

    fn peek_identifier(&self) -> Option<String> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::Identifier(text)) => Some(text.clone()),
            _ => None,
        }
    }

    fn consume_identifier(&mut self) -> Option<String> {
        let result = self.peek_identifier();
        if result.is_some() {
            self.cursor += 1;
        }
        result
    }

    fn peek_right_brace(&self) -> bool {
        matches!(
            self.atoms.get(self.cursor),
            Some(WorthUiArtifactInputBodyAtom::RightBrace)
        )
    }

    fn peek_semicolon(&self) -> bool {
        matches!(
            self.atoms.get(self.cursor),
            Some(WorthUiArtifactInputBodyAtom::Semicolon)
        )
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.atoms.len()
    }

    fn syntax_failure(&self, locus: &str) -> WorthUiStructuralParseFailure {
        WorthUiStructuralParseFailure {
            code: WorthUiStructuralLegalityDiagnosticCode::InvalidStructuralSyntax,
            authored_text: self
                .peek_identifier()
                .unwrap_or_else(|| "unexpected_eof".to_owned()),
            structural_locus: locus.to_owned(),
        }
    }
}
