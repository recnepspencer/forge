use std::cell::RefCell;
use std::rc::Rc;

use super::authority::{
    UiMountedFrameRetentionAuthority, UiMountedRetainedFrameState,
    UiMountedRetentionReservationIdentity,
};
use super::successor_admission::UiMountedSuccessorRetentionAdmission;

pub(crate) struct UiRetentionPreparedMountedFrame {
    frame: super::super::UiPreparedMountedFrame,
    reservation: UiMountedRetentionReservation,
}

pub(crate) struct UiMountedRetentionReservation {
    successor: UiMountedRetainedFrameState,
    expected_revision: u64,
    successor_revision: u64,
    structural_bytes: usize,
    identity: UiMountedRetentionReservationIdentity,
    authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
    release_on_drop: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedRetentionCommitDenial {
    RevisionChanged,
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
        identity: UiMountedRetentionReservationIdentity,
        authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
    ) -> Self {
        let structural_bytes = admission.structural_bytes();
        let (successor, expected_revision, successor_revision) = admission.into_parts();
        Self {
            successor,
            expected_revision,
            successor_revision,
            structural_bytes,
            identity,
            authority,
            release_on_drop: true,
        }
    }

    pub(crate) fn commit(
        mut self,
        mount_cost: super::super::UiMountCostReport,
        presentation: super::super::UiMountedPresentationReceipt,
    ) -> Result<(), UiMountedRetentionCommitDenial> {
        let mut authority = self.authority.borrow_mut();
        if authority.revision != self.expected_revision {
            drop(authority);
            return Err(UiMountedRetentionCommitDenial::RevisionChanged);
        }
        if let Some(current) = self.successor.current.as_mut() {
            let current = Rc::make_mut(current);
            current.set_mount_cost(mount_cost);
            current.set_presentation_receipt(presentation);
        }
        authority.frames = std::mem::take(&mut self.successor);
        authority.revision = self.successor_revision;
        release_reservation(&mut authority, self.identity, self.structural_bytes);
        self.release_on_drop = false;
        Ok(())
    }

    pub(crate) const fn identity(&self) -> UiMountedRetentionReservationIdentity {
        self.identity
    }
}

impl Drop for UiMountedRetentionReservation {
    fn drop(&mut self) {
        if self.release_on_drop {
            let mut authority = self.authority.borrow_mut();
            release_reservation(&mut authority, self.identity, self.structural_bytes);
        }
    }
}

fn release_reservation(
    authority: &mut UiMountedFrameRetentionAuthority,
    identity: UiMountedRetentionReservationIdentity,
    structural_bytes: usize,
) {
    let removed = authority
        .reservations
        .remove(&identity)
        .expect("retention authority includes the released reservation");
    assert_eq!(removed, structural_bytes, "reservation bytes remain exact");
    authority.in_flight_structural_bytes = authority
        .in_flight_structural_bytes
        .checked_sub(structural_bytes)
        .expect("retention reservation bytes include the released reservation");
}
