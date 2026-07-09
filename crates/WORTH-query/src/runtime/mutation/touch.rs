use crate::memory_workspace::WorthQueryWorkspaceError;

use super::WorthQueryParsedAspectTarget;
use worth_foundational::facade::{AspectKey, CanonicalFieldPath};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryAspectTouch {
    target: WorthQueryParsedAspectTarget,
}

impl WorthQueryAspectTouch {
    pub fn from_authoring_ingress_text(
        authored_touch_text: impl Into<String>,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        let target =
            WorthQueryParsedAspectTarget::parse_authoring_ingress_text(authored_touch_text)
                .map_err(|error| WorthQueryWorkspaceError::new(error.to_string()))?;
        Ok(Self { target })
    }

    pub fn whole_aspect(aspect_key: AspectKey) -> Self {
        Self::from_native_parts(aspect_key, None)
    }

    pub fn aspect_field_path(aspect_key: AspectKey, field_path: CanonicalFieldPath) -> Self {
        Self::from_native_parts(aspect_key, Some(field_path))
    }

    pub(crate) fn from_parsed_target(target: WorthQueryParsedAspectTarget) -> Self {
        Self { target }
    }

    pub(crate) fn from_native_parts(
        aspect_key: AspectKey,
        field_path: Option<CanonicalFieldPath>,
    ) -> Self {
        Self {
            target: WorthQueryParsedAspectTarget::from_native_parts(aspect_key, field_path),
        }
    }

    pub(crate) fn into_parsed_target(self) -> WorthQueryParsedAspectTarget {
        self.target
    }

    pub(crate) fn parsed_target(&self) -> &WorthQueryParsedAspectTarget {
        &self.target
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        self.target.aspect_key()
    }

    pub fn native_field_path(&self) -> Option<&CanonicalFieldPath> {
        self.target.field_path()
    }

    pub(crate) fn matches_or_contains(&self, other: &Self) -> bool {
        if self.native_aspect_key() != other.native_aspect_key() {
            return false;
        }
        self == other
            || Self::field_path_has_prefix(self.native_field_path(), other.native_field_path())
            || Self::field_path_has_prefix(other.native_field_path(), self.native_field_path())
    }

    fn field_path_has_prefix(
        candidate: Option<&CanonicalFieldPath>,
        prefix: Option<&CanonicalFieldPath>,
    ) -> bool {
        let Some(prefix) = prefix else {
            return true;
        };
        let Some(candidate) = candidate else {
            return false;
        };
        let candidate_fields = candidate.fields();
        let prefix_fields = prefix.fields();
        candidate_fields.len() >= prefix_fields.len()
            && candidate_fields
                .iter()
                .zip(prefix_fields.iter())
                .all(|(candidate, prefix)| candidate == prefix)
    }

    pub(crate) fn admitted_touch_digest_part(&self) -> String {
        let field_path = self
            .target
            .field_path()
            .map(|path| {
                path.fields()
                    .iter()
                    .map(|field| field.as_str())
                    .collect::<Vec<_>>()
                    .join(".")
            })
            .unwrap_or_else(|| "<whole-aspect>".to_string());
        format!("{}:{field_path}", self.target.aspect_key().as_str())
    }
}
