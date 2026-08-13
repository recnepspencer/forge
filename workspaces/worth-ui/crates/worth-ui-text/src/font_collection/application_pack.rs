pub(in crate::font_collection) mod color_tables;
mod container;
mod glyph_expansion;
mod metadata;
mod metadata_inventory;
mod name_inventory;
pub(super) mod qualification;

use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiQualifiedFontFaceIdentity,
    UiQualifiedFontFamilyIdentity, UiQualifiedFontPackIdentity,
};

use super::UiFontFaceSource;

pub(super) use qualification::qualify;

#[cfg(test)]
pub(super) fn validate_face_definition_for_test(
    face: &UiApplicationFontFaceDefinition,
) -> Result<(), super::UiFontCollectionAdmissionDenial> {
    metadata::validate_face_definition(face).map(|_| ())
}

pub struct UiApplicationFontLicenseRecord {
    pub identifier: Arc<str>,
    pub notice: Arc<str>,
}

pub struct UiApplicationFontFaceDefinition {
    pub family: Arc<str>,
    pub bytes: Arc<[u8]>,
    pub face_index: u32,
    pub weight: u16,
    pub width_milli_percent: u32,
    pub slant: UiFontSlant,
    pub license: UiApplicationFontLicenseRecord,
}

pub struct UiApplicationFontPackDefinition {
    pub name: Arc<str>,
    pub faces: Box<[UiApplicationFontFaceDefinition]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQualifiedFontFamilyReceipt {
    name: Arc<str>,
    identity: UiQualifiedFontFamilyIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQualifiedFontPackReceipt {
    pub(super) identity: UiQualifiedFontPackIdentity,
    pub(super) collection_generation: UiFontCollectionGeneration,
    pub(super) families: Box<[UiQualifiedFontFamilyReceipt]>,
    pub(super) faces: Box<[UiQualifiedFontFaceReceipt]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedFontAxisReceipt {
    tag: [u8; 4],
    minimum_milli: i32,
    default_milli: i32,
    maximum_milli: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQualifiedFontFaceReceipt {
    identity: UiQualifiedFontFaceIdentity,
    family: UiQualifiedFontFamilyIdentity,
    weight: u16,
    width_milli_percent: u32,
    slant: UiFontSlant,
    family_names: Box<[UiQualifiedFontNameRecordReceipt]>,
    style_names: Box<[UiQualifiedFontNameRecordReceipt]>,
    axes: Box<[UiQualifiedFontAxisReceipt]>,
    feature_tags: Box<[[u8; 4]]>,
    coverage_range_count: u32,
    intrinsic_color: bool,
    max_glyphs_per_input_byte: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedFontNameRecordReceipt {
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: u16,
    content_digest: [u8; 32],
}

pub(super) struct UiQualifiedApplicationPack {
    pub(super) receipt: UiQualifiedFontPackReceipt,
    pub(super) sources: Box<[UiFontFaceSource]>,
}

pub(super) struct UiPreflightedApplicationPack {
    pub(super) definition: UiApplicationFontPackDefinition,
    pub(super) face_digests: Box<[[u8; 32]]>,
    pub(super) bytes_hashed: usize,
    pub(super) application_bytes: usize,
}

impl UiQualifiedFontFamilyReceipt {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub const fn identity(&self) -> UiQualifiedFontFamilyIdentity {
        self.identity
    }
}

impl UiQualifiedFontPackReceipt {
    pub const fn identity(&self) -> UiQualifiedFontPackIdentity {
        self.identity
    }
    pub const fn collection_generation(&self) -> UiFontCollectionGeneration {
        self.collection_generation
    }
    pub fn families(&self) -> &[UiQualifiedFontFamilyReceipt] {
        &self.families
    }
    pub fn family(&self, name: &str) -> Option<UiQualifiedFontFamilyIdentity> {
        self.families
            .iter()
            .find(|family| family.name() == name)
            .map(UiQualifiedFontFamilyReceipt::identity)
    }
    pub fn faces(&self) -> &[UiQualifiedFontFaceReceipt] {
        &self.faces
    }
}

impl UiQualifiedFontFaceReceipt {
    pub const fn identity(&self) -> UiQualifiedFontFaceIdentity {
        self.identity
    }
    pub const fn family(&self) -> UiQualifiedFontFamilyIdentity {
        self.family
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
    pub const fn family_name_records(&self) -> u16 {
        self.family_names.len() as u16
    }
    pub const fn style_name_records(&self) -> u16 {
        self.style_names.len() as u16
    }
    pub fn family_names(&self) -> &[UiQualifiedFontNameRecordReceipt] {
        &self.family_names
    }
    pub fn style_names(&self) -> &[UiQualifiedFontNameRecordReceipt] {
        &self.style_names
    }
    pub fn axes(&self) -> &[UiQualifiedFontAxisReceipt] {
        &self.axes
    }
    pub fn feature_tags(&self) -> &[[u8; 4]] {
        &self.feature_tags
    }
    pub const fn coverage_range_count(&self) -> u32 {
        self.coverage_range_count
    }
    pub const fn has_intrinsic_color(&self) -> bool {
        self.intrinsic_color
    }
    pub const fn max_glyphs_per_input_byte(&self) -> u32 {
        self.max_glyphs_per_input_byte
    }
}

impl UiQualifiedFontNameRecordReceipt {
    pub const fn platform_id(self) -> u16 {
        self.platform_id
    }
    pub const fn encoding_id(self) -> u16 {
        self.encoding_id
    }
    pub const fn language_id(self) -> u16 {
        self.language_id
    }
    pub const fn name_id(self) -> u16 {
        self.name_id
    }
    pub const fn content_digest(self) -> [u8; 32] {
        self.content_digest
    }
}

impl UiQualifiedFontAxisReceipt {
    pub const fn tag(self) -> [u8; 4] {
        self.tag
    }
    pub const fn minimum_milli(self) -> i32 {
        self.minimum_milli
    }
    pub const fn default_milli(self) -> i32 {
        self.default_milli
    }
    pub const fn maximum_milli(self) -> i32 {
        self.maximum_milli
    }
}
