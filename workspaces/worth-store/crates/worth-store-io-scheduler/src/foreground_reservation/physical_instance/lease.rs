use std::sync::{Arc, Mutex};

use super::capacity::{release, PhysicalInstanceForegroundCapacityState};
use crate::foreground_reservation::{ForegroundReservationReceipt, ForegroundResourceBudget};

#[derive(Debug)]
pub struct PhysicalInstanceForegroundCapacityLease {
    state: Arc<Mutex<PhysicalInstanceForegroundCapacityState>>,
    reserved: ForegroundResourceBudget,
}

#[derive(Debug)]
pub struct PhysicalInstanceForegroundReservation {
    receipt: ForegroundReservationReceipt,
    capacity: PhysicalInstanceForegroundCapacityLease,
}

impl PhysicalInstanceForegroundCapacityLease {
    pub(super) fn new(
        state: Arc<Mutex<PhysicalInstanceForegroundCapacityState>>,
        reserved: ForegroundResourceBudget,
    ) -> Self {
        Self { state, reserved }
    }

    pub const fn reserved_budget(&self) -> ForegroundResourceBudget {
        self.reserved
    }
}

impl Drop for PhysicalInstanceForegroundCapacityLease {
    fn drop(&mut self) {
        release(&self.state, self.reserved);
    }
}

impl PhysicalInstanceForegroundReservation {
    pub(super) const fn new(
        receipt: ForegroundReservationReceipt,
        capacity: PhysicalInstanceForegroundCapacityLease,
    ) -> Self {
        Self { receipt, capacity }
    }

    pub const fn receipt(&self) -> ForegroundReservationReceipt {
        self.receipt
    }

    pub fn into_parts(
        self,
    ) -> (
        ForegroundReservationReceipt,
        PhysicalInstanceForegroundCapacityLease,
    ) {
        (self.receipt, self.capacity)
    }
}
