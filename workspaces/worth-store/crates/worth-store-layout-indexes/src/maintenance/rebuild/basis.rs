use crate::{CanonicalKeyBytes, LayoutCoverageWitness};
use std::cmp::Ordering;
use worth_proof::CanonicalVec;

use super::denial::DerivedIndexRebuildDenied;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexParityRow {
    key: CanonicalKeyBytes,
    value_fingerprint: String,
}

impl DerivedIndexParityRow {
    pub fn new(key: CanonicalKeyBytes, value_fingerprint: impl Into<String>) -> Self {
        Self {
            key,
            value_fingerprint: value_fingerprint.into(),
        }
    }

    pub const fn key(&self) -> &CanonicalKeyBytes {
        &self.key
    }

    pub fn value_fingerprint(&self) -> &str {
        &self.value_fingerprint
    }
}

impl PartialOrd for DerivedIndexParityRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DerivedIndexParityRow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key
            .as_bytes()
            .cmp(other.key.as_bytes())
            .then_with(|| self.value_fingerprint.cmp(&other.value_fingerprint))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexParityBasis {
    unique_keys: Vec<CanonicalKeyBytes>,
    ordered_rows: CanonicalVec<DerivedIndexParityRow>,
    coverage: LayoutCoverageWitness,
    cost_envelope_compliant: bool,
    counter_shape: CanonicalVec<u64>,
}

impl DerivedIndexParityBasis {
    pub(super) fn from_admitted_source(
        ordered_rows: Vec<DerivedIndexParityRow>,
        coverage: LayoutCoverageWitness,
        cost_envelope_compliant: bool,
        counter_shape: Vec<u64>,
    ) -> Self {
        Self::new(
            ordered_rows,
            coverage,
            cost_envelope_compliant,
            counter_shape,
        )
        .expect("sealed rebuild sources establish canonical rows, unique keys, and counters")
    }

    pub fn new(
        ordered_rows: Vec<DerivedIndexParityRow>,
        coverage: LayoutCoverageWitness,
        cost_envelope_compliant: bool,
        counter_shape: Vec<u64>,
    ) -> Result<Self, DerivedIndexRebuildDenied> {
        let ordered_rows = CanonicalVec::try_from_sorted(ordered_rows)
            .map_err(|_| DerivedIndexRebuildDenied::ParityRowsMustBeCanonical)?;
        let unique_keys = ordered_rows
            .as_slice()
            .iter()
            .map(|row| row.key().clone())
            .collect::<Vec<_>>();
        if unique_keys
            .windows(2)
            .any(|keys| keys[0].as_bytes() >= keys[1].as_bytes())
        {
            return Err(DerivedIndexRebuildDenied::ParityKeysMustBeUnique);
        }
        let counter_shape = CanonicalVec::try_from_sorted(counter_shape)
            .map_err(|_| DerivedIndexRebuildDenied::ParityCounterShapeMustBeCanonical)?;

        Ok(Self {
            unique_keys,
            ordered_rows,
            coverage,
            cost_envelope_compliant,
            counter_shape,
        })
    }

    pub fn unique_keys(&self) -> &[CanonicalKeyBytes] {
        &self.unique_keys
    }

    pub fn ordered_rows(&self) -> &[DerivedIndexParityRow] {
        self.ordered_rows.as_slice()
    }

    pub const fn coverage(&self) -> &LayoutCoverageWitness {
        &self.coverage
    }

    pub const fn cost_envelope_compliant(&self) -> bool {
        self.cost_envelope_compliant
    }

    pub fn counter_shape(&self) -> &[u64] {
        self.counter_shape.as_slice()
    }

    pub fn row_count(&self) -> usize {
        self.ordered_rows.as_slice().len()
    }
}
