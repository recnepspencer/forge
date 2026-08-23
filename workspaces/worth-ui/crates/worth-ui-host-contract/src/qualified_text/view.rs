use super::{
    UiFontCollectionGeneration, UiPositionedTextGlyphRecord, UiQualifiedTextCaretRecord,
    UiQualifiedTextCostRecord, UiQualifiedTextCoverageRecord, UiQualifiedTextGlyphRecord,
    UiQualifiedTextGraphemeRecord, UiQualifiedTextLayoutIdentity,
    UiQualifiedTextLayoutRequestIdentity, UiQualifiedTextLineRecord, UiQualifiedTextRunRecord,
    UiQualifiedTextStyleRecord, UiQualifiedTextVisualRunRecord, UiQualifiedTextWordBoundaryRecord,
    UiTextProfileGeneration, UiTextRect, UiTextScaleGeneration,
};

#[derive(Clone, Copy)]
pub struct UiQualifiedTextLayoutView<'layout> {
    request_identity: UiQualifiedTextLayoutRequestIdentity,
    identity: UiQualifiedTextLayoutIdentity,
    source: &'layout str,
    graphemes: &'layout [UiQualifiedTextGraphemeRecord],
    word_boundaries: &'layout [UiQualifiedTextWordBoundaryRecord],
    styles: &'layout [UiQualifiedTextStyleRecord],
    logical_runs: &'layout [UiQualifiedTextRunRecord],
    glyphs: &'layout [UiQualifiedTextGlyphRecord],
    lines: &'layout [UiQualifiedTextLineRecord],
    visual_runs: &'layout [UiQualifiedTextVisualRunRecord],
    positioned_glyphs: &'layout [UiPositionedTextGlyphRecord],
    logical_bounds: UiTextRect,
    ink_bounds: UiTextRect,
    carets: &'layout [UiQualifiedTextCaretRecord],
    coverage: &'layout [UiQualifiedTextCoverageRecord],
    cost: UiQualifiedTextCostRecord,
    profile: UiTextProfileGeneration,
    font_collection: UiFontCollectionGeneration,
    text_scale: UiTextScaleGeneration,
    width_basis: super::UiQualifiedTextLayoutWidthBasis,
}

#[doc(hidden)]
pub struct UiQualifiedTextLayoutViewInput<'layout> {
    pub request_identity: UiQualifiedTextLayoutRequestIdentity,
    pub identity: UiQualifiedTextLayoutIdentity,
    pub source: &'layout str,
    pub graphemes: &'layout [UiQualifiedTextGraphemeRecord],
    pub word_boundaries: &'layout [UiQualifiedTextWordBoundaryRecord],
    pub styles: &'layout [UiQualifiedTextStyleRecord],
    pub logical_runs: &'layout [UiQualifiedTextRunRecord],
    pub glyphs: &'layout [UiQualifiedTextGlyphRecord],
    pub lines: &'layout [UiQualifiedTextLineRecord],
    pub visual_runs: &'layout [UiQualifiedTextVisualRunRecord],
    pub positioned_glyphs: &'layout [UiPositionedTextGlyphRecord],
    pub logical_bounds: UiTextRect,
    pub ink_bounds: UiTextRect,
    pub carets: &'layout [UiQualifiedTextCaretRecord],
    pub coverage: &'layout [UiQualifiedTextCoverageRecord],
    pub cost: UiQualifiedTextCostRecord,
    pub profile: UiTextProfileGeneration,
    pub font_collection: UiFontCollectionGeneration,
    pub text_scale: UiTextScaleGeneration,
    pub width_basis: super::UiQualifiedTextLayoutWidthBasis,
}

impl<'layout> UiQualifiedTextLayoutView<'layout> {
    #[doc(hidden)]
    pub const fn from_text_mechanics(input: UiQualifiedTextLayoutViewInput<'layout>) -> Self {
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
            cost: input.cost,
            profile: input.profile,
            font_collection: input.font_collection,
            text_scale: input.text_scale,
            width_basis: input.width_basis,
        }
    }

    pub const fn identity(self) -> UiQualifiedTextLayoutIdentity {
        self.identity
    }
    pub const fn request_identity(self) -> UiQualifiedTextLayoutRequestIdentity {
        self.request_identity
    }
    pub const fn source(self) -> &'layout str {
        self.source
    }
    pub const fn graphemes(self) -> &'layout [UiQualifiedTextGraphemeRecord] {
        self.graphemes
    }
    pub const fn word_boundaries(self) -> &'layout [UiQualifiedTextWordBoundaryRecord] {
        self.word_boundaries
    }
    pub const fn styles(self) -> &'layout [UiQualifiedTextStyleRecord] {
        self.styles
    }
    pub const fn logical_runs(self) -> &'layout [UiQualifiedTextRunRecord] {
        self.logical_runs
    }
    pub const fn glyphs(self) -> &'layout [UiQualifiedTextGlyphRecord] {
        self.glyphs
    }
    pub const fn lines(self) -> &'layout [UiQualifiedTextLineRecord] {
        self.lines
    }
    pub const fn visual_runs(self) -> &'layout [UiQualifiedTextVisualRunRecord] {
        self.visual_runs
    }
    pub const fn positioned_glyphs(self) -> &'layout [UiPositionedTextGlyphRecord] {
        self.positioned_glyphs
    }
    pub const fn logical_bounds(self) -> UiTextRect {
        self.logical_bounds
    }
    pub const fn ink_bounds(self) -> UiTextRect {
        self.ink_bounds
    }
    pub const fn carets(self) -> &'layout [UiQualifiedTextCaretRecord] {
        self.carets
    }
    pub const fn coverage(self) -> &'layout [UiQualifiedTextCoverageRecord] {
        self.coverage
    }
    pub const fn cost(self) -> UiQualifiedTextCostRecord {
        self.cost
    }
    pub const fn profile_generation(self) -> UiTextProfileGeneration {
        self.profile
    }
    pub const fn font_collection_generation(self) -> UiFontCollectionGeneration {
        self.font_collection
    }
    pub const fn text_scale_generation(self) -> UiTextScaleGeneration {
        self.text_scale
    }
    pub const fn width_basis(self) -> super::UiQualifiedTextLayoutWidthBasis {
        self.width_basis
    }
}
