use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::overlay::published_overlay_cost;
use super::{
    seal_cleared_overlay, seal_pending_overlay, seal_published_overlay,
    UiClearedVisualOverlayReceipt, UiClearingVisualOverlay, UiPendingVisualOverlay,
    UiPublishedVisualOverlay, UiPublishingVisualOverlay, UiVisualOverlayIdentity,
    UiVisualOverlaySelection, UiVisualOverlayTarget,
};

#[derive(Clone)]
pub(crate) struct UiVisualOverlayRegistry {
    state: Rc<RefCell<UiVisualOverlayRegistryState>>,
}

pub(crate) struct UiPendingVisualOverlayRegistration {
    state: Weak<RefCell<UiVisualOverlayRegistryState>>,
    identity: UiVisualOverlayIdentity,
    active: bool,
}

struct UiVisualOverlayRegistryState {
    closed: bool,
    revision: u64,
    entry: Option<UiVisualOverlayEntry>,
}

struct UiVisualOverlayEntry {
    identity: UiVisualOverlayIdentity,
    selection: UiVisualOverlaySelection,
    _lease: crate::mounting::UiMountedVisualOverlayLease,
    posture: UiVisualOverlayPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiVisualOverlayPosture {
    Pending,
    Publishing,
    Published(worth_ui_host_contract::UiMountedFrameIdentity),
    Clearing(worth_ui_host_contract::UiMountedFrameIdentity),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiVisualOverlayShutdownReport {
    cancelled_pending_count: usize,
    disposed_published_count: usize,
    disposed_clearing_count: usize,
}

impl UiVisualOverlayRegistry {
    pub(crate) fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(UiVisualOverlayRegistryState {
                closed: false,
                revision: 0,
                entry: None,
            })),
        }
    }

    pub(crate) fn register(
        &self,
        identity: UiVisualOverlayIdentity,
        target: UiVisualOverlayTarget,
    ) -> Result<UiPendingVisualOverlay, worth_ui_inspection::UiVisualOverlayDenial> {
        let (selection, lease) = target.into_parts();
        let mut state = self.state.borrow_mut();
        if state.closed || state.entry.is_some() {
            return Err(worth_ui_inspection::UiVisualOverlayDenial::CapacityExceeded);
        }
        state.entry = Some(UiVisualOverlayEntry {
            identity,
            selection: selection.clone(),
            _lease: lease,
            posture: UiVisualOverlayPosture::Pending,
        });
        Ok(seal_pending_overlay(
            identity,
            selection,
            UiPendingVisualOverlayRegistration {
                state: Rc::downgrade(&self.state),
                identity,
                active: true,
            },
        ))
    }

    pub(crate) fn begin_publication(
        &self,
        pending: UiPendingVisualOverlay,
    ) -> UiPublishingVisualOverlay {
        let (identity, selection, registration) = pending.into_parts();
        let mut state = self.state.borrow_mut();
        let entry = matching_entry(&mut state, identity)
            .expect("same-session pending overlay remains registered");
        assert_eq!(entry.posture, UiVisualOverlayPosture::Pending);
        entry.posture = UiVisualOverlayPosture::Publishing;
        state.revision =
            next_revision(state.revision).expect("an admitted overlay revision can advance");
        registration.commit();
        UiPublishingVisualOverlay {
            identity,
            selection,
        }
    }

    pub(crate) fn commit_publication(
        &self,
        publishing: UiPublishingVisualOverlay,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
    ) -> Result<UiPublishedVisualOverlay, worth_ui_inspection::UiVisualOverlayDenial> {
        let mut state = self.state.borrow_mut();
        let entry = matching_entry(&mut state, publishing.identity)?;
        if entry.posture != UiVisualOverlayPosture::Publishing {
            return Err(worth_ui_inspection::UiVisualOverlayDenial::Superseded);
        }
        entry.posture = UiVisualOverlayPosture::Published(frame);
        let cost = published_overlay_cost(entry._lease.structural_bytes());
        Ok(seal_published_overlay(publishing, frame, cost))
    }

    pub(crate) fn rollback_publication(
        &self,
        publishing: UiPublishingVisualOverlay,
    ) -> UiPendingVisualOverlay {
        let mut state = self.state.borrow_mut();
        let entry = matching_entry(&mut state, publishing.identity)
            .expect("publishing overlay remains registered until presentation settles");
        entry.posture = UiVisualOverlayPosture::Pending;
        state.revision = next_revision(state.revision)
            .expect("an admitted overlay revision cannot exhaust during rollback");
        seal_pending_overlay(
            publishing.identity,
            publishing.selection,
            UiPendingVisualOverlayRegistration {
                state: Rc::downgrade(&self.state),
                identity: publishing.identity,
                active: true,
            },
        )
    }

    pub(crate) fn begin_clear(
        &self,
        published: UiPublishedVisualOverlay,
    ) -> UiClearingVisualOverlay {
        let (identity, selection, published_frame, published_cost) = published.into_parts();
        let mut state = self.state.borrow_mut();
        let entry = matching_entry(&mut state, identity)
            .expect("same-session published overlay remains registered");
        assert_eq!(
            entry.posture,
            UiVisualOverlayPosture::Published(published_frame)
        );
        entry.posture = UiVisualOverlayPosture::Clearing(published_frame);
        state.revision =
            next_revision(state.revision).expect("an admitted overlay revision can advance");
        UiClearingVisualOverlay {
            identity,
            selection,
            published_frame,
            published_cost,
        }
    }

    pub(crate) fn commit_clear(
        &self,
        clearing: UiClearingVisualOverlay,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
    ) -> Result<UiClearedVisualOverlayReceipt, worth_ui_inspection::UiVisualOverlayDenial> {
        let mut state = self.state.borrow_mut();
        let entry = matching_entry(&mut state, clearing.identity)?;
        if entry.posture != UiVisualOverlayPosture::Clearing(clearing.published_frame) {
            return Err(worth_ui_inspection::UiVisualOverlayDenial::Superseded);
        }
        state.entry = None;
        Ok(seal_cleared_overlay(clearing, frame))
    }

    pub(crate) fn rollback_clear(
        &self,
        clearing: UiClearingVisualOverlay,
    ) -> UiPublishedVisualOverlay {
        let mut state = self.state.borrow_mut();
        let entry = matching_entry(&mut state, clearing.identity)
            .expect("clearing overlay remains registered until presentation settles");
        entry.posture = UiVisualOverlayPosture::Published(clearing.published_frame);
        state.revision = next_revision(state.revision)
            .expect("an admitted overlay revision cannot exhaust during rollback");
        seal_published_overlay(
            UiPublishingVisualOverlay {
                identity: clearing.identity,
                selection: clearing.selection,
            },
            clearing.published_frame,
            clearing.published_cost,
        )
    }

    pub(crate) fn revision(&self) -> u64 {
        self.state.borrow().revision
    }

    pub(crate) fn active_selection(
        &self,
    ) -> Option<(UiVisualOverlayIdentity, UiVisualOverlaySelection)> {
        self.state.borrow().entry.as_ref().and_then(|entry| {
            matches!(
                entry.posture,
                UiVisualOverlayPosture::Publishing | UiVisualOverlayPosture::Published(_)
            )
            .then(|| (entry.identity, entry.selection.clone()))
        })
    }

    pub(crate) fn shutdown(&self) -> UiVisualOverlayShutdownReport {
        let mut state = self.state.borrow_mut();
        state.closed = true;
        let report = match state.entry.as_ref().map(|entry| entry.posture) {
            Some(UiVisualOverlayPosture::Pending | UiVisualOverlayPosture::Publishing) => {
                UiVisualOverlayShutdownReport {
                    cancelled_pending_count: 1,
                    ..Default::default()
                }
            }
            Some(UiVisualOverlayPosture::Published(_)) => UiVisualOverlayShutdownReport {
                disposed_published_count: 1,
                ..Default::default()
            },
            Some(UiVisualOverlayPosture::Clearing(_)) => UiVisualOverlayShutdownReport {
                disposed_clearing_count: 1,
                ..Default::default()
            },
            None => UiVisualOverlayShutdownReport::default(),
        };
        state.entry = None;
        report
    }
}

impl UiPendingVisualOverlayRegistration {
    fn commit(mut self) {
        self.active = false;
    }
}

impl Drop for UiPendingVisualOverlayRegistration {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let Some(state) = self.state.upgrade() else {
            return;
        };
        let mut state = state.borrow_mut();
        if state
            .entry
            .as_ref()
            .is_some_and(|entry| entry.identity == self.identity)
        {
            state.entry = None;
        }
    }
}

impl UiVisualOverlayShutdownReport {
    pub const fn cancelled_pending_count(self) -> usize {
        self.cancelled_pending_count
    }

    pub const fn disposed_published_count(self) -> usize {
        self.disposed_published_count
    }

    pub const fn disposed_clearing_count(self) -> usize {
        self.disposed_clearing_count
    }
}

fn matching_entry(
    state: &mut UiVisualOverlayRegistryState,
    identity: UiVisualOverlayIdentity,
) -> Result<&mut UiVisualOverlayEntry, worth_ui_inspection::UiVisualOverlayDenial> {
    state
        .entry
        .as_mut()
        .filter(|entry| entry.identity == identity)
        .ok_or(worth_ui_inspection::UiVisualOverlayDenial::Superseded)
}

fn next_revision(revision: u64) -> Result<u64, worth_ui_inspection::UiVisualOverlayDenial> {
    revision
        .checked_add(1)
        .ok_or(worth_ui_inspection::UiVisualOverlayDenial::CapacityExceeded)
}
