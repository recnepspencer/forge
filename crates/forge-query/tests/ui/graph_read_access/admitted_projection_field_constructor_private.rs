use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadProjectionField, ForgeQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadProjectionField {
        aspect: "identity".to_string(),
        field: "id".to_string(),
        delivered_name: "id".to_string(),
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
