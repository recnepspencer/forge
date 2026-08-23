use super::case::Phase5LocalityAxis;
use super::RETAINED_SIZES;

pub(super) const SHARD_COUNT: usize = 16;
pub(super) const ROW_COUNT: usize = RETAINED_SIZES.len() * Phase5LocalityAxis::ALL.len();

pub(super) fn report_name(shard: usize) -> String {
    format!("worth-ui-phase5-locality-{shard}.jsonl")
}

pub(super) fn expected_rows(shard: usize) -> usize {
    (0..ROW_COUNT)
        .filter(|ordinal| ordinal % SHARD_COUNT == shard)
        .count()
}

pub(super) fn is_large(shard: usize) -> bool {
    shard >= SHARD_COUNT / 2
}

pub(super) fn validate_shard(shard: usize, count: usize) -> Result<(), String> {
    if count != SHARD_COUNT {
        return Err(format!(
            "matrix shard count {count} does not match governed count {SHARD_COUNT}"
        ));
    }
    if shard >= count {
        return Err(format!("matrix shard {shard}/{count} is out of range"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{expected_rows, is_large, SHARD_COUNT};

    #[test]
    fn every_shard_owns_exactly_two_rows() {
        assert_eq!((0..SHARD_COUNT).map(expected_rows).sum::<usize>(), 32);
        assert!((0..SHARD_COUNT).all(|shard| expected_rows(shard) == 2));
    }

    #[test]
    fn only_the_four_thousand_ninety_six_shards_are_large() {
        assert!((0..8).all(|shard| !is_large(shard)));
        assert!((8..SHARD_COUNT).all(is_large));
    }

    #[test]
    fn each_shard_pairs_one_large_or_one_middle_world_with_a_smoke_world() {
        let large_rows = (8..SHARD_COUNT).map(expected_rows).sum::<usize>();
        let ordinary_rows = (0..8).map(expected_rows).sum::<usize>();
        assert_eq!(large_rows, 16);
        assert_eq!(ordinary_rows, 16);
        assert!((0..SHARD_COUNT).all(|shard| expected_rows(shard) == 2));
    }
}
