use std::collections::{BTreeMap, BTreeSet};

use super::super::posture_resolution::WorthGraphReadAccessResolvedPosture;
use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureFamilyCount {
    family: String,
    observed_count: usize,
    cap_count: usize,
    row_digest: String,
}

impl WorthGraphReadAccessPostureFamilyCount {
    pub(crate) fn new(family: String, observed_count: usize, cap_count: usize) -> Self {
        let row_digest = stable_digest(&[
            "worth_graph_read_access_posture_family_count_v1".to_string(),
            format!("family:{family}"),
            format!("observed_count:{observed_count}"),
            format!("cap_count:{cap_count}"),
        ]);
        Self {
            family,
            observed_count,
            cap_count,
            row_digest,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub const fn observed_count(&self) -> usize {
        self.observed_count
    }

    pub const fn cap_count(&self) -> usize {
        self.cap_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn count_posture_families(
    rows: &[WorthGraphReadAccessResolvedPosture],
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        let mut row_families = BTreeSet::new();
        row_families.insert(row.posture_family().to_string());
        if let Some(denial_kind) = row.denial_kind() {
            row_families.insert(denial_kind.to_string());
        }
        for family in row_families {
            *counts.entry(family).or_insert(0) += 1;
        }
    }
    counts
}
