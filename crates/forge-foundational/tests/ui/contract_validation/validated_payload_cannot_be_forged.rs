use forge_foundational::{
    AspectContractRevision, AspectKey, AspectValue, ContractValidatedAspectValue,
};

fn main() {
    let _forged = ContractValidatedAspectValue::scalar(
        AspectKey::new("count").unwrap(),
        AspectValue::Int64(1),
        AspectContractRevision(1),
    );
}
