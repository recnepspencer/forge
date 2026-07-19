use std::hash::{Hash, Hasher};

use super::WorthQueryAspectTouch;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAspectMutationOperationKind {
    Set,
    Clear,
}

impl WorthQueryAspectMutationOperationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Set => "set",
            Self::Clear => "clear",
        }
    }
}

impl std::fmt::Display for WorthQueryAspectMutationOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq)]
pub struct WorthQueryAspectMutationOperation {
    aspect_touch: WorthQueryAspectTouch,
    kind: WorthQueryAspectMutationOperationKind,
}

impl WorthQueryAspectMutationOperation {
    pub fn set(aspect_touch: WorthQueryAspectTouch) -> Self {
        Self::from_touch(aspect_touch, WorthQueryAspectMutationOperationKind::Set)
    }

    pub fn clear(aspect_touch: WorthQueryAspectTouch) -> Self {
        Self::from_touch(aspect_touch, WorthQueryAspectMutationOperationKind::Clear)
    }

    pub(crate) fn from_touch(
        aspect_touch: WorthQueryAspectTouch,
        kind: WorthQueryAspectMutationOperationKind,
    ) -> Self {
        Self { aspect_touch, kind }
    }

    pub fn aspect_touch(&self) -> &WorthQueryAspectTouch {
        &self.aspect_touch
    }

    pub fn kind(&self) -> WorthQueryAspectMutationOperationKind {
        self.kind
    }
}

impl PartialEq for WorthQueryAspectMutationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.aspect_touch == other.aspect_touch
    }
}

impl Ord for WorthQueryAspectMutationOperation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.aspect_touch, self.kind).cmp(&(&other.aspect_touch, other.kind))
    }
}

impl PartialOrd for WorthQueryAspectMutationOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for WorthQueryAspectMutationOperation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.aspect_touch.hash(state);
        self.kind.hash(state);
    }
}
