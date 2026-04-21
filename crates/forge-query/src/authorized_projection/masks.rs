use std::collections::BTreeMap;

use crate::authoring::AspectFieldKey;
use crate::identity::hash_parts;

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
    entries: BTreeMap<String, ProjectionVisibility>,
}

impl PolicyAspectMask {
    pub fn allow_all() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn with_masked(mut self, aspect: impl AsRef<str>, field: impl AsRef<str>) -> Self {
        self.entries.insert(
            key_from_parts(aspect.as_ref(), field.as_ref()),
            ProjectionVisibility::Masked,
        );
        self
    }

    pub fn with_non_disclosing_use_only(
        mut self,
        aspect: impl AsRef<str>,
        field: impl AsRef<str>,
    ) -> Self {
        self.entries.insert(
            key_from_parts(aspect.as_ref(), field.as_ref()),
            ProjectionVisibility::NonDisclosingUseOnly,
        );
        self
    }

    pub fn visibility_for(&self, key: &AspectFieldKey) -> ProjectionVisibility {
        self.entries
            .get(&key_from_parts(key.aspect().as_str(), key.field().as_str()))
            .copied()
            .unwrap_or(ProjectionVisibility::Visible)
    }

    pub(crate) fn visibility_for_parts(&self, aspect: &str, field: &str) -> ProjectionVisibility {
        self.entries
            .get(&key_from_parts(aspect, field))
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

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec!["policy_aspect_mask".to_string()];
        parts.extend(
            self.entries
                .iter()
                .map(|(field, visibility)| format!("{field}:{}", visibility.as_str())),
        );
        parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyMaskSnapshot {
    policy_digest: String,
    mask: PolicyAspectMask,
    digest: String,
}

impl PolicyMaskSnapshot {
    pub fn synthetic_authority(policy_digest: impl Into<String>, mask: PolicyAspectMask) -> Self {
        let policy_digest = policy_digest.into();
        let mut parts = vec![
            "policy_mask_snapshot".to_string(),
            format!("policy:{policy_digest}"),
        ];
        parts.extend(mask.digest_parts());
        Self {
            policy_digest,
            mask,
            digest: hash_parts(&parts),
        }
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn mask(&self) -> &PolicyAspectMask {
        &self.mask
    }
}

fn key_from_parts(aspect: &str, field: &str) -> String {
    format!("{aspect}.{field}")
}
