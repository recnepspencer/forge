#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedTextLayoutIdentity([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedTextLayoutRequestIdentity([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedTextLayoutWidthBasis(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedFontFaceIdentity {
    font_bytes_digest: [u8; 32],
    face_index: u32,
    selection_digest: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedFontFamilyIdentity([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiQualifiedFontPackIdentity([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiTextProfileGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiFontCollectionGeneration(u64);

/// Opaque content identity for the exact qualified font collection lineage.
///
/// The numeric generation is useful for lifecycle ordering, but it is not a
/// raster-equivalence identity. Text mechanics mints this digest from the
/// admitted collection and consumers may only carry it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiFontCollectionLineageIdentity([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiTextScaleGeneration(u64);

impl UiQualifiedTextLayoutIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> [u8; 32] {
        self.0
    }
}

impl UiQualifiedTextLayoutRequestIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> [u8; 32] {
        self.0
    }
}

impl UiQualifiedTextLayoutWidthBasis {
    pub const fn new(width_millipoints: u32) -> Option<Self> {
        if width_millipoints == 0 {
            None
        } else {
            Some(Self(width_millipoints))
        }
    }

    pub const fn width_millipoints(self) -> u32 {
        self.0
    }
}

impl UiQualifiedFontFaceIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(font_bytes_digest: [u8; 32], face_index: u32) -> Self {
        Self {
            font_bytes_digest,
            face_index,
            selection_digest: font_bytes_digest,
        }
    }

    #[doc(hidden)]
    pub const fn from_application_text_mechanics(
        font_bytes_digest: [u8; 32],
        face_index: u32,
        selection_digest: [u8; 32],
    ) -> Self {
        Self {
            font_bytes_digest,
            face_index,
            selection_digest,
        }
    }

    pub const fn font_bytes_digest(self) -> [u8; 32] {
        self.font_bytes_digest
    }

    pub const fn face_index(self) -> u32 {
        self.face_index
    }

    pub const fn selection_digest(self) -> [u8; 32] {
        self.selection_digest
    }
}

impl UiQualifiedFontFamilyIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> [u8; 32] {
        self.0
    }
}

impl UiQualifiedFontPackIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> [u8; 32] {
        self.0
    }
}

impl UiFontCollectionLineageIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> [u8; 32] {
        self.0
    }
}

macro_rules! generation {
    ($name:ident) => {
        impl $name {
            pub const fn new(value: u64) -> Option<Self> {
                if value == 0 {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

generation!(UiTextProfileGeneration);
generation!(UiFontCollectionGeneration);
generation!(UiTextScaleGeneration);
