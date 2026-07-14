use std::collections::BTreeSet;

use super::g27_geometric_fractional::G27GeometricFractionalError;

const G27_COEFFS: &str = include_str!("g27_geometric_fractional/g27_coeffs.txt");
const G27_ADJACENCY: &str = include_str!("g27_geometric_fractional/adjacency_matrix.txt");
const G27_ATOMS: &str = include_str!("g27_geometric_fractional/atom_dictionary.txt");
const G27_ISOMETRIES: &str = include_str!("g27_geometric_fractional/g27_isometries.txt");
const G27_WITNESS: &str = include_str!("g27_geometric_fractional/witness.txt");
const G27_VERTEX_COUNT: usize = 27;
const G27_ATOM_COUNT: usize = 182_304;
const G27_ISOMETRY_COUNT: usize = 16_855;

pub(super) struct G27RetainedStructuralReplay {
    pub(super) vertex_count: usize,
    pub(super) edge_count: usize,
    pub(super) independent_set_count: usize,
    pub(super) isometry_count: usize,
    pub(super) witness_coordinate_count: usize,
}

pub(super) fn g27_dimacs_edge_list_from_retained_data() -> String {
    let adjacency = parse_adjacency_matrix().expect("retained G27 adjacency matrix is valid");
    let edges = adjacency_edges(&adjacency);
    let mut text = format!("p edge {} {}\n", G27_VERTEX_COUNT, edges.len());
    for (left, right) in edges {
        text.push_str(&format!("e {} {}\n", left + 1, right + 1));
    }
    text
}

pub(super) fn retained_g27_coefficients() -> Result<Vec<[i32; 4]>, G27GeometricFractionalError> {
    parse_coefficients()
}

pub(super) fn is_retained_g27_moser_unit_difference(coefficients: [i32; 4]) -> bool {
    is_moser_unit_difference(coefficients)
}

pub(super) fn replay_g27_retained_structural_certificate(
) -> Result<G27RetainedStructuralReplay, G27GeometricFractionalError> {
    let coeffs = parse_coefficients()?;
    let adjacency = parse_adjacency_matrix()?;
    verify_moser_adjacency(&coeffs, &adjacency)?;
    let adjacency_masks = adjacency_masks(&adjacency);
    let atoms = parse_atom_dictionary()?;
    verify_independent_set_dictionary(&adjacency_masks, &atoms)?;
    verify_isometry_rows(&atoms)?;
    let witness_count = verify_witness_shape()?;
    Ok(G27RetainedStructuralReplay {
        vertex_count: G27_VERTEX_COUNT,
        edge_count: adjacency_edges(&adjacency).len(),
        independent_set_count: atoms.len(),
        isometry_count: G27_ISOMETRY_COUNT,
        witness_coordinate_count: witness_count,
    })
}

fn parse_coefficients() -> Result<Vec<[i32; 4]>, G27GeometricFractionalError> {
    let rows = G27_COEFFS
        .lines()
        .map(|line| {
            let values = parse_i32_row(line, "coeffs")?;
            match values.as_slice() {
                [a, b, c, d] => Ok([*a, *b, *c, *d]),
                _ => Err(G27GeometricFractionalError::MalformedData { source: "coeffs" }),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    require_count(rows, G27_VERTEX_COUNT, "coeffs")
}

fn parse_adjacency_matrix() -> Result<Vec<Vec<bool>>, G27GeometricFractionalError> {
    let rows = G27_ADJACENCY
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|value| match value {
                    "0" => Ok(false),
                    "1" => Ok(true),
                    _ => Err(G27GeometricFractionalError::MalformedData {
                        source: "adjacency",
                    }),
                })
                .collect::<Result<Vec<_>, _>>()?;
            require_count(values, G27_VERTEX_COUNT, "adjacency")
        })
        .collect::<Result<Vec<_>, _>>()?;
    require_count(rows, G27_VERTEX_COUNT, "adjacency")
}

fn verify_moser_adjacency(
    coeffs: &[[i32; 4]],
    adjacency: &[Vec<bool>],
) -> Result<(), G27GeometricFractionalError> {
    for left in 0..G27_VERTEX_COUNT {
        for right in 0..G27_VERTEX_COUNT {
            let actual =
                left != right && is_moser_unit_difference(subtract(coeffs[left], coeffs[right]));
            if actual != adjacency[left][right] {
                return Err(G27GeometricFractionalError::AdjacencyMismatch { left, right });
            }
        }
    }
    Ok(())
}

fn parse_atom_dictionary() -> Result<Vec<u32>, G27GeometricFractionalError> {
    let mut atoms = Vec::with_capacity(G27_ATOM_COUNT);
    for line in G27_ATOMS.lines() {
        atoms.push(parse_atom_mask(line)?);
    }
    atoms.sort_unstable();
    atoms.dedup();
    require_count(atoms, G27_ATOM_COUNT, "atoms")
}

fn parse_atom_mask(line: &str) -> Result<u32, G27GeometricFractionalError> {
    let mut mask = 0u32;
    let mut count = 0usize;
    for (index, value) in line.split_whitespace().enumerate() {
        match value {
            "0" => {}
            "1" => mask |= 1u32 << index,
            _ => return Err(G27GeometricFractionalError::MalformedData { source: "atoms" }),
        }
        count += 1;
    }
    if count == G27_VERTEX_COUNT {
        Ok(mask)
    } else {
        Err(G27GeometricFractionalError::MalformedData { source: "atoms" })
    }
}

fn verify_independent_set_dictionary(
    adjacency_masks: &[u32],
    atoms: &[u32],
) -> Result<(), G27GeometricFractionalError> {
    let mut enumerated = Vec::with_capacity(G27_ATOM_COUNT);
    enumerate_independent_sets(
        adjacency_masks,
        (1u32 << G27_VERTEX_COUNT) - 1,
        0,
        &mut enumerated,
    );
    enumerated.sort_unstable();
    if enumerated == atoms {
        Ok(())
    } else {
        Err(G27GeometricFractionalError::IndependentSetMismatch)
    }
}

fn verify_isometry_rows(atoms: &[u32]) -> Result<(), G27GeometricFractionalError> {
    let atom_set = atoms.iter().copied().collect::<BTreeSet<_>>();
    let mut row_count = 0usize;
    for (row, line) in G27_ISOMETRIES.lines().enumerate() {
        let (domain, image) = parse_isometry_masks(line, row + 1)?;
        if domain == 0 || !atom_set.contains(&domain) || !atom_set.contains(&image) {
            return Err(G27GeometricFractionalError::InvalidIsometryRow { row: row + 1 });
        }
        row_count += 1;
    }
    if row_count == G27_ISOMETRY_COUNT {
        Ok(())
    } else {
        Err(G27GeometricFractionalError::WitnessShapeMismatch)
    }
}

fn parse_isometry_masks(line: &str, row: usize) -> Result<(u32, u32), G27GeometricFractionalError> {
    let values = parse_i32_row(line, "isometry")
        .map_err(|_| G27GeometricFractionalError::InvalidIsometryRow { row })?;
    let values = require_count(values, G27_VERTEX_COUNT, "isometry")
        .map_err(|_| G27GeometricFractionalError::InvalidIsometryRow { row })?;
    let mut domain = 0u32;
    let mut image = 0u32;
    for (index, mapped) in values.into_iter().enumerate() {
        if mapped >= 0 {
            if mapped as usize >= G27_VERTEX_COUNT {
                return Err(G27GeometricFractionalError::InvalidIsometryRow { row });
            }
            domain |= 1u32 << index;
            let image_bit = 1u32 << (mapped as usize);
            if image & image_bit != 0 {
                return Err(G27GeometricFractionalError::InvalidIsometryRow { row });
            }
            image |= image_bit;
        }
    }
    Ok((domain, image))
}

fn verify_witness_shape() -> Result<usize, G27GeometricFractionalError> {
    let mut count = 0usize;
    for line in G27_WITNESS.lines() {
        let values = line.split_whitespace().collect::<Vec<_>>();
        if values.len() != 2 || values[1].starts_with('0') {
            return Err(G27GeometricFractionalError::WitnessShapeMismatch);
        }
        count += 1;
    }
    if count == G27_ISOMETRY_COUNT {
        Ok(count)
    } else {
        Err(G27GeometricFractionalError::WitnessShapeMismatch)
    }
}

fn adjacency_edges(adjacency: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for left in 0..G27_VERTEX_COUNT {
        for right in (left + 1)..G27_VERTEX_COUNT {
            if adjacency[left][right] {
                edges.push((left, right));
            }
        }
    }
    edges
}

fn adjacency_masks(adjacency: &[Vec<bool>]) -> Vec<u32> {
    adjacency
        .iter()
        .map(|row| {
            row.iter().enumerate().fold(
                0u32,
                |mask, (index, edge)| {
                    if *edge {
                        mask | (1u32 << index)
                    } else {
                        mask
                    }
                },
            )
        })
        .collect()
}

fn enumerate_independent_sets(
    adjacency_masks: &[u32],
    candidates: u32,
    chosen: u32,
    output: &mut Vec<u32>,
) {
    if candidates == 0 {
        output.push(chosen);
        return;
    }
    let vertex = candidates.trailing_zeros() as usize;
    let vertex_bit = 1u32 << vertex;
    enumerate_independent_sets(adjacency_masks, candidates & !vertex_bit, chosen, output);
    enumerate_independent_sets(
        adjacency_masks,
        candidates & !vertex_bit & !adjacency_masks[vertex],
        chosen | vertex_bit,
        output,
    );
}

fn subtract(left: [i32; 4], right: [i32; 4]) -> [i32; 4] {
    [
        left[0] - right[0],
        left[1] - right[1],
        left[2] - right[2],
        left[3] - right[3],
    ]
}

fn is_moser_unit_difference(coefficients: [i32; 4]) -> bool {
    let abs_sum = coefficients.iter().map(|value| value.abs()).sum::<i32>();
    if abs_sum == 1 {
        return true;
    }
    if coefficients.iter().sum::<i32>() != 0 {
        return false;
    }
    let first = abs_sum == 2
        && (coefficients[..2]
            .iter()
            .map(|value| value.abs())
            .sum::<i32>()
            == 0
            || coefficients[2..]
                .iter()
                .map(|value| value.abs())
                .sum::<i32>()
                == 0);
    let second =
        coefficients.iter().all(|value| value.abs() == 1) && coefficients[0] == coefficients[1];
    let third = abs_sum == 6
        && coefficients[0] == -coefficients[2]
        && coefficients[1] == -coefficients[3]
        && (coefficients[0] + coefficients[1]).abs() == 1;
    first || second || third
}

fn parse_i32_row(
    line: &str,
    source: &'static str,
) -> Result<Vec<i32>, G27GeometricFractionalError> {
    line.split_whitespace()
        .map(|value| {
            value
                .parse::<i32>()
                .map_err(|_| G27GeometricFractionalError::MalformedData { source })
        })
        .collect()
}

fn require_count<T>(
    values: Vec<T>,
    expected: usize,
    source: &'static str,
) -> Result<Vec<T>, G27GeometricFractionalError> {
    if values.len() == expected {
        Ok(values)
    } else {
        Err(G27GeometricFractionalError::MalformedData { source })
    }
}
