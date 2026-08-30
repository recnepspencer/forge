#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalServiceDisposition {
    Opened,
    Closing,
    Idempotent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiPortalServiceReceipt {
    portal: super::UiPortalIdentity,
    posture: super::UiPortalLifecyclePosture,
    disposition: UiPortalServiceDisposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiPortalExitRetentionReceipt {
    portal: super::UiPortalIdentity,
    revision: u64,
    causal_lineage: u64,
}

impl UiPortalServiceReceipt {
    pub(super) const fn new(
        portal: super::UiPortalIdentity,
        posture: super::UiPortalLifecyclePosture,
        disposition: UiPortalServiceDisposition,
    ) -> Self {
        Self {
            portal,
            posture,
            disposition,
        }
    }

    #[cfg(test)]
    pub(crate) const fn posture(self) -> super::UiPortalLifecyclePosture {
        self.posture
    }

    #[cfg(test)]
    pub(crate) const fn disposition(self) -> UiPortalServiceDisposition {
        self.disposition
    }
}

impl UiPortalExitRetentionReceipt {
    pub(super) const fn new(
        portal: super::UiPortalIdentity,
        revision: u64,
        causal_lineage: u64,
    ) -> Self {
        Self {
            portal,
            revision,
            causal_lineage,
        }
    }

    pub(crate) const fn portal(self) -> super::UiPortalIdentity {
        self.portal
    }

    #[cfg(test)]
    pub(crate) const fn revision(self) -> u64 {
        self.revision
    }

    pub(crate) const fn causal_lineage(self) -> u64 {
        self.causal_lineage
    }
}
