use crate::aspects::contracts::contract::MaskModeAdmission;
use crate::aspects::contracts::AspectShape;
use crate::aspects::masks::MaskAdmissibilityDenial;
use crate::aspects::structs::CanonicalFieldPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectMaskContract {
    projection: bool,
    mutation: bool,
    diagnostic: bool,
}

impl AspectMaskContract {
    pub const fn scalar() -> Self {
        Self {
            projection: true,
            mutation: true,
            diagnostic: true,
        }
    }

    pub const fn struct_fields() -> Self {
        Self {
            projection: true,
            mutation: true,
            diagnostic: true,
        }
    }

    pub const fn opaque_diagnostic_only() -> Self {
        Self {
            projection: false,
            mutation: false,
            diagnostic: true,
        }
    }

    pub const fn projection_allowed(&self) -> bool {
        self.projection
    }

    pub const fn mutation_allowed(&self) -> bool {
        self.mutation
    }

    pub const fn diagnostic_allowed(&self) -> bool {
        self.diagnostic
    }

    pub(crate) fn admit_paths_for_shape(
        &self,
        paths: &[CanonicalFieldPath],
        shape: &AspectShape,
        mode: MaskModeAdmission,
    ) -> Result<(), MaskAdmissibilityDenial> {
        if !self.mode_is_allowed(mode) {
            return Err(MaskAdmissibilityDenial::ModeNotAllowed);
        }

        match shape {
            AspectShape::Struct(shape) => {
                for path in paths {
                    if path.fields().len() != 1 || shape.field(&path.fields()[0]).is_none() {
                        return Err(MaskAdmissibilityDenial::UnknownField);
                    }
                }
                Ok(())
            }
            _ if paths.is_empty() => Ok(()),
            _ => Err(MaskAdmissibilityDenial::FieldMaskRequiresStruct),
        }
    }

    fn mode_is_allowed(&self, mode: MaskModeAdmission) -> bool {
        match mode {
            MaskModeAdmission::Projection => self.projection,
            MaskModeAdmission::Mutation => self.mutation,
            MaskModeAdmission::Diagnostic => self.diagnostic,
        }
    }
}
