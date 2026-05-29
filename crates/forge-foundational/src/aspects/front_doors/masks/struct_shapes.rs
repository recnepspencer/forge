use super::super::vocabulary::AspectFrontDoorConstructionDenial;
use crate::aspects::{AbsenceLaw, AspectEvolutionPolicy};
use crate::values::ScalarAspectType;
use crate::{FieldDeclaration, FieldKey, FieldRequirement, StructAspectShape};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StructFieldsFrontDoor;

impl StructFieldsFrontDoor {
    pub fn required(
        self,
        key: impl Into<String>,
        value_type: ScalarAspectType,
    ) -> StructFieldBuilder {
        StructFieldBuilder::default().required(key, value_type)
    }

    pub fn optional(
        self,
        key: impl Into<String>,
        value_type: ScalarAspectType,
    ) -> StructFieldBuilder {
        StructFieldBuilder::default().optional(key, value_type)
    }

    pub fn defaulted(
        self,
        key: impl Into<String>,
        value_type: ScalarAspectType,
    ) -> StructFieldBuilder {
        StructFieldBuilder::default().defaulted(key, value_type)
    }
}

#[derive(Debug, Clone, Default)]
pub struct StructFieldBuilder {
    declarations: Vec<(String, ScalarAspectType, FieldRequirement)>,
}

impl StructFieldBuilder {
    pub fn required(mut self, key: impl Into<String>, value_type: ScalarAspectType) -> Self {
        self.declarations
            .push((key.into(), value_type, FieldRequirement::Required));
        self
    }

    pub fn optional(mut self, key: impl Into<String>, value_type: ScalarAspectType) -> Self {
        self.declarations
            .push((key.into(), value_type, FieldRequirement::Optional));
        self
    }

    pub fn defaulted(mut self, key: impl Into<String>, value_type: ScalarAspectType) -> Self {
        self.declarations
            .push((key.into(), value_type, FieldRequirement::Defaulted));
        self
    }

    pub fn finish(self) -> Result<StructAspectShape, AspectFrontDoorConstructionDenial> {
        if self.declarations.is_empty() {
            return Err(AspectFrontDoorConstructionDenial::EmptyStructShape);
        }

        let mut declarations = Vec::with_capacity(self.declarations.len());
        for (raw_key, value_type, requirement) in self.declarations {
            let field_key = FieldKey::new(raw_key.clone()).ok_or(
                AspectFrontDoorConstructionDenial::InvalidFieldKey(raw_key.clone()),
            )?;
            let absence = match requirement {
                FieldRequirement::Required => AbsenceLaw::Required,
                FieldRequirement::Optional => AbsenceLaw::Optional,
                FieldRequirement::Defaulted => AbsenceLaw::Defaulted,
            };
            let declaration = FieldDeclaration::new(
                field_key,
                value_type,
                requirement,
                absence,
                AspectEvolutionPolicy::AdditiveFieldsAllowed,
            )
            .ok_or(AspectFrontDoorConstructionDenial::DuplicateFieldDeclaration(raw_key.clone()))?;
            declarations.push(declaration);
        }

        StructAspectShape::new(declarations).ok_or(
            AspectFrontDoorConstructionDenial::DuplicateFieldDeclaration(
                "duplicate field declaration".into(),
            ),
        )
    }
}
