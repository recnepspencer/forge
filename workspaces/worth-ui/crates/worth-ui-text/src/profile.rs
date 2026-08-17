pub struct UiGlobalTextProfile;

impl UiGlobalTextProfile {
    pub const IDENTITY: &str = "worth-ui-global-text-v2";
    pub const MANIFEST_SHA256: &str =
        "cec6005c5baef6d69ada9c30c02ced25b0f253f80c012784fe925e307935c3f2";
    pub const MAX_RETAINED_PARAGRAPHS: usize = 4_096;
    pub const MAX_RETAINED_UTF8_BYTES: usize = 8 * 1_024 * 1_024;
    pub const MAX_PARAGRAPH_UTF8_BYTES: usize = 65_536;
    pub const MAX_GLYPHS: usize = 262_144;
    pub const MAX_GRAPHEME_RECORDS: usize = 262_144;
    pub const MAX_LINE_RECORDS: usize = 65_536;
    pub const MAX_RUNS_PER_PARAGRAPH: usize = 32;
    pub const MAX_APPLICATION_FONT_FACES: usize = 64;
    pub const MAX_APPLICATION_FONT_BYTES: usize = 64 * 1_024 * 1_024;
    pub const MAX_GLYPH_EXPANSION_PER_INPUT_BYTE: usize = 4;
}
