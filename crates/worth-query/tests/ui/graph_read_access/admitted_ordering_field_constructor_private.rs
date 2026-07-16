use worth_foundational::facade::{AspectKey, FieldKey};
use worth_query::facade::runtime::WorthQueryAdmittedGraphReadOrderingField;

#[allow(unreachable_code)]
fn main() {
    let _ = WorthQueryAdmittedGraphReadOrderingField {
        aspect: AspectKey::new("profile").unwrap(),
        field: FieldKey::new("display_name").unwrap(),
        direction: "ascending".to_string(),
        kind: panic!("compile-fail fixture must not construct admitted evidence"),
    };
}
