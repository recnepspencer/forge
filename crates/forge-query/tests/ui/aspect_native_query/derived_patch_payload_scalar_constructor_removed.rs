use forge_foundational::facade::AspectValue;
use forge_query::facade::ForgeQueryDerivedPatchPayload;

fn main() {
    let _ = ForgeQueryDerivedPatchPayload::from_scalar_value(AspectValue::Null);
}
