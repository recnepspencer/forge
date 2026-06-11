#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLoopBoundaryGeometry {
    owning_face_identity: String,
    outer_points: Vec<[f64; 2]>,
    containment_candidate_points: Option<Vec<[f64; 2]>>,
}

impl PlanarLoopBoundaryGeometry {
    pub(crate) fn new(
        owning_face_identity: impl Into<String>,
        outer_points: Vec<[f64; 2]>,
        containment_candidate_points: Option<Vec<[f64; 2]>>,
    ) -> Self {
        Self {
            owning_face_identity: owning_face_identity.into(),
            outer_points,
            containment_candidate_points,
        }
    }

    pub fn owning_face_identity(&self) -> &str {
        &self.owning_face_identity
    }

    pub fn outer_points(&self) -> &[[f64; 2]] {
        &self.outer_points
    }

    pub fn containment_candidate_points(&self) -> Option<&[[f64; 2]]> {
        self.containment_candidate_points.as_deref()
    }
}

pub(crate) fn catalog_loop_boundary_geometry(
    index: usize,
    owning_face_identity: String,
) -> PlanarLoopBoundaryGeometry {
    let pair_index = index / 2;
    let first_in_pair = index % 2 == 0;
    let x = pair_index as f64 * 1.0e-6;
    let y = (pair_index % 16) as f64 * 1.0e-6;
    let shape = pair_index % 4;
    let (outer, candidate) = match (first_in_pair, shape) {
        (true, 0) => (rectangle(x, y, x + 4.0e-9, y + 4.0e-9), None),
        (false, 0) => (
            rectangle(x + 4.0e-9, y + 1.0e-9, x + 7.0e-9, y + 3.0e-9),
            None,
        ),
        (true, 1) => (rectangle(x, y, x + 4.0e-9, y + 4.0e-9), None),
        (false, 1) => (
            rectangle(x + 1.0e-9, y + 1.0e-9, x + 5.0e-9, y + 5.0e-9),
            None,
        ),
        (true, 2) => (
            rectangle(x, y, x + 5.0e-9, y + 5.0e-9),
            Some(rectangle(x + 1.0e-9, y + 1.0e-9, x + 2.0e-9, y + 2.0e-9)),
        ),
        (false, 2) => (
            rectangle(x + 5.0e-9, y + 1.0e-9, x + 8.0e-9, y + 4.0e-9),
            None,
        ),
        (true, _) => (
            rectangle(x, y, x + 5.0e-9, y + 5.0e-9),
            Some(rectangle(x + 20.0e-9, y, x + 22.0e-9, y + 2.0e-9)),
        ),
        (false, _) => (
            rectangle(x + 5.0e-9, y + 1.0e-9, x + 8.0e-9, y + 4.0e-9),
            None,
        ),
    };
    PlanarLoopBoundaryGeometry::new(owning_face_identity, outer, candidate)
}

fn rectangle(left: f64, bottom: f64, right: f64, top: f64) -> Vec<[f64; 2]> {
    vec![[left, bottom], [right, bottom], [right, top], [left, top]]
}
