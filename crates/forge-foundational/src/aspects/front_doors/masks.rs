use super::super::{
    AspectMask, AspectMaskContract, CanonicalFieldPath, DiagnosticMask, FieldDeclaration, FieldKey,
    FieldRequirement, MutationMask, ProjectionMask, StructAspectShape,
};
use super::vocabulary::AspectFrontDoorConstructionDenial;
use crate::aspects::{AbsenceLaw, AspectEvolutionPolicy};
use crate::values::ScalarAspectType;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectMaskContractFrontDoor;

impl AspectMaskContractFrontDoor {
    pub const fn scalar(self) -> AspectMaskContract {
        AspectMaskContract::scalar()
    }

    pub const fn struct_fields(self) -> AspectMaskContract {
        AspectMaskContract::struct_fields()
    }

    pub const fn opaque_diagnostic_only(self) -> AspectMaskContract {
        AspectMaskContract::opaque_diagnostic_only()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProjectionMaskFrontDoor;

impl ProjectionMaskFrontDoor {
    pub fn whole_aspect(self) -> AspectMask<ProjectionMask> {
        AspectMask::whole_aspect()
    }

    pub fn fields(
        self,
        keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<AspectMask<ProjectionMask>, AspectFrontDoorConstructionDenial> {
        build_field_mask(keys)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MutationMaskFrontDoor;

impl MutationMaskFrontDoor {
    pub fn whole_aspect(self) -> AspectMask<MutationMask> {
        AspectMask::whole_aspect()
    }

    pub fn fields(
        self,
        keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<AspectMask<MutationMask>, AspectFrontDoorConstructionDenial> {
        build_field_mask(keys)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DiagnosticMaskFrontDoor;

impl DiagnosticMaskFrontDoor {
    pub fn whole_aspect(self) -> AspectMask<DiagnosticMask> {
        AspectMask::whole_aspect()
    }

    pub fn fields(
        self,
        keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<AspectMask<DiagnosticMask>, AspectFrontDoorConstructionDenial> {
        build_field_mask(keys)
    }
}

fn build_field_mask<Mode>(
    keys: impl IntoIterator<Item = impl Into<String>>,
) -> Result<AspectMask<Mode>, AspectFrontDoorConstructionDenial> {
    let mut paths = Vec::new();
    for raw_key in keys {
        let raw_key = raw_key.into();
        let field_key = FieldKey::new(raw_key.clone()).ok_or(
            AspectFrontDoorConstructionDenial::InvalidFieldKey(raw_key.clone()),
        )?;
        paths.push(CanonicalFieldPath::single(field_key));
    }
    Ok(AspectMask::new(paths))
}
