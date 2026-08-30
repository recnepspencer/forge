use super::super::{
    WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind,
};

pub(in crate::domain_computation::primary_graph::conditional_operation) fn bridge_reconstruction_denial(
    detail: impl Into<String>,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    reconstruction_denial(
        WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionIntent,
        detail,
    )
}

pub(in crate::domain_computation::primary_graph::conditional_operation) fn snapshot_capacity_reconstruction_denial(
    maximum_active_snapshots: usize,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    reconstruction_denial(
        WorthQueryConditionalRuntimeInstallationDenialKind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        },
        "conditional reconstruction deferred by the relational snapshot capacity owner",
    )
}

pub(in crate::domain_computation::primary_graph::conditional_operation) fn retention_capacity_reconstruction_denial(
) -> WorthQueryConditionalRuntimeInstallationDenial {
    reconstruction_denial(
        WorthQueryConditionalRuntimeInstallationDenialKind::RetentionCapacityExhausted,
        "conditional reconstruction deferred by the relational retention owner",
    )
}

pub(in crate::domain_computation::primary_graph::conditional_operation) fn retention_identity_reconstruction_denial(
) -> WorthQueryConditionalRuntimeInstallationDenial {
    reconstruction_denial(
        WorthQueryConditionalRuntimeInstallationDenialKind::RetentionIdentityExhausted,
        "conditional reconstruction exhausted relational retention identity space",
    )
}

pub(in crate::domain_computation::primary_graph::conditional_operation) fn snapshot_identity_reconstruction_denial(
) -> WorthQueryConditionalRuntimeInstallationDenial {
    reconstruction_denial(
        WorthQueryConditionalRuntimeInstallationDenialKind::SnapshotIdentityExhausted,
        "conditional reconstruction exhausted relational snapshot identity space",
    )
}

pub(in crate::domain_computation::primary_graph::conditional_operation) fn reconstruction_denial(
    kind: WorthQueryConditionalRuntimeInstallationDenialKind,
    detail: impl Into<String>,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    WorthQueryConditionalRuntimeInstallationDenial::new(kind, detail)
}

pub(super) fn principal(
    denial: crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionDenial,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionDenialKind as Kind;
    match denial.kind() {
        Kind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        } => snapshot_capacity_reconstruction_denial(maximum_active_snapshots),
        Kind::RetentionCapacityExhausted => retention_capacity_reconstruction_denial(),
        Kind::RetentionIdentityExhausted => retention_identity_reconstruction_denial(),
        Kind::SnapshotIdentityExhausted => snapshot_identity_reconstruction_denial(),
        _ => reconstruction_denial(
            WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionPrincipal,
            format!("{:?}: {}", denial.kind(), denial.binding()),
        ),
    }
}

pub(super) fn entity(
    denial: crate::domain_computation::primary_graph::WorthQueryEntityResolutionDenial,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    use crate::domain_computation::primary_graph::WorthQueryEntityResolutionDenialKind as Kind;
    match denial.kind() {
        Kind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        } => snapshot_capacity_reconstruction_denial(maximum_active_snapshots),
        Kind::RetentionCapacityExhausted => retention_capacity_reconstruction_denial(),
        Kind::RetentionIdentityExhausted => retention_identity_reconstruction_denial(),
        Kind::SnapshotIdentityExhausted => snapshot_identity_reconstruction_denial(),
        _ => reconstruction_denial(
            WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionScope,
            format!("{:?}: {}", denial.kind(), denial.subject()),
        ),
    }
}

pub(super) fn query_authorization(
    denial: super::super::WorthQueryTemporalQueryAuthorizationDenial,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    use super::super::WorthQueryTemporalQueryAuthorizationDenial as Denial;
    use crate::domain_computation::primary_graph::{
        WorthQueryApplicationQueryAdmissionDenialKind as Query,
        WorthQueryOperationAuthorizationDenialKind as Authorization,
    };
    let snapshot_maximum = match &denial {
        Denial::Query(query) => match query.kind() {
            Query::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            }
            | Query::Authorization(Authorization::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            }) => Some(maximum_active_snapshots),
            _ => None,
        },
        Denial::Authorization(authorization) => match authorization.kind() {
            Authorization::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            } => Some(maximum_active_snapshots),
            _ => None,
        },
    };
    let retention_exhausted = match &denial {
        Denial::Query(query) => matches!(
            query.kind(),
            Query::RetentionCapacityExhausted
                | Query::Authorization(Authorization::RetentionCapacityExhausted)
        ),
        Denial::Authorization(authorization) => {
            authorization.kind() == Authorization::RetentionCapacityExhausted
        }
    };
    if retention_exhausted {
        return retention_capacity_reconstruction_denial();
    }
    let retention_identity_exhausted = match &denial {
        Denial::Query(query) => matches!(
            query.kind(),
            Query::RetentionIdentityExhausted
                | Query::Authorization(Authorization::RetentionIdentityExhausted)
        ),
        Denial::Authorization(authorization) => {
            authorization.kind() == Authorization::RetentionIdentityExhausted
        }
    };
    if retention_identity_exhausted {
        return retention_identity_reconstruction_denial();
    }
    let snapshot_identity_exhausted = match &denial {
        Denial::Query(query) => matches!(
            query.kind(),
            Query::SnapshotIdentityExhausted
                | Query::Authorization(Authorization::SnapshotIdentityExhausted)
        ),
        Denial::Authorization(authorization) => {
            authorization.kind() == Authorization::SnapshotIdentityExhausted
        }
    };
    if snapshot_identity_exhausted {
        return snapshot_identity_reconstruction_denial();
    }
    snapshot_maximum.map_or_else(
        || {
            reconstruction_denial(
                WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionQuery,
                denial.to_string(),
            )
        },
        snapshot_capacity_reconstruction_denial,
    )
}

pub(super) fn one_shot(
    denial: crate::domain_computation::primary_graph::WorthQueryApplicationOneShotDenial,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    use crate::domain_computation::primary_graph::WorthQueryApplicationOneShotDenialKind as Kind;
    match denial.kind() {
        Kind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        } => snapshot_capacity_reconstruction_denial(maximum_active_snapshots),
        Kind::RetentionCapacityExhausted => retention_capacity_reconstruction_denial(),
        Kind::RetentionIdentityExhausted => retention_identity_reconstruction_denial(),
        Kind::SnapshotIdentityExhausted => snapshot_identity_reconstruction_denial(),
        _ => reconstruction_denial(
            WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionQuery,
            format!("{:?}: {}", denial.kind(), denial.subject()),
        ),
    }
}
