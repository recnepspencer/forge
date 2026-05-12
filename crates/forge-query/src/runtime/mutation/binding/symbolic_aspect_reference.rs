use crate::memory_workspace::ForgeQueryWorkspaceError;

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
    aspect_path: String,
    reference: ForgeQuerySymbolicTargetReference,
}

impl ForgeQuerySymbolicAspectReference {
    pub fn same_batch_entity_identity(
        aspect_path: impl Into<String>,
        reference: ForgeQuerySymbolicTargetReference,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let aspect_path = aspect_path.into();
        if aspect_path.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "symbolic aspect reference path may not be empty",
            ));
        }
        Ok(Self {
            family: ForgeQuerySymbolicAspectReferenceFamily::SameBatchDeclaredEntityIdentity,
            aspect_path,
            reference,
        })
    }

    pub fn family(&self) -> ForgeQuerySymbolicAspectReferenceFamily {
        self.family
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn reference(&self) -> &ForgeQuerySymbolicTargetReference {
        &self.reference
    }
}
