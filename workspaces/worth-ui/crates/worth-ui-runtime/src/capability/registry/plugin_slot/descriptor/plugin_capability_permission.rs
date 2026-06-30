/// Permission posture required before a plugin may contribute through a slot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PluginCapabilityPermission {
    HostGranted,
    UserConsent,
    WorkspacePolicy,
}

impl PluginCapabilityPermission {
    pub fn host_granted() -> Self {
        Self::HostGranted
    }

    pub fn user_consent() -> Self {
        Self::UserConsent
    }

    pub fn workspace_policy() -> Self {
        Self::WorkspacePolicy
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::HostGranted => "host_granted",
            Self::UserConsent => "user_consent",
            Self::WorkspacePolicy => "workspace_policy",
        }
    }
}
