use super::admitted_input::SpatialCompiledProductFamilyAdmittedInput;
use super::catalog::SpatialCompiledProductFamilyCatalog;
use super::compiled_product::{
    lower_spatial_compiled_product_identity, SpatialCompiledProductLoweredIdentity,
};
use super::declaration::SpatialCompiledProductFamilyDeclaration;
use super::error::{SpatialCompiledProductFamilyError, SpatialCompiledProductFamilyErrorKind};

#[derive(Debug, Clone)]
pub struct SelectedSpatialCompiledProductFamily {
    declaration: SpatialCompiledProductFamilyDeclaration,
    admitted_input: SpatialCompiledProductFamilyAdmittedInput,
}

impl SelectedSpatialCompiledProductFamily {
    pub fn declaration(&self) -> &SpatialCompiledProductFamilyDeclaration {
        &self.declaration
    }

    pub fn admitted_input(&self) -> &SpatialCompiledProductFamilyAdmittedInput {
        &self.admitted_input
    }

    pub fn compile_product_identity(
        &self,
    ) -> Result<SpatialCompiledProductLoweredIdentity, SpatialCompiledProductFamilyError> {
        lower_spatial_compiled_product_identity(&self.declaration, &self.admitted_input)
    }
}

pub fn select_spatial_compiled_product_family(
    catalog: &SpatialCompiledProductFamilyCatalog,
    admitted_input: SpatialCompiledProductFamilyAdmittedInput,
) -> Result<SelectedSpatialCompiledProductFamily, SpatialCompiledProductFamilyError> {
    let declaration = catalog
        .family(admitted_input.family_identity())
        .ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::NoDeclaredFamilyForConsumer,
                "admitted spatial compiled-product family input referenced a missing declaration",
            )
        })?;
    if !declaration.supports(admitted_input.consumer()) {
        return Err(SpatialCompiledProductFamilyError::new(
            SpatialCompiledProductFamilyErrorKind::NoDeclaredFamilyForConsumer,
            "admitted spatial compiled-product family input referenced an undeclared consumer",
        ));
    }
    Ok(SelectedSpatialCompiledProductFamily {
        declaration: declaration.clone(),
        admitted_input,
    })
}
