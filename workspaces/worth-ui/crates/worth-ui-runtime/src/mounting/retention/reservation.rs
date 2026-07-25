use std::cell::RefCell;
use std::rc::Rc;

use super::authority::{UiMountedFrameRetentionAuthority, UiMountedRetainedFrameState};
use super::successor_admission::UiMountedSuccessorRetentionAdmission;

pub(crate) struct UiRetentionPreparedMountedFrame {
    frame: super::super::UiPreparedMountedFrame,
    reservation: UiMountedRetentionReservation,
}

pub(crate) struct UiMountedRetentionReservation {
    successor: UiMountedRetainedFrameState,
    expected_revision: u64,
    successor_revision: u64,
    authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
    release_on_drop: bool,
}

impl UiRetentionPreparedMountedFrame {
    pub(super) fn new(
        frame: super::super::UiPreparedMountedFrame,
        reservation: UiMountedRetentionReservation,
    ) -> Self {
        Self { frame, reservation }
    }

    pub(crate) fn frame(&self) -> &super::super::UiPreparedMountedFrame {
        &self.frame
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::super::UiPreparedMountedFrame,
        UiMountedRetentionReservation,
    ) {
        (self.frame, self.reservation)
    }
}

impl UiMountedRetentionReservation {
    pub(super) fn new(
        admission: UiMountedSuccessorRetentionAdmission,
        authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
    ) -> Self {
        let (successor, expected_revision, successor_revision) = admission.into_parts();
        Self {
            successor,
            expected_revision,
            successor_revision,
            authority,
            release_on_drop: true,
        }
    }

    pub(crate) fn commit(mut self, mount_cost: super::super::UiMountCostReport) {
        let mut authority = self.authority.borrow_mut();
        debug_assert_eq!(
            authority.revision, self.expected_revision,
            "retention authority cannot change while its presentation is in flight"
        );
        if let Some(current) = self.successor.current.as_mut() {
            Rc::make_mut(current).set_mount_cost(mount_cost);
        }
        authority.frames = std::mem::take(&mut self.successor);
        authority.revision = self.successor_revision;
        authority.reservation_active = false;
        authority.in_flight_structural_bytes = 0;
        self.release_on_drop = false;
    }
}

impl Drop for UiMountedRetentionReservation {
    fn drop(&mut self) {
        if self.release_on_drop {
            let mut authority = self.authority.borrow_mut();
            authority.reservation_active = false;
            authority.in_flight_structural_bytes = 0;
        }
    }
}
