use forge_foundational::facade::{CanonicalFieldPath, FieldKey};
use forge_query::facade::ForgeQueryRetainedFieldPath;

fn main() {
    let _ = ForgeQueryRetainedFieldPath::from_authoring_path("title.value");

    let field_path = ForgeQueryRetainedFieldPath::from_canonical_field_path(
        CanonicalFieldPath::single(FieldKey::new("title".to_string()).unwrap()),
    );
    let _ = field_path.terminal_projection();
}
