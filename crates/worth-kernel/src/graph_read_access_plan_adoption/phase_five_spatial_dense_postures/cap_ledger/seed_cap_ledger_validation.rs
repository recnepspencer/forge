use std::collections::BTreeMap;

use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPostureCapRow, WorthGraphReadAccessResolvedPosture,
};

use super::super::errors::{
    WorthGraphReadAccessSpatialDensePostureError, WorthGraphReadAccessSpatialDensePostureErrorKind,
};

pub(crate) fn validate_spatial_dense_seed_cap_ledger(
    postures: &[WorthGraphReadAccessResolvedPosture],
    cap_rows: &[WorthGraphReadAccessPostureCapRow],
) -> Result<(), WorthGraphReadAccessSpatialDensePostureError> {
    let observed_family_counts = observed_posture_family_counts(postures, cap_rows);

    for (family, observed_count) in observed_family_counts {
        let Some(cap_row) = cap_rows.iter().find(|row| row.family() == family) else {
            return Err(WorthGraphReadAccessSpatialDensePostureError::new(
                WorthGraphReadAccessSpatialDensePostureErrorKind::RequiredPostureMissingCap,
            ));
        };
        if observed_count > cap_row.max_count() {
            return Err(WorthGraphReadAccessSpatialDensePostureError::new(
                WorthGraphReadAccessSpatialDensePostureErrorKind::RequiredPostureExceedsCap,
            ));
        }
    }
    Ok(())
}

fn observed_posture_family_counts(
    postures: &[WorthGraphReadAccessResolvedPosture],
    cap_rows: &[WorthGraphReadAccessPostureCapRow],
) -> BTreeMap<String, usize> {
    let mut observed_counts = BTreeMap::<String, usize>::new();
    for posture in postures {
        let family = cap_family_for_posture(posture, cap_rows);
        *observed_counts.entry(family).or_default() += 1;
    }
    observed_counts
}

fn cap_family_for_posture(
    posture: &WorthGraphReadAccessResolvedPosture,
    cap_rows: &[WorthGraphReadAccessPostureCapRow],
) -> String {
    [
        posture.posture_family(),
        posture.denial_kind().unwrap_or_default(),
        posture.query_posture(),
        posture.suggested_posture().unwrap_or_default(),
    ]
    .into_iter()
    .find(|candidate| {
        !candidate.is_empty() && cap_rows.iter().any(|row| row.family() == *candidate)
    })
    .unwrap_or(posture.query_posture())
    .to_string()
}
