use super::super::{
    AspectContractRevision, AspectIdentity, AspectKey, CanonicalFieldPath, StructAspectValue,
};
use super::vocabulary::AspectFrontDoorConstructionDenial;
use crate::aspects::FieldKey;
use crate::values::AspectValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectVocabularyFrontDoor;

impl AspectVocabularyFrontDoor {
    pub fn key(
        self,
        raw: impl Into<String>,
    ) -> Result<AspectKey, AspectFrontDoorConstructionDenial> {
        let raw = raw.into();
        AspectKey::new(raw.clone()).ok_or(AspectFrontDoorConstructionDenial::InvalidAspectKey(raw))
    }

    pub const fn identity(self, raw: u64) -> AspectIdentity {
        AspectIdentity(raw)
    }

    pub const fn revision(self, raw: u64) -> AspectContractRevision {
        AspectContractRevision(raw)
    }

    pub fn field_key(
        self,
        raw: impl Into<String>,
    ) -> Result<FieldKey, AspectFrontDoorConstructionDenial> {
        let raw = raw.into();
        FieldKey::new(raw.clone()).ok_or(AspectFrontDoorConstructionDenial::InvalidFieldKey(raw))
    }

    pub fn field_path(
        self,
        fields: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<CanonicalFieldPath, AspectFrontDoorConstructionDenial> {
        let mut canonical_fields = Vec::new();
        for raw_field in fields {
            let raw_field = raw_field.into();
            canonical_fields.push(FieldKey::new(raw_field.clone()).ok_or(
                AspectFrontDoorConstructionDenial::InvalidFieldKey(raw_field),
            )?);
        }

        CanonicalFieldPath::new(canonical_fields)
            .ok_or(AspectFrontDoorConstructionDenial::EmptyFieldPath)
    }

    pub fn struct_value(self) -> StructValueBuilder {
        StructValueBuilder::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct StructValueBuilder {
    fields: Vec<(String, AspectValue)>,
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
            crate::aspects::StructAspectValueConstructionDenial::DuplicateField(field) => {
                AspectFrontDoorConstructionDenial::DuplicateStructValueField(field.as_str().into())
            }
        })
    }
}
