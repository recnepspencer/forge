use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanSharedAreaAdmissionInput<'a> {
    boundary_contact_classification: &'a PlanarBooleanBoundaryContactClassificationBundle,
    containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
    winding_field: &'a PlanarBooleanOverlapCellWindingField,
}

impl<'a> PlanarBooleanSharedAreaAdmissionInput<'a> {
    pub fn new(
        boundary_contact_classification: &'a PlanarBooleanBoundaryContactClassificationBundle,
        containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
        winding_field: &'a PlanarBooleanOverlapCellWindingField,
    ) -> Self {
        Self {
            boundary_contact_classification,
            containment_map,
            winding_field,
        }
    }

    pub fn from_boundary_contact_classification(
        boundary_contact_classification: &'a PlanarBooleanBoundaryContactClassificationBundle,
        containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
        winding_field: &'a PlanarBooleanOverlapCellWindingField,
    ) -> Self {
        Self::new(boundary_contact_classification, containment_map, winding_field)
    }

    pub fn boundary_contact_classification(
        self,
    ) -> &'a PlanarBooleanBoundaryContactClassificationBundle {
        self.boundary_contact_classification
    }

    pub fn containment_map(self) -> &'a PlanarBooleanOverlapCellContainmentMap {
        self.containment_map
    }

    pub fn winding_field(self) -> &'a PlanarBooleanOverlapCellWindingField {
        self.winding_field
    }
}
