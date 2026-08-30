/// Stable authored identity of the control or portal scope consumed by a
/// command route.
///
/// The identity is branded command-routing vocabulary. It is not a generic
/// string namespace and cannot be substituted for mounted or graph identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRouteScopeIdentity(u64);

impl UiCommandRouteScopeIdentity {
    /// Names the authored component that owns a focused-control or active-Portal route.
    ///
    /// This is the only raw authored-name boundary for command scope identity.
    /// Runtime graph lookup and semantic-handoff admission both converge here.
    pub fn for_authored_component(name: &str) -> Self {
        assert!(
            !name.trim().is_empty(),
            "command route scope identity cannot be empty"
        );
        Self(crate::declaration::stable_text_digest(name.trim()))
    }

    pub(crate) fn from_component_declaration(
        declaration: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<Self> {
        declaration
            .authored_semantic_name()
            .strip_prefix("component:")
            .filter(|name| !name.is_empty())
            .map(Self::for_authored_component)
    }

    pub(crate) const fn digest(self) -> u64 {
        self.0
    }
}
