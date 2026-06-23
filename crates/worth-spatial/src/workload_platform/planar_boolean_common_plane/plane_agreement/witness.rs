#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneWitness {
    plane_identity_digest: String,
    supporting_face_rows: usize,
}

impl PlanarBooleanCommonPlaneWitness {
    pub(crate) fn new(
        plane_identity_digest: impl Into<String>,
        supporting_face_rows: usize,
    ) -> Self {
        Self {
            plane_identity_digest: plane_identity_digest.into(),
            supporting_face_rows,
        }
    }

    pub fn plane_identity_digest(&self) -> &str {
        &self.plane_identity_digest
    }

    pub fn supporting_face_rows(&self) -> usize {
        self.supporting_face_rows
    }
}
