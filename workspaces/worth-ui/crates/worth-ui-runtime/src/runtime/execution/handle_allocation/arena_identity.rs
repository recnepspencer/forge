#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiHandleArenaIdentity {
    value: u64,
}

impl WorthUiHandleArenaIdentity {
    pub(crate) fn from_host_session(session: crate::facade::WorthUiHostSessionIdentity) -> Self {
        Self {
            value: session.as_u64(),
        }
    }

    pub fn as_u64(self) -> u64 {
        self.value
    }
}
