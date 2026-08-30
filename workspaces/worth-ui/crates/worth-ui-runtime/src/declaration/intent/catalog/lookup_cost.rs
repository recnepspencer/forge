#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentRouteResolutionCost {
    product_index_probes: u8,
    confirmation_index_probes: u8,
    command_index_probes: u8,
    route_rows_resolved: u8,
}

impl UiIntentRouteResolutionCost {
    pub(super) const fn product_route() -> Self {
        Self {
            product_index_probes: 1,
            confirmation_index_probes: 0,
            command_index_probes: 0,
            route_rows_resolved: 1,
        }
    }

    pub(super) const fn confirmation_route() -> Self {
        Self {
            product_index_probes: 1,
            confirmation_index_probes: 1,
            command_index_probes: 0,
            route_rows_resolved: 1,
        }
    }

    pub(super) fn command_route(candidates: usize) -> Self {
        Self {
            product_index_probes: 0,
            confirmation_index_probes: 0,
            command_index_probes: 1,
            route_rows_resolved: u8::try_from(candidates).unwrap_or(u8::MAX),
        }
    }

    pub const fn product_index_probes(self) -> usize {
        self.product_index_probes as usize
    }

    pub const fn confirmation_index_probes(self) -> usize {
        self.confirmation_index_probes as usize
    }

    pub const fn command_index_probes(self) -> usize {
        self.command_index_probes as usize
    }

    pub const fn total_index_probes(self) -> usize {
        self.product_index_probes as usize
            + self.confirmation_index_probes as usize
            + self.command_index_probes as usize
    }

    pub const fn route_rows_resolved(self) -> usize {
        self.route_rows_resolved as usize
    }
}
