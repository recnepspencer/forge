use super::{AspectFrontDoorConstructionDenial, AspectVocabularyFrontDoor};
use crate::{CanonicalFieldPath, FieldKey};

impl AspectVocabularyFrontDoor {
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
        let canonical_fields: Vec<_> = fields
            .into_iter()
            .map(|raw_field| {
                let raw_field = raw_field.into();
                FieldKey::new(raw_field.clone()).ok_or(
                    AspectFrontDoorConstructionDenial::InvalidFieldKey(raw_field),
                )
            })
            .collect::<Result<_, _>>()?;

        match canonical_fields.as_slice() {
            [] => Err(AspectFrontDoorConstructionDenial::EmptyFieldPath),
            [field] => Ok(CanonicalFieldPath::single(field.clone())),
            _ => Err(AspectFrontDoorConstructionDenial::FieldPathMustTargetSingleField),
        }
    }
}
