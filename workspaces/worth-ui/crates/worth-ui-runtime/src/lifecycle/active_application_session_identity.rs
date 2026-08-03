/// Opaque identity of one launched application session.
///
/// Equal prepared generations launched independently still receive distinct
/// identities, so move-only replacement work cannot cross between them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiActiveApplicationSessionIdentity {
    value: u64,
}

impl WorthUiActiveApplicationSessionIdentity {
    pub(crate) const fn from_host_session_value(value: u64) -> Self {
        Self { value }
    }

    pub fn as_u64(self) -> u64 {
        self.value
    }
}
