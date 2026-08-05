use worth_foundational::facade::{AspectFieldLocator, AspectMask, FieldKey, ProjectionMask};

use super::super::contract::WorthQueryAdmittedApplicationDisclosureField;

pub(in crate::domain_computation) struct WorthQueryApplicationInternalProjectionAdmission<'a> {
    field: &'a FieldKey,
    projection_mask: Option<&'a AspectMask<ProjectionMask>>,
}

pub(in crate::domain_computation) struct WorthQueryApplicationDisclosedProjectionAdmission;

impl<'a> WorthQueryApplicationInternalProjectionAdmission<'a> {
    pub(super) const fn public(field: &'a FieldKey) -> Self {
        Self {
            field,
            projection_mask: None,
        }
    }

    pub(super) fn governed(
        admitted: &'a WorthQueryAdmittedApplicationDisclosureField,
        field: &'a FieldKey,
    ) -> Option<Self> {
        let projection_mask = admitted.projection_mask();
        projection_mask_admits(projection_mask, field).then_some(Self {
            field,
            projection_mask: Some(projection_mask),
        })
    }

    pub(in crate::domain_computation) fn projection_fields(&self) -> Vec<FieldKey> {
        match self.projection_mask {
            None => vec![self.field.clone()],
            Some(mask) => mask
                .paths()
                .iter()
                .flat_map(|path| path.fields())
                .filter(|field| *field == self.field)
                .cloned()
                .collect(),
        }
    }

    pub(in crate::domain_computation) const fn field_key(&self) -> &FieldKey {
        self.field
    }

    pub(in crate::domain_computation) fn admits_locator(
        &self,
        locator: &AspectFieldLocator,
    ) -> bool {
        locator.field_path().fields() == std::slice::from_ref(self.field)
    }
}

impl WorthQueryApplicationDisclosedProjectionAdmission {
    pub(super) const fn public() -> Self {
        Self
    }

    pub(super) fn governed(
        admitted: &WorthQueryAdmittedApplicationDisclosureField,
        field: &FieldKey,
    ) -> Option<Self> {
        projection_mask_admits(admitted.projection_mask(), field).then_some(Self)
    }
}

fn projection_mask_admits(mask: &AspectMask<ProjectionMask>, field: &FieldKey) -> bool {
    mask.paths()
        .iter()
        .any(|path| path.fields() == std::slice::from_ref(field))
}
