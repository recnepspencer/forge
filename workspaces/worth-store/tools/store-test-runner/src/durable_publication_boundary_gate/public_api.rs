use std::collections::BTreeSet;

use super::read_repository_document;

const API_DOCUMENT: &str = "_docs/worth-store/physical-reconstruction-c7-public-api.csv";
const HEADER: &str =
    "surface,path,source_anchor,current_semantics,disposition,destination_owner,phase";
const INHERITED_SURFACES: [&str; 21] = [
    "AcknowledgmentPrecondition",
    "CheckpointCutoverReceipt",
    "CheckpointPublicationPlan",
    "DurableAckReceipt",
    "DurablePublicationDeclaration",
    "PageLsn",
    "PhysicalPublicationReceipt",
    "PhysicalRecordSubmission",
    "PhysicalRecordSubmission::append_batch",
    "PhysicalRecordSubmission::append_batch_reconstructing_manifest_capacity",
    "PhysicalRecordSubmission::prepare_append",
    "PhysicalRootPublicationRuntime",
    "PhysicalWritebackSettlement",
    "PreparedRecordAppend",
    "PublishedRecordBatch",
    "StoreDurabilityAdmission",
    "StoreDurabilityExecutionProof",
    "StoreDurabilityRuntime",
    "WalAppendPlanner",
    "WalDurabilityBarrierReceipt",
    "WalRetentionEligibility",
];
const PHASE_TWO_SURFACES: [&str; 40] = [
    "AdmittedPhysicalDurabilityPolicy",
    "CheckpointMemoryLimit",
    "GroupCommitDelay",
    "GroupCommitLimit",
    "IdempotencyRetentionGenerations",
    "PendingUnresolvedMutationLimit",
    "PhysicalCheckpointPolicy",
    "PhysicalDurabilityDeclaration",
    "PhysicalDurabilityDeclarationBuilder",
    "PhysicalDurabilityObservation",
    "PhysicalDurabilityPolicyAdmissionOutcome",
    "PhysicalDurabilityPolicyDeferred",
    "PhysicalDurabilityPolicyDenial",
    "PhysicalDurabilityPolicyFailure",
    "PhysicalDurabilityPolicyIdentity",
    "PhysicalDurabilityPolicyRebindRequired",
    "PhysicalDurabilityPolicyStale",
    "PhysicalIdempotencyPolicy",
    "PhysicalMutationAdmissionDisposition",
    "PhysicalMutationDeadline",
    "PhysicalMutationIdempotencyIssuanceDenial",
    "PhysicalMutationIdempotencyKey",
    "PhysicalMutationIdempotencyKeyIdentity",
    "PhysicalMutationIdempotencyLease",
    "PhysicalMutationIdempotencyMaterial",
    "PhysicalMutationIdentity",
    "PhysicalMutationPreparationDeferred",
    "PhysicalMutationPreparationDenial",
    "PhysicalMutationPreparationFailure",
    "PhysicalMutationPreparationOutcome",
    "PhysicalMutationPreparationRebindRequired",
    "PhysicalMutationPreparationStale",
    "PhysicalMutationRequest",
    "PhysicalMutationRequestFingerprint",
    "PhysicalMutationResourceShape",
    "PhysicalNamespaceDurableCheckpointGeneration",
    "PhysicalRecordSubmission::issue_idempotency_key",
    "PhysicalRecordSubmission::prepare_durable_append",
    "PreparedPhysicalMutation",
    "RetainedWalTailLimit",
];
const PHASE_THREE_SURFACES: [&str; 19] = [
    "CanonicalRedoRecords",
    "PhysicalRecordSubmission::append_prepared_wal",
    "PhysicalRecordSubmission::wal_observation",
    "PhysicalWalAppendDeclaration",
    "PhysicalWalAppendFailureCause",
    "PhysicalWalAppendOutcome",
    "PhysicalWalAppendSettlement",
    "PhysicalWalAppendSettlement::matches_completion_binding",
    "PhysicalWalMemberBasis",
    "PhysicalWalMemberIdentity",
    "PhysicalWalObservation",
    "PhysicalWalReservationDenial",
    "PlannedWalFrameAppend",
    "RedoRecord",
    "WalAppendFrontier",
    "WalAppendedPhysicalMutation",
    "WalFramePlanningDenial",
    "WalRangeReservedPhysicalMutation",
    "plan_wal_frame_append",
];
const PHASE_TWO_DURABILITY_ROOT_EXPORTS: [&str; 29] = [
    "AdmittedPhysicalDurabilityPolicy",
    "CheckpointMemoryLimit",
    "GroupCommitDelay",
    "GroupCommitLimit",
    "IdempotencyRetentionGenerations",
    "PendingUnresolvedMutationLimit",
    "PhysicalCheckpointPolicy",
    "PhysicalDurabilityDeclaration",
    "PhysicalDurabilityDeclarationBuilder",
    "PhysicalDurabilityObservation",
    "PhysicalDurabilityPolicyAdmissionOutcome",
    "PhysicalDurabilityPolicyDeferred",
    "PhysicalDurabilityPolicyDenial",
    "PhysicalDurabilityPolicyFailure",
    "PhysicalDurabilityPolicyIdentity",
    "PhysicalDurabilityPolicyRebindRequired",
    "PhysicalDurabilityPolicyStale",
    "PhysicalIdempotencyPolicy",
    "PhysicalMutationDeadline",
    "PhysicalMutationIdempotencyIssuanceDenial",
    "PhysicalMutationIdempotencyKey",
    "PhysicalMutationIdempotencyKeyIdentity",
    "PhysicalMutationIdempotencyLease",
    "PhysicalMutationIdempotencyMaterial",
    "PhysicalMutationIdentity",
    "PhysicalMutationRequest",
    "PhysicalMutationRequestFingerprint",
    "PhysicalNamespaceDurableCheckpointGeneration",
    "RetainedWalTailLimit",
];
const PHASE_TWO_PREPARATION_EXPORTS: [&str; 9] = [
    "PhysicalMutationAdmissionDisposition",
    "PhysicalMutationPreparationDeferred",
    "PhysicalMutationPreparationDenial",
    "PhysicalMutationPreparationFailure",
    "PhysicalMutationPreparationOutcome",
    "PhysicalMutationPreparationRebindRequired",
    "PhysicalMutationPreparationStale",
    "PhysicalMutationResourceShape",
    "PreparedPhysicalMutation",
];

#[test]
fn every_locked_public_surface_resolves_and_has_one_final_disposition() {
    let document = read_repository_document(API_DOCUMENT).expect("read C.7 public API inventory");
    let rows = parse_api(&document).expect("parse C.7 public API inventory");
    let mut surfaces = BTreeSet::new();
    for row in rows {
        assert!(
            surfaces.insert(row.surface.clone()),
            "duplicate C.7 API disposition for {}",
            row.surface
        );
        let source = read_repository_document(&format!("workspaces/worth-store/{}", row.path))
            .unwrap_or_else(|denial| panic!("{denial}"));
        assert!(
            source.contains(&row.anchor),
            "C.7 API `{}` lost source anchor `{}`",
            row.surface,
            row.anchor
        );
        assert!(!row.current_semantics.is_empty());
        assert!(!row.destination_owner.is_empty());
        assert!(matches!(
            row.disposition.as_str(),
            "preserve" | "narrow" | "move" | "replace" | "delete"
        ));
        assert!(matches!(
            row.phase.as_str(),
            "phase-2" | "phase-3" | "phase-4" | "phase-6" | "phase-7" | "phase-8"
        ));
    }
    let expected = INHERITED_SURFACES
        .into_iter()
        .chain(PHASE_TWO_SURFACES)
        .chain(PHASE_THREE_SURFACES)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        expected.len(),
        INHERITED_SURFACES.len() + PHASE_TWO_SURFACES.len() + PHASE_THREE_SURFACES.len(),
        "C.7 public API boundary sets contain a duplicate"
    );
    let actual = surfaces.iter().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(
        actual, expected,
        "C.7 public API inventory must equal the locked boundary set"
    );
    assert_phase_two_facade_reachability();
    assert_phase_three_facade_reachability();
}

fn assert_phase_three_facade_reachability() {
    let runtime = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    )
    .expect("read physical runtime facade");
    let durability_exports = export_block(&runtime, "pub use durability::{");
    for surface in [
        "CanonicalRedoRecords",
        "PhysicalWalAppendDeclaration",
        "PhysicalWalAppendFailureCause",
        "PhysicalWalAppendOutcome",
        "PhysicalWalAppendSettlement",
        "PhysicalWalMemberBasis",
        "PhysicalWalMemberIdentity",
        "PhysicalWalObservation",
        "PhysicalWalReservationDenial",
        "RedoRecord",
        "WalAppendedPhysicalMutation",
        "WalRangeReservedPhysicalMutation",
    ] {
        assert!(
            durability_exports.contains(surface),
            "Phase 3 Store surface `{surface}` is not exported by physical_runtime"
        );
    }

    let wal = read_repository_document("workspaces/worth-store/crates/worth-store-wal/src/lib.rs")
        .expect("read WAL facade");
    let append_exports = export_block(&wal, "pub use append::{");
    for surface in [
        "WalAppendFrontier",
        "PlannedWalFrameAppend",
        "WalFramePlanningDenial",
        "plan_wal_frame_append",
    ] {
        assert!(
            append_exports.contains(surface),
            "Phase 3 WAL surface `{surface}` is not exported by worth-store-wal"
        );
    }

    let submission = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    )
    .expect("read submission facade");
    assert!(submission.contains("pub fn append_prepared_wal("));
    assert!(submission.contains("pub fn wal_observation("));
}

fn assert_phase_two_facade_reachability() {
    let runtime = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    )
    .expect("read physical runtime facade");
    let durability_exports = export_block(&runtime, "pub use durability::{");
    for surface in PHASE_TWO_DURABILITY_ROOT_EXPORTS {
        assert!(
            durability_exports.contains(surface),
            "Phase 2 durability surface `{surface}` is not exported by physical_runtime"
        );
    }
    assert!(runtime.contains("pub use record_serving::*;"));

    let serving = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/mod.rs",
    )
    .expect("read record-serving facade");
    let serving_exports = export_block(&serving, "pub use publication::{");
    assert!(serving_exports.contains("PhysicalRecordSubmission"));
    for surface in PHASE_TWO_PREPARATION_EXPORTS {
        assert!(
            serving_exports.contains(surface),
            "Phase 2 preparation surface `{surface}` is not exported by record_serving"
        );
    }

    let publication = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    )
    .expect("read publication facade");
    assert!(
        publication.contains("pub use director::{PhysicalRecordSubmission, PreparedRecordAppend};")
    );
    let preparation_exports = export_block(&publication, "pub use durable_preparation::{");
    for surface in PHASE_TWO_PREPARATION_EXPORTS {
        assert!(
            preparation_exports.contains(surface),
            "Phase 2 preparation surface `{surface}` is not exported by publication"
        );
    }
}

fn export_block<'a>(source: &'a str, start: &str) -> &'a str {
    source
        .split_once(start)
        .and_then(|(_, tail)| tail.split_once("};"))
        .map(|(body, _)| body)
        .unwrap_or_else(|| panic!("public facade lost export block `{start}`"))
}

fn parse_api(document: &str) -> Result<Vec<ApiRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.7 public API inventory has an invalid schema".to_owned());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
            if columns.len() != 7 || columns.iter().any(|column| column.is_empty()) {
                return Err(format!("invalid C.7 public API row {}", index + 2));
            }
            Ok(ApiRow {
                surface: columns[0].to_owned(),
                path: columns[1].to_owned(),
                anchor: columns[2].to_owned(),
                current_semantics: columns[3].to_owned(),
                disposition: columns[4].to_owned(),
                destination_owner: columns[5].to_owned(),
                phase: columns[6].to_owned(),
            })
        })
        .collect()
}

struct ApiRow {
    surface: String,
    path: String,
    anchor: String,
    current_semantics: String,
    disposition: String,
    destination_owner: String,
    phase: String,
}
