pub use crate::corruption::S8LayoutReadmissionWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadmissionFacade;

impl LayoutReadmissionFacade {
    pub const fn boundary(&self) -> S8LayoutReadmissionWitness {
        let lowered = crate::execution::S8LoweredAccessPlan::readmission_boundary();
        S8LayoutReadmissionWitness::new(lowered.path_kind(), lowered.planned())
    }
}

pub const fn layout_readmission() -> LayoutReadmissionFacade {
    LayoutReadmissionFacade
}
