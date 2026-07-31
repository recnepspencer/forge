use super::parser::StructuralParser;
use super::{
    WorthUiAuthoredMount, WorthUiAuthoredRegion, WorthUiStructuralLanguageDiagnosticCode,
    WorthUiStructuralParseFailure,
};

struct RegionDraft {
    identity: String,
    sizing: Option<String>,
    state: Option<String>,
    children: Vec<WorthUiAuthoredRegion>,
    mounts: Vec<WorthUiAuthoredMount>,
}

struct MountDraft {
    surface: String,
    placement: Option<String>,
    state: Option<String>,
}

impl StructuralParser<'_> {
    pub(super) fn parse_region(
        &mut self,
        parent_locus: &str,
    ) -> Result<WorthUiAuthoredRegion, WorthUiStructuralParseFailure> {
        self.cursor.expect_keyword("region", parent_locus)?;
        let identity = self.cursor.expect_identifier(parent_locus)?;
        self.cursor.expect_left_brace(parent_locus)?;
        let locus = format!("{parent_locus}/region:{identity}");
        let mut draft = RegionDraft::new(identity);
        while !self.cursor.peek_right_brace() {
            self.parse_region_member(&locus, &mut draft)?;
        }
        self.cursor.expect_right_brace(parent_locus)?;
        Ok(draft.finish())
    }

    fn parse_region_member(
        &mut self,
        locus: &str,
        draft: &mut RegionDraft,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        match self.cursor.peek_identifier().as_deref() {
            Some("region") => draft.children.push(self.parse_region(locus)?),
            Some("mount") => draft.mounts.push(self.parse_mount(locus)?),
            Some("sizing") => {
                self.cursor.consume_identifier();
                let identity = self.cursor.expect_identifier(locus)?;
                self.cursor.expect_semicolon(locus)?;
                draft.admit_sizing(identity, locus)?;
            }
            Some("state") => {
                self.cursor.consume_identifier();
                let identity = self.cursor.expect_identifier(locus)?;
                self.cursor.expect_semicolon(locus)?;
                draft.admit_state(identity, locus)?;
            }
            Some(other) => return Err(invalid_member(other, locus)),
            None => return Err(self.cursor.syntax_failure(locus)),
        }
        Ok(())
    }

    fn parse_mount(
        &mut self,
        locus: &str,
    ) -> Result<WorthUiAuthoredMount, WorthUiStructuralParseFailure> {
        self.cursor.expect_keyword("mount", locus)?;
        let surface = self.cursor.expect_identifier(locus)?;
        let mut draft = MountDraft::new(surface);
        while !self.cursor.peek_semicolon() {
            self.parse_mount_member(locus, &mut draft)?;
        }
        self.cursor.expect_semicolon(locus)?;
        Ok(draft.finish())
    }

    fn parse_mount_member(
        &mut self,
        locus: &str,
        draft: &mut MountDraft,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        match self.cursor.peek_identifier().as_deref() {
            Some("placement") => {
                self.cursor.consume_identifier();
                let identity = self.cursor.expect_identifier(locus)?;
                draft.admit_placement(identity, locus)?;
            }
            Some("state") => {
                self.cursor.consume_identifier();
                let identity = self.cursor.expect_identifier(locus)?;
                draft.admit_state(identity, locus)?;
            }
            Some(other) => return Err(invalid_member(other, locus)),
            None => return Err(self.cursor.syntax_failure(locus)),
        }
        Ok(())
    }
}

impl RegionDraft {
    fn new(identity: String) -> Self {
        Self {
            identity,
            sizing: None,
            state: None,
            children: Vec::new(),
            mounts: Vec::new(),
        }
    }

    fn admit_sizing(
        &mut self,
        identity: String,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        admit_unique(
            &mut self.sizing,
            identity,
            WorthUiStructuralLanguageDiagnosticCode::DuplicateRegionSizingDeclaration,
            locus,
        )
    }

    fn admit_state(
        &mut self,
        identity: String,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        admit_unique(
            &mut self.state,
            identity,
            WorthUiStructuralLanguageDiagnosticCode::DuplicateRegionStateDeclaration,
            locus,
        )
    }

    fn finish(self) -> WorthUiAuthoredRegion {
        WorthUiAuthoredRegion::new(
            self.identity,
            self.sizing,
            self.state,
            self.children,
            self.mounts,
        )
    }
}

impl MountDraft {
    fn new(surface: String) -> Self {
        Self {
            surface,
            placement: None,
            state: None,
        }
    }

    fn admit_placement(
        &mut self,
        identity: String,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        admit_unique(
            &mut self.placement,
            identity,
            WorthUiStructuralLanguageDiagnosticCode::DuplicateMountPlacementDeclaration,
            &format!("{locus}/mount:{}", self.surface),
        )
    }

    fn admit_state(
        &mut self,
        identity: String,
        locus: &str,
    ) -> Result<(), WorthUiStructuralParseFailure> {
        admit_unique(
            &mut self.state,
            identity,
            WorthUiStructuralLanguageDiagnosticCode::DuplicateMountStateDeclaration,
            &format!("{locus}/mount:{}", self.surface),
        )
    }

    fn finish(self) -> WorthUiAuthoredMount {
        WorthUiAuthoredMount::new(self.surface, self.placement, self.state)
    }
}

fn admit_unique(
    slot: &mut Option<String>,
    identity: String,
    code: WorthUiStructuralLanguageDiagnosticCode,
    locus: &str,
) -> Result<(), WorthUiStructuralParseFailure> {
    if slot.replace(identity.clone()).is_some() {
        Err(WorthUiStructuralParseFailure {
            code,
            authored_text: identity,
            structural_locus: locus.to_owned(),
        })
    } else {
        Ok(())
    }
}

fn invalid_member(authored: &str, locus: &str) -> WorthUiStructuralParseFailure {
    WorthUiStructuralParseFailure {
        code: WorthUiStructuralLanguageDiagnosticCode::InvalidStructuralSyntax,
        authored_text: authored.to_owned(),
        structural_locus: locus.to_owned(),
    }
}
