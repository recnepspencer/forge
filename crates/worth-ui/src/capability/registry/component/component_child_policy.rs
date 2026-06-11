/// Component child policy declared as capability metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentChildPolicy {
    NoChildren,
    TextChildren,
    ComponentChildren,
    ShellLayoutAuthority,
}

impl ComponentChildPolicy {
    pub fn no_children() -> Self {
        Self::NoChildren
    }

    pub fn text_children() -> Self {
        Self::TextChildren
    }

    pub fn component_children() -> Self {
        Self::ComponentChildren
    }

    pub fn shell_layout_authority_for_diagnostics() -> Self {
        Self::ShellLayoutAuthority
    }

    pub fn is_illegal(self) -> bool {
        matches!(self, Self::ShellLayoutAuthority)
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NoChildren => "no_children",
            Self::TextChildren => "text_children",
            Self::ComponentChildren => "component_children",
            Self::ShellLayoutAuthority => "shell_layout_authority",
        }
    }
}
