use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::{ColoringRefutationCertificate, GraphVersion};

use super::cnf_encoding::encode_graph_coloring;
use super::HadwigerColorabilityError;

const EXHAUSTIVE_UNSAT_VERTEX_LIMIT: usize = 12;
const EXHAUSTIVE_UNSAT_VARIABLE_LIMIT: u32 = 24;

pub(super) fn checked_exhaustive_certificate(
    graph_version: &GraphVersion,
    color_count: u32,
    variable_count: u32,
) -> Option<ColoringRefutationCertificate> {
    if graph_version.vertices().len() > EXHAUSTIVE_UNSAT_VERTEX_LIMIT
        || variable_count > EXHAUSTIVE_UNSAT_VARIABLE_LIMIT
    {
        return None;
    }
    if has_coloring(graph_version, color_count) {
        return None;
    }
    let (_, clauses) = encode_graph_coloring(graph_version, color_count);
    let exhausted_assignment_count = 1_u128.checked_shl(variable_count).unwrap_or(u128::MAX);
    ColoringRefutationCertificate::checked_exhaustive(
        variable_count,
        exhausted_assignment_count,
        clauses,
    )
    .ok()
}

pub(super) fn replay_refutation(
    clauses: &[Vec<i32>],
    certificate: &ColoringRefutationCertificate,
) -> Result<(), HadwigerColorabilityError> {
    if clauses != certificate.clauses() {
        return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
    }
    if certificate.variable_count() > EXHAUSTIVE_UNSAT_VARIABLE_LIMIT {
        return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
    }
    let replayed_assignment_count =
        replay_exhaustive_cnf_denial(clauses, certificate.variable_count())?;
    if replayed_assignment_count != certificate.exhausted_assignment_count() {
        return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
    }
    Ok(())
}

fn replay_exhaustive_cnf_denial(
    clauses: &[Vec<i32>],
    variable_count: u32,
) -> Result<u128, HadwigerColorabilityError> {
    let assignment_count = 1_u128
        .checked_shl(variable_count)
        .ok_or(HadwigerColorabilityError::CorruptRefutationCertificate)?;
    for assignment_bits in 0..assignment_count {
        if clauses_satisfied_by_assignment_bits(clauses, assignment_bits) {
            return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
        }
    }
    Ok(assignment_count)
}

fn clauses_satisfied_by_assignment_bits(clauses: &[Vec<i32>], assignment_bits: u128) -> bool {
    clauses.iter().all(|clause| {
        clause.iter().any(|literal| {
            let variable_index = literal.unsigned_abs() - 1;
            let assigned_true = ((assignment_bits >> variable_index) & 1) == 1;
            (*literal > 0 && assigned_true) || (*literal < 0 && !assigned_true)
        })
    })
}

fn has_coloring(graph_version: &GraphVersion, color_count: u32) -> bool {
    let labels = graph_version
        .vertices()
        .iter()
        .map(|vertex| vertex.vertex_label().to_string())
        .collect::<Vec<_>>();
    let edges = graph_version
        .edges()
        .iter()
        .map(|edge| {
            let (left, right) = edge.endpoints();
            (left.to_string(), right.to_string())
        })
        .collect::<BTreeSet<_>>();
    let mut colors = BTreeMap::new();
    color_vertex(0, &labels, &edges, color_count, &mut colors)
}

fn color_vertex(
    index: usize,
    labels: &[String],
    edges: &BTreeSet<(String, String)>,
    color_count: u32,
    colors: &mut BTreeMap<String, u32>,
) -> bool {
    if index == labels.len() {
        return true;
    }
    let label = &labels[index];
    for color in 0..color_count {
        if edges.iter().all(|(left, right)| {
            (left != label || colors.get(right) != Some(&color))
                && (right != label || colors.get(left) != Some(&color))
        }) {
            colors.insert(label.clone(), color);
            if color_vertex(index + 1, labels, edges, color_count, colors) {
                return true;
            }
            colors.remove(label);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhaustive_replay_rejects_satisfiable_clause_set() {
        let clauses = vec![vec![1]];
        let certificate =
            ColoringRefutationCertificate::checked_exhaustive(1, 2, clauses.clone()).unwrap();

        assert_eq!(
            replay_refutation(&clauses, &certificate),
            Err(HadwigerColorabilityError::CorruptRefutationCertificate)
        );
    }

    #[test]
    fn exhaustive_replay_rejects_assignment_count_drift() {
        let clauses = vec![vec![1], vec![-1]];
        let certificate =
            ColoringRefutationCertificate::checked_exhaustive(1, 1, clauses.clone()).unwrap();

        assert_eq!(
            replay_refutation(&clauses, &certificate),
            Err(HadwigerColorabilityError::CorruptRefutationCertificate)
        );
    }
}
