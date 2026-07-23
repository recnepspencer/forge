use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};

use crate::authorized_projection::AuthorizedProjectionFieldPath;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectionFactFieldPath {
    locator: ProjectionFactFieldLocator,
    terminal_projection: String,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ProjectionFactFieldLocator {
    Canonical(CanonicalFieldPath),
    Native(AuthorizedProjectionFieldPath),
}

impl ProjectionFactFieldPath {
    pub fn from_canonical_field_path(path: CanonicalFieldPath) -> Self {
        let terminal_projection = path
            .fields()
            .iter()
            .map(FieldKey::as_str)
            .collect::<Vec<_>>()
            .join(".");
        Self {
            locator: ProjectionFactFieldLocator::Canonical(path),
            terminal_projection,
        }
    }

    pub(crate) fn from_native_aspect_key(aspect_key: AspectKey) -> Self {
        let path = AuthorizedProjectionFieldPath::from_native_aspect_key(aspect_key);
        Self {
            terminal_projection: path.terminal_projection_for_boundary().to_string(),
            locator: ProjectionFactFieldLocator::Native(path),
        }
    }

    pub(crate) fn from_native_keys(aspect_key: AspectKey, field_key: FieldKey) -> Self {
        let path = AuthorizedProjectionFieldPath::from_native_keys(aspect_key, field_key);
        Self {
            terminal_projection: path.terminal_projection_for_boundary().to_string(),
            locator: ProjectionFactFieldLocator::Native(path),
        }
    }

    pub(crate) fn terminal_projection_for_boundary(&self) -> &str {
        &self.terminal_projection
    }

    pub fn canonical_field_path(&self) -> Option<&CanonicalFieldPath> {
        match &self.locator {
            ProjectionFactFieldLocator::Canonical(path) => Some(path),
            ProjectionFactFieldLocator::Native(_) => None,
        }
    }

    pub fn native_aspect_key(&self) -> Option<&AspectKey> {
        match &self.locator {
            ProjectionFactFieldLocator::Canonical(_) => None,
            ProjectionFactFieldLocator::Native(path) => Some(path.native_aspect_key()),
        }
    }

    pub fn native_field_key(&self) -> Option<&FieldKey> {
        match &self.locator {
            ProjectionFactFieldLocator::Canonical(_) => None,
            ProjectionFactFieldLocator::Native(path) => path.native_field_key(),
        }
    }
}

pub(crate) fn projection_fact_field_path_from_segments(
    segments: impl IntoIterator<Item = FieldKey>,
) -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(segments.into_iter().collect::<Vec<_>>())
            .expect("projection fact field paths must contain at least one field"),
    )
}
