use worth_foundational::facade::AspectValue;
use worth_query::facade::WorthQueryDerivedPatchPayload;

fn main() {
    let _ = WorthQueryDerivedPatchPayload::from_scalar_value(AspectValue::Null);
}
