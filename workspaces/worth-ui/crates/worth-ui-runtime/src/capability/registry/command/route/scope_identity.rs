/// Stable authored identity of the control or portal scope consumed by a
/// command route.
///
/// The identity is branded command-routing vocabulary. It is not a generic
/// string namespace and cannot be substituted for mounted or graph identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRouteScopeIdentity(u64);

impl UiCommandRouteScopeIdentity {
    pub fn for_authored_semantic_name(name: &str) -> Self {
        assert!(
            !name.trim().is_empty(),
            "command route scope identity cannot be empty"
        );
        Self(crate::declaration::stable_text_digest(name.trim()))
    }

    pub(crate) const fn digest(self) -> u64 {
        self.0
    }
}
