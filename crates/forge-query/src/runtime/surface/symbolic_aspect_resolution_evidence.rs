use crate::runtime::ForgeQuerySymbolicAspectReference;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySymbolicAspectResolutionEvidence {
    aspect_path: String,
    family: crate::runtime::ForgeQuerySymbolicAspectReferenceFamily,
    symbol: String,
    resolved_entity_identity: String,
    target_collection: Option<String>,
}

impl ForgeQuerySymbolicAspectResolutionEvidence {
    pub(in crate::runtime) fn from_reference(
        reference: &ForgeQuerySymbolicAspectReference,
        resolved_entity_identity: impl Into<String>,
    ) -> Self {
        Self {
            aspect_path: reference.aspect_path().to_string(),
            family: reference.family(),
            symbol: reference.reference().symbol().to_string(),
            resolved_entity_identity: resolved_entity_identity.into(),
            target_collection: reference
                .reference()
                .target_collection()
                .map(str::to_string),
        }
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn family(&self) -> crate::runtime::ForgeQuerySymbolicAspectReferenceFamily {
        self.family
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn resolved_entity_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }
}
