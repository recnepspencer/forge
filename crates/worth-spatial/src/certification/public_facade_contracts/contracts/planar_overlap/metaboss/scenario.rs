#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegionShape {
    PartialFlush,
    CrossingAmbiguous,
    ContainedCandidate,
}

#[derive(Clone, Debug)]
pub(crate) struct StormRegion {
    pub(crate) region_index: usize,
    pub(crate) pair_index: usize,
    pub(crate) first_face: Vec<[f64; 2]>,
    pub(crate) second_face: Vec<[f64; 2]>,
    pub(crate) containment_candidate: Option<Vec<[f64; 2]>>,
}

impl StormRegion {
    pub(crate) fn region_identity(&self) -> String {
        format!("region:{}:pair:{}", self.region_index, self.pair_index)
    }

    pub(crate) fn first_face_identity(&self) -> String {
        format!("face:identity:{}:a", self.region_identity())
    }

    pub(crate) fn second_face_identity(&self) -> String {
        format!("face:identity:{}:b", self.region_identity())
    }
}

pub(crate) fn near_graze_region() -> StormRegion {
    region(99, 0, RegionShape::PartialFlush)
}

pub(crate) fn policy_required_region() -> StormRegion {
    let mut region = region(100, 0, RegionShape::PartialFlush);
    region.containment_candidate = Some(rectangle(20.0e-9, 20.0e-9, 22.0e-9, 22.0e-9));
    region
}

pub(crate) fn storm_regions() -> Vec<StormRegion> {
    (0..12)
        .flat_map(|region_index| {
            [
                region(region_index, 0, RegionShape::PartialFlush),
                region(region_index, 1, RegionShape::CrossingAmbiguous),
                region(region_index, 2, RegionShape::ContainedCandidate),
            ]
        })
        .collect()
}

fn region(region_index: usize, pair_index: usize, shape: RegionShape) -> StormRegion {
    let x = region_index as f64 * 1.0e-6;
    let y = pair_index as f64 * 1.0e-6;
    match shape {
        RegionShape::PartialFlush => partial_flush(region_index, pair_index, x, y),
        RegionShape::CrossingAmbiguous => crossing_ambiguous(region_index, pair_index, x, y),
        RegionShape::ContainedCandidate => contained_candidate(region_index, pair_index, x, y),
    }
}

fn partial_flush(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        first_face: rectangle(x, y, x + 4.0e-9, y + 4.0e-9),
        second_face: rectangle(x + 4.0e-9, y + 1.0e-9, x + 7.0e-9, y + 3.0e-9),
        containment_candidate: None,
    }
}

fn crossing_ambiguous(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        first_face: rectangle(x, y, x + 4.0e-9, y + 4.0e-9),
        second_face: rectangle(x + 1.0e-9, y + 1.0e-9, x + 5.0e-9, y + 5.0e-9),
        containment_candidate: None,
    }
}

fn contained_candidate(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        first_face: rectangle(x, y, x + 5.0e-9, y + 5.0e-9),
        second_face: rectangle(x + 5.0e-9, y + 1.0e-9, x + 8.0e-9, y + 4.0e-9),
        containment_candidate: Some(rectangle(x + 1.0e-9, y + 1.0e-9, x + 2.0e-9, y + 2.0e-9)),
    }
}

fn rectangle(left: f64, bottom: f64, right: f64, top: f64) -> Vec<[f64; 2]> {
    vec![[left, bottom], [right, bottom], [right, top], [left, top]]
}
