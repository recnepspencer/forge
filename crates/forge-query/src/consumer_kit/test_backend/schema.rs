use std::collections::BTreeSet;

use crate::memory_workspace::ForgeQueryAspect;

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
        external_projection_path: impl Into<String>,
    ) -> Result<Self, ForgeQueryTestBackendError> {
        self.aspects.push(ForgeQueryTestBackendAspect {
            label: label.into(),
            external_projection_path: external_projection_path.into(),
        });
        self.validate()?;
        Ok(self)
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn aspects(&self) -> impl Iterator<Item = (&str, &str)> {
        self.aspects.iter().map(|aspect| {
            (
                aspect.label.as_str(),
                aspect.external_projection_path.as_str(),
            )
        })
    }

    pub(crate) fn memory_aspects(
        &self,
    ) -> Result<Vec<ForgeQueryAspect>, ForgeQueryTestBackendError> {
        self.validate()?;
        Ok(self
            .aspects()
            .map(|(label, path)| ForgeQueryAspect::new(label, path))
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
            ensure_non_blank(
                &aspect.label,
                ForgeQueryTestBackendErrorKind::BlankAspectLabel,
                "test backend aspect label may not be blank",
            )?;
            ensure_non_blank(
                &aspect.external_projection_path,
                ForgeQueryTestBackendErrorKind::BlankProjectionPath,
                "test backend external projection path may not be blank",
            )?;
            ensure_unique(
                labels.insert(aspect.label.clone()),
                ForgeQueryTestBackendErrorKind::DuplicateAspectLabel,
                format!(
                    "test backend schema declares duplicate aspect label `{}`",
                    aspect.label
                ),
            )?;
            ensure_unique(
                paths.insert(aspect.external_projection_path.clone()),
                ForgeQueryTestBackendErrorKind::DuplicateProjectionPath,
                format!(
                    "test backend schema declares duplicate external projection path `{}`",
                    aspect.external_projection_path
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
    label: String,
    external_projection_path: String,
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
