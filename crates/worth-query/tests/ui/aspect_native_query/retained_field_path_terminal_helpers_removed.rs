use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::WorthQueryRetainedFieldPath;

fn main() {
    let _ = WorthQueryRetainedFieldPath::from_authoring_path("title.value");

    let field_path = WorthQueryRetainedFieldPath::from_canonical_field_path(
        CanonicalFieldPath::single(FieldKey::new("title".to_string()).unwrap()),
    );
    let _ = field_path.terminal_projection();
}
