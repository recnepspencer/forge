use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::WorthQueryEntity;

use super::{WorthQueryComparisonRowChange, WorthQueryComparisonRowChangeFamily};

pub(super) struct WorthQueryComparisonDiffAssembly {
    row_changes: Vec<WorthQueryComparisonRowChange>,
    left_row_scan_count: usize,
    right_row_scan_count: usize,
}

impl WorthQueryComparisonDiffAssembly {
    pub(super) fn row_changes(&self) -> &[WorthQueryComparisonRowChange] {
        &self.row_changes
    }

    pub(super) fn left_row_scan_count(&self) -> usize {
        self.left_row_scan_count
    }

    pub(super) fn right_row_scan_count(&self) -> usize {
        self.right_row_scan_count
    }

    pub(super) fn into_row_changes(self) -> Vec<WorthQueryComparisonRowChange> {
        self.row_changes
    }
}

pub(super) struct WorthQueryComparisonDiffAssemblyFailure {
    reason: &'static str,
    left_row_scan_count: usize,
    right_row_scan_count: usize,
}

impl WorthQueryComparisonDiffAssemblyFailure {
    pub(super) fn reason(&self) -> &'static str {
        self.reason
    }

    pub(super) fn left_row_scan_count(&self) -> usize {
        self.left_row_scan_count
    }

    pub(super) fn right_row_scan_count(&self) -> usize {
        self.right_row_scan_count
    }
}

pub(super) fn assemble_query_shaped_row_changes(
    left_rows: &[WorthQueryEntity],
    right_rows: &[WorthQueryEntity],
) -> Result<WorthQueryComparisonDiffAssembly, WorthQueryComparisonDiffAssemblyFailure> {
    let left = index_unique_rows(left_rows).map_err(|scanned| {
        WorthQueryComparisonDiffAssemblyFailure {
            reason: "left comparison result contains duplicate typed row identity",
            left_row_scan_count: scanned,
            right_row_scan_count: 0,
        }
    })?;
    let right = index_unique_rows(right_rows).map_err(|scanned| {
        WorthQueryComparisonDiffAssemblyFailure {
            reason: "right comparison result contains duplicate typed row identity",
            left_row_scan_count: left_rows.len(),
            right_row_scan_count: scanned,
        }
    })?;
    let row_changes = changed_identity_union(&left, &right);
    Ok(WorthQueryComparisonDiffAssembly {
        row_changes,
        left_row_scan_count: left_rows.len(),
        right_row_scan_count: right_rows.len(),
    })
}

fn index_unique_rows(
    rows: &[WorthQueryEntity],
) -> Result<BTreeMap<String, &WorthQueryEntity>, usize> {
    let mut indexed = BTreeMap::new();
    for (ordinal, row) in rows.iter().enumerate() {
        let identity = row.identity().evidence_identity().as_str().to_string();
        if indexed.insert(identity, row).is_some() {
            return Err(ordinal + 1);
        }
    }
    Ok(indexed)
}

fn changed_identity_union(
    left: &BTreeMap<String, &WorthQueryEntity>,
    right: &BTreeMap<String, &WorthQueryEntity>,
) -> Vec<WorthQueryComparisonRowChange> {
    left.keys()
        .chain(right.keys())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter_map(|identity| classify_row_change(left.get(&identity), right.get(&identity)))
        .collect()
}

fn classify_row_change(
    left: Option<&&WorthQueryEntity>,
    right: Option<&&WorthQueryEntity>,
) -> Option<WorthQueryComparisonRowChange> {
    match (left, right) {
        (Some(left), Some(right)) if left == right => None,
        (Some(left), Some(right)) => Some(WorthQueryComparisonRowChange::new(
            WorthQueryComparisonRowChangeFamily::Modified,
            Some((*left).clone()),
            Some((*right).clone()),
        )),
        (Some(left), None) => Some(WorthQueryComparisonRowChange::new(
            WorthQueryComparisonRowChangeFamily::Removed,
            Some((*left).clone()),
            None,
        )),
        (None, Some(right)) => Some(WorthQueryComparisonRowChange::new(
            WorthQueryComparisonRowChangeFamily::Added,
            None,
            Some((*right).clone()),
        )),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::assemble_query_shaped_row_changes;
    use crate::memory_workspace::{admit_authored_entity_label, WorthQueryEntity};

    #[test]
    fn duplicate_left_identity_fails_closed_at_exact_scan_width() {
        let row = test_row("duplicate-left");
        let failure = assemble_query_shaped_row_changes(&[row.clone(), row], &[])
            .err()
            .expect("duplicate identity must deny assembly");

        assert_eq!(failure.left_row_scan_count(), 2);
        assert_eq!(failure.right_row_scan_count(), 0);
        assert!(failure.reason().contains("left"));
    }

    #[test]
    fn duplicate_right_identity_fails_closed_after_left_indexing() {
        let left = test_row("unique-left");
        let right = test_row("duplicate-right");
        let failure = assemble_query_shaped_row_changes(&[left], &[right.clone(), right])
            .err()
            .expect("duplicate identity must deny assembly");

        assert_eq!(failure.left_row_scan_count(), 1);
        assert_eq!(failure.right_row_scan_count(), 2);
        assert!(failure.reason().contains("right"));
    }

    fn test_row(label: &str) -> WorthQueryEntity {
        WorthQueryEntity::from_native_field_values(
            admit_authored_entity_label(label),
            BTreeMap::new(),
        )
    }
}
