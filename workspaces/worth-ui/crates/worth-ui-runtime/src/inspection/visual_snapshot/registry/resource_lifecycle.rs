use std::cell::Cell;
use std::rc::Rc;

use super::{
    UiVisualCaptureRegistrationLease, UiVisualRetainedResource, UiVisualRetainedResourceUsage,
    UiVisualSnapshotResourceLease,
};

impl UiVisualCaptureRegistrationLease {
    pub(crate) fn complete(
        mut self,
        snapshot_identity: u64,
        usage: UiVisualRetainedResourceUsage,
    ) -> UiVisualSnapshotResourceLease {
        let validity = Rc::new(Cell::new(true));
        if let Some(state) = self.state.upgrade() {
            let mut state = state.borrow_mut();
            let registration = state
                .active
                .remove(&self.surface)
                .expect("registered capture completion owns its active entry");
            assert_eq!(registration.capture_identity, self.capture_identity);
            assert!(
                usage.fits(registration.reservation),
                "retained resource usage cannot exceed the admitted capture reservation"
            );
            state.reserved_pixel_bytes = state
                .reserved_pixel_bytes
                .checked_sub(registration.reservation.pixel_bytes)
                .expect("active registration bytes are reserved exactly once");
            state.reserved_structural_bytes = state
                .reserved_structural_bytes
                .checked_sub(registration.reservation.structural_bytes)
                .expect("active registration structure is reserved exactly once");
            state.retained_pixel_bytes = state
                .retained_pixel_bytes
                .checked_add(usage.pixel_bytes)
                .expect("admitted capture reservation bounds retained pixels");
            state.retained_structural_bytes = state
                .retained_structural_bytes
                .checked_add(usage.structural_bytes)
                .expect("admitted capture reservation bounds retained structure");
            assert!(
                state
                    .retained
                    .insert(
                        snapshot_identity,
                        UiVisualRetainedResource {
                            usage,
                            validity: Rc::clone(&validity),
                        },
                    )
                    .is_none(),
                "owner-minted snapshot identities are unique"
            );
        }
        self.active = false;
        UiVisualSnapshotResourceLease {
            state: self.state.clone(),
            snapshot_identity,
            validity,
            active: true,
        }
    }
}

impl Drop for UiVisualCaptureRegistrationLease {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let Some(state) = self.state.upgrade() else {
            return;
        };
        let mut state = state.borrow_mut();
        let Some(registration) = state.active.remove(&self.surface) else {
            return;
        };
        if registration.capture_identity == self.capture_identity {
            state.reserved_pixel_bytes = state
                .reserved_pixel_bytes
                .checked_sub(registration.reservation.pixel_bytes)
                .expect("active registration bytes are reserved exactly once");
            state.reserved_structural_bytes = state
                .reserved_structural_bytes
                .checked_sub(registration.reservation.structural_bytes)
                .expect("active registration structure is reserved exactly once");
        }
    }
}

impl UiVisualSnapshotResourceLease {
    pub(crate) fn pixel_validity(&self) -> worth_ui_inspection::UiVisualPixelArtifactValidity {
        worth_ui_inspection::UiVisualPixelArtifactValidity::issued_by_runtime(Rc::clone(
            &self.validity,
        ))
    }

    pub(crate) fn dispose(mut self) -> bool {
        let released = self.release();
        self.active = false;
        released
    }

    pub(crate) fn replace(
        mut self,
        snapshot_identity: u64,
        usage: UiVisualRetainedResourceUsage,
    ) -> Self {
        self.validity.set(false);
        let validity = Rc::new(Cell::new(false));
        let mut active = false;
        if let Some(state) = self.state.upgrade() {
            let mut state = state.borrow_mut();
            let prior = state
                .retained
                .remove(&self.snapshot_identity)
                .expect("a derived successor consumes its registered parent");
            state.retained_pixel_bytes = replace_retained_total(
                state.retained_pixel_bytes,
                prior.usage.pixel_bytes,
                usage.pixel_bytes,
                state.maximum_retained_pixel_bytes,
            );
            state.retained_structural_bytes = replace_retained_total(
                state.retained_structural_bytes,
                prior.usage.structural_bytes,
                usage.structural_bytes,
                state.maximum_retained_structural_bytes,
            );
            assert!(
                state
                    .retained
                    .insert(
                        snapshot_identity,
                        UiVisualRetainedResource {
                            usage,
                            validity: Rc::clone(&validity),
                        },
                    )
                    .is_none(),
                "owner-minted successor snapshot identities are unique"
            );
            validity.set(true);
            active = true;
        }
        self.active = false;
        Self {
            state: self.state.clone(),
            snapshot_identity,
            validity,
            active,
        }
    }

    fn release(&mut self) -> bool {
        self.validity.set(false);
        let Some(state) = self.state.upgrade() else {
            return false;
        };
        let mut state = state.borrow_mut();
        let Some(resource) = state.retained.remove(&self.snapshot_identity) else {
            return false;
        };
        state.retained_pixel_bytes = state
            .retained_pixel_bytes
            .checked_sub(resource.usage.pixel_bytes)
            .expect("retained resource bytes are released exactly once");
        state.retained_structural_bytes = state
            .retained_structural_bytes
            .checked_sub(resource.usage.structural_bytes)
            .expect("retained resource structure is released exactly once");
        true
    }
}

fn replace_retained_total(total: u64, prior: u64, successor: u64, maximum: u64) -> u64 {
    let next = total
        .checked_sub(prior)
        .and_then(|remaining| remaining.checked_add(successor))
        .expect("a retained successor replaces one exactly accounted parent");
    assert!(
        next <= maximum,
        "a derived successor must remain inside the session reservation"
    );
    next
}

impl Drop for UiVisualSnapshotResourceLease {
    fn drop(&mut self) {
        if self.active {
            let _ = self.release();
        }
    }
}
