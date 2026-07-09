use worth_foundational::facade::{AspectKey, FieldKey};
use worth_query::facade::AuthorizedProjectionFieldPath;

fn main() {
    let _ = AuthorizedProjectionFieldPath::from_terminal_projection("profile.display_name");
    let field_path = AuthorizedProjectionFieldPath::from_native_keys(
        AspectKey::new("profile".to_string()).unwrap(),
        FieldKey::new("display_name".to_string()).unwrap(),
    );
    let _ = field_path.terminal_projection();
}
