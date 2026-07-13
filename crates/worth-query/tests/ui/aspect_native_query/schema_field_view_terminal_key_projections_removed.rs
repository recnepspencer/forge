use worth_query::facade::runtime::{SchemaFieldKind, SchemaFieldView};

fn main() {
    let field = SchemaFieldView::new(worth_query::facade::foundation::AspectName::new("identity").expect("schema aspect literal must be valid"), worth_query::facade::foundation::FieldName::new("id").expect("schema field literal must be valid"), SchemaFieldKind::String);
    let _ = field.terminal_aspect_projection();
    let _ = field.terminal_field_projection();
}
