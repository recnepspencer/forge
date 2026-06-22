use forge_query::facade::{SchemaFieldKind, SchemaFieldView};

fn main() {
    let field = SchemaFieldView::new(forge_query::facade::AspectName::new("identity").expect("schema aspect literal must be valid"), forge_query::facade::FieldName::new("id").expect("schema field literal must be valid"), SchemaFieldKind::String);
    let _ = field.terminal_aspect_projection();
    let _ = field.terminal_field_projection();
}
