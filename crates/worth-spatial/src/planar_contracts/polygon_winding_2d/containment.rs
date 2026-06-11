#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedLoopContainment {
    ContainedHole,
    Outside,
}

impl CertifiedLoopContainment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContainedHole => "contained-hole",
            Self::Outside => "outside",
        }
    }
}

pub(crate) fn point_strictly_inside_loop(point: [f64; 2], loop_points: &[[f64; 2]]) -> bool {
    let mut inside = false;
    for (left, right) in loop_points
        .iter()
        .zip(loop_points.iter().cycle().skip(1))
        .take(loop_points.len())
    {
        let y_crosses = (left[1] > point[1]) != (right[1] > point[1]);
        if y_crosses {
            let x_intersection =
                (right[0] - left[0]) * (point[1] - left[1]) / (right[1] - left[1]) + left[0];
            if point[0] < x_intersection {
                inside = !inside;
            }
        }
    }
    inside
}
