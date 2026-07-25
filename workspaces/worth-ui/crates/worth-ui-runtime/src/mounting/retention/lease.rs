use std::cell::RefCell;
use std::rc::{Rc, Weak};

use worth_ui_host_contract::UiMountedFrameIdentity;

use super::coordinator::UiMountedFrameRetentionAuthority;
use super::UiMountedRetentionClass;

pub struct UiMountedRetentionLease {
    pin: UiMountedRetentionPin,
}

#[derive(Clone)]
pub(crate) struct UiMountedObservationBasisLease {
    pin: UiMountedRetentionPin,
}

#[derive(Clone)]
struct UiMountedRetentionPin {
    release: Rc<UiMountedRetentionPinRelease>,
}

struct UiMountedRetentionPinRelease {
    authority: Weak<RefCell<UiMountedFrameRetentionAuthority>>,
    frame: UiMountedFrameIdentity,
    class: UiMountedRetentionClass,
    structural_bytes: usize,
}

impl UiMountedRetentionLease {
    pub(crate) fn new(
        authority: &Rc<RefCell<UiMountedFrameRetentionAuthority>>,
        frame: UiMountedFrameIdentity,
        structural_bytes: usize,
    ) -> Self {
        Self {
            pin: UiMountedRetentionPin::new(
                authority,
                frame,
                UiMountedRetentionClass::PredecessorInspection,
                structural_bytes,
            ),
        }
    }

    pub fn frame(&self) -> UiMountedFrameIdentity {
        self.pin.release.frame
    }

    pub fn class(&self) -> UiMountedRetentionClass {
        self.pin.release.class
    }
}

impl UiMountedObservationBasisLease {
    pub(crate) fn new(
        authority: &Rc<RefCell<UiMountedFrameRetentionAuthority>>,
        frame: UiMountedFrameIdentity,
        structural_bytes: usize,
    ) -> Self {
        Self {
            pin: UiMountedRetentionPin::new(
                authority,
                frame,
                UiMountedRetentionClass::ObservationBasis,
                structural_bytes,
            ),
        }
    }
}

impl UiMountedRetentionPin {
    fn new(
        authority: &Rc<RefCell<UiMountedFrameRetentionAuthority>>,
        frame: UiMountedFrameIdentity,
        class: UiMountedRetentionClass,
        structural_bytes: usize,
    ) -> Self {
        Self {
            release: Rc::new(UiMountedRetentionPinRelease {
                authority: Rc::downgrade(authority),
                frame,
                class,
                structural_bytes,
            }),
        }
    }
}

impl Drop for UiMountedRetentionPinRelease {
    fn drop(&mut self) {
        let Some(authority) = self.authority.upgrade() else {
            return;
        };
        authority
            .borrow_mut()
            .release_pin(self.frame, self.class, self.structural_bytes);
    }
}
