#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindSessionDeadline {
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindCancellationRequest {
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
}

impl UiRebindSessionDeadline {
    pub(crate) const fn new(
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        tick: u64,
    ) -> Self {
        Self { session, tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }

    pub(crate) fn admits(
        self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) -> bool {
        self.session.as_u64() == session.as_u64()
    }
}

impl UiRebindCancellationRequest {
    pub(crate) const fn new(
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) -> Self {
        Self { session }
    }

    pub(crate) fn admits(
        self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) -> bool {
        self.session.as_u64() == session.as_u64()
    }
}
