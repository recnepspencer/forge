/// Component state ownership posture for later lowering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentStateOwnership {
    RuntimeOwned,
    ComponentLocal,
    Stateless,
}

impl ComponentStateOwnership {
    pub fn runtime_owned() -> Self {
        Self::RuntimeOwned
    }

    pub fn component_local() -> Self {
        Self::ComponentLocal
    }

    pub fn stateless() -> Self {
        Self::Stateless
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeOwned => "runtime_owned",
            Self::ComponentLocal => "component_local",
            Self::Stateless => "stateless",
        }
    }
}
