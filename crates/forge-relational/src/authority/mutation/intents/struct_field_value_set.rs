use forge_foundational::facade::{
    aspects, AspectFrontDoorConstructionDenial, AspectMask, AspectValue, CanonicalFieldPath,
    FieldKey, MutationMask, StructAspectValue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructFieldValueSet {
    fields: Vec<StructFieldValue>,
}

impl StructFieldValueSet {
    pub(super) fn push(&mut self, field: FieldKey, value: AspectValue) {
        self.fields.push(StructFieldValue { field, value });
    }

    pub(super) fn mutation_mask(&self) -> AspectMask<MutationMask> {
        AspectMask::<MutationMask>::new(
            self.fields
                .iter()
                .map(|field_value| CanonicalFieldPath::single(field_value.field.clone())),
        )
    }

    pub(super) fn into_struct_value(
        self,
    ) -> Result<StructAspectValue, AspectFrontDoorConstructionDenial> {
        let mut builder = aspects().vocabulary().struct_value();
        for field_value in self.fields {
            builder = builder.with_field(field_value.field.as_str(), field_value.value);
        }
        builder.finish()
    }

    pub(super) fn into_field_values(self) -> impl Iterator<Item = (FieldKey, AspectValue)> {
        self.fields
            .into_iter()
            .map(|field_value| (field_value.field, field_value.value))
    }
}

impl Default for StructFieldValueSet {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructFieldValue {
    field: FieldKey,
    value: AspectValue,
}
