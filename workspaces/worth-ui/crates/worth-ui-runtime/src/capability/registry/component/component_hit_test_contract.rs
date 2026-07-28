use super::{ComponentAllocationMeasurementContract, ComponentHitTestOrder};

/// Complete component-owned meaning for one allocation-bounded hit region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentHitTestContract {
    order: ComponentHitTestOrder,
    allocation: ComponentAllocationMeasurementContract,
}

impl ComponentHitTestContract {
    pub const fn allocation_bounds(
        order: ComponentHitTestOrder,
        allocation: ComponentAllocationMeasurementContract,
    ) -> Self {
        Self { order, allocation }
    }

    pub const fn order(self) -> ComponentHitTestOrder {
        self.order
    }

    pub const fn allocation(self) -> ComponentAllocationMeasurementContract {
        self.allocation
    }

    pub(crate) fn digest_basis(self) -> String {
        format!(
            "allocation-bounds:{}:{}",
            self.order.rank(),
            self.allocation.digest_basis()
        )
    }
}
