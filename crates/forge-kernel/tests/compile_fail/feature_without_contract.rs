/// This test verifies that implementing `Feature` without `FeatureContract`
/// is a compile error. The `Feature` trait has `FeatureContract` as a
/// supertrait — this is the sealed supertrait guarantee.
use forge_kernel::features::traits::{Feature, FeatureOutput};
use forge_kernel::features::pipeline::contract::FeatureInputs;
use forge_kernel::core::ModelingContext;
use forge_core::KernelError;
use forge_signal::handles::NodeId;
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
    fn parse_inputs(&self, _: &HashMap<NodeId, FeatureOutput>) -> Result<EmptyInputs, KernelError> {
        Ok(EmptyInputs)
    }
    fn execute_typed(&self, _: &EmptyInputs, _: &mut ModelingContext) -> Result<FeatureOutput, KernelError> {
        unimplemented!()
    }
    fn dependencies(&self) -> Vec<NodeId> { vec![] }
    fn name(&self) -> &str { "naked" }
}

fn main() {}
