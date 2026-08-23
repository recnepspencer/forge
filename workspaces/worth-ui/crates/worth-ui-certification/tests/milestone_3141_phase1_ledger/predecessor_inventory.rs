use std::collections::BTreeSet;

use super::schema;

pub(super) fn predecessor_requirements(count: usize) -> BTreeSet<&'static str> {
    let max_digit = match count {
        30 => b'2',
        47 => b'3',
        68 => b'4',
        80 => b'5',
        _ => 0,
    };
    schema::EXPECTED_REQUIREMENTS
        .iter()
        .copied()
        .filter(|requirement| phase_at_most(requirement, max_digit))
        .collect()
}

fn phase_at_most(requirement: &str, max_digit: u8) -> bool {
    requirement
        .as_bytes()
        .get(1)
        .is_some_and(|digit| *digit <= max_digit)
}
