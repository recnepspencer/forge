#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLoopBoundaryGeometry {
    owning_face_identity: String,
    outer_points: Vec<[f64; 2]>,
    outer_segments: Vec<PlanarLoopBoundarySegmentGeometry>,
    containment_candidate_points: Option<Vec<[f64; 2]>>,
}

impl PlanarLoopBoundaryGeometry {
    pub(crate) fn new(
        owning_face_identity: impl Into<String>,
        outer_points: Vec<[f64; 2]>,
        outer_segments: Vec<PlanarLoopBoundarySegmentGeometry>,
        containment_candidate_points: Option<Vec<[f64; 2]>>,
    ) -> Self {
        Self {
            owning_face_identity: owning_face_identity.into(),
            outer_points,
            outer_segments,
            containment_candidate_points,
        }
    }

    pub fn owning_face_identity(&self) -> &str {
        &self.owning_face_identity
    }

    pub fn outer_points(&self) -> &[[f64; 2]] {
        &self.outer_points
    }

    pub fn outer_segments(&self) -> &[PlanarLoopBoundarySegmentGeometry] {
        &self.outer_segments
    }

    pub fn containment_candidate_points(&self) -> Option<&[[f64; 2]]> {
        self.containment_candidate_points.as_deref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLoopBoundarySegmentGeometry {
    source_edge_identity: String,
    start_point: [f64; 2],
    end_point: [f64; 2],
}

impl PlanarLoopBoundarySegmentGeometry {
    fn new(
        source_edge_identity: impl Into<String>,
        start_point: [f64; 2],
        end_point: [f64; 2],
    ) -> Self {
        Self {
            source_edge_identity: source_edge_identity.into(),
            start_point,
            end_point,
        }
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn start_point(&self) -> [f64; 2] {
        self.start_point
    }

    pub fn end_point(&self) -> [f64; 2] {
        self.end_point
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarLoopBoundaryCatalogProfile {
    Default,
    BooleanEventMetabossLeft,
    BooleanEventMetabossRight,
    BooleanBoundaryOnlyLeft,
    BooleanBoundaryOnlyRight,
    BooleanMixedBoundaryAreaLeft,
    BooleanMixedBoundaryAreaRight,
}

pub(crate) fn catalog_loop_boundary_geometry_for_profile(
    profile: PlanarLoopBoundaryCatalogProfile,
    index: usize,
    owning_face_identity: String,
    edge_identities: &[String],
) -> PlanarLoopBoundaryGeometry {
    if index == 0 {
        match profile {
            PlanarLoopBoundaryCatalogProfile::BooleanEventMetabossLeft => {
                return profiled_boundary_geometry(
                    owning_face_identity,
                    metaboss_left_outer_points(),
                    edge_identities,
                );
            }
            PlanarLoopBoundaryCatalogProfile::BooleanEventMetabossRight => {
                return profiled_boundary_geometry(
                    owning_face_identity,
                    metaboss_right_outer_points(),
                    edge_identities,
                );
            }
            PlanarLoopBoundaryCatalogProfile::BooleanBoundaryOnlyLeft => {
                return profiled_boundary_geometry(
                    owning_face_identity,
                    boundary_only_left_outer_points(),
                    edge_identities,
                );
            }
            PlanarLoopBoundaryCatalogProfile::BooleanBoundaryOnlyRight => {
                return profiled_boundary_geometry(
                    owning_face_identity,
                    boundary_only_right_outer_points(),
                    edge_identities,
                );
            }
            PlanarLoopBoundaryCatalogProfile::BooleanMixedBoundaryAreaLeft => {
                return profiled_boundary_geometry(
                    owning_face_identity,
                    mixed_boundary_area_left_outer_points(),
                    edge_identities,
                );
            }
            PlanarLoopBoundaryCatalogProfile::BooleanMixedBoundaryAreaRight => {
                return profiled_boundary_geometry(
                    owning_face_identity,
                    mixed_boundary_area_right_outer_points(),
                    edge_identities,
                );
            }
            PlanarLoopBoundaryCatalogProfile::Default => {}
        }
    }
    default_catalog_loop_boundary_geometry(index, owning_face_identity, edge_identities)
}

fn default_catalog_loop_boundary_geometry(
    index: usize,
    owning_face_identity: String,
    edge_identities: &[String],
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
    let outer_segments = outer_boundary_segments(index, &outer, edge_identities);
    PlanarLoopBoundaryGeometry::new(owning_face_identity, outer, outer_segments, candidate)
}

fn profiled_boundary_geometry(
    owning_face_identity: String,
    outer: Vec<[f64; 2]>,
    edge_identities: &[String],
) -> PlanarLoopBoundaryGeometry {
    let outer_segments = outer_boundary_segments(0, &outer, edge_identities);
    PlanarLoopBoundaryGeometry::new(owning_face_identity, outer, outer_segments, None)
}

fn metaboss_left_outer_points() -> Vec<[f64; 2]> {
    metaboss_target_segments()
        .into_iter()
        .flat_map(|target| [target.left_start, target.left_end])
        .collect()
}

fn metaboss_right_outer_points() -> Vec<[f64; 2]> {
    metaboss_target_segments()
        .into_iter()
        .flat_map(|target| [target.right_start, target.right_end])
        .collect()
}

fn boundary_only_left_outer_points() -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]
}

fn boundary_only_right_outer_points() -> Vec<[f64; 2]> {
    vec![[10.0, 2.0], [20.0, 2.0], [20.0, 8.0], [10.0, 8.0]]
}

fn mixed_boundary_area_left_outer_points() -> Vec<[f64; 2]> {
    vec![
        [0.0, 0.0],
        [10.0, 0.0],
        [10.0, 3.0],
        [10.0, 7.0],
        [10.0, 10.0],
        [0.0, 10.0],
    ]
}

fn mixed_boundary_area_right_outer_points() -> Vec<[f64; 2]> {
    vec![
        [10.0, 0.0],
        [18.0, 0.0],
        [18.0, 10.0],
        [10.0, 10.0],
        [10.0, 7.0],
        [8.0, 7.0],
        [8.0, 3.0],
        [10.0, 3.0],
    ]
}

fn metaboss_target_segments() -> Vec<MetabossTargetSegment> {
    vec![
        MetabossTargetSegment::new([0.0, 0.0], [10.0, 10.0], [0.0, 10.0], [10.0, 0.0]),
        MetabossTargetSegment::new([1000.0, 0.0], [1000.0, 10.0], [995.0, 0.0], [1005.0, 0.0]),
        MetabossTargetSegment::new([2000.0, 0.0], [2010.0, 0.0], [2000.0, 0.0], [2000.0, 10.0]),
        MetabossTargetSegment::new([3000.0, 0.0], [3010.0, 0.0], [3020.0, 0.0], [3030.0, 0.0]),
        MetabossTargetSegment::new([4000.0, 0.0], [4010.0, 0.0], [4010.0, 0.0], [4020.0, 0.0]),
        MetabossTargetSegment::new([5000.0, 0.0], [5020.0, 0.0], [5010.0, 0.0], [5030.0, 0.0]),
        MetabossTargetSegment::new([6000.0, 0.0], [6030.0, 0.0], [6010.0, 0.0], [6020.0, 0.0]),
        MetabossTargetSegment::new([7000.0, 0.0], [7010.0, 0.0], [7000.0, 0.0], [7010.0, 0.0]),
        MetabossTargetSegment::new([8000.0, 0.0], [8010.0, 0.0], [8010.0, 0.0], [8000.0, 0.0]),
        MetabossTargetSegment::new([9000.0, 0.0], [9020.0, 0.0], [9000.0, 0.0], [9000.0, 20.0]),
        MetabossTargetSegment::new(
            [10010.0, 10.0],
            [10000.0, 0.0],
            [10010.0, 0.0],
            [10000.0, 10.0],
        ),
        MetabossTargetSegment::new(
            [11000.0, 0.0],
            [11010.0, 0.0],
            [11005.0, 0.0],
            [11005.0, 10.0],
        ),
    ]
}

struct MetabossTargetSegment {
    left_start: [f64; 2],
    left_end: [f64; 2],
    right_start: [f64; 2],
    right_end: [f64; 2],
}

impl MetabossTargetSegment {
    fn new(
        left_start: [f64; 2],
        left_end: [f64; 2],
        right_start: [f64; 2],
        right_end: [f64; 2],
    ) -> Self {
        Self {
            left_start,
            left_end,
            right_start,
            right_end,
        }
    }
}

fn rectangle(left: f64, bottom: f64, right: f64, top: f64) -> Vec<[f64; 2]> {
    vec![[left, bottom], [right, bottom], [right, top], [left, top]]
}

fn outer_boundary_segments(
    loop_index: usize,
    points: &[[f64; 2]],
    edge_identities: &[String],
) -> Vec<PlanarLoopBoundarySegmentGeometry> {
    if points.len() < 2 || edge_identities.is_empty() {
        return Vec::new();
    }

    let edge_offset = loop_index * points.len();
    (0..points.len())
        .map(|segment_index| {
            let edge_index = (edge_offset + segment_index) % edge_identities.len();
            PlanarLoopBoundarySegmentGeometry::new(
                edge_identities[edge_index].clone(),
                points[segment_index],
                points[(segment_index + 1) % points.len()],
            )
        })
        .collect()
}
