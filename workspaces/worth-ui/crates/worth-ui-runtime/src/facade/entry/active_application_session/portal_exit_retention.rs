use std::collections::BTreeMap;

pub(super) struct UiPortalExitRetentionCoordinator {
    retentions:
        BTreeMap<crate::runtime::motion::UiMotionTrackIdentity, UiPortalMotionExitRetention>,
    pending: Option<UiPortalExitTerminalPending>,
}

pub(in crate::facade::entry) enum UiPortalExitTerminalPending {
    Retry(crate::runtime::motion::UiMotionTrackIdentity),
    InFlight {
        track: crate::runtime::motion::UiMotionTrackIdentity,
        completion: super::super::portal_dismissal::DetachedUiPortalDismissalInFlight,
    },
    Indeterminate {
        track: crate::runtime::motion::UiMotionTrackIdentity,
        recovery: super::super::portal_dismissal::DetachedUiPortalDismissalIndeterminate,
    },
    Reconstruction {
        track: crate::runtime::motion::UiMotionTrackIdentity,
        proposal: crate::runtime::session::UiIndeterminatePortalProposalTransaction,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
    },
}

pub(super) struct UiPortalMotionExitRetention {
    portal: crate::runtime::portal::UiPortalExitRetentionReceipt,
    motion: crate::runtime::motion::UiMotionExitRetentionReceipt,
    terminal: Option<crate::runtime::motion::UiMotionTerminalReceipt>,
}

impl UiPortalExitRetentionCoordinator {
    pub(super) fn new() -> Self {
        Self {
            retentions: BTreeMap::new(),
            pending: None,
        }
    }

    pub(super) fn install(
        &mut self,
        portal: crate::runtime::portal::UiPortalExitRetentionReceipt,
        motion: crate::runtime::motion::UiMotionExitRetentionReceipt,
    ) -> Result<(), ()> {
        let target = motion.target();
        if portal.portal().diagnostic_value() != target.owner_key()
            || portal.portal().owner().mounted_instance_identity() != target.mounted_instance()
        {
            return Err(());
        }
        if self.retentions.contains_key(&motion.track()) {
            return Err(());
        }
        self.retentions.insert(
            motion.track(),
            UiPortalMotionExitRetention {
                portal,
                motion,
                terminal: None,
            },
        );
        Ok(())
    }

    pub(super) fn observe_terminal(
        &mut self,
        terminal: crate::runtime::motion::UiMotionTerminalReceipt,
    ) -> Result<bool, ()> {
        let Some(exit) = terminal.exit_retention() else {
            return Ok(false);
        };
        let retention = self.retentions.get_mut(&terminal.track()).ok_or(())?;
        if retention.motion != exit || retention.terminal.is_some() {
            return Err(());
        }
        retention.terminal = Some(terminal);
        Ok(true)
    }

    pub(super) fn next_terminal(&self) -> Option<&UiPortalMotionExitRetention> {
        self.retentions
            .values()
            .find(|retention| retention.terminal.is_some())
    }

    pub(super) fn pending(&self) -> Option<&UiPortalExitTerminalPending> {
        self.pending.as_ref()
    }

    pub(super) fn take_pending(&mut self) -> Option<UiPortalExitTerminalPending> {
        self.pending.take()
    }

    pub(super) fn retain_pending(&mut self, pending: UiPortalExitTerminalPending) {
        assert!(self.pending.replace(pending).is_none());
    }

    pub(super) fn remove(
        &mut self,
        track: crate::runtime::motion::UiMotionTrackIdentity,
    ) -> Option<UiPortalMotionExitRetention> {
        self.retentions.remove(&track)
    }

    pub(super) fn remove_displaced(
        &mut self,
        motion: crate::runtime::motion::UiMotionExitRetentionReceipt,
    ) -> Result<UiPortalMotionExitRetention, ()> {
        let retention = self.retentions.get(&motion.track()).ok_or(())?;
        if retention.motion != motion {
            return Err(());
        }
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.track() == motion.track())
        {
            match self.pending.take() {
                Some(UiPortalExitTerminalPending::Retry(_)) => {}
                Some(pending) => {
                    self.pending = Some(pending);
                    return Err(());
                }
                None => unreachable!("matched pending portal exit remains present"),
            }
        }
        Ok(self
            .retentions
            .remove(&motion.track())
            .expect("validated displaced portal exit remains retained"))
    }

    pub(super) fn clear_for_shutdown(&mut self) -> usize {
        assert!(self.pending.is_none());
        let retained = self.retentions.len();
        self.retentions.clear();
        retained
    }

    pub(super) fn get(
        &self,
        track: crate::runtime::motion::UiMotionTrackIdentity,
    ) -> Option<&UiPortalMotionExitRetention> {
        self.retentions.get(&track)
    }

    pub(super) fn len(&self) -> usize {
        self.retentions.len()
    }

    pub(super) fn has_terminal_work(&self) -> bool {
        self.pending.is_some() || self.next_terminal().is_some()
    }

    pub(super) fn awaits_physical_progress(&self) -> bool {
        matches!(
            self.pending,
            Some(
                UiPortalExitTerminalPending::InFlight { .. }
                    | UiPortalExitTerminalPending::Indeterminate { .. }
                    | UiPortalExitTerminalPending::Reconstruction { .. }
            )
        )
    }
}

impl UiPortalExitTerminalPending {
    pub(super) const fn track(&self) -> crate::runtime::motion::UiMotionTrackIdentity {
        match self {
            Self::Retry(track)
            | Self::InFlight { track, .. }
            | Self::Indeterminate { track, .. }
            | Self::Reconstruction { track, .. } => *track,
        }
    }

    pub(super) fn matches_native_physical(
        &self,
        class: worth_ui_host_native::UiNativePhysicalProgressClass,
        presentation: Option<worth_ui_host_native::UiNativePhysicalPresentationCorrelation>,
    ) -> bool {
        match self {
            Self::InFlight { completion, .. } => {
                completion.matches_native_physical(class, presentation)
            }
            Self::Indeterminate { .. } => matches!(
                class,
                worth_ui_host_native::UiNativePhysicalProgressClass::PresentationRecovery
                    | worth_ui_host_native::UiNativePhysicalProgressClass::Presentation
            ),
            Self::Reconstruction { in_flight, .. } => {
                let progress_class = match class {
                    worth_ui_host_native::UiNativePhysicalProgressClass::Presentation => {
                        worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface
                    }
                    worth_ui_host_native::UiNativePhysicalProgressClass::TextAtlas => {
                        worth_ui_host_contract::UiHostPresentationProgressClass::TextAtlas
                    }
                    worth_ui_host_native::UiNativePhysicalProgressClass::PresentationRecovery => {
                        return false;
                    }
                };
                in_flight.awaits_progress_class(progress_class)
                    && presentation.map_or(true, |presentation| {
                        presentation.attempt() == in_flight.attempt()
                            && in_flight
                                .pending_bindings()
                                .any(|binding| binding == presentation.binding())
                    })
            }
            Self::Retry(_) => false,
        }
    }
}

impl UiPortalMotionExitRetention {
    pub(super) const fn portal(&self) -> crate::runtime::portal::UiPortalExitRetentionReceipt {
        self.portal
    }

    pub(super) const fn motion(&self) -> crate::runtime::motion::UiMotionExitRetentionReceipt {
        self.motion
    }

    pub(super) const fn terminal(&self) -> crate::runtime::motion::UiMotionTerminalReceipt {
        self.terminal
            .expect("terminal exit-retention selection requires terminal Motion evidence")
    }
}
