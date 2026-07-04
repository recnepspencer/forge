use topology::facade::compiled_product_family::{
    TopologyCompiledProductFamilyAdmittedInput, TopologyCompiledProductLoweredIdentity,
};

fn main() {
    let _: TopologyCompiledProductFamilyAdmittedInput =
        serde_json::from_str("{}").expect("compile-fail fixture does not execute");
    let _: TopologyCompiledProductLoweredIdentity =
        serde_json::from_str("{}").expect("compile-fail fixture does not execute");
}
