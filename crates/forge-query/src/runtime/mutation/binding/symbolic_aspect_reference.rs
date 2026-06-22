use crate::runtime::ForgeQueryAspectTouch;

use super::ForgeQuerySymbolicTargetReference;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQuerySymbolicAspectReferenceFamily {
    SameBatchDeclaredEntityIdentity,
}

impl ForgeQuerySymbolicAspectReferenceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SameBatchDeclaredEntityIdentity => "same-batch-declared-entity-identity",
        }
    }
}

impl std::fmt::Display for ForgeQuerySymbolicAspectReferenceFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySymbolicAspectReference {
    family: ForgeQuerySymbolicAspectReferenceFamily,
    aspect_touch: ForgeQueryAspectTouch,
    reference: ForgeQuerySymbolicTargetReference,
}

impl ForgeQuerySymbolicAspectReference {
    pub fn same_batch_entity_identity(
        aspect_touch: ForgeQueryAspectTouch,
        reference: ForgeQuerySymbolicTargetReference,
    ) -> Self {
        Self {
            family: ForgeQuerySymbolicAspectReferenceFamily::SameBatchDeclaredEntityIdentity,
            aspect_touch,
            reference,
        }
    }

    pub fn family(&self) -> ForgeQuerySymbolicAspectReferenceFamily {
        self.family
    }

    pub fn aspect_touch(&self) -> &ForgeQueryAspectTouch {
        &self.aspect_touch
    }

    pub fn reference(&self) -> &ForgeQuerySymbolicTargetReference {
        &self.reference
    }
}
