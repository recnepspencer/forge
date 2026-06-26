use forge_foundational::facade::{AspectKey, FieldKey};
use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadProjectionField, ForgeQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadProjectionField {
        aspect: AspectKey::new("identity").unwrap(),
        field: FieldKey::new("id").unwrap(),
        delivered_name: "id".to_string(),
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
