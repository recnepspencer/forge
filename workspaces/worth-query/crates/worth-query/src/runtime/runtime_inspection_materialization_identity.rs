use std::collections::BTreeMap;

use crate::memory_workspace::WorthQuerySnapshotIdentity;

use super::{
    WorthQueryDerivedMaterializationResult, WorthQueryDerivedMaterializationTarget,
    WorthQueryRuntimeError,
};

pub(super) fn bundle_snapshot_identity(
    materializations: &BTreeMap<
        WorthQueryDerivedMaterializationTarget,
        WorthQueryDerivedMaterializationResult,
    >,
) -> Result<WorthQuerySnapshotIdentity, WorthQueryRuntimeError> {
    let snapshot_identities = materializations
        .iter()
        .map(|(target, result)| {
            (
                target.terminal_view_name_projection(),
                result.receipt().snapshot_identity().clone(),
            )
        })
        .collect::<Vec<_>>();
    let shared_snapshot_identity = snapshot_identities
        .first()
        .map(|(_, snapshot_identity)| snapshot_identity);
    let has_single_snapshot_identity = shared_snapshot_identity
        .map(|expected| {
            snapshot_identities.iter().all(|(_, snapshot_identity)| {
                expected.is_same_current_identity_as(snapshot_identity)
            })
        })
        .unwrap_or(true);
    match snapshot_identities.as_slice() {
        [] => Ok(WorthQuerySnapshotIdentity::empty_relational_state()),
        [(_, snapshot_identity)] if has_single_snapshot_identity => Ok(snapshot_identity.clone()),
        _ if has_single_snapshot_identity => Ok(snapshot_identities[0].1.clone()),
        _ => Err(WorthQueryRuntimeError::RetainedRowDecode {
            view_name: materializations
                .keys()
                .map(|target| target.terminal_view_name_projection().to_string())
                .collect::<Vec<_>>()
                .join("|"),
            stage: "derived-materialization-bundle",
            message: format!(
                "bundle materialized multiple snapshot identities: {}",
                snapshot_identities
                    .iter()
                    .map(|(view_name, snapshot_identity)| {
                        format!(
                            "{view_name}:{}",
                            snapshot_identity
                                .evidence_identity()
                                .terminal_projection_for_reporting()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }),
    }
}
