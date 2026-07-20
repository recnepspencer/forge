use super::error::{WorthQuerySupportSnapshotError, WorthQuerySupportSnapshotErrorKind};

pub(crate) fn admit_support_snapshot_backend_posture(
    backend_posture: &str,
) -> Result<(), WorthQuerySupportSnapshotError> {
    match backend_posture {
        "primary" | "scaffold" => Ok(()),
        found => Err(WorthQuerySupportSnapshotError::with_expected_found(
            WorthQuerySupportSnapshotErrorKind::InvalidBackendPosture,
            "support snapshot backend posture is not part of schema v1",
            "primary|scaffold",
            found,
        )),
    }
}

pub(crate) fn admit_support_snapshot_facade_family(
    surface: &str,
    facade_family: Option<&str>,
) -> Result<(), WorthQuerySupportSnapshotError> {
    match facade_family {
        None => Ok(()),
        Some(
            "read"
            | "live"
            | "computed"
            | "shared-read"
            | "submission"
            | "replay"
            | "effect"
            | "branch-preview"
            | "write"
            | "intent"
            | "inspect"
            | "temporal"
            | "async-resource"
            | "mixed-cause-delivery"
            | "store-backed-execution"
            | "durable-artifacts",
        ) => Ok(()),
        Some(found) => Err(WorthQuerySupportSnapshotError::with_surface_found(
            WorthQuerySupportSnapshotErrorKind::InvalidFacadeFamily,
            "support snapshot facade family is not part of schema v1",
            surface,
            found,
        )),
    }
}

pub(crate) fn admit_support_snapshot_status(
    surface: &str,
    status: &str,
) -> Result<(), WorthQuerySupportSnapshotError> {
    match status {
        "supported" | "deferred-debt" | "unsupported" => Ok(()),
        found => Err(WorthQuerySupportSnapshotError::with_surface_found(
            WorthQuerySupportSnapshotErrorKind::InvalidSupportStatus,
            "support snapshot support status is not part of schema v1",
            surface,
            found,
        )),
    }
}

pub(crate) fn admit_support_snapshot_teaching_posture(
    surface: &str,
    teaching_posture: &str,
) -> Result<(), WorthQuerySupportSnapshotError> {
    match teaching_posture {
        "ordinary-runtime-dx"
        | "visible-but-deferred"
        | "visible-vocabulary-only"
        | "support-gate-only" => Ok(()),
        found => Err(WorthQuerySupportSnapshotError::with_surface_found(
            WorthQuerySupportSnapshotErrorKind::InvalidTeachingPosture,
            "support snapshot teaching posture is not part of schema v1",
            surface,
            found,
        )),
    }
}
