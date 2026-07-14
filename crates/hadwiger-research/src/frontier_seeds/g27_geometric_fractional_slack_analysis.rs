use num_bigint::BigInt;
use num_traits::Zero;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_dual_replay::RetainedCsrMatrix;

const G27_VERTEX_COUNT: usize = 27;
const TOP_PRESSURE_ROWS: usize = 16;
const TOP_PRESSURE_PAIRS: usize = 16;
const G27_ISOMETRIES: &str = include_str!("g27_geometric_fractional/g27_isometries.txt");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27PressureVertex {
    vertex_label: String,
    tight_atom_participation: usize,
}

impl G27PressureVertex {
    pub fn vertex_label(&self) -> &str {
        &self.vertex_label
    }

    pub fn tight_atom_participation(&self) -> usize {
        self.tight_atom_participation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27PressureIsometryRow {
    row_index: usize,
    mapping_size: usize,
    tight_atom_touches: usize,
    sparse_touches: usize,
}

impl G27PressureIsometryRow {
    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn mapping_size(&self) -> usize {
        self.mapping_size
    }

    pub fn tight_atom_touches(&self) -> usize {
        self.tight_atom_touches
    }

    pub fn sparse_touches(&self) -> usize {
        self.sparse_touches
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27TightAtomVertexPair {
    left_vertex_label: String,
    right_vertex_label: String,
    tight_atom_co_participation: usize,
}

impl G27TightAtomVertexPair {
    pub fn left_vertex_label(&self) -> &str {
        &self.left_vertex_label
    }

    pub fn right_vertex_label(&self) -> &str {
        &self.right_vertex_label
    }

    pub fn tight_atom_co_participation(&self) -> usize {
        self.tight_atom_co_participation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27GeometricFractionalPressureReport {
    tight_atom_count: usize,
    tight_atom_size_distribution: Vec<(usize, usize)>,
    top_vertices: Vec<G27PressureVertex>,
    top_vertex_pairs: Vec<G27TightAtomVertexPair>,
    top_isometry_rows: Vec<G27PressureIsometryRow>,
    top_non_singleton_isometry_rows: Vec<G27PressureIsometryRow>,
}

impl G27GeometricFractionalPressureReport {
    pub fn tight_atom_count(&self) -> usize {
        self.tight_atom_count
    }

    pub fn tight_atom_size_distribution(&self) -> &[(usize, usize)] {
        &self.tight_atom_size_distribution
    }

    pub fn top_vertices(&self) -> &[G27PressureVertex] {
        &self.top_vertices
    }

    pub fn top_vertex_pairs(&self) -> &[G27TightAtomVertexPair] {
        &self.top_vertex_pairs
    }

    pub fn top_isometry_rows(&self) -> &[G27PressureIsometryRow] {
        &self.top_isometry_rows
    }

    pub fn top_non_singleton_isometry_rows(&self) -> &[G27PressureIsometryRow] {
        &self.top_non_singleton_isometry_rows
    }

    pub fn stable_token(&self) -> String {
        let sizes = self
            .tight_atom_size_distribution
            .iter()
            .map(|(size, count)| format!("{size}:{count}"))
            .collect::<Vec<_>>()
            .join(",");
        let vertices = self
            .top_vertices
            .iter()
            .map(|row| format!("{}:{}", row.vertex_label, row.tight_atom_participation))
            .collect::<Vec<_>>()
            .join(",");
        let rows = self
            .top_isometry_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.row_index, row.mapping_size, row.tight_atom_touches, row.sparse_touches
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let non_singleton_rows = self
            .top_non_singleton_isometry_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.row_index, row.mapping_size, row.tight_atom_touches, row.sparse_touches
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let pairs = self
            .top_vertex_pairs
            .iter()
            .map(|row| {
                format!(
                    "{}-{}:{}",
                    row.left_vertex_label, row.right_vertex_label, row.tight_atom_co_participation
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "g27_pressure:tight{}:sizes[{sizes}]:vertices[{vertices}]:pairs[{pairs}]:rows[{rows}]:non_singleton_rows[{non_singleton_rows}]",
            self.tight_atom_count
        )
    }
}

pub(super) fn analyze_g27_dual_slacks(
    slacks: &[BigInt],
    matrix: &RetainedCsrMatrix,
    atoms: &[u32],
) -> Result<G27GeometricFractionalPressureReport, G27GeometricFractionalError> {
    if slacks.len() != atoms.len() {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch {
            source: "slack_atom_alignment",
        });
    }
    let tight = tight_columns(slacks);
    let tight_atom_count = tight.iter().filter(|is_tight| **is_tight).count();
    let tight_atom_size_distribution = tight_atom_size_distribution(atoms, &tight);
    let top_vertices = top_pressure_vertices(atoms, &tight);
    let top_vertex_pairs = top_pressure_vertex_pairs(atoms, &tight);
    let isometry_rows = pressure_isometry_rows(matrix, &tight);
    let top_isometry_rows = isometry_rows
        .iter()
        .take(TOP_PRESSURE_ROWS)
        .cloned()
        .collect();
    let top_non_singleton_isometry_rows = isometry_rows
        .into_iter()
        .filter(|row| row.mapping_size() > 1)
        .take(TOP_PRESSURE_ROWS)
        .collect();
    Ok(G27GeometricFractionalPressureReport {
        tight_atom_count,
        tight_atom_size_distribution,
        top_vertices,
        top_vertex_pairs,
        top_isometry_rows,
        top_non_singleton_isometry_rows,
    })
}

fn tight_columns(slacks: &[BigInt]) -> Vec<bool> {
    slacks.iter().map(BigInt::is_zero).collect()
}

fn tight_atom_size_distribution(atoms: &[u32], tight: &[bool]) -> Vec<(usize, usize)> {
    let mut counts = [0usize; G27_VERTEX_COUNT + 1];
    for (atom, is_tight) in atoms.iter().zip(tight.iter()) {
        if *is_tight {
            counts[atom.count_ones() as usize] += 1;
        }
    }
    counts
        .into_iter()
        .enumerate()
        .filter(|(_, count)| *count > 0)
        .collect()
}

fn top_pressure_vertices(atoms: &[u32], tight: &[bool]) -> Vec<G27PressureVertex> {
    let mut counts = [0usize; G27_VERTEX_COUNT];
    for (atom, is_tight) in atoms.iter().zip(tight.iter()) {
        if *is_tight {
            for (vertex, count) in counts.iter_mut().enumerate() {
                if atom & (1u32 << vertex) != 0 {
                    *count += 1;
                }
            }
        }
    }
    let mut rows = counts
        .into_iter()
        .enumerate()
        .map(|(index, count)| G27PressureVertex {
            vertex_label: (index + 1).to_string(),
            tight_atom_participation: count,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .tight_atom_participation
            .cmp(&left.tight_atom_participation)
            .then_with(|| left.vertex_label.cmp(&right.vertex_label))
    });
    rows
}

fn top_pressure_vertex_pairs(atoms: &[u32], tight: &[bool]) -> Vec<G27TightAtomVertexPair> {
    let mut counts = [[0usize; G27_VERTEX_COUNT]; G27_VERTEX_COUNT];
    for (atom, is_tight) in atoms.iter().zip(tight.iter()) {
        if *is_tight {
            for left in 0..G27_VERTEX_COUNT {
                if atom & (1u32 << left) == 0 {
                    continue;
                }
                for right in (left + 1)..G27_VERTEX_COUNT {
                    if atom & (1u32 << right) != 0 {
                        counts[left][right] += 1;
                    }
                }
            }
        }
    }
    let mut pairs = Vec::new();
    for (left, row) in counts.iter().enumerate() {
        for (right, count) in row.iter().enumerate().skip(left + 1) {
            if *count > 0 {
                pairs.push(G27TightAtomVertexPair {
                    left_vertex_label: (left + 1).to_string(),
                    right_vertex_label: (right + 1).to_string(),
                    tight_atom_co_participation: *count,
                });
            }
        }
    }
    pairs.sort_by(|left, right| {
        right
            .tight_atom_co_participation
            .cmp(&left.tight_atom_co_participation)
            .then_with(|| left.left_vertex_label.cmp(&right.left_vertex_label))
            .then_with(|| left.right_vertex_label.cmp(&right.right_vertex_label))
    });
    pairs.truncate(TOP_PRESSURE_PAIRS);
    pairs
}

fn pressure_isometry_rows(
    matrix: &RetainedCsrMatrix,
    tight: &[bool],
) -> Vec<G27PressureIsometryRow> {
    let mapping_sizes = isometry_mapping_sizes();
    let mut rows = (0..matrix.indptr.len() - 1)
        .map(|row| {
            let start = matrix.indptr[row] as usize;
            let end = matrix.indptr[row + 1] as usize;
            let tight_atom_touches = matrix.indices[start..end]
                .iter()
                .filter(|column| tight[**column as usize])
                .count();
            G27PressureIsometryRow {
                row_index: row,
                mapping_size: mapping_sizes[row],
                tight_atom_touches,
                sparse_touches: end - start,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .tight_atom_touches
            .cmp(&left.tight_atom_touches)
            .then_with(|| right.sparse_touches.cmp(&left.sparse_touches))
            .then_with(|| left.row_index.cmp(&right.row_index))
    });
    rows
}

fn isometry_mapping_sizes() -> Vec<usize> {
    G27_ISOMETRIES
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter(|value| *value != "-1")
                .count()
        })
        .collect()
}
