use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub enum AssertionClass {
    Equality,
    Inequality,
    TypedFailure,
    ExactCounter,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LaneResult<T> {
    lane: String,
    payload: T,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CanonicalRow<T> {
    name: String,
    lanes: Vec<LaneResult<T>>,
    assertion_classes: BTreeSet<AssertionClass>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RejectionRow<T> {
    name: String,
    lanes: Vec<LaneResult<T>>,
    assertion_classes: BTreeSet<AssertionClass>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CertificationSuite<T, E> {
    suite_name: String,
    canonical_rows: Vec<CanonicalRow<T>>,
    rejection_rows: Vec<RejectionRow<E>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompletenessReport {
    missing_rows: Vec<String>,
    missing_assertion_classes: Vec<AssertionClass>,
    matrix_digest: String,
}

impl<T> LaneResult<T> {
    pub fn new(lane: impl Into<String>, payload: T) -> Self {
        Self {
            lane: lane.into(),
            payload,
        }
    }

    pub fn lane(&self) -> &str {
        &self.lane
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

impl<T: Serialize, E: Serialize> CertificationSuite<T, E> {
    pub fn new(suite_name: impl Into<String>) -> Self {
        Self {
            suite_name: suite_name.into(),
            canonical_rows: Vec::new(),
            rejection_rows: Vec::new(),
        }
    }

    pub fn with_canonical_row(mut self, row: CanonicalRow<T>) -> Self {
        self.canonical_rows.push(row);
        self
    }

    pub fn with_rejection_row(mut self, row: RejectionRow<E>) -> Self {
        self.rejection_rows.push(row);
        self
    }

    pub fn canonical_rows(&self) -> &[CanonicalRow<T>] {
        &self.canonical_rows
    }

    pub fn rejection_rows(&self) -> &[RejectionRow<E>] {
        &self.rejection_rows
    }

    pub fn matrix_digest(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("certification suite should serialize");
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        format!("{:x}", hasher.finalize())
    }
}

impl<T> CanonicalRow<T> {
    pub fn new(
        name: impl Into<String>,
        lanes: Vec<LaneResult<T>>,
        assertion_classes: &[AssertionClass],
    ) -> Self {
        Self {
            name: name.into(),
            lanes,
            assertion_classes: assertion_classes.iter().copied().collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lanes(&self) -> &[LaneResult<T>] {
        &self.lanes
    }

    pub fn assertion_classes(&self) -> &BTreeSet<AssertionClass> {
        &self.assertion_classes
    }
}

impl<T> RejectionRow<T> {
    pub fn new(
        name: impl Into<String>,
        lanes: Vec<LaneResult<T>>,
        assertion_classes: &[AssertionClass],
    ) -> Self {
        Self {
            name: name.into(),
            lanes,
            assertion_classes: assertion_classes.iter().copied().collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lanes(&self) -> &[LaneResult<T>] {
        &self.lanes
    }

    pub fn assertion_classes(&self) -> &BTreeSet<AssertionClass> {
        &self.assertion_classes
    }
}

impl CompletenessReport {
    pub fn new(
        missing_rows: Vec<String>,
        missing_assertion_classes: Vec<AssertionClass>,
        matrix_digest: String,
    ) -> Self {
        Self {
            missing_rows,
            missing_assertion_classes,
            matrix_digest,
        }
    }

    pub fn missing_rows(&self) -> &[String] {
        &self.missing_rows
    }

    pub fn missing_assertion_classes(&self) -> &[AssertionClass] {
        &self.missing_assertion_classes
    }
}
