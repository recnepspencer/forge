use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{ForgeQueryAspectTouch, ForgeQuerySymbolicAspectReference};
use crate::runtime::{
    ForgeQueryMutationSymbolIdentity, ForgeQueryMutationTargetCollectionIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySymbolicAspectResolutionEvidence {
    aspect_touch: ForgeQueryAspectTouch,
    family: crate::runtime::ForgeQuerySymbolicAspectReferenceFamily,
    symbol: ForgeQueryMutationSymbolIdentity,
    resolved_entity_identity: ForgeQueryEntityIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQuerySymbolicAspectResolutionEvidence {
    pub(in crate::runtime) fn from_reference(
        reference: &ForgeQuerySymbolicAspectReference,
        resolved_entity_identity: &ForgeQueryEntityIdentity,
    ) -> Self {
        Self {
            aspect_touch: reference.aspect_touch().clone(),
            family: reference.family(),
            symbol: ForgeQueryMutationSymbolIdentity::new(
                "symbolic-aspect-reference",
                reference.reference().symbol(),
            ),
            resolved_entity_identity: resolved_entity_identity.clone(),
            target_collection: reference.reference().target_collection().map(|collection| {
                ForgeQueryMutationTargetCollectionIdentity::new("symbolic-aspect", collection)
            }),
        }
    }

    pub fn aspect_touch(&self) -> &ForgeQueryAspectTouch {
        &self.aspect_touch
    }

    pub fn family(&self) -> crate::runtime::ForgeQuerySymbolicAspectReferenceFamily {
        self.family
    }

    pub fn symbol(&self) -> &ForgeQueryMutationSymbolIdentity {
        &self.symbol
    }

    pub fn resolved_entity_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }
}
