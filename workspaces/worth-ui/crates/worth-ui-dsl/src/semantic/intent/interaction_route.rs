use crate::source::WorthUiArtifactInputBodyAtom;

use super::WorthUiIntentInteractionFamily;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiIntentInteractionRouteKind {
    Product,
    Confirmation,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiIntentInteractionRoute {
    family: WorthUiIntentInteractionFamily,
    declaration_identity: Box<str>,
    kind: WorthUiIntentInteractionRouteKind,
}

impl WorthUiIntentInteractionRoute {
    pub fn product(
        family: WorthUiIntentInteractionFamily,
        declaration_identity: impl Into<Box<str>>,
    ) -> Self {
        Self::new(
            family,
            declaration_identity,
            WorthUiIntentInteractionRouteKind::Product,
        )
    }

    pub fn confirmation(declaration_identity: impl Into<Box<str>>) -> Self {
        Self::new(
            WorthUiIntentInteractionFamily::Activate,
            declaration_identity,
            WorthUiIntentInteractionRouteKind::Confirmation,
        )
    }

    pub const fn family(&self) -> WorthUiIntentInteractionFamily {
        self.family
    }

    pub fn declaration_identity(&self) -> &str {
        &self.declaration_identity
    }

    pub const fn kind(&self) -> WorthUiIntentInteractionRouteKind {
        self.kind
    }

    pub(crate) fn body_atoms(&self) -> [WorthUiArtifactInputBodyAtom; 4] {
        [
            WorthUiArtifactInputBodyAtom::Identifier("interaction".to_owned()),
            WorthUiArtifactInputBodyAtom::Identifier(self.family.as_str().to_owned()),
            WorthUiArtifactInputBodyAtom::Identifier(
                match self.kind {
                    WorthUiIntentInteractionRouteKind::Product => "routes",
                    WorthUiIntentInteractionRouteKind::Confirmation => "confirms",
                }
                .to_owned(),
            ),
            WorthUiArtifactInputBodyAtom::Identifier(self.declaration_identity.to_string()),
        ]
    }

    pub(crate) fn from_authored_parts(
        family: WorthUiIntentInteractionFamily,
        declaration_identity: String,
        kind: WorthUiIntentInteractionRouteKind,
    ) -> Self {
        Self::new(family, declaration_identity, kind)
    }

    fn new(
        family: WorthUiIntentInteractionFamily,
        declaration_identity: impl Into<Box<str>>,
        kind: WorthUiIntentInteractionRouteKind,
    ) -> Self {
        let declaration_identity = declaration_identity.into();
        assert!(
            !declaration_identity.trim().is_empty(),
            "intent route declaration identity cannot be empty"
        );
        Self {
            family,
            declaration_identity,
            kind,
        }
    }
}
