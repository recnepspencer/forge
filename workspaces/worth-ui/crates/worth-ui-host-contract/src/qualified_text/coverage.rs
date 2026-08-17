use super::{UiFontCollectionGeneration, UiQualifiedFontFaceIdentity, UiTextOriginalRange};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextCoverageDisposition {
    QualifiedFace,
    MissingCluster,
    LayoutControl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextCoverageRecord {
    original_range: UiTextOriginalRange,
    face: Option<UiQualifiedFontFaceIdentity>,
    disposition: UiTextCoverageDisposition,
    attempted_collection: UiFontCollectionGeneration,
}

impl UiQualifiedTextCoverageRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        original_range: UiTextOriginalRange,
        face: Option<UiQualifiedFontFaceIdentity>,
        disposition: UiTextCoverageDisposition,
        attempted_collection: UiFontCollectionGeneration,
    ) -> Self {
        Self {
            original_range,
            face,
            disposition,
            attempted_collection,
        }
    }

    pub const fn original_range(self) -> UiTextOriginalRange {
        self.original_range
    }
    pub const fn face(self) -> Option<UiQualifiedFontFaceIdentity> {
        self.face
    }
    pub const fn disposition(self) -> UiTextCoverageDisposition {
        self.disposition
    }
    pub const fn attempted_collection(self) -> UiFontCollectionGeneration {
        self.attempted_collection
    }
}
