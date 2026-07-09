use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::rebuild_debt_artifact_id,
    },
    failure::{StoreError, StoreErrorKind},
    retention::{RetainedRangeRebuildReport, RetainedReadPath, RetentionTargetStateVerification},
};

use super::{
    derived_family_support::{derived_artifact_exists, milestone_6_family_artifact_count},
    maintenance_verification::maintenance_verification,
};
use crate::backend::retention::basis::retained_cost_surface_for_basis;

pub(crate) fn rebuild_reclaimed_derived_family<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    rebuild_unit: crate::RetainedRangeRebuildUnit,
) -> Result<RetainedRangeRebuildReport, StoreError> {
    let debt_id = rebuild_debt_artifact_id(
        rebuild_unit.family_label(),
        rebuild_unit.retained_basis_label(),
        rebuild_unit.rebuild_target_id(),
    );
    let debt_record = backend
        .state()
        .rebuild_debt_records
        .get(&debt_id)
        .cloned()
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::ReclaimEligibilityViolation,
                format!("rebuild debt `{debt_id}` was not published before rebuild execution"),
            )
        })?;
    if debt_record.cleared {
        let verification = maintenance_verification(
            backend.state(),
            "rebuild_reclaimed_derived_family",
            Some(RetentionTargetStateVerification::new(
                debt_record.family_label.as_str(),
                rebuild_unit.rebuild_target_id(),
                true,
                derived_artifact_exists(
                    backend.state(),
                    debt_record.family_label.as_str(),
                    rebuild_unit.rebuild_target_id(),
                ),
            )),
        )
        .inspect_err(|_| backend.counters().record_retention_restore_parity_failure())?;
        return Ok(RetainedRangeRebuildReport::new(
            rebuild_unit.clone(),
            0,
            retained_cost_surface_for_basis(
                backend,
                debt_record.retained_basis_label.as_str(),
                RetainedReadPath::CanonicalRetainedAuthority,
                0,
                0,
                0,
                0,
            ),
            verification,
        ));
    }

    let rebuilt_artifact_count = match rebuild_unit.family_label() {
        "milestone_6_layout_materialization" => {
            let before =
                milestone_6_family_artifact_count(backend.state(), rebuild_unit.family_label());
            backend.rebuild_milestone_6_derived_artifacts_from_authority()?;
            let after =
                milestone_6_family_artifact_count(backend.state(), rebuild_unit.family_label());
            after.saturating_sub(before)
        }
        "milestone_6_scope_slice_membership"
        | "milestone_6_chunk_membership"
        | "milestone_6_structural_block" => {
            let before =
                milestone_6_family_artifact_count(backend.state(), rebuild_unit.family_label());
            backend.rebuild_milestone_6_derived_artifacts_from_materializations()?;
            let after =
                milestone_6_family_artifact_count(backend.state(), rebuild_unit.family_label());
            after.saturating_sub(before)
        }
        family_label => {
            backend
                .counters()
                .record_retention_artifact_rebuild_failure();
            backend.counters().record_retention_restore_parity_failure();
            return Err(StoreError::new(
                StoreErrorKind::ReclaimEligibilityViolation,
                format!("rebuild execution does not support derived family `{family_label}`"),
            ));
        }
    };

    if !derived_artifact_exists(
        backend.state(),
        rebuild_unit.family_label(),
        rebuild_unit.rebuild_target_id(),
    ) {
        backend
            .counters()
            .record_retention_artifact_rebuild_failure();
        backend.counters().record_retention_restore_parity_failure();
        return Err(StoreError::new(
            StoreErrorKind::ReclaimEligibilityViolation,
            format!(
                "rebuild execution for `{}` did not restore target `{}`",
                rebuild_unit.family_label(),
                rebuild_unit.rebuild_target_id()
            ),
        ));
    }

    let mut next = backend.state().clone();
    if let Some(record) = next.rebuild_debt_records.get_mut(&debt_id) {
        record.cleared = true;
    }
    backend.commit_replacement_state(next)?;
    backend.counters().record_retained_range_rebuild();
    let verification = maintenance_verification(
        backend.state(),
        "rebuild_reclaimed_derived_family",
        Some(RetentionTargetStateVerification::new(
            debt_record.family_label.as_str(),
            rebuild_unit.rebuild_target_id(),
            true,
            derived_artifact_exists(
                backend.state(),
                debt_record.family_label.as_str(),
                rebuild_unit.rebuild_target_id(),
            ),
        )),
    )
    .inspect_err(|_| backend.counters().record_retention_restore_parity_failure())?;

    Ok(RetainedRangeRebuildReport::new(
        rebuild_unit.clone(),
        rebuilt_artifact_count,
        retained_cost_surface_for_basis(
            backend,
            debt_record.retained_basis_label.as_str(),
            RetainedReadPath::CanonicalRetainedAuthority,
            0,
            0,
            0,
            -1,
        ),
        verification,
    ))
}
