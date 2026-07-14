use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{WorthQueryRetainedFieldPath, WorthQueryRetainedMaterializedRow};

fn main() {
    let field_path = WorthQueryRetainedFieldPath::from_canonical_field_path(
        CanonicalFieldPath::single(FieldKey::new("title".to_string()).unwrap()),
    );
    let _ = WorthQueryRetainedMaterializedRow::from_scalar_values(BTreeMap::from([(
        field_path,
        AspectValue::String("direct".to_string().into()),
    )]));
}
