use forge_foundational::facade::{AspectKey, FieldKey};
use forge_query::facade::{
    AuthorizedProjectionFieldPath, PolicyPlaceholderMaskingDenial,
    PolicyPlaceholderMaskingRequest,
};

fn main() {
    let _ = PolicyPlaceholderMaskingRequest::terminal_requested_placeholder_fields(vec![
        "secret.salary".to_string(),
    ]);

    let request = PolicyPlaceholderMaskingRequest::from_authorized_field_paths(vec![
        authorized_field("secret", "salary"),
    ]);
    let _ = request.terminal_requested_placeholder_fields_projection();

    let denial = denial_fixture();
    let _ = denial.terminal_requested_placeholder_fields_projection();
}

fn authorized_field(aspect: &str, field: &str) -> AuthorizedProjectionFieldPath {
    AuthorizedProjectionFieldPath::from_native_keys(
        AspectKey::new(aspect).unwrap(),
        FieldKey::new(field).unwrap(),
    )
}

fn denial_fixture() -> PolicyPlaceholderMaskingDenial {
    panic!("fixture only")
}
