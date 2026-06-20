use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadOrderingField, ForgeQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadOrderingField {
        aspect: "profile".to_string(),
        field: "display_name".to_string(),
        direction: "ascending".to_string(),
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
