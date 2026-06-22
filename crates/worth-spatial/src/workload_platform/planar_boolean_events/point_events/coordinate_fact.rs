use super::identity::coordinate_fact_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointEventCoordinateFact {
    point_2d: [f64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    coordinate_fact_identity: String,
}

impl PlanarBooleanPointEventCoordinateFact {
    pub(crate) fn new(
        point_2d: [f64; 2],
        local_frame_identity: &str,
        precision_basis_identity: &str,
    ) -> Self {
        Self {
            point_2d,
            local_frame_identity: local_frame_identity.to_string(),
            precision_basis_identity: precision_basis_identity.to_string(),
            coordinate_fact_identity: coordinate_fact_identity(
                point_2d,
                local_frame_identity,
                precision_basis_identity,
            ),
        }
    }

    pub fn point_2d(&self) -> [f64; 2] {
        self.point_2d
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn coordinate_fact_identity(&self) -> &str {
        &self.coordinate_fact_identity
    }
}
