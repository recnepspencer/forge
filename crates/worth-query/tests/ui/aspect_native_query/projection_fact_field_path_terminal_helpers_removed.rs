use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::foundation::ProjectionFactFieldPath;

fn main() {
    let _ = ProjectionFactFieldPath::from_terminal_projection("profile.display_name");
    let field_path = ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("profile".to_string()).unwrap(),
            FieldKey::new("display_name".to_string()).unwrap(),
        ])
        .unwrap(),
    );
    let _ = field_path.terminal_projection();
}
