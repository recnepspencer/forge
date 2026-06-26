use crate::capability::IconId;

use super::{
    IconAccessibilityPosture, IconFamily, IconSourceDescriptor, IconThemePosture,
    RawIconAssetReference,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconDescriptor {
    id: IconId,
    family: IconFamily,
    source: Option<IconSourceDescriptor>,
    theme_posture: IconThemePosture,
    accessibility_posture: IconAccessibilityPosture,
    raw_asset_reference: Option<RawIconAssetReference>,
}

impl IconDescriptor {
    pub fn new(id: IconId, family: IconFamily, source: IconSourceDescriptor) -> Self {
        Self {
            id,
            family,
            source: Some(source),
            theme_posture: IconThemePosture::inherits_text_color(),
            accessibility_posture: IconAccessibilityPosture::labelled_by_consumer(),
            raw_asset_reference: None,
        }
    }

    pub fn missing_source_for_diagnostics(id: IconId, family: IconFamily) -> Self {
        Self {
            id,
            family,
            source: None,
            theme_posture: IconThemePosture::inherits_text_color(),
            accessibility_posture: IconAccessibilityPosture::labelled_by_consumer(),
            raw_asset_reference: None,
        }
    }

    pub fn raw_asset_path_for_diagnostics(id: IconId, raw_asset: RawIconAssetReference) -> Self {
        Self {
            id,
            family: IconFamily::custom_admitted(),
            source: None,
            theme_posture: IconThemePosture::inherits_text_color(),
            accessibility_posture: IconAccessibilityPosture::labelled_by_consumer(),
            raw_asset_reference: Some(raw_asset),
        }
    }

    pub fn with_theme_posture(mut self, theme_posture: IconThemePosture) -> Self {
        self.theme_posture = theme_posture;
        self
    }

    pub fn with_accessibility_posture(
        mut self,
        accessibility_posture: IconAccessibilityPosture,
    ) -> Self {
        self.accessibility_posture = accessibility_posture;
        self
    }

    pub fn id(&self) -> &IconId {
        &self.id
    }

    pub fn family(&self) -> &IconFamily {
        &self.family
    }

    pub fn source(&self) -> Option<&IconSourceDescriptor> {
        self.source.as_ref()
    }

    pub fn theme_posture(&self) -> IconThemePosture {
        self.theme_posture
    }

    pub fn accessibility_posture(&self) -> IconAccessibilityPosture {
        self.accessibility_posture
    }

    pub(crate) fn has_raw_asset_reference(&self) -> bool {
        self.raw_asset_reference.is_some()
    }
}
