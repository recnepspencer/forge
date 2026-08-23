use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextBaseDirection {
    Auto,
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextWrap {
    None,
    UnicodeWord,
    Grapheme,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextAlignment {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiTextParagraphConstraints {
    language: Arc<str>,
    base_direction: UiTextBaseDirection,
    wrap: UiTextWrap,
    alignment: UiTextAlignment,
    overflow: UiTextOverflow,
    font_size_millipoints: u32,
    width_millipoints: u32,
    line_height_millipoints: u32,
    letter_spacing_millipoints: i32,
    word_spacing_millipoints: i32,
    tab_interval_millipoints: u32,
    maximum_lines: u32,
}

pub struct UiTextParagraphConstraintsInput {
    pub language: Arc<str>,
    pub base_direction: UiTextBaseDirection,
    pub wrap: UiTextWrap,
    pub alignment: UiTextAlignment,
    pub overflow: UiTextOverflow,
    pub font_size_millipoints: u32,
    pub width_millipoints: u32,
    pub line_height_millipoints: u32,
    pub letter_spacing_millipoints: i32,
    pub word_spacing_millipoints: i32,
    pub tab_interval_millipoints: u32,
    pub maximum_lines: u32,
}

impl UiTextParagraphConstraints {
    pub fn new(input: UiTextParagraphConstraintsInput) -> Option<Self> {
        let language = crate::language::admit_language(&input.language)?;
        if input.font_size_millipoints == 0
            || input.width_millipoints == 0
            || input.line_height_millipoints == 0
            || input.tab_interval_millipoints == 0
            || input.maximum_lines == 0
        {
            return None;
        }
        Some(Self {
            language,
            base_direction: input.base_direction,
            wrap: input.wrap,
            alignment: input.alignment,
            overflow: input.overflow,
            font_size_millipoints: input.font_size_millipoints,
            width_millipoints: input.width_millipoints,
            line_height_millipoints: input.line_height_millipoints,
            letter_spacing_millipoints: input.letter_spacing_millipoints,
            word_spacing_millipoints: input.word_spacing_millipoints,
            tab_interval_millipoints: input.tab_interval_millipoints,
            maximum_lines: input.maximum_lines,
        })
    }

    pub fn language(&self) -> &str {
        &self.language
    }
    pub const fn base_direction(&self) -> UiTextBaseDirection {
        self.base_direction
    }
    pub const fn wrap(&self) -> UiTextWrap {
        self.wrap
    }
    pub const fn alignment(&self) -> UiTextAlignment {
        self.alignment
    }
    pub const fn overflow(&self) -> UiTextOverflow {
        self.overflow
    }
    pub const fn font_size_millipoints(&self) -> u32 {
        self.font_size_millipoints
    }
    pub const fn width_millipoints(&self) -> u32 {
        self.width_millipoints
    }
    pub const fn line_height_millipoints(&self) -> u32 {
        self.line_height_millipoints
    }
    pub const fn letter_spacing_millipoints(&self) -> i32 {
        self.letter_spacing_millipoints
    }
    pub const fn word_spacing_millipoints(&self) -> i32 {
        self.word_spacing_millipoints
    }
    pub const fn tab_interval_millipoints(&self) -> u32 {
        self.tab_interval_millipoints
    }
    pub const fn maximum_lines(&self) -> u32 {
        self.maximum_lines
    }
}
