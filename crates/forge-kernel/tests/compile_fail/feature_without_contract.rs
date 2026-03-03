/// This test verifies that implementing `Feature` without `FeatureContract`
/// is a compile error. The `Feature` trait has `FeatureContract` as a
/// supertrait — this is the sealed supertrait guarantee.
use forge_kernel::engine::facade::{Feature, SolidEnvelope, FeatureInputs};
use forge_kernel::context::scope::OperationScope;
use forge_core::KernelError;
use forge_signal::facade::NodeId;
use std::collections::HashMap;

#[derive(Debug)]
struct NakedFeature;

struct EmptyInputs;
impl FeatureInputs for EmptyInputs {
    fn validate(&self) -> Result<(), KernelError> { Ok(()) }
}

// This should fail because NakedFeature does NOT implement FeatureContract.
impl Feature for NakedFeature {
    type Inputs = EmptyInputs;
    fn parse_inputs(&self, _: &HashMap<NodeId, SolidEnvelope>) -> Result<EmptyInputs, KernelError> {
        Ok(EmptyInputs)
    }
    fn execute_typed(&self, _: &EmptyInputs, _: &mut OperationScope<'_>) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, KernelError> {
        unimplemented!()
    }
    fn dependencies(&self) -> Vec<NodeId> { vec![] }
    fn name(&self) -> &str { "naked" }
}

fn main() {}
