use serde_json::Value;

use super::super::JsonCompatibilityLoweringDenial;
use crate::aspects::{FieldKey, StructAspectShape, StructAspectValue};
use crate::compatibility::scalar_lowering::lower_json_scalar;
use crate::locators::BoundarySourceLocator;

use super::source_loci::field_source;

pub(super) fn lower_json_struct(
    source: &BoundarySourceLocator,
    value: &Value,
    shape: &StructAspectShape,
) -> Result<StructAspectValue, JsonCompatibilityLoweringDenial> {
    let Value::Object(object) = value else {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "JSON object",
        });
    };

    let mut fields = Vec::new();
    for (field_name, field_value) in object {
        let Some(field_key) = FieldKey::new(field_name.clone()) else {
            return Err(JsonCompatibilityLoweringDenial::InvalidFieldKey {
                source: source.clone(),
                field: field_name.clone(),
            });
        };
        let field_source = field_source(source, &field_key);
        let Some(field) = shape.field(&field_key) else {
            return Err(JsonCompatibilityLoweringDenial::UnknownStructField {
                source: field_source,
                field: field_key,
            });
        };
        fields.push((
            field_key,
            lower_json_scalar(&field_source, field_value, field.value_type())?,
        ));
    }

    StructAspectValue::new(fields).map_err(|denial| {
        JsonCompatibilityLoweringDenial::StructConstructionDenied {
            source: source.clone(),
            denial,
        }
    })
}
