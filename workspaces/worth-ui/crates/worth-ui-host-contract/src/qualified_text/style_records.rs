use super::{UiQualifiedFontFamilyIdentity, UiTextOriginalRange};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiFontSlant {
    Upright,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextFeatureRecord {
    tag: [u8; 4],
    value: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedTextVariationRecord {
    axis: [u8; 4],
    value_milli: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextStyleRecord {
    original_range: UiTextOriginalRange,
    language: Box<str>,
    font_size_millipoints: u32,
    letter_spacing_millipoints: i32,
    word_spacing_millipoints: i32,
    family_stack: Box<[UiQualifiedFontFamilyIdentity]>,
    weight: u16,
    width_milli_percent: u32,
    slant: UiFontSlant,
    features: Box<[UiQualifiedTextFeatureRecord]>,
    variations: Box<[UiQualifiedTextVariationRecord]>,
}

#[doc(hidden)]
pub struct UiQualifiedTextStyleInput {
    pub original_range: UiTextOriginalRange,
    pub language: Box<str>,
    pub font_size_millipoints: u32,
    pub letter_spacing_millipoints: i32,
    pub word_spacing_millipoints: i32,
    pub family_stack: Box<[UiQualifiedFontFamilyIdentity]>,
    pub weight: u16,
    pub width_milli_percent: u32,
    pub slant: UiFontSlant,
    pub features: Box<[UiQualifiedTextFeatureRecord]>,
    pub variations: Box<[UiQualifiedTextVariationRecord]>,
}

impl UiQualifiedTextFeatureRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(tag: [u8; 4], value: u32) -> Self {
        Self { tag, value }
    }
    pub const fn tag(self) -> [u8; 4] {
        self.tag
    }
    pub const fn value(self) -> u32 {
        self.value
    }
}

impl UiQualifiedTextVariationRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(axis: [u8; 4], value_milli: i32) -> Self {
        Self { axis, value_milli }
    }
    pub const fn axis(self) -> [u8; 4] {
        self.axis
    }
    pub const fn value_milli(self) -> i32 {
        self.value_milli
    }
}

impl UiQualifiedTextStyleRecord {
    #[doc(hidden)]
    pub fn from_text_mechanics(input: UiQualifiedTextStyleInput) -> Self {
        Self {
            original_range: input.original_range,
            language: input.language,
            font_size_millipoints: input.font_size_millipoints,
            letter_spacing_millipoints: input.letter_spacing_millipoints,
            word_spacing_millipoints: input.word_spacing_millipoints,
            family_stack: input.family_stack,
            weight: input.weight,
            width_milli_percent: input.width_milli_percent,
            slant: input.slant,
            features: input.features,
            variations: input.variations,
        }
    }
    pub const fn original_range(&self) -> UiTextOriginalRange {
        self.original_range
    }
    pub fn language(&self) -> &str {
        &self.language
    }
    pub const fn font_size_millipoints(&self) -> u32 {
        self.font_size_millipoints
    }
    pub const fn letter_spacing_millipoints(&self) -> i32 {
        self.letter_spacing_millipoints
    }
    pub const fn word_spacing_millipoints(&self) -> i32 {
        self.word_spacing_millipoints
    }
    pub fn family_stack(&self) -> &[UiQualifiedFontFamilyIdentity] {
        &self.family_stack
    }
    pub const fn weight(&self) -> u16 {
        self.weight
    }
    pub const fn width_milli_percent(&self) -> u32 {
        self.width_milli_percent
    }
    pub const fn slant(&self) -> UiFontSlant {
        self.slant
    }
    pub fn features(&self) -> &[UiQualifiedTextFeatureRecord] {
        &self.features
    }
    pub fn variations(&self) -> &[UiQualifiedTextVariationRecord] {
        &self.variations
    }
}
