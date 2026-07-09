use super::{AspectFrontDoorConstructionDenial, AspectVocabularyFrontDoor};
use crate::{AspectValue, FieldKey, StructAspectValue, StructAspectValueConstructionDenial};

#[derive(Debug, Clone, Default)]
pub struct StructValueBuilder {
    fields: Vec<(String, AspectValue)>,
}

impl AspectVocabularyFrontDoor {
    pub fn struct_value(self) -> StructValueBuilder {
        StructValueBuilder::default()
    }
}

impl StructValueBuilder {
    pub fn with_field(mut self, key: impl Into<String>, value: AspectValue) -> Self {
        self.fields.push((key.into(), value));
        self
    }

    pub fn finish(self) -> Result<StructAspectValue, AspectFrontDoorConstructionDenial> {
        let mut fields = Vec::with_capacity(self.fields.len());
        for (raw_key, value) in self.fields {
            let field_key = FieldKey::new(raw_key.clone()).ok_or(
                AspectFrontDoorConstructionDenial::InvalidFieldKey(raw_key.clone()),
            )?;
            fields.push((field_key, value));
        }

        StructAspectValue::new(fields).map_err(|denial| match denial {
            StructAspectValueConstructionDenial::DuplicateField(field) => {
                AspectFrontDoorConstructionDenial::DuplicateStructValueField(field.as_str().into())
            }
        })
    }
}
