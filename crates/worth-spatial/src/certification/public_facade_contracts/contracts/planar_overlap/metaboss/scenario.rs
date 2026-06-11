#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StormTransform {
    Identity,
    Translated,
    HalfTurn,
    MoveThenRotate,
    RotateThenMove,
}

impl StormTransform {
    pub(crate) const fn canonical_motion(self) -> &'static str {
        match self {
            Self::Identity => "motion:coplanar-storm-canonical",
            Self::Translated => "motion:coplanar-storm-canonical",
            Self::HalfTurn => "motion:coplanar-storm-canonical",
            Self::MoveThenRotate => "motion:coplanar-storm-canonical",
            Self::RotateThenMove => "motion:coplanar-storm-canonical",
        }
    }

    pub(crate) const fn semantic_label(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Translated => "translated",
            Self::HalfTurn => "half-turn",
            Self::MoveThenRotate => "move-then-rotate",
            Self::RotateThenMove => "rotate-then-move",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RegionShape {
    PartialFlush,
    NestedHole,
    BoundaryTouch,
    CollinearRun,
}

#[derive(Clone, Debug)]
pub(crate) struct StormRegion {
    pub(crate) region_index: usize,
    pub(crate) pair_index: usize,
    pub(crate) shape: RegionShape,
    pub(crate) first_face: Vec<[f64; 2]>,
    pub(crate) second_face: Vec<[f64; 2]>,
    pub(crate) containment_candidate: Option<Vec<[f64; 2]>>,
}

impl StormRegion {
    pub(crate) fn region_identity(&self) -> String {
        format!("region:{}:pair:{}", self.region_index, self.pair_index)
    }

    pub(crate) fn first_face_identity(&self, transform: StormTransform) -> String {
        format!(
            "face:{}:{}:a",
            transform.semantic_label(),
            self.region_identity()
        )
    }

    pub(crate) fn second_face_identity(&self, transform: StormTransform) -> String {
        format!(
            "face:{}:{}:b",
            transform.semantic_label(),
            self.region_identity()
        )
    }
}

pub(crate) fn coplanar_storm_regions() -> Vec<StormRegion> {
    let mut regions = Vec::new();
    for region_index in 0..12 {
        for pair_index in 0..9 {
            let shape = match (region_index + pair_index) % 4 {
                0 => RegionShape::PartialFlush,
                1 => RegionShape::NestedHole,
                2 => RegionShape::BoundaryTouch,
                _ => RegionShape::CollinearRun,
            };
            regions.push(region(region_index, pair_index, shape));
        }
    }
    regions
}

pub(crate) fn coplanar_equivalence_regions() -> Vec<StormRegion> {
    vec![
        region(0, 0, RegionShape::PartialFlush),
        region(1, 0, RegionShape::NestedHole),
        region(2, 0, RegionShape::BoundaryTouch),
        region(3, 0, RegionShape::CollinearRun),
    ]
}

pub(crate) fn near_graze_region() -> StormRegion {
    region(99, 0, RegionShape::PartialFlush)
}

fn region(region_index: usize, pair_index: usize, shape: RegionShape) -> StormRegion {
    let x = region_index as f64 * 1.0e-6;
    let y = pair_index as f64 * 1.0e-6;
    match shape {
        RegionShape::PartialFlush => partial_flush(region_index, pair_index, x, y),
        RegionShape::NestedHole => nested_hole(region_index, pair_index, x, y),
        RegionShape::BoundaryTouch => boundary_touch(region_index, pair_index, x, y),
        RegionShape::CollinearRun => collinear_run(region_index, pair_index, x, y),
    }
}

fn partial_flush(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        shape: RegionShape::PartialFlush,
        first_face: rectangle(x, y, x + 4.0e-9, y + 4.0e-9),
        second_face: rectangle(x + 4.0e-9, y + 1.0e-9, x + 7.0e-9, y + 3.0e-9),
        containment_candidate: None,
    }
}

fn nested_hole(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        shape: RegionShape::NestedHole,
        first_face: rectangle(x, y, x + 5.0e-9, y + 5.0e-9),
        second_face: rectangle(x + 5.0e-9, y + 1.0e-9, x + 8.0e-9, y + 4.0e-9),
        containment_candidate: Some(rectangle(x + 1.0e-9, y + 1.0e-9, x + 2.0e-9, y + 2.0e-9)),
    }
}

fn boundary_touch(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        shape: RegionShape::BoundaryTouch,
        first_face: rectangle(x, y, x + 3.0e-9, y + 3.0e-9),
        second_face: rectangle(x + 3.0e-9, y + 3.0e-9, x + 6.0e-9, y + 6.0e-9),
        containment_candidate: None,
    }
}

fn collinear_run(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        shape: RegionShape::CollinearRun,
        first_face: rectangle(x, y, x + 8.0e-9, y + 8.0e-9),
        second_face: rectangle(x + 8.0e-9, y + 2.0e-9, x + 11.0e-9, y + 6.0e-9),
        containment_candidate: None,
    }
}

fn rectangle(left: f64, bottom: f64, right: f64, top: f64) -> Vec<[f64; 2]> {
    vec![[left, bottom], [right, bottom], [right, top], [left, top]]
}
