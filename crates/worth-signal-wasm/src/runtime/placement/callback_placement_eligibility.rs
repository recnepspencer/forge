use serde::Serialize;

use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::core::certification_digest::canonical_certification_digest;

use super::declaration_candidate::{PlacementDeclarationCandidate, PlacementDeclarationOrigin};
use super::declaration_classification::classify_declaration_placement;
use super::placement_category::WorkerPlacementCategory;
use worth_proof::TransitionOutcome;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCallbackPlacementEligibilityPackage {
    pub certification_family: &'static str,
    pub callback_declaration_count: u64,
    pub worker_executable_callback_count: u64,
    pub main_thread_hosted_callback_count: u64,
    pub unavailable_callback_count: u64,
    pub fallback_count: u64,
    pub raw_callback_transport_denied: bool,
    pub broad_placement_collapse_denied: bool,
    pub placement_digest: String,
    pub denial_digest: String,
    pub fallback_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub performance_digest: String,
    pub rows: Vec<WorkerCallbackPlacementEligibilityRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCallbackPlacementEligibilityRow {
    pub declaration_id: String,
    pub signal_kind: String,
    pub declaration_origin: String,
    pub category: WorkerPlacementCategory,
    pub outcome: String,
    pub reason: String,
    pub callback_runtime_read_count: u64,
    pub host_capability_read_count: u64,
    pub main_thread_hosted_lane_requires_closed_request: bool,
    pub unavailable_artifact_emitted: bool,
}

pub(crate) fn certify_callback_placement_eligibility(
    declarations: Vec<PlacementDeclarationCandidate>,
) -> Result<WorkerCallbackPlacementEligibilityPackage, WorthSignalJsError> {
    let rows = classify_callback_declarations(declarations);
    let counts = CallbackPlacementEligibilityCounts::from_rows(&rows);

    Ok(WorkerCallbackPlacementEligibilityPackage {
        certification_family: "callbackPlacementEligibility",
        callback_declaration_count: counts.callback_declaration_count,
        worker_executable_callback_count: counts.worker_executable_callback_count,
        main_thread_hosted_callback_count: counts.main_thread_hosted_callback_count,
        unavailable_callback_count: counts.unavailable_callback_count,
        fallback_count: 0,
        raw_callback_transport_denied: true,
        broad_placement_collapse_denied: counts
            .main_thread_hosted_callback_count
            .saturating_add(counts.unavailable_callback_count)
            > 0,
        placement_digest: canonical_certification_digest(&("callbackPlacement", &rows))?,
        denial_digest: canonical_certification_digest(&(
            "callbackPlacementDenials",
            counts.main_thread_hosted_callback_count,
            counts.unavailable_callback_count,
            &rows,
        ))?,
        fallback_digest: canonical_certification_digest(&(
            "callbackPlacementFallback",
            0_u64,
            "undeclaredFallbackDenied",
        ))?,
        capability_availability_digest: canonical_certification_digest(&(
            "callbackCapabilityAvailability",
            counts.unavailable_callback_count,
            &rows,
        ))?,
        replay_import_compatibility_digest: canonical_certification_digest(&(
            "callbackReplayImportCompatibility",
            counts.worker_executable_callback_count,
            counts.main_thread_hosted_callback_count,
            counts.unavailable_callback_count,
            "sameRuntimeCallbackReattachmentRequired",
            "portableImportEmitsUnavailability",
            &rows,
        ))?,
        placement_identity_digest: canonical_certification_digest(&(
            "callbackPlacementIdentity",
            rows.iter()
                .map(|row| {
                    (
                        row.declaration_id.as_str(),
                        row.declaration_origin.as_str(),
                        row.signal_kind.as_str(),
                        row.category,
                    )
                })
                .collect::<Vec<_>>(),
        ))?,
        performance_digest: canonical_certification_digest(&(
            "callbackPlacementClassificationCost",
            counts.callback_declaration_count,
            counts.callback_runtime_read_count,
            counts.host_capability_read_count,
        ))?,
        rows,
    })
}

fn classify_callback_declarations(
    declarations: Vec<PlacementDeclarationCandidate>,
) -> Vec<WorkerCallbackPlacementEligibilityRow> {
    declarations
        .into_iter()
        .filter(callback_declaration_requires_worker_placement_certification)
        .map(classify_callback_declaration)
        .collect()
}

fn callback_declaration_requires_worker_placement_certification(
    declaration: &PlacementDeclarationCandidate,
) -> bool {
    declaration.has_live_callback
        || declaration.is_unavailable
        || matches!(
            declaration.origin,
            PlacementDeclarationOrigin::CallbackConstantizedNoSignalReads
                | PlacementDeclarationOrigin::CallbackSignalTracked
        )
}

fn classify_callback_declaration(
    declaration: PlacementDeclarationCandidate,
) -> WorkerCallbackPlacementEligibilityRow {
    let callback_runtime_read_count = declaration.callback_runtime_read_count as u64;
    let host_capability_read_count = declaration.host_capability_read_count as u64;
    match classify_declaration_placement(declaration) {
        TransitionOutcome::Success(classified) => {
            let raw = classified.raw.payload();
            WorkerCallbackPlacementEligibilityRow {
                declaration_id: raw.id().to_owned(),
                signal_kind: raw.signal_kind().to_owned(),
                declaration_origin: raw.declaration_origin().to_owned(),
                category: classified.category,
                outcome: "success".to_owned(),
                reason: classified.reason,
                callback_runtime_read_count,
                host_capability_read_count,
                main_thread_hosted_lane_requires_closed_request: false,
                unavailable_artifact_emitted: false,
            }
        }
        TransitionOutcome::Denied(denial) => {
            let raw = denial.raw.payload();
            WorkerCallbackPlacementEligibilityRow {
                declaration_id: raw.id().to_owned(),
                signal_kind: raw.signal_kind().to_owned(),
                declaration_origin: raw.declaration_origin().to_owned(),
                category: denial.category,
                outcome: "denied".to_owned(),
                reason: denial.reason,
                callback_runtime_read_count,
                host_capability_read_count,
                main_thread_hosted_lane_requires_closed_request: matches!(
                    denial.category,
                    WorkerPlacementCategory::MainThreadHosted
                ),
                unavailable_artifact_emitted: matches!(
                    denial.category,
                    WorkerPlacementCategory::Unavailable
                ),
            }
        }
        TransitionOutcome::Deferred(impossible)
        | TransitionOutcome::Stale(impossible)
        | TransitionOutcome::RebindRequired(impossible)
        | TransitionOutcome::Failed(impossible) => match impossible {},
    }
}

#[derive(Default)]
struct CallbackPlacementEligibilityCounts {
    callback_declaration_count: u64,
    worker_executable_callback_count: u64,
    main_thread_hosted_callback_count: u64,
    unavailable_callback_count: u64,
    callback_runtime_read_count: u64,
    host_capability_read_count: u64,
}

impl CallbackPlacementEligibilityCounts {
    fn from_rows(rows: &[WorkerCallbackPlacementEligibilityRow]) -> Self {
        let mut counts = Self::default();
        for row in rows {
            counts.callback_declaration_count = counts.callback_declaration_count.saturating_add(1);
            counts.callback_runtime_read_count = counts
                .callback_runtime_read_count
                .saturating_add(row.callback_runtime_read_count);
            counts.host_capability_read_count = counts
                .host_capability_read_count
                .saturating_add(row.host_capability_read_count);
            match row.category {
                WorkerPlacementCategory::WorkerExecutable => {
                    counts.worker_executable_callback_count =
                        counts.worker_executable_callback_count.saturating_add(1);
                }
                WorkerPlacementCategory::MainThreadHosted => {
                    counts.main_thread_hosted_callback_count =
                        counts.main_thread_hosted_callback_count.saturating_add(1);
                }
                WorkerPlacementCategory::Unavailable => {
                    counts.unavailable_callback_count =
                        counts.unavailable_callback_count.saturating_add(1);
                }
            }
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::core::WebSignalKind;

    use super::*;

    fn callback_candidate(
        id: &str,
        origin: PlacementDeclarationOrigin,
    ) -> PlacementDeclarationCandidate {
        PlacementDeclarationCandidate {
            id: id.to_owned(),
            signal_kind: Some(WebSignalKind::Computed),
            origin,
            has_live_callback: true,
            callback_runtime_read_count: 0,
            host_capability_read_count: 0,
            is_unavailable: false,
        }
    }

    #[test]
    fn eligibility_matrix_emits_typed_denials_without_fallback() {
        let mut portable = callback_candidate(
            "portableCallback",
            PlacementDeclarationOrigin::CallbackConstantizedNoSignalReads,
        );
        portable.has_live_callback = false;
        let mut signal_bound = callback_candidate(
            "signalBoundCallback",
            PlacementDeclarationOrigin::CallbackSignalTracked,
        );
        signal_bound.callback_runtime_read_count = 2;
        let mut host_bound = callback_candidate(
            "hostBoundCallback",
            PlacementDeclarationOrigin::CallbackSignalTracked,
        );
        host_bound.host_capability_read_count = 1;
        let mut unavailable = callback_candidate(
            "unavailableCallback",
            PlacementDeclarationOrigin::CallbackSignalTracked,
        );
        unavailable.is_unavailable = true;

        let package = certify_callback_placement_eligibility(vec![
            portable,
            signal_bound,
            host_bound,
            unavailable,
        ])
        .unwrap();

        assert_eq!(package.callback_declaration_count, 4);
        assert_eq!(package.worker_executable_callback_count, 1);
        assert_eq!(package.main_thread_hosted_callback_count, 2);
        assert_eq!(package.unavailable_callback_count, 1);
        assert_eq!(package.fallback_count, 0);
        assert!(package.raw_callback_transport_denied);
        assert!(package.broad_placement_collapse_denied);

        let unavailable = package
            .rows
            .iter()
            .find(|row| row.declaration_id == "unavailableCallback")
            .expect("unavailable callback row exists");
        assert_eq!(unavailable.category, WorkerPlacementCategory::Unavailable);
        assert!(unavailable.unavailable_artifact_emitted);
        assert!(!unavailable.main_thread_hosted_lane_requires_closed_request);

        let host_bound = package
            .rows
            .iter()
            .find(|row| row.declaration_id == "hostBoundCallback")
            .expect("host-bound callback row exists");
        assert_eq!(
            host_bound.category,
            WorkerPlacementCategory::MainThreadHosted
        );
        assert_eq!(host_bound.host_capability_read_count, 1);
        assert!(host_bound.main_thread_hosted_lane_requires_closed_request);
        assert!(host_bound.reason.contains("typed host capabilities"));
    }

    #[test]
    fn eligibility_identity_preserves_equivalent_callback_postures() {
        let process_local = callback_candidate(
            "sharedShapeProcessLocal",
            PlacementDeclarationOrigin::CallbackSignalTracked,
        );
        let mut host_bound = callback_candidate(
            "sharedShapeHostBound",
            PlacementDeclarationOrigin::CallbackSignalTracked,
        );
        host_bound.host_capability_read_count = 1;

        let package =
            certify_callback_placement_eligibility(vec![host_bound, process_local]).unwrap();

        assert_eq!(
            package
                .rows
                .iter()
                .map(|row| (
                    row.declaration_id.as_str(),
                    row.category,
                    row.host_capability_read_count,
                ))
                .collect::<Vec<_>>(),
            [
                (
                    "sharedShapeHostBound",
                    WorkerPlacementCategory::MainThreadHosted,
                    1,
                ),
                (
                    "sharedShapeProcessLocal",
                    WorkerPlacementCategory::MainThreadHosted,
                    0,
                ),
            ]
        );
        assert_ne!(
            package.rows[0].reason.as_str(),
            package.rows[1].reason.as_str()
        );
    }
}
