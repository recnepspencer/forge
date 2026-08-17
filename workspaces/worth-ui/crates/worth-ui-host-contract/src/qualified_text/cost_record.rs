#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiQualifiedTextCostRecord {
    analyzed_bytes: u32,
    graphemes: u32,
    word_boundaries: u32,
    line_opportunities: u32,
    bidi_contexts: u32,
    fallback_clusters: u32,
    coverage_index_queries: u32,
    face_shape_attempts: u32,
    probed_glyphs: u32,
    shaped_runs: u32,
    shaped_scalars: u32,
    emitted_glyphs: u32,
    fitted_units: u32,
    emitted_lines: u32,
    emitted_visual_runs: u32,
    positioned_glyphs: u32,
    emitted_carets: u32,
}

#[doc(hidden)]
pub struct UiQualifiedTextCostInput {
    pub analyzed_bytes: u32,
    pub graphemes: u32,
    pub word_boundaries: u32,
    pub line_opportunities: u32,
    pub bidi_contexts: u32,
    pub fallback_clusters: u32,
    pub coverage_index_queries: u32,
    pub face_shape_attempts: u32,
    pub probed_glyphs: u32,
    pub shaped_runs: u32,
    pub shaped_scalars: u32,
    pub emitted_glyphs: u32,
    pub fitted_units: u32,
    pub emitted_lines: u32,
    pub emitted_visual_runs: u32,
    pub positioned_glyphs: u32,
    pub emitted_carets: u32,
}

impl UiQualifiedTextCostRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(input: UiQualifiedTextCostInput) -> Self {
        Self {
            analyzed_bytes: input.analyzed_bytes,
            graphemes: input.graphemes,
            word_boundaries: input.word_boundaries,
            line_opportunities: input.line_opportunities,
            bidi_contexts: input.bidi_contexts,
            fallback_clusters: input.fallback_clusters,
            coverage_index_queries: input.coverage_index_queries,
            face_shape_attempts: input.face_shape_attempts,
            probed_glyphs: input.probed_glyphs,
            shaped_runs: input.shaped_runs,
            shaped_scalars: input.shaped_scalars,
            emitted_glyphs: input.emitted_glyphs,
            fitted_units: input.fitted_units,
            emitted_lines: input.emitted_lines,
            emitted_visual_runs: input.emitted_visual_runs,
            positioned_glyphs: input.positioned_glyphs,
            emitted_carets: input.emitted_carets,
        }
    }

    pub const fn analyzed_bytes(self) -> u32 {
        self.analyzed_bytes
    }
    pub const fn graphemes(self) -> u32 {
        self.graphemes
    }
    pub const fn word_boundaries(self) -> u32 {
        self.word_boundaries
    }
    pub const fn line_opportunities(self) -> u32 {
        self.line_opportunities
    }
    pub const fn bidi_contexts(self) -> u32 {
        self.bidi_contexts
    }
    pub const fn fallback_clusters(self) -> u32 {
        self.fallback_clusters
    }
    pub const fn coverage_index_queries(self) -> u32 {
        self.coverage_index_queries
    }
    pub const fn face_shape_attempts(self) -> u32 {
        self.face_shape_attempts
    }
    pub const fn probed_glyphs(self) -> u32 {
        self.probed_glyphs
    }
    pub const fn shaped_runs(self) -> u32 {
        self.shaped_runs
    }
    pub const fn shaped_scalars(self) -> u32 {
        self.shaped_scalars
    }
    pub const fn emitted_glyphs(self) -> u32 {
        self.emitted_glyphs
    }
    pub const fn fitted_units(self) -> u32 {
        self.fitted_units
    }
    pub const fn emitted_lines(self) -> u32 {
        self.emitted_lines
    }
    pub const fn emitted_visual_runs(self) -> u32 {
        self.emitted_visual_runs
    }
    pub const fn positioned_glyphs(self) -> u32 {
        self.positioned_glyphs
    }
    pub const fn emitted_carets(self) -> u32 {
        self.emitted_carets
    }
}
