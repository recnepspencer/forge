#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ExpectedTarget {
    Rank(u32),
    None,
}

pub(super) fn expected_target(point: [i64; 2], clipped_outer: bool) -> ExpectedTarget {
    if contains([16, 12, 128, 72], point) {
        return ExpectedTarget::Rank(0);
    }
    let outer_clip = if clipped_outer {
        [20, 16, 120, 64]
    } else {
        [8, 8, 144, 80]
    };
    if contains([8, 8, 144, 80], point) && contains(outer_clip, point) {
        ExpectedTarget::Rank(1)
    } else {
        ExpectedTarget::None
    }
}

fn contains(bounds: [i64; 4], point: [i64; 2]) -> bool {
    point[0] >= bounds[0]
        && point[0] < bounds[0] + bounds[2]
        && point[1] >= bounds[1]
        && point[1] < bounds[1] + bounds[3]
}

#[cfg(test)]
mod tests {
    use super::{expected_target, ExpectedTarget};

    #[test]
    fn independent_oracle_preserves_overlap_clip_and_half_open_edges() {
        assert_eq!(expected_target([20, 20], false), ExpectedTarget::Rank(0));
        assert_eq!(expected_target([10, 20], false), ExpectedTarget::Rank(1));
        assert_eq!(expected_target([10, 20], true), ExpectedTarget::None);
        assert_eq!(expected_target([152, 20], false), ExpectedTarget::None);
    }
}
