#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegionShape {
    PartialFlush,
}

#[derive(Clone, Debug)]
pub(crate) struct StormRegion {
    pub(crate) region_index: usize,
    pub(crate) pair_index: usize,
    pub(crate) first_face: Vec<[f64; 2]>,
    pub(crate) second_face: Vec<[f64; 2]>,
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

fn region(region_index: usize, pair_index: usize, shape: RegionShape) -> StormRegion {
    let x = region_index as f64 * 1.0e-6;
    let y = pair_index as f64 * 1.0e-6;
    match shape {
        RegionShape::PartialFlush => partial_flush(region_index, pair_index, x, y),
    }
}

fn partial_flush(region_index: usize, pair_index: usize, x: f64, y: f64) -> StormRegion {
    StormRegion {
        region_index,
        pair_index,
        first_face: rectangle(x, y, x + 4.0e-9, y + 4.0e-9),
        second_face: rectangle(x + 4.0e-9, y + 1.0e-9, x + 7.0e-9, y + 3.0e-9),
    }
}

fn rectangle(left: f64, bottom: f64, right: f64, top: f64) -> Vec<[f64; 2]> {
    vec![[left, bottom], [right, bottom], [right, top], [left, top]]
}
