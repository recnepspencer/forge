#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiPortalClosedInspectionRecord {
    portal: super::UiPortalIdentity,
    cause: super::UiPortalDismissalCause,
    closed_descendants: u16,
    revision: u64,
}

impl UiPortalClosedInspectionRecord {
    pub(super) fn new(
        portal: super::UiPortalIdentity,
        cause: super::UiPortalDismissalCause,
        closed_descendants: usize,
        revision: u64,
    ) -> Self {
        Self {
            portal,
            cause,
            closed_descendants: u16::try_from(closed_descendants).unwrap_or(u16::MAX),
            revision,
        }
    }

    pub(crate) const fn portal(self) -> super::UiPortalIdentity {
        self.portal
    }
    pub(crate) const fn cause(self) -> super::UiPortalDismissalCause {
        self.cause
    }
    pub(crate) const fn closed_descendants(self) -> u16 {
        self.closed_descendants
    }
    pub(crate) const fn revision(self) -> u64 {
        self.revision
    }
}
