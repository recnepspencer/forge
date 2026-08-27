use crate::capability::ComponentId;

/// Associates one authored component with the exact component capability that
/// owns its transient Portal surface.
///
/// The component's ordinary allocation is interpreted as Portal-relative
/// geometry while the owner has a presented Portal. It produces no mounted
/// paint, text, accessibility, or hit-test surface otherwise.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentPortalChildContract {
    owner: ComponentId,
}

impl ComponentPortalChildContract {
    pub fn new(owner: ComponentId) -> Self {
        Self { owner }
    }

    pub fn owner(&self) -> &ComponentId {
        &self.owner
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("portal-child:{}", self.owner.as_str())
    }
}
