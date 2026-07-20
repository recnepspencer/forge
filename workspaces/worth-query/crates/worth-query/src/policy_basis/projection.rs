use std::collections::BTreeMap;

use crate::authoring::AspectFieldKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ProjectionVisibility {
    Visible,
    Masked,
    NonDisclosingUseOnly,
    DeniedHiddenInfluence,
}

impl ProjectionVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Masked => "masked",
            Self::NonDisclosingUseOnly => "non_disclosing_use_only",
            Self::DeniedHiddenInfluence => "denied_hidden_influence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAspectMask {
    entries: BTreeMap<AspectFieldKey, ProjectionVisibility>,
}

impl PolicyAspectMask {
    pub fn allow_all() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn with_masked(mut self, field: AspectFieldKey) -> Self {
        self.entries.insert(field, ProjectionVisibility::Masked);
        self
    }

    pub fn with_non_disclosing_use_only(mut self, field: AspectFieldKey) -> Self {
        self.entries
            .insert(field, ProjectionVisibility::NonDisclosingUseOnly);
        self
    }

    pub fn visibility_for(&self, key: &AspectFieldKey) -> ProjectionVisibility {
        self.entries
            .get(key)
            .copied()
            .unwrap_or(ProjectionVisibility::Visible)
    }

    pub fn masked_entry_count(&self) -> usize {
        self.entries
            .values()
            .filter(|visibility| {
                matches!(
                    visibility,
                    ProjectionVisibility::Masked
                        | ProjectionVisibility::NonDisclosingUseOnly
                        | ProjectionVisibility::DeniedHiddenInfluence
                )
            })
            .count()
    }

    pub(crate) fn has_restricted_fields(&self) -> bool {
        !self.entries.is_empty()
    }

    pub(crate) fn has_non_disclosing_fields(&self) -> bool {
        self.entries
            .values()
            .any(|visibility| *visibility == ProjectionVisibility::NonDisclosingUseOnly)
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec!["policy_aspect_mask".to_string()];
        parts.extend(self.entries.iter().map(|(field, visibility)| {
            format!(
                "{}.{}:{}",
                field.aspect().as_str(),
                field.field().as_str(),
                visibility.as_str()
            )
        }));
        parts
    }
}
