use crate::runtime::WorthQueryAspectTouch;

use super::WorthQuerySymbolicTargetReference;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQuerySymbolicAspectReferenceFamily {
    SameBatchDeclaredEntityIdentity,
}

impl WorthQuerySymbolicAspectReferenceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SameBatchDeclaredEntityIdentity => "same-batch-declared-entity-identity",
        }
    }
}

impl std::fmt::Display for WorthQuerySymbolicAspectReferenceFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySymbolicAspectReference {
    family: WorthQuerySymbolicAspectReferenceFamily,
    aspect_touch: WorthQueryAspectTouch,
    reference: WorthQuerySymbolicTargetReference,
}

impl WorthQuerySymbolicAspectReference {
    pub fn same_batch_entity_identity(
        aspect_touch: WorthQueryAspectTouch,
        reference: WorthQuerySymbolicTargetReference,
    ) -> Self {
        Self {
            family: WorthQuerySymbolicAspectReferenceFamily::SameBatchDeclaredEntityIdentity,
            aspect_touch,
            reference,
        }
    }

    pub fn family(&self) -> WorthQuerySymbolicAspectReferenceFamily {
        self.family
    }

    pub fn aspect_touch(&self) -> &WorthQueryAspectTouch {
        &self.aspect_touch
    }

    pub fn reference(&self) -> &WorthQuerySymbolicTargetReference {
        &self.reference
    }
}
