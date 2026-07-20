use super::support::*;
use crate::application::{
    scan_journal_identity_forbidden_patterns, scan_journal_identity_required_pattern_failures,
    worth_query_journal_identity_inventory, WorthQueryJournalIdentityBoundaryPosture,
    WorthQueryJournalIdentityCertification, WorthQueryJournalIdentityInventoryEvidence,
    WorthQueryJournalIdentityOperationKind, WorthQueryJournalIdentityScheduleEvidence,
};
#[test]
fn journal_identity_inventory_is_seeded_and_scans_clean() {
    let workspace_root = workspace_root();
    let inventory = worth_query_journal_identity_inventory();
    let operations = inventory
        .iter()
        .map(|row| row.kind())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(inventory.iter().all(|row| !row.path().is_empty()));
    assert!(inventory
        .iter()
        .all(|row| !row.required_patterns().is_empty()));
    assert_eq!(missing_operation_count(&operations), 0);
    assert_eq!(
        scan_journal_identity_forbidden_patterns(&workspace_root),
        Vec::new()
    );
    assert_eq!(
        scan_journal_identity_required_pattern_failures(&workspace_root),
        Vec::new()
    );
}

#[test]
fn submitted_write_receipt_carries_typed_committed_journal_position() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.journal-position.single")
        .expect("task runtime should open a named workspace");
    let command = WorthQueryAspectMutationBuilder::new()
        .set_aspect(
            test_aspect_touch("identity.id"),
            test_authored_string_aspect_value("task-1"),
        )
        .set_aspect(
            test_aspect_touch("title.value"),
            test_authored_string_aspect_value("First"),
        )
        .build_insert("Task")
        .expect("insert command should build");
    let receipt = workspace
        .submissions()
        .expect("submission lane should be admitted")
        .submit(command)
        .expect("submission should commit");
    let position = receipt.journal_position();

    assert_eq!(
        position.authority(),
        WorthQueryJournalPositionAuthority::Committed
    );
    assert_eq!(position.ordinal_for_reporting(), 1);
    assert_ne!(
        position.evidence_identity(),
        *receipt.commit_evidence_identity(),
        "journal position identity must not collapse into commit evidence identity"
    );
}

#[test]
fn submitted_schedule_records_monotonic_positions_and_stable_replay() {
    let first_run = submitted_schedule_positions("tasks.journal-position.replay-a");
    let second_run = submitted_schedule_positions("tasks.journal-position.replay-b");

    assert_eq!(first_run.ordinals, vec![1, 2, 3]);
    assert_eq!(first_run.ordinals, second_run.ordinals);
    assert_eq!(
        first_run.evidence_identities,
        second_run.evidence_identities
    );
    assert_eq!(
        first_run.unique_identity_count(),
        first_run.evidence_identities.len()
    );
}

#[test]
fn journal_identity_certification_closes_only_with_real_inventory_and_schedule_evidence() {
    let first_run = submitted_schedule_positions("tasks.journal-position.certification-a");
    let second_run = submitted_schedule_positions("tasks.journal-position.certification-b");
    let first_schedule = first_run.position_schedule();
    let second_schedule = second_run.position_schedule();
    let inventory = journal_identity_inventory_evidence();
    let schedule =
        WorthQueryJournalIdentityScheduleEvidence::derive(&first_schedule, &second_schedule);
    let replay = super::journal_identity_support::journal_replay_surface_evidence();
    let certification = WorthQueryJournalIdentityCertification::from_evidence(
        inventory.clone(),
        schedule.clone(),
        replay.clone(),
    );

    assert!(certification.closed());
    assert!(!certification.artifact_digest().is_empty());
    assert!(!certification.failure_digest().is_empty());
    super::journal_identity_support::assert_closed_replay_boundary_certification(&certification);
    assert!(!WorthQueryJournalIdentityCertification::from_evidence(
        inventory.with_forbidden_failure_for_sabotage(),
        schedule.clone(),
        replay.clone()
    )
    .closed());
    assert!(!WorthQueryJournalIdentityCertification::from_evidence(
        inventory.clone(),
        WorthQueryJournalIdentityScheduleEvidence::derive(
            &first_run.duplicate_first_position_schedule(),
            &second_schedule
        ),
        replay.clone()
    )
    .closed());
    assert!(!WorthQueryJournalIdentityCertification::from_evidence(
        inventory,
        schedule,
        replay.with_gap_for_sabotage()
    )
    .closed());
    let truth_mismatch = WorthQueryJournalIdentityCertification::from_evidence(
        journal_identity_inventory_evidence(),
        WorthQueryJournalIdentityScheduleEvidence::derive(&first_schedule, &second_schedule),
        super::journal_identity_support::journal_replay_surface_evidence()
            .with_truth_mismatch_for_sabotage(),
    );
    assert_eq!(
        truth_mismatch.posture(),
        WorthQueryJournalIdentityBoundaryPosture::Partial
    );
}

#[test]
fn batch_receipt_carries_component_journal_positions_in_order() {
    let (positions, inspection_identities) =
        submitted_batch_positions_with_inspection("tasks.journal-position.batch");

    assert_eq!(positions.ordinals, vec![1, 2, 3]);
    assert_eq!(positions.unique_identity_count(), 3);
    assert_eq!(positions.evidence_identities, inspection_identities);
}

#[test]
fn preview_receipt_carries_preview_journal_position_without_commit_collision() {
    let mut runtime = stateful_bridge_task_runtime();
    let (first_receipt, second_receipt) = {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("journal-preview"),
                WorthQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should admit");
        let first_receipt = preview
            .insert("Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("preview-task"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Preview"),
                )
            })
            .expect("preview insert should stage");
        let second_receipt = preview
            .insert("Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("preview-task-2"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Preview Two"),
                )
            })
            .expect("second preview insert should stage");
        (first_receipt, second_receipt)
    };

    assert_eq!(
        first_receipt.journal_position().authority(),
        WorthQueryJournalPositionAuthority::Preview
    );
    assert_eq!(first_receipt.journal_position().ordinal_for_reporting(), 1);
    assert_eq!(second_receipt.journal_position().ordinal_for_reporting(), 2);
    assert_ne!(
        first_receipt.journal_position().evidence_identity(),
        *first_receipt.commit_evidence_identity()
    );
    assert_ne!(
        first_receipt.journal_position().evidence_identity(),
        second_receipt.journal_position().evidence_identity()
    );
}

#[derive(Debug, Eq, PartialEq)]
struct JournalPositionSchedule {
    ordinals: Vec<u64>,
    evidence_identities: Vec<String>,
    positions: Vec<WorthQueryJournalPosition>,
}

impl JournalPositionSchedule {
    fn unique_identity_count(&self) -> usize {
        self.evidence_identities
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    }

    fn position_schedule(&self) -> WorthQueryJournalPositionSchedule {
        WorthQueryJournalPositionSchedule::derive(self.positions.clone())
    }

    fn duplicate_first_position_schedule(&self) -> WorthQueryJournalPositionSchedule {
        let mut positions = self.positions.clone();
        positions.push(
            self.positions
                .first()
                .expect("schedule should carry at least one position")
                .clone(),
        );
        WorthQueryJournalPositionSchedule::derive(positions)
    }
}

fn submitted_batch_positions_with_inspection(
    workspace_name: &str,
) -> (JournalPositionSchedule, Vec<String>) {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace(workspace_name)
        .expect("task runtime should open a named workspace");
    let receipt = workspace
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("task-1"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("First"),
                    )
                })
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("task-2"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Second"),
                    )
                })
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("task-3"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Third"),
                    )
                })
        })
        .expect("batch should commit");
    let inspection_identities = match workspace
        .inspect(&receipt)
        .expect("batch receipt should inspect")
    {
        WorthQueryInspection::BatchWriteReceipt(inspection) => inspection
            .journal_position_identities()
            .iter()
            .map(|identity| identity.as_str().to_string())
            .collect::<Vec<_>>(),
        _ => panic!("batch receipt should inspect as batch write receipt"),
    };
    let positions = receipt.journal_positions().collect::<Vec<_>>();

    assert!(positions
        .iter()
        .all(|position| position.authority() == WorthQueryJournalPositionAuthority::Committed));
    (
        JournalPositionSchedule {
            ordinals: positions
                .iter()
                .map(|position| position.ordinal_for_reporting())
                .collect(),
            evidence_identities: positions
                .iter()
                .map(|position| position.evidence_identity().as_str().to_string())
                .collect(),
            positions: positions.into_iter().cloned().collect(),
        },
        inspection_identities,
    )
}

fn submitted_schedule_positions(workspace_name: &str) -> JournalPositionSchedule {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace(workspace_name)
        .expect("task runtime should open a named workspace");
    let mut ordinals = Vec::new();
    let mut evidence_identities = Vec::new();
    let mut positions = Vec::new();
    for command in submission_schedule_commands() {
        let receipt = workspace
            .submissions()
            .expect("submission lane should be admitted")
            .submit(command)
            .expect("scheduled submission should commit");
        assert_eq!(
            receipt.journal_position().authority(),
            WorthQueryJournalPositionAuthority::Committed
        );
        ordinals.push(receipt.journal_position().ordinal_for_reporting());
        evidence_identities.push(
            receipt
                .journal_position()
                .evidence_identity()
                .as_str()
                .to_string(),
        );
        positions.push(receipt.journal_position().clone());
    }
    JournalPositionSchedule {
        ordinals,
        evidence_identities,
        positions,
    }
}

fn submission_schedule_commands() -> Vec<WorthQueryWriteCommand> {
    ["First", "Second", "Third"]
        .iter()
        .enumerate()
        .map(|(index, title)| {
            WorthQueryAspectMutationBuilder::new()
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value(format!("task-{}", index + 1)),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value(*title),
                )
                .build_insert("Task")
                .expect("schedule insert command should build")
        })
        .collect()
}

fn journal_identity_inventory_evidence() -> WorthQueryJournalIdentityInventoryEvidence {
    let workspace_root = workspace_root();
    let forbidden_count = scan_journal_identity_forbidden_patterns(&workspace_root).len();
    let required_count = scan_journal_identity_required_pattern_failures(&workspace_root).len();
    let operations = worth_query_journal_identity_inventory()
        .iter()
        .map(|row| row.kind())
        .collect::<std::collections::BTreeSet<_>>();
    let missing_count = missing_operation_count(&operations);
    WorthQueryJournalIdentityInventoryEvidence::new(
        forbidden_count,
        required_count,
        missing_count,
        inventory_digest(forbidden_count, required_count, missing_count),
    )
}

fn missing_operation_count(
    operations: &std::collections::BTreeSet<WorthQueryJournalIdentityOperationKind>,
) -> usize {
    required_operations()
        .iter()
        .filter(|operation| !operations.contains(operation))
        .count()
}

fn required_operations() -> &'static [WorthQueryJournalIdentityOperationKind] {
    &[
        WorthQueryJournalIdentityOperationKind::MintCommittedPosition,
        WorthQueryJournalIdentityOperationKind::MintPreviewPosition,
        WorthQueryJournalIdentityOperationKind::AdmitSubmissionLanePosition,
        WorthQueryJournalIdentityOperationKind::CarryWriteReceiptPosition,
        WorthQueryJournalIdentityOperationKind::ReadWriteReceiptPosition,
        WorthQueryJournalIdentityOperationKind::CarryPreviewReceiptPosition,
        WorthQueryJournalIdentityOperationKind::CarryBatchReceiptPositions,
        WorthQueryJournalIdentityOperationKind::BuildJournalSegmentIdentity,
        WorthQueryJournalIdentityOperationKind::BuildJournalReplayRequest,
        WorthQueryJournalIdentityOperationKind::RetainReplayRegistryReceipt,
        WorthQueryJournalIdentityOperationKind::MaterializeReplayOutcome,
        WorthQueryJournalIdentityOperationKind::ExposeWorkspaceReplayFacade,
        WorthQueryJournalIdentityOperationKind::CertifyJournalIdentityBoundary,
    ]
}

fn inventory_digest(forbidden_count: usize, required_count: usize, missing_count: usize) -> String {
    crate::evidence_identity::worth_query_evidence_identity(
        crate::evidence_identity::WorthQueryEvidenceScope::ApplicationSupportReport,
    )
    .field_usize(
        crate::evidence_identity::WorthQueryEvidenceTag::new("forbidden_count"),
        forbidden_count,
    )
    .field_usize(
        crate::evidence_identity::WorthQueryEvidenceTag::new("required_count"),
        required_count,
    )
    .field_usize(
        crate::evidence_identity::WorthQueryEvidenceTag::new("missing_count"),
        missing_count,
    )
    .seal()
    .as_str()
    .to_string()
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root should resolve")
}
