use forge_foundational::{AspectContractRevision, AspectKey, ContractValidatedAspectValue};

fn main() {
    let _forged = ContractValidatedAspectValue {
        key: AspectKey::new("count").unwrap(),
        contract_revision: AspectContractRevision(1),
    };
}
