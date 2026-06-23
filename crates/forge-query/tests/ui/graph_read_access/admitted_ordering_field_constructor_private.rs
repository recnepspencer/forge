use forge_foundational::facade::{AspectKey, FieldKey};
use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadOrderingField, ForgeQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadOrderingField {
        aspect: AspectKey::new("profile").unwrap(),
        field: FieldKey::new("display_name").unwrap(),
        direction: "ascending".to_string(),
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
