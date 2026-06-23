use std::collections::BTreeSet;

use crate::memory_workspace::ForgeQueryAspect;
use crate::runtime::ForgeQueryAspectTouch;
use forge_foundational::facade::{CanonicalFieldPath, FieldKey};

use super::error::{ForgeQueryTestBackendError, ForgeQueryTestBackendErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendSchema {
    collection: String,
    aspects: Vec<ForgeQueryTestBackendAspect>,
}

impl ForgeQueryTestBackendSchema {
    pub fn single_collection(collection: impl Into<String>) -> Self {
        Self {
            collection: collection.into(),
            aspects: Vec::new(),
        }
    }

    pub fn aspect(
        mut self,
        label: impl Into<String>,
        projection_field_path_text: impl Into<String>,
    ) -> Result<Self, ForgeQueryTestBackendError> {
        let label = label.into();
        ensure_non_blank(
            &label,
            ForgeQueryTestBackendErrorKind::BlankAspectLabel,
            "test backend aspect label may not be blank",
        )?;
        let aspect_touch = ForgeQueryAspectTouch::admit_authoring_ingress_text(label.clone())
            .map_err(|error| {
                ForgeQueryTestBackendError::new(
                    ForgeQueryTestBackendErrorKind::InvalidAspectLabel,
                    format!(
                        "test backend aspect label `{label}` is not a native aspect touch: {error}"
                    ),
                )
            })?;
        let projection_field_path_text = projection_field_path_text.into();
        ensure_non_blank(
            &projection_field_path_text,
            ForgeQueryTestBackendErrorKind::BlankProjectionPath,
            "test backend projection field path authoring text may not be blank",
        )?;
        let native_field_path =
            canonical_field_path_from_authoring_text(&projection_field_path_text).ok_or_else(
                || {
                    ForgeQueryTestBackendError::new(
                        ForgeQueryTestBackendErrorKind::InvalidProjectionPath,
                        format!(
                            "test backend projection field path authoring text `{projection_field_path_text}` is not a canonical field path"
                        ),
                    )
                },
            )?;
        self.aspects.push(ForgeQueryTestBackendAspect {
            touch: aspect_touch,
            native_field_path,
        });
        self.validate()?;
        Ok(self)
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn aspects(&self) -> impl Iterator<Item = (&ForgeQueryAspectTouch, &CanonicalFieldPath)> {
        self.aspects
            .iter()
            .map(|aspect| (&aspect.touch, &aspect.native_field_path))
    }

    pub(crate) fn memory_aspects(
        &self,
    ) -> Result<Vec<ForgeQueryAspect>, ForgeQueryTestBackendError> {
        self.validate()?;
        Ok(self
            .aspects()
            .map(|(touch, path)| {
                ForgeQueryAspect::from_native_field_path(touch.clone(), path.clone())
            })
            .collect())
    }

    pub(crate) fn validate(&self) -> Result<(), ForgeQueryTestBackendError> {
        ensure_non_blank(
            self.collection(),
            ForgeQueryTestBackendErrorKind::BlankCollectionName,
            "test backend collection name may not be blank",
        )?;
        let mut labels = BTreeSet::new();
        let mut paths = BTreeSet::new();
        for aspect in &self.aspects {
            ensure_unique(
                labels.insert(aspect.touch.clone()),
                ForgeQueryTestBackendErrorKind::DuplicateAspectLabel,
                format!(
                    "test backend schema declares duplicate admitted aspect touch `{}`",
                    reporting_projection_from_admitted_touch(&aspect.touch)
                ),
            )?;
            ensure_unique(
                paths.insert(aspect.native_field_path.clone()),
                ForgeQueryTestBackendErrorKind::DuplicateProjectionPath,
                format!(
                    "test backend schema declares duplicate native field path `{}`",
                    terminal_projection_from_field_path(&aspect.native_field_path)
                ),
            )?;
        }
        if self.aspects.is_empty() {
            return Err(ForgeQueryTestBackendError::new(
                ForgeQueryTestBackendErrorKind::EmptyAspectSet,
                "test backend schema must declare at least one aspect",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ForgeQueryTestBackendAspect {
    touch: ForgeQueryAspectTouch,
    native_field_path: CanonicalFieldPath,
}

fn ensure_non_blank(
    value: &str,
    kind: ForgeQueryTestBackendErrorKind,
    message: impl Into<String>,
) -> Result<(), ForgeQueryTestBackendError> {
    if value.trim().is_empty() {
        return Err(ForgeQueryTestBackendError::new(kind, message));
    }
    Ok(())
}

fn ensure_unique(
    unique: bool,
    kind: ForgeQueryTestBackendErrorKind,
    message: impl Into<String>,
) -> Result<(), ForgeQueryTestBackendError> {
    if !unique {
        return Err(ForgeQueryTestBackendError::new(kind, message));
    }
    Ok(())
}

fn canonical_field_path_from_authoring_text(path: &str) -> Option<CanonicalFieldPath> {
    CanonicalFieldPath::new(
        path.split('.')
            .map(FieldKey::new)
            .collect::<Option<Vec<_>>>()?,
    )
}

fn terminal_projection_from_field_path(field_path: &CanonicalFieldPath) -> String {
    field_path
        .fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}

fn reporting_projection_from_admitted_touch(touch: &ForgeQueryAspectTouch) -> String {
    touch.admitted_touch_digest_part()
}
