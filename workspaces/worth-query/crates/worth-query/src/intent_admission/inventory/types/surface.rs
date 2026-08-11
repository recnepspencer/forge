#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionSurfaceDescriptor {
    Available(&'static str),
    Deferred(&'static str),
}

impl WorthQueryIntentAdmissionSurfaceDescriptor {
    pub const fn available(label: &'static str) -> Self {
        Self::Available(label)
    }

    pub const fn deferred(reason: &'static str) -> Self {
        Self::Deferred(reason)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Available(label) | Self::Deferred(label) => label,
        }
    }

    pub fn deferred_reason(self) -> Option<&'static str> {
        match self {
            Self::Available(_) => None,
            Self::Deferred(reason) => Some(reason),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionExecutionHandoffInventory {
    Available(&'static str),
    NoExecutionHandoff(&'static str),
}

impl WorthQueryIntentAdmissionExecutionHandoffInventory {
    pub const fn available(type_name: &'static str) -> Self {
        Self::Available(type_name)
    }

    pub const fn no_execution_handoff(reason: &'static str) -> Self {
        Self::NoExecutionHandoff(reason)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Available(label) | Self::NoExecutionHandoff(label) => label,
        }
    }

    pub fn no_execution_handoff_reason(self) -> Option<&'static str> {
        match self {
            Self::Available(_) => None,
            Self::NoExecutionHandoff(reason) => Some(reason),
        }
    }
}
