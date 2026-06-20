use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadPredicateField, ForgeQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadPredicateField {
        aspect: "status".to_string(),
        field: "value".to_string(),
        family: "equality".to_string(),
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
