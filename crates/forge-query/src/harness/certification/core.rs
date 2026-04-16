use sha2::{Digest, Sha256};

use super::requirements::RequiredAssertionClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostileExpectation {
    EquivalentToControl,
    DistinctFromControl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParityAnchor {
    Control,
    Hostile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalCertificationRow<
    PerturbationClass,
    LaneBundle,
    HostileExpectationClass = HostileExpectation,
> {
    pub row_name: &'static str,
    pub perturbation_class: PerturbationClass,
    pub hostile_expectation: HostileExpectationClass,
    pub parity_anchor: ParityAnchor,
    pub control_lane: LaneBundle,
    pub hostile_lane: LaneBundle,
    pub parity_lane: LaneBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectionCertificationRow<PerturbationClass, LaneBundle, RejectionBundle> {
    pub row_name: &'static str,
    pub perturbation_class: PerturbationClass,
    pub control_lane: LaneBundle,
    pub hostile_lane: RejectionBundle,
    pub parity_lane: LaneBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificationMatrix<
    PerturbationClass,
    LaneBundle,
    RejectionBundle,
    HostileExpectationClass = HostileExpectation,
> {
    pub suite_name: &'static str,
    pub rows:
        Vec<CanonicalCertificationRow<PerturbationClass, LaneBundle, HostileExpectationClass>>,
    pub rejection_rows:
        Vec<RejectionCertificationRow<PerturbationClass, LaneBundle, RejectionBundle>>,
}

pub fn digest_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
}

pub fn contains_row<P, L, R, H>(matrix: &CertificationMatrix<P, L, R, H>, row_name: &str) -> bool {
    matrix.rows.iter().any(|row| row.row_name == row_name)
        || matrix
            .rejection_rows
            .iter()
            .any(|row| row.row_name == row_name)
}

pub fn unmet_required_rows<P, L, R, H>(
    matrix: &CertificationMatrix<P, L, R, H>,
    required_canonical_rows: &[&'static str],
    required_rejection_rows: &[&'static str],
) -> Vec<&'static str> {
    required_canonical_rows
        .iter()
        .chain(required_rejection_rows.iter())
        .copied()
        .filter(|row_name| !contains_row(matrix, row_name))
        .collect()
}

pub fn covered_perturbation_classes<P, L, R, H>(matrix: &CertificationMatrix<P, L, R, H>) -> Vec<P>
where
    P: Copy + Ord,
{
    let mut classes: Vec<_> = matrix
        .rows
        .iter()
        .map(|row| row.perturbation_class)
        .chain(
            matrix
                .rejection_rows
                .iter()
                .map(|row| row.perturbation_class),
        )
        .collect();
    classes.sort();
    classes.dedup();
    classes
}

pub fn unmet_required_assertion_classes(
    covered_assertion_classes: &[RequiredAssertionClass],
    required_assertion_classes: &[RequiredAssertionClass],
) -> Vec<RequiredAssertionClass> {
    required_assertion_classes
        .iter()
        .copied()
        .filter(|class| !covered_assertion_classes.contains(class))
        .collect()
}
