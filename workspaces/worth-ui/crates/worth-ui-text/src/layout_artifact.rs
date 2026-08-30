use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiPositionedTextGlyphRecord, UiQualifiedFontFaceIdentity,
    UiQualifiedFontFamilyIdentity, UiQualifiedFontPackIdentity, UiQualifiedTextCaretRecord,
    UiQualifiedTextCostRecord, UiQualifiedTextCoverageRecord, UiQualifiedTextGlyphRecord,
    UiQualifiedTextGraphemeRecord, UiQualifiedTextLayoutIdentity,
    UiQualifiedTextLayoutRequestIdentity, UiQualifiedTextLayoutView,
    UiQualifiedTextLayoutViewInput, UiQualifiedTextLineRecord, UiQualifiedTextRunRecord,
    UiQualifiedTextStyleRecord, UiQualifiedTextVisualRunRecord, UiQualifiedTextWordBoundaryRecord,
    UiTextProfileGeneration, UiTextScaleGeneration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiQualifiedTextFaceResource {
    identity: UiQualifiedFontFaceIdentity,
    family: UiQualifiedFontFamilyIdentity,
    pack: Option<UiQualifiedFontPackIdentity>,
    bytes: Arc<[u8]>,
    intrinsic_color: bool,
    color_glyphs: Box<[UiQualifiedTextColorGlyph]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiQualifiedTextColorSource {
    Outline,
    Bitmap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiQualifiedTextColorGlyph {
    glyph_id: u16,
    source: UiQualifiedTextColorSource,
}

impl UiQualifiedTextColorGlyph {
    pub(crate) const fn new(glyph_id: u16, source: UiQualifiedTextColorSource) -> Self {
        Self { glyph_id, source }
    }

    pub(crate) const fn glyph_id(self) -> u16 {
        self.glyph_id
    }

    pub(crate) const fn source(self) -> UiQualifiedTextColorSource {
        self.source
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiQualifiedTextLayoutArtifact {
    request_identity: UiQualifiedTextLayoutRequestIdentity,
    identity: UiQualifiedTextLayoutIdentity,
    source: Arc<str>,
    graphemes: Arc<[UiQualifiedTextGraphemeRecord]>,
    word_boundaries: Arc<[UiQualifiedTextWordBoundaryRecord]>,
    styles: Arc<[UiQualifiedTextStyleRecord]>,
    logical_runs: Arc<[UiQualifiedTextRunRecord]>,
    glyphs: Arc<[UiQualifiedTextGlyphRecord]>,
    lines: Arc<[UiQualifiedTextLineRecord]>,
    visual_runs: Arc<[UiQualifiedTextVisualRunRecord]>,
    positioned_glyphs: Arc<[UiPositionedTextGlyphRecord]>,
    logical_bounds: worth_ui_host_contract::UiTextRect,
    ink_bounds: worth_ui_host_contract::UiTextRect,
    carets: Arc<[UiQualifiedTextCaretRecord]>,
    coverage: Arc<[UiQualifiedTextCoverageRecord]>,
    faces: Arc<[UiQualifiedTextFaceResource]>,
    cost: UiQualifiedTextCostRecord,
    profile: UiTextProfileGeneration,
    font_collection: UiFontCollectionGeneration,
    text_scale: UiTextScaleGeneration,
    width_basis: worth_ui_host_contract::UiQualifiedTextLayoutWidthBasis,
}

pub(crate) struct UiQualifiedTextLayoutArtifactInput {
    pub request_identity: UiQualifiedTextLayoutRequestIdentity,
    pub identity: UiQualifiedTextLayoutIdentity,
    pub source: Arc<str>,
    pub graphemes: Arc<[UiQualifiedTextGraphemeRecord]>,
    pub word_boundaries: Arc<[UiQualifiedTextWordBoundaryRecord]>,
    pub styles: Arc<[UiQualifiedTextStyleRecord]>,
    pub logical_runs: Arc<[UiQualifiedTextRunRecord]>,
    pub glyphs: Arc<[UiQualifiedTextGlyphRecord]>,
    pub lines: Arc<[UiQualifiedTextLineRecord]>,
    pub visual_runs: Arc<[UiQualifiedTextVisualRunRecord]>,
    pub positioned_glyphs: Arc<[UiPositionedTextGlyphRecord]>,
    pub logical_bounds: worth_ui_host_contract::UiTextRect,
    pub ink_bounds: worth_ui_host_contract::UiTextRect,
    pub carets: Arc<[UiQualifiedTextCaretRecord]>,
    pub coverage: Arc<[UiQualifiedTextCoverageRecord]>,
    pub faces: Arc<[UiQualifiedTextFaceResource]>,
    pub cost: UiQualifiedTextCostRecord,
    pub profile: UiTextProfileGeneration,
    pub font_collection: UiFontCollectionGeneration,
    pub text_scale: UiTextScaleGeneration,
    pub width_basis: worth_ui_host_contract::UiQualifiedTextLayoutWidthBasis,
}

impl UiQualifiedTextFaceResource {
    pub(crate) fn new(
        identity: UiQualifiedFontFaceIdentity,
        family: UiQualifiedFontFamilyIdentity,
        pack: Option<UiQualifiedFontPackIdentity>,
        bytes: Arc<[u8]>,
        intrinsic_color: bool,
        color_glyphs: Box<[UiQualifiedTextColorGlyph]>,
    ) -> Self {
        Self {
            identity,
            family,
            pack,
            bytes,
            intrinsic_color,
            color_glyphs,
        }
    }

    pub(crate) const fn identity(&self) -> UiQualifiedFontFaceIdentity {
        self.identity
    }
    pub(crate) const fn family(&self) -> UiQualifiedFontFamilyIdentity {
        self.family
    }
    pub(crate) const fn pack(&self) -> Option<UiQualifiedFontPackIdentity> {
        self.pack
    }
    pub(crate) fn bytes(&self) -> &Arc<[u8]> {
        &self.bytes
    }
    pub(crate) const fn intrinsic_color(&self) -> bool {
        self.intrinsic_color
    }

    pub(crate) fn color_source(&self, glyph_id: u32) -> Option<UiQualifiedTextColorSource> {
        let glyph_id = u16::try_from(glyph_id).ok()?;
        self.color_glyphs
            .iter()
            .find(|glyph| glyph.glyph_id() == glyph_id)
            .map(|glyph| glyph.source())
    }

    pub(crate) fn color_glyphs(&self) -> &[UiQualifiedTextColorGlyph] {
        &self.color_glyphs
    }
}

impl UiQualifiedTextLayoutArtifact {
    pub(crate) fn new(input: UiQualifiedTextLayoutArtifactInput) -> Self {
        assert!(input
            .logical_runs
            .iter()
            .all(|run| input.faces.iter().any(|face| face.identity() == run.face())));
        Self {
            request_identity: input.request_identity,
            identity: input.identity,
            source: input.source,
            graphemes: input.graphemes,
            word_boundaries: input.word_boundaries,
            styles: input.styles,
            logical_runs: input.logical_runs,
            glyphs: input.glyphs,
            lines: input.lines,
            visual_runs: input.visual_runs,
            positioned_glyphs: input.positioned_glyphs,
            logical_bounds: input.logical_bounds,
            ink_bounds: input.ink_bounds,
            carets: input.carets,
            coverage: input.coverage,
            faces: input.faces,
            cost: input.cost,
            profile: input.profile,
            font_collection: input.font_collection,
            text_scale: input.text_scale,
            width_basis: input.width_basis,
        }
    }

    #[cfg(test)]
    pub(crate) const fn identity(&self) -> UiQualifiedTextLayoutIdentity {
        self.identity
    }
    pub(crate) fn coverage(&self) -> &[UiQualifiedTextCoverageRecord] {
        &self.coverage
    }
    pub(crate) fn face_resource(
        &self,
        identity: UiQualifiedFontFaceIdentity,
    ) -> Option<&UiQualifiedTextFaceResource> {
        self.faces.iter().find(|face| face.identity() == identity)
    }
    pub(crate) fn view(&self) -> UiQualifiedTextLayoutView<'_> {
        UiQualifiedTextLayoutView::from_text_mechanics(UiQualifiedTextLayoutViewInput {
            request_identity: self.request_identity,
            identity: self.identity,
            source: &self.source,
            graphemes: &self.graphemes,
            word_boundaries: &self.word_boundaries,
            styles: &self.styles,
            logical_runs: &self.logical_runs,
            glyphs: &self.glyphs,
            lines: &self.lines,
            visual_runs: &self.visual_runs,
            positioned_glyphs: &self.positioned_glyphs,
            logical_bounds: self.logical_bounds,
            ink_bounds: self.ink_bounds,
            carets: &self.carets,
            coverage: &self.coverage,
            cost: self.cost,
            profile: self.profile,
            font_collection: self.font_collection,
            text_scale: self.text_scale,
            width_basis: self.width_basis,
        })
    }
}
