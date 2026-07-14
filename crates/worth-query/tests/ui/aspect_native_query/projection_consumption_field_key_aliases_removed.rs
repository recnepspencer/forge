use worth_query::facade::foundation::{BoundProjectionFactFamily, ConsumedFieldValueFact, ProjectionFactFieldPath, ProjectionFactRequest};
use worth_foundational::facade::{CanonicalFieldPath, FieldKey};

fn main() {
    let request = ProjectionFactRequest::DisplayField(field_path());
    let _ = request.field_key();

    let family = bound_family_fixture();
    let _ = family.field_key();

    let fact = consumed_field_fixture();
    let _ = fact.field_key();
}

fn bound_family_fixture() -> BoundProjectionFactFamily {
    panic!("fixture only")
}

fn consumed_field_fixture() -> ConsumedFieldValueFact {
    panic!("fixture only")
}

fn field_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("profile".to_string()).unwrap(),
            FieldKey::new("display_name".to_string()).unwrap(),
        ])
        .unwrap(),
    )
}
