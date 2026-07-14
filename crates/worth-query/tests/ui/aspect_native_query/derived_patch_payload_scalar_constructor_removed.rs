use worth_foundational::facade::AspectValue;
use worth_query::facade::runtime::WorthQueryDerivedPatchPayload;

fn main() {
    let _ = WorthQueryDerivedPatchPayload::from_scalar_value(AspectValue::Null);
}
