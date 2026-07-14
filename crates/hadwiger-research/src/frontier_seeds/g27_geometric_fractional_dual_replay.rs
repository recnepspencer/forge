use std::fs::File;
use std::io::Read;
use std::path::Path;

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, Zero};
use zip::ZipArchive;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_slack_analysis::{
    analyze_g27_dual_slacks, G27GeometricFractionalPressureReport,
};

const G27_ATOMS: &str = include_str!("g27_geometric_fractional/atom_dictionary.txt");
const G27_WITNESS: &str = include_str!("g27_geometric_fractional/witness.txt");
const G27_MATRIX_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/frontier_seeds/g27_geometric_fractional/iec.npz"
);
const G27_ATOM_COUNT: usize = 182_304;
const G27_ISOMETRY_COUNT: usize = 16_855;
const G27_MATRIX_NONZEROS: usize = 39_072_252;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27GeometricFractionalDualReplay {
    atom_columns_checked: usize,
    matrix_nonzero_count: usize,
    witness_coordinate_count: usize,
    zero_slack_columns: usize,
    positive_slack_columns: usize,
    common_denominator_digits: usize,
    pressure_report: G27GeometricFractionalPressureReport,
}

impl G27GeometricFractionalDualReplay {
    pub fn atom_columns_checked(&self) -> usize {
        self.atom_columns_checked
    }

    pub fn matrix_nonzero_count(&self) -> usize {
        self.matrix_nonzero_count
    }

    pub fn witness_coordinate_count(&self) -> usize {
        self.witness_coordinate_count
    }

    pub fn zero_slack_columns(&self) -> usize {
        self.zero_slack_columns
    }

    pub fn positive_slack_columns(&self) -> usize {
        self.positive_slack_columns
    }

    pub fn common_denominator_digits(&self) -> usize {
        self.common_denominator_digits
    }

    pub fn pressure_report(&self) -> &G27GeometricFractionalPressureReport {
        &self.pressure_report
    }

    pub fn stable_token(&self) -> String {
        format!(
            "g27_dual_replay:columns{}:nnz{}:witness{}:zero{}:positive{}:den_digits{}:{}",
            self.atom_columns_checked,
            self.matrix_nonzero_count,
            self.witness_coordinate_count,
            self.zero_slack_columns,
            self.positive_slack_columns,
            self.common_denominator_digits,
            self.pressure_report.stable_token()
        )
    }
}

pub(super) fn replay_g27_retained_dual_witness(
) -> Result<G27GeometricFractionalDualReplay, G27GeometricFractionalError> {
    let atoms = retained_atom_masks()?;
    let e_vector = atoms.iter().map(|mask| mask & 1 != 0).collect::<Vec<_>>();
    let witness = parse_scaled_witness()?;
    let matrix = load_retained_csr_matrix(Path::new(G27_MATRIX_PATH))?;
    require_matrix_shape(&matrix, witness.values.len())?;
    let slacks = replay_dual_slacks(&matrix, &witness, &e_vector)?;
    let mut zero_slack_columns = 0usize;
    let mut positive_slack_columns = 0usize;
    for (column, slack) in slacks.iter().enumerate() {
        if slack.is_negative() {
            return Err(G27GeometricFractionalError::DualInequalityViolation { column });
        }
        if slack.is_zero() {
            zero_slack_columns += 1;
        } else {
            positive_slack_columns += 1;
        }
    }
    let pressure_report = analyze_g27_dual_slacks(&slacks, &matrix, &atoms)?;
    Ok(G27GeometricFractionalDualReplay {
        atom_columns_checked: G27_ATOM_COUNT,
        matrix_nonzero_count: matrix.indices.len(),
        witness_coordinate_count: witness.values.len(),
        zero_slack_columns,
        positive_slack_columns,
        common_denominator_digits: witness.common_denominator.to_string().len(),
        pressure_report,
    })
}

pub(super) fn retained_g27_tight_atom_masks() -> Result<Vec<u32>, G27GeometricFractionalError> {
    let atoms = retained_atom_masks()?;
    let e_vector = atoms.iter().map(|mask| mask & 1 != 0).collect::<Vec<_>>();
    let witness = parse_scaled_witness()?;
    let matrix = load_retained_csr_matrix(Path::new(G27_MATRIX_PATH))?;
    require_matrix_shape(&matrix, witness.values.len())?;
    let slacks = replay_dual_slacks(&matrix, &witness, &e_vector)?;
    if slacks.iter().any(BigInt::is_negative) {
        return Err(G27GeometricFractionalError::DualInequalityViolation {
            column: slacks
                .iter()
                .position(BigInt::is_negative)
                .expect("negative slack exists"),
        });
    }
    Ok(atoms
        .into_iter()
        .zip(slacks)
        .filter_map(|(atom, slack)| if slack.is_zero() { Some(atom) } else { None })
        .collect())
}

pub(super) fn retained_g27_atom_slacks(
) -> Result<(BigInt, Vec<(u32, BigInt)>), G27GeometricFractionalError> {
    let atoms = retained_atom_masks()?;
    let e_vector = atoms.iter().map(|mask| mask & 1 != 0).collect::<Vec<_>>();
    let witness = parse_scaled_witness()?;
    let matrix = load_retained_csr_matrix(Path::new(G27_MATRIX_PATH))?;
    require_matrix_shape(&matrix, witness.values.len())?;
    let slacks = replay_dual_slacks(&matrix, &witness, &e_vector)?;
    if slacks.iter().any(BigInt::is_negative) {
        return Err(G27GeometricFractionalError::DualInequalityViolation {
            column: slacks
                .iter()
                .position(BigInt::is_negative)
                .expect("negative slack exists"),
        });
    }
    Ok((
        witness.common_denominator,
        atoms.into_iter().zip(slacks).collect(),
    ))
}

fn replay_dual_slacks(
    matrix: &RetainedCsrMatrix,
    witness: &ScaledWitness,
    e_vector: &[bool],
) -> Result<Vec<BigInt>, G27GeometricFractionalError> {
    let mut slacks = e_vector
        .iter()
        .map(|e_value| {
            if *e_value {
                -BigInt::from(3u8) * &witness.common_denominator
            } else {
                witness.common_denominator.clone()
            }
        })
        .collect::<Vec<_>>();
    for row in 0..witness.values.len() {
        let start = matrix.indptr[row] as usize;
        let end = matrix.indptr[row + 1] as usize;
        let witness_value = &witness.values[row];
        for offset in start..end {
            let column = matrix.indices[offset] as usize;
            match matrix.data[offset] {
                1 => slacks[column] += witness_value,
                -1 => slacks[column] -= witness_value,
                _ => {
                    return Err(G27GeometricFractionalError::MatrixShapeMismatch {
                        source: "data.npy",
                    })
                }
            }
        }
    }
    Ok(slacks)
}

struct ScaledWitness {
    common_denominator: BigInt,
    values: Vec<BigInt>,
}

fn parse_scaled_witness() -> Result<ScaledWitness, G27GeometricFractionalError> {
    let mut rationals = Vec::with_capacity(G27_ISOMETRY_COUNT);
    for line in G27_WITNESS.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(G27GeometricFractionalError::WitnessShapeMismatch);
        }
        let numerator = parse_bigint(parts[0])?;
        let denominator = parse_bigint(parts[1])?;
        if denominator.is_zero() {
            return Err(G27GeometricFractionalError::WitnessShapeMismatch);
        }
        rationals.push((numerator, denominator));
    }
    if rationals.len() != G27_ISOMETRY_COUNT {
        return Err(G27GeometricFractionalError::WitnessShapeMismatch);
    }
    let common_denominator = rationals
        .iter()
        .fold(BigInt::from(1u8), |lcm, (_, denominator)| {
            lcm.lcm(denominator)
        });
    let values = rationals
        .into_iter()
        .map(|(numerator, denominator)| numerator * (&common_denominator / denominator))
        .collect();
    Ok(ScaledWitness {
        common_denominator,
        values,
    })
}

pub(super) fn retained_atom_masks() -> Result<Vec<u32>, G27GeometricFractionalError> {
    let mut atoms = Vec::with_capacity(G27_ATOM_COUNT);
    for line in G27_ATOMS.lines() {
        atoms.push(parse_atom_mask(line)?);
    }
    if atoms.len() == G27_ATOM_COUNT {
        Ok(atoms)
    } else {
        Err(G27GeometricFractionalError::MalformedData { source: "atoms" })
    }
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
    if count == 27 {
        Ok(mask)
    } else {
        Err(G27GeometricFractionalError::MalformedData { source: "atoms" })
    }
}

pub(super) struct RetainedCsrMatrix {
    pub(super) indices: Vec<i32>,
    pub(super) indptr: Vec<i32>,
    pub(super) data: Vec<i8>,
}

fn load_retained_csr_matrix(path: &Path) -> Result<RetainedCsrMatrix, G27GeometricFractionalError> {
    let file = File::open(path).map_err(|error| {
        G27GeometricFractionalError::MatrixZip(format!("open iec.npz failed: {error}"))
    })?;
    let mut archive = ZipArchive::new(file)
        .map_err(|error| G27GeometricFractionalError::MatrixZip(error.to_string()))?;
    Ok(RetainedCsrMatrix {
        indices: read_npy_i32(&mut archive, "indices.npy")?,
        indptr: read_npy_i32(&mut archive, "indptr.npy")?,
        data: read_npy_i8(&mut archive, "data.npy")?,
    })
}

fn require_matrix_shape(
    matrix: &RetainedCsrMatrix,
    row_count: usize,
) -> Result<(), G27GeometricFractionalError> {
    if matrix.indptr.len() != row_count + 1 {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch {
            source: "indptr.npy",
        });
    }
    if matrix.indices.len() != G27_MATRIX_NONZEROS || matrix.data.len() != G27_MATRIX_NONZEROS {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch { source: "iec.npz" });
    }
    if matrix.indptr.first() != Some(&0)
        || matrix.indptr.last() != Some(&(G27_MATRIX_NONZEROS as i32))
    {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch {
            source: "indptr.npy",
        });
    }
    if matrix
        .indices
        .iter()
        .any(|column| *column < 0 || *column as usize >= G27_ATOM_COUNT)
    {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch {
            source: "indices.npy",
        });
    }
    Ok(())
}

fn read_npy_i32(
    archive: &mut ZipArchive<File>,
    name: &'static str,
) -> Result<Vec<i32>, G27GeometricFractionalError> {
    let bytes = read_archive_member(archive, name)?;
    let data = parse_npy_payload(&bytes, "<i4", name)?;
    if data.len() % 4 != 0 {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch { source: name });
    }
    Ok(data
        .chunks_exact(4)
        .map(|chunk| i32::from_le_bytes(chunk.try_into().expect("chunk length is 4")))
        .collect())
}

fn read_npy_i8(
    archive: &mut ZipArchive<File>,
    name: &'static str,
) -> Result<Vec<i8>, G27GeometricFractionalError> {
    let bytes = read_archive_member(archive, name)?;
    let data = parse_npy_payload(&bytes, "|i1", name)?;
    Ok(data.iter().map(|value| *value as i8).collect())
}

fn read_archive_member(
    archive: &mut ZipArchive<File>,
    name: &'static str,
) -> Result<Vec<u8>, G27GeometricFractionalError> {
    let mut member = archive
        .by_name(name)
        .map_err(|error| G27GeometricFractionalError::MatrixZip(error.to_string()))?;
    let mut bytes = Vec::with_capacity(member.size() as usize);
    member
        .read_to_end(&mut bytes)
        .map_err(|error| G27GeometricFractionalError::MatrixZip(error.to_string()))?;
    Ok(bytes)
}

fn parse_npy_payload<'a>(
    bytes: &'a [u8],
    dtype: &'static str,
    source: &'static str,
) -> Result<&'a [u8], G27GeometricFractionalError> {
    if bytes.len() < 10 || &bytes[0..6] != b"\x93NUMPY" {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch { source });
    }
    let major = bytes[6];
    let (header_len, payload_start) = match major {
        1 => (u16::from_le_bytes([bytes[8], bytes[9]]) as usize, 10usize),
        2 | 3 => {
            if bytes.len() < 12 {
                return Err(G27GeometricFractionalError::MatrixShapeMismatch { source });
            }
            (
                u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize,
                12usize,
            )
        }
        _ => return Err(G27GeometricFractionalError::MatrixShapeMismatch { source }),
    };
    let header_end = payload_start + header_len;
    if bytes.len() < header_end {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch { source });
    }
    let header = std::str::from_utf8(&bytes[payload_start..header_end])
        .map_err(|_| G27GeometricFractionalError::MatrixShapeMismatch { source })?;
    if !header.contains(&format!("'descr': '{dtype}'"))
        && !header.contains(&format!("\"descr\": \"{dtype}\""))
    {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch { source });
    }
    if !header.contains("'fortran_order': False") && !header.contains("\"fortran_order\": False") {
        return Err(G27GeometricFractionalError::MatrixShapeMismatch { source });
    }
    Ok(&bytes[header_end..])
}

fn parse_bigint(value: &str) -> Result<BigInt, G27GeometricFractionalError> {
    BigInt::parse_bytes(value.as_bytes(), 10)
        .ok_or(G27GeometricFractionalError::WitnessShapeMismatch)
}
