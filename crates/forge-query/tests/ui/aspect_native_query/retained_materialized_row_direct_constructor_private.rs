use std::collections::BTreeMap;

use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryRetainedFieldPath, ForgeQueryRetainedMaterializedRow};

fn main() {
    let field_path = ForgeQueryRetainedFieldPath::from_canonical_field_path(
        CanonicalFieldPath::single(FieldKey::new("title".to_string()).unwrap()),
    );
    let _ = ForgeQueryRetainedMaterializedRow::from_scalar_values(BTreeMap::from([(
        field_path,
        AspectValue::String("direct".into()),
    )]));
}
