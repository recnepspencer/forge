use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::is_retained_g27_moser_unit_difference;

const ATTACHMENT_SEARCH_EXPANSION: i32 = 3;

pub(super) fn count_common_moser_basis_attachments(
    coefficients: &[[i32; 4]],
    slack_vertex: usize,
    top_neighbor: usize,
    second_neighbor: usize,
) -> Result<usize, G27GeometricFractionalError> {
    let bounds = coefficient_bounds(coefficients);
    let targets = [
        coefficients[slack_vertex],
        coefficients[top_neighbor],
        coefficients[second_neighbor],
    ];
    let mut count = 0usize;
    for a in bounds[0].0..=bounds[0].1 {
        for b in bounds[1].0..=bounds[1].1 {
            for c in bounds[2].0..=bounds[2].1 {
                for d in bounds[3].0..=bounds[3].1 {
                    let point = [a, b, c, d];
                    if coefficients.contains(&point) {
                        continue;
                    }
                    if targets.iter().all(|target| unit_difference(point, *target)) {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

fn unit_difference(point: [i32; 4], target: [i32; 4]) -> bool {
    is_retained_g27_moser_unit_difference([
        point[0] - target[0],
        point[1] - target[1],
        point[2] - target[2],
        point[3] - target[3],
    ])
}

fn coefficient_bounds(existing: &[[i32; 4]]) -> [(i32, i32); 4] {
    let mut bounds = [(0, 0); 4];
    for index in 0..4 {
        let min = existing.iter().map(|row| row[index]).min().unwrap_or(0);
        let max = existing.iter().map(|row| row[index]).max().unwrap_or(0);
        bounds[index] = (
            min - ATTACHMENT_SEARCH_EXPANSION,
            max + ATTACHMENT_SEARCH_EXPANSION,
        );
    }
    bounds
}
