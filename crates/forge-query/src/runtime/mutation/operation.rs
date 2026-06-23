use std::hash::{Hash, Hasher};

use super::ForgeQueryAspectTouch;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAspectMutationOperationKind {
    Set,
    Clear,
}

impl ForgeQueryAspectMutationOperationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Set => "set",
            Self::Clear => "clear",
        }
    }
}

impl std::fmt::Display for ForgeQueryAspectMutationOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq)]
pub struct ForgeQueryAspectMutationOperation {
    aspect_touch: ForgeQueryAspectTouch,
    kind: ForgeQueryAspectMutationOperationKind,
}

impl ForgeQueryAspectMutationOperation {
    pub fn set(aspect_touch: ForgeQueryAspectTouch) -> Self {
        Self::from_touch(aspect_touch, ForgeQueryAspectMutationOperationKind::Set)
    }

    pub fn clear(aspect_touch: ForgeQueryAspectTouch) -> Self {
        Self::from_touch(aspect_touch, ForgeQueryAspectMutationOperationKind::Clear)
    }

    pub(crate) fn from_touch(
        aspect_touch: ForgeQueryAspectTouch,
        kind: ForgeQueryAspectMutationOperationKind,
    ) -> Self {
        Self { aspect_touch, kind }
    }

    pub fn aspect_touch(&self) -> &ForgeQueryAspectTouch {
        &self.aspect_touch
    }

    pub fn kind(&self) -> ForgeQueryAspectMutationOperationKind {
        self.kind
    }
}

impl PartialEq for ForgeQueryAspectMutationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.aspect_touch == other.aspect_touch
    }
}

impl Ord for ForgeQueryAspectMutationOperation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.aspect_touch, self.kind).cmp(&(&other.aspect_touch, other.kind))
    }
}

impl PartialOrd for ForgeQueryAspectMutationOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for ForgeQueryAspectMutationOperation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.aspect_touch.hash(state);
        self.kind.hash(state);
    }
}
