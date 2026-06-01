use super::super::vocabulary::AspectFrontDoorConstructionDenial;
use crate::{
    AspectMask, CanonicalFieldPath, DiagnosticMask, FieldKey, MutationMask, ProjectionMask,
};

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
        build_single_field_mask(keys)
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
        build_single_field_mask(keys)
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
        build_single_field_mask(keys)
    }
}

fn build_single_field_mask<Mode>(
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
