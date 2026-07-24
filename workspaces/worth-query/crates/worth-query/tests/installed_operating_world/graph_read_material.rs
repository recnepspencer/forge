use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};
use worth_query::facade::domain::{WorthQueryGraphReadMaterial, WorthQueryGraphReadRow};

pub(super) fn graph_read_material(label: &str) -> WorthQueryGraphReadMaterial {
    let identity_path = CanonicalFieldPath::single(FieldKey::new("id").unwrap());
    let fields = BTreeMap::from([(
        identity_path,
        AspectValue::String(InternedString::from(label)),
    )]);
    let row = WorthQueryGraphReadRow::from_native_fields(label, fields).unwrap();
    WorthQueryGraphReadMaterial::new([row])
}
