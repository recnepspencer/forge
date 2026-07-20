#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerPresentationPosture {
    Headless,
    Interactive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerAllocationPosture {
    Borrowed,
    Owned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerBoundaryRequirements {
    pub presentation: WorthQueryConsumerPresentationPosture,
    pub allocation: WorthQueryConsumerAllocationPosture,
}

pub struct WorthQueryConsumerBoundary<D, O, F, L: BasisOperationLane> {
    query: super::WorthQueryConsumerProjectionContract<D, O, F, L>,
    downstream: WorthQueryConsumerBoundaryRequirements,
}

impl<D, O, F, L: BasisOperationLane> super::WorthQueryConsumerProjectionContract<D, O, F, L> {
    pub fn with_downstream_requirements(
        self,
        downstream: WorthQueryConsumerBoundaryRequirements,
    ) -> WorthQueryConsumerBoundary<D, O, F, L> {
        WorthQueryConsumerBoundary {
            query: self,
            downstream,
        }
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryConsumerBoundary<D, O, F, L> {
    pub fn query_contract(&self) -> &super::WorthQueryConsumerProjectionContract<D, O, F, L> {
        &self.query
    }

    pub fn downstream_requirements(&self) -> WorthQueryConsumerBoundaryRequirements {
        self.downstream
    }

    pub fn into_query_contract(self) -> super::WorthQueryConsumerProjectionContract<D, O, F, L> {
        self.query
    }
}
use crate::basis_lifecycle::BasisOperationLane;
