use crate::domain_artifacts::digest_basis::HadwigerArtifactPayloadEntry;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::is_retained_g27_moser_unit_difference;
use super::g27_pressure_followup_rounds::{
    G27HittingSetPosture, G27OneAnchorTransversalPosture, G27SpindlePreflightPosture,
    G27TightAtomTransversal,
};

const VERTEX_COUNT: usize = 27;
const SEARCH_EXPANSION: i32 = 3;

pub(super) fn enumerate_combinations(
    remaining: usize,
    start: usize,
    mask: u32,
    visit: &mut impl FnMut(u32),
) {
    if remaining == 0 {
        visit(mask);
        return;
    }
    for vertex in start..=VERTEX_COUNT - remaining {
        enumerate_combinations(remaining - 1, vertex + 1, mask | (1u32 << vertex), visit);
    }
}

pub(super) fn hits_all(mask: u32, atoms: &[u32]) -> bool {
    atoms.iter().all(|atom| atom & mask != 0)
}

pub(super) fn mask_vertices(mask: u32) -> Vec<String> {
    (0..VERTEX_COUNT)
        .filter(|index| mask & (1u32 << index) != 0)
        .map(|index| (index + 1).to_string())
        .collect()
}

pub(super) fn count_moser_basis_common_anchors(
    coefficients: &[[i32; 4]],
    transversal: &G27TightAtomTransversal,
) -> Result<usize, G27GeometricFractionalError> {
    let bounds = coefficient_bounds(coefficients);
    let mut count = 0usize;
    for a in bounds[0].0..=bounds[0].1 {
        for b in bounds[1].0..=bounds[1].1 {
            for c in bounds[2].0..=bounds[2].1 {
                for d in bounds[3].0..=bounds[3].1 {
                    if is_common_anchor([a, b, c, d], coefficients, transversal)? {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

pub(super) fn select_spindle_fragment(atoms: &[u32]) -> Result<u32, G27GeometricFractionalError> {
    let required = (1u32 << 7) | (1u32 << 17) | (1u32 << 22);
    atoms
        .iter()
        .copied()
        .filter(|atom| atom & required == required)
        .max_by_key(|atom| (atom.count_ones(), std::cmp::Reverse(*atom)))
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "g27_spindle_fragment",
        })
}

pub(super) fn hitting_payload(
    tight_atom_count: usize,
    minimum_size: usize,
    size_le_four_count: usize,
    posture: G27HittingSetPosture,
    transversals: &[G27TightAtomTransversal],
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_hitting_set.v1"),
        HadwigerArtifactPayloadEntry::unsigned("tight_atom_count", tight_atom_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("minimum_size", minimum_size as u128),
        HadwigerArtifactPayloadEntry::unsigned("size_le_four_count", size_le_four_count as u128),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for row in transversals {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "minimum_transversal",
            row.stable_token(),
        ));
    }
    payload
}

pub(super) fn one_anchor_payload(
    transversal: &G27TightAtomTransversal,
    anchor_count: usize,
    posture: G27OneAnchorTransversalPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_one_anchor.v1"),
        HadwigerArtifactPayloadEntry::text("tested_transversal", transversal.stable_token()),
        HadwigerArtifactPayloadEntry::unsigned("moser_anchor_count", anchor_count as u128),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

pub(super) fn spindle_payload(
    fragment_vertices: &[String],
    tight_atoms_containing_fragment: usize,
    posture: G27SpindlePreflightPosture,
    next_test: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_spindle_preflight.v1"),
        HadwigerArtifactPayloadEntry::text("hinge_vertex", "8"),
        HadwigerArtifactPayloadEntry::text("fragment_vertices", fragment_vertices.join(",")),
        HadwigerArtifactPayloadEntry::unsigned(
            "tight_atoms_containing_fragment",
            tight_atoms_containing_fragment as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("next_test", next_test),
    ]
}

fn coefficient_bounds(existing: &[[i32; 4]]) -> [(i32, i32); 4] {
    let mut bounds = [(0, 0); 4];
    for index in 0..4 {
        let min = existing.iter().map(|row| row[index]).min().unwrap_or(0);
        let max = existing.iter().map(|row| row[index]).max().unwrap_or(0);
        bounds[index] = (min - SEARCH_EXPANSION, max + SEARCH_EXPANSION);
    }
    bounds
}

fn is_common_anchor(
    point: [i32; 4],
    coefficients: &[[i32; 4]],
    transversal: &G27TightAtomTransversal,
) -> Result<bool, G27GeometricFractionalError> {
    for vertex in transversal.vertices() {
        let index = vertex
            .parse::<usize>()
            .ok()
            .and_then(|value| value.checked_sub(1))
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "g27_vertex_label",
            })?;
        let other = coefficients[index];
        if !is_retained_g27_moser_unit_difference([
            point[0] - other[0],
            point[1] - other[1],
            point[2] - other[2],
            point[3] - other[3],
        ]) {
            return Ok(false);
        }
    }
    Ok(true)
}
