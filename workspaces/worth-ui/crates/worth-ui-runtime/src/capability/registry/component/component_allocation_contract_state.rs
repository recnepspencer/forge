use super::ComponentAllocationMeasurementContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ComponentAllocationContractState {
    resolved: Option<ComponentAllocationMeasurementContract>,
    conflicting: bool,
}

impl ComponentAllocationContractState {
    pub(crate) const fn empty() -> Self {
        Self {
            resolved: None,
            conflicting: false,
        }
    }

    pub(crate) fn record(mut self, contract: ComponentAllocationMeasurementContract) -> Self {
        match self.resolved {
            Some(resolved) if resolved != contract => self.conflicting = true,
            Some(_) => {}
            None => self.resolved = Some(contract),
        }
        self
    }

    pub(crate) const fn resolved(self) -> Option<ComponentAllocationMeasurementContract> {
        self.resolved
    }

    pub(crate) const fn is_conflicting(self) -> bool {
        self.conflicting
    }
}
