use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::receipt::{
    WorthUiAccessibilityAssociationKind, WorthUiAccessibilityParticipationPosture,
    WorthUiFocusParticipationPosture,
};

impl WorthUiAccessibilityAssociationKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::Description => "description",
            Self::Error => "error",
        }
    }
}

impl WorthUiAccessibilityParticipationPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Exposed => "exposed",
            Self::Hidden => "hidden",
            Self::Inert => "inert",
            Self::Disabled => "disabled",
        }
    }
}

impl WorthUiFocusParticipationPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Focusable => "focusable",
            Self::NotFocusable => "not_focusable",
            Self::Disabled => "disabled",
            Self::Inert => "inert",
            Self::Hidden => "hidden",
        }
    }
}

pub(in crate::runtime::composition_participation) fn digest_parts(
    parts: impl IntoIterator<Item = impl AsRef<str>>,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.as_ref().hash(&mut hasher);
    }
    hasher.finish()
}
