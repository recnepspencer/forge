use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

use worth_ui_host_contract::UiHostSurfaceIdentity;

mod resource_lifecycle;

#[derive(Clone)]
pub(crate) struct UiVisualCaptureRegistry {
    state: Rc<RefCell<UiVisualCaptureRegistryState>>,
}

pub(crate) struct UiVisualCaptureRegistrationLease {
    state: Weak<RefCell<UiVisualCaptureRegistryState>>,
    capture_identity: u64,
    surface: UiHostSurfaceIdentity,
    active: bool,
}

pub(crate) struct UiVisualSnapshotResourceLease {
    state: Weak<RefCell<UiVisualCaptureRegistryState>>,
    snapshot_identity: u64,
    validity: Rc<Cell<bool>>,
    active: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiVisualResourceReservation {
    pixel_bytes: u64,
    structural_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiVisualRetainedResourceUsage {
    pixel_bytes: u64,
    structural_bytes: u64,
}

struct UiVisualCaptureRegistryState {
    closed: bool,
    maximum_snapshot_count: usize,
    maximum_retained_pixel_bytes: u64,
    maximum_retained_structural_bytes: u64,
    active: BTreeMap<UiHostSurfaceIdentity, UiVisualCaptureRegistration>,
    retained: BTreeMap<u64, UiVisualRetainedResource>,
    reserved_pixel_bytes: u64,
    reserved_structural_bytes: u64,
    retained_pixel_bytes: u64,
    retained_structural_bytes: u64,
}

#[derive(Clone, Copy)]
struct UiVisualCaptureRegistration {
    capture_identity: u64,
    reservation: UiVisualResourceReservation,
}

struct UiVisualRetainedResource {
    usage: UiVisualRetainedResourceUsage,
    validity: Rc<Cell<bool>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiVisualCaptureRegistrationDenial {
    RegistryClosed,
    SnapshotCapacityExceeded,
    SurfaceCaptureInFlight,
    PixelRetentionCapacityExceeded,
    StructuralRetentionCapacityExceeded,
    AccountingOverflow,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiVisualCaptureShutdownReport {
    cancelled_capture_count: usize,
    disposed_snapshot_count: usize,
    disposed_pixel_bytes: u64,
    disposed_structural_bytes: u64,
}

impl UiVisualCaptureRegistry {
    pub(crate) fn new(policy: worth_ui_inspection::UiVisualInspectionPolicy) -> Self {
        Self {
            state: Rc::new(RefCell::new(UiVisualCaptureRegistryState {
                closed: false,
                maximum_snapshot_count: usize::from(policy.maximum_snapshot_count()),
                maximum_retained_pixel_bytes: policy.maximum_retained_pixel_bytes(),
                maximum_retained_structural_bytes: policy
                    .maximum_retained_structural_bytes_per_session(),
                active: BTreeMap::new(),
                retained: BTreeMap::new(),
                reserved_pixel_bytes: 0,
                reserved_structural_bytes: 0,
                retained_pixel_bytes: 0,
                retained_structural_bytes: 0,
            })),
        }
    }

    pub(crate) fn register(
        &self,
        capture_identity: u64,
        surface: UiHostSurfaceIdentity,
        reservation: UiVisualResourceReservation,
    ) -> Result<UiVisualCaptureRegistrationLease, UiVisualCaptureRegistrationDenial> {
        let mut state = self.state.borrow_mut();
        let admitted = state.admit(surface, reservation)?;
        state.active.insert(
            surface,
            UiVisualCaptureRegistration {
                capture_identity,
                reservation,
            },
        );
        state.reserved_pixel_bytes = admitted.pixel_bytes;
        state.reserved_structural_bytes = admitted.structural_bytes;
        Ok(UiVisualCaptureRegistrationLease {
            state: Rc::downgrade(&self.state),
            capture_identity,
            surface,
            active: true,
        })
    }

    pub(crate) fn shutdown(&self) -> UiVisualCaptureShutdownReport {
        let mut state = self.state.borrow_mut();
        state.closed = true;
        let report = UiVisualCaptureShutdownReport {
            cancelled_capture_count: state.active.len(),
            disposed_snapshot_count: state.retained.len(),
            disposed_pixel_bytes: state.retained_pixel_bytes,
            disposed_structural_bytes: state.retained_structural_bytes,
        };
        for retained in state.retained.values() {
            retained.validity.set(false);
        }
        state.active.clear();
        state.retained.clear();
        state.reserved_pixel_bytes = 0;
        state.reserved_structural_bytes = 0;
        state.retained_pixel_bytes = 0;
        state.retained_structural_bytes = 0;
        report
    }
}

impl UiVisualCaptureRegistryState {
    fn admit(
        &self,
        surface: UiHostSurfaceIdentity,
        reservation: UiVisualResourceReservation,
    ) -> Result<UiVisualResourceReservation, UiVisualCaptureRegistrationDenial> {
        if self.closed {
            return Err(UiVisualCaptureRegistrationDenial::RegistryClosed);
        }
        if self.active.contains_key(&surface) {
            return Err(UiVisualCaptureRegistrationDenial::SurfaceCaptureInFlight);
        }
        let resource_count = self
            .active
            .len()
            .checked_add(self.retained.len())
            .ok_or(UiVisualCaptureRegistrationDenial::AccountingOverflow)?;
        if resource_count >= self.maximum_snapshot_count {
            return Err(UiVisualCaptureRegistrationDenial::SnapshotCapacityExceeded);
        }
        let next_reserved_pixel_bytes = self
            .reserved_pixel_bytes
            .checked_add(reservation.pixel_bytes)
            .ok_or(UiVisualCaptureRegistrationDenial::AccountingOverflow)?;
        let total = next_reserved_pixel_bytes
            .checked_add(self.retained_pixel_bytes)
            .ok_or(UiVisualCaptureRegistrationDenial::AccountingOverflow)?;
        if total > self.maximum_retained_pixel_bytes {
            return Err(UiVisualCaptureRegistrationDenial::PixelRetentionCapacityExceeded);
        }
        let next_reserved_structural_bytes = self
            .reserved_structural_bytes
            .checked_add(reservation.structural_bytes)
            .ok_or(UiVisualCaptureRegistrationDenial::AccountingOverflow)?;
        let total = next_reserved_structural_bytes
            .checked_add(self.retained_structural_bytes)
            .ok_or(UiVisualCaptureRegistrationDenial::AccountingOverflow)?;
        if total > self.maximum_retained_structural_bytes {
            return Err(UiVisualCaptureRegistrationDenial::StructuralRetentionCapacityExceeded);
        }
        Ok(UiVisualResourceReservation::new(
            next_reserved_pixel_bytes,
            next_reserved_structural_bytes,
        ))
    }
}

impl UiVisualCaptureShutdownReport {
    pub const fn cancelled_capture_count(self) -> usize {
        self.cancelled_capture_count
    }

    pub const fn disposed_snapshot_count(self) -> usize {
        self.disposed_snapshot_count
    }

    pub const fn disposed_pixel_bytes(self) -> u64 {
        self.disposed_pixel_bytes
    }

    pub const fn disposed_structural_bytes(self) -> u64 {
        self.disposed_structural_bytes
    }
}

impl UiVisualResourceReservation {
    pub(crate) const fn new(pixel_bytes: u64, structural_bytes: u64) -> Self {
        Self {
            pixel_bytes,
            structural_bytes,
        }
    }
}

impl UiVisualRetainedResourceUsage {
    pub(crate) const fn new(pixel_bytes: u64, structural_bytes: u64) -> Self {
        Self {
            pixel_bytes,
            structural_bytes,
        }
    }

    const fn fits(self, reservation: UiVisualResourceReservation) -> bool {
        self.pixel_bytes <= reservation.pixel_bytes
            && self.structural_bytes <= reservation.structural_bytes
    }
}
