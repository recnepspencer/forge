use worth_foundational::AspectValue;
use worth_store::physical_runtime::PhysicalWorkSemanticBasis;

fn accept_basis(_basis: PhysicalWorkSemanticBasis) {}

fn main() {
    accept_basis(AspectValue::Null);

    let signal_aspect: worth_signal::facade::Aspect = todo!();
    accept_basis(signal_aspect);

    accept_basis(serde_json::json!({"branch": "caller-branch"}));
}
