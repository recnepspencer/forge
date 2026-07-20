use std::sync::{mpsc, Arc, Barrier};
use std::thread;

use crate::application::WorthQueryConcurrentHostileMatrixArtifact;
use crate::runtime::tests::support::*;
use crate::runtime::{
    WorthQueryConcurrentHostileMatrixCounterSnapshot, WorthQueryConcurrentHostileMatrixTopology,
    WorthQueryConcurrentSubmissionIntake, WorthQueryConcurrentSubmissionRecord,
};

use super::concurrent_hostile_matrix_digest::{consume_artifact_title, raw_matrix_digest};
use super::concurrent_hostile_matrix_maintainer::ConcurrentMatrixMaintainer;
use super::concurrent_hostile_matrix_submission::{
    planned_phase_sixteen_submissions, planned_submissions_for_submitter,
    submitter_thread_ordinals, PlannedSubmission, SubmitterInterleaving, PHASE_SIXTEEN_READERS,
    PHASE_SIXTEEN_SUBMISSION_ROUNDS, PHASE_SIXTEEN_SUBMITTERS,
};

mod closure_assertions;

pub(in crate::runtime::tests) use closure_assertions::*;

struct ConcurrentMatrixRun {
    topology: WorthQueryConcurrentHostileMatrixTopology,
    receipt_digests: Vec<String>,
    reader_result_digests: Vec<String>,
    published_artifact_digests: Vec<String>,
    preview_closeout_digests: Vec<String>,
    branch_basis_digests: Vec<String>,
    replay_digest: String,
    counters: WorthQueryConcurrentHostileMatrixCounterSnapshot,
    raw_digest: String,
}

pub(in crate::runtime::tests) fn execute_phase_sixteen_concurrent_hostile_matrix(
) -> WorthQueryConcurrentHostileMatrixArtifact {
    let primary = run_phase_sixteen_matrix("primary");
    let serialized_replay = run_phase_sixteen_serialized_replay("serialized-replay");
    let repeated =
        run_phase_sixteen_matrix_with_interleaving("repeated", SubmitterInterleaving::Descending);
    let artifact_replay_equal = primary.raw_digest == serialized_replay.raw_digest;
    let repeated_run_equal = primary.raw_digest == repeated.raw_digest;
    WorthQueryConcurrentHostileMatrixArtifact::certify(
        primary.topology,
        primary.receipt_digests,
        primary.reader_result_digests,
        primary.published_artifact_digests,
        primary.preview_closeout_digests,
        primary.branch_basis_digests,
        primary.replay_digest,
        primary.counters,
        artifact_replay_equal,
        repeated_run_equal,
        true,
    )
}

fn run_phase_sixteen_matrix(label: &'static str) -> ConcurrentMatrixRun {
    run_phase_sixteen_matrix_with_interleaving(label, SubmitterInterleaving::Ascending)
}

fn run_phase_sixteen_matrix_with_interleaving(
    label: &'static str,
    interleaving: SubmitterInterleaving,
) -> ConcurrentMatrixRun {
    let mut state = ConcurrentMatrixState::new(label);
    state.capture_unpublished_posture();
    state.record_branch_basis("phase16-branch-a");
    state.record_preview_discard();
    state.execute_concurrent_reader_and_submitter_matrix(interleaving);
    state.reconsume_published_artifacts();
    state.record_preview_promotion();
    state.record_branch_basis("phase16-branch-b");
    state.finish()
}

fn run_phase_sixteen_serialized_replay(label: &'static str) -> ConcurrentMatrixRun {
    let mut state = ConcurrentMatrixState::new(label);
    state.capture_unpublished_posture();
    state.record_branch_basis("phase16-branch-a");
    state.record_preview_discard();
    state.execute_serialized_reader_matrix();
    for submission in planned_phase_sixteen_submissions() {
        state.submit_planned(submission);
    }
    state.reconsume_published_artifacts();
    state.record_preview_promotion();
    state.record_branch_basis("phase16-branch-b");
    state.finish()
}

struct ConcurrentMatrixState {
    workspace: WorthQueryWorkspace,
    derived: WorthQueryDerivedViewHandle<WorthQueryUnrefinedLiveShape>,
    receipt_digests: Vec<String>,
    reader_result_digests: Vec<String>,
    published_artifact_digests: Vec<String>,
    preview_closeout_digests: Vec<String>,
    branch_basis_digests: Vec<String>,
    receipts: Vec<WorthQueryWriteReceipt>,
    delivery_residue_count: usize,
    last_artifact: Option<WorthQueryPublishedDerivedArtifactHandle>,
}

impl ConcurrentMatrixState {
    fn new(_label: &'static str) -> Self {
        let mut workspace = stateful_bridge_task_runtime()
            .workspace("runtime.phase16.concurrent-hostile")
            .expect("workspace should build");
        let live: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
            .live_view_request("tasks.phase16", task_live_request(), task_schema())
            .expect("live view should declare");
        let derived = workspace
            .computed_view(
                crate::program::WorthQueryDerivedView::new(
                    "derived.phase16",
                    [test_aspect_touch("title.value")],
                )
                .depends_on_live(&live),
                ConcurrentMatrixMaintainer::seeded(),
            )
            .expect("derived view should declare");
        Self {
            workspace,
            derived,
            receipt_digests: Vec::new(),
            reader_result_digests: Vec::new(),
            published_artifact_digests: Vec::new(),
            preview_closeout_digests: Vec::new(),
            branch_basis_digests: Vec::new(),
            receipts: Vec::new(),
            delivery_residue_count: 0,
            last_artifact: None,
        }
    }

    fn capture_unpublished_posture(&mut self) {
        let artifact = self
            .workspace
            .shared_read_context()
            .expect("shared read context should mint")
            .published_derived_artifact(&self.derived)
            .expect("unpublished artifact should mint");
        let result = consume_artifact_title(&artifact);
        self.reader_result_digests.push(result);
        self.published_artifact_digests
            .push(hostile_published_artifact_digest(&artifact, None));
    }

    fn execute_concurrent_reader_and_submitter_matrix(
        &mut self,
        interleaving: SubmitterInterleaving,
    ) {
        let intake = WorthQueryConcurrentSubmissionIntake::new();
        let (tx, rx) = mpsc::channel();
        let barrier = Arc::new(Barrier::new(
            PHASE_SIXTEEN_READERS + PHASE_SIXTEEN_SUBMITTERS,
        ));
        let contexts = (0..PHASE_SIXTEEN_READERS)
            .map(|_| {
                self.workspace
                    .shared_read_context()
                    .expect("shared read context should mint")
                    .published_derived_artifact(&self.derived)
                    .expect("reader artifact should mint")
            })
            .collect::<Vec<_>>();
        let mut reader_results = thread::scope(|scope| {
            let reader_handles = contexts
                .into_iter()
                .map(|artifact| {
                    let barrier = Arc::clone(&barrier);
                    scope.spawn(move || {
                        barrier.wait();
                        consume_artifact_title(&artifact)
                    })
                })
                .collect::<Vec<_>>();
            for submitter in submitter_thread_ordinals(interleaving) {
                let tx = tx.clone();
                let lane = intake.lane(submitter);
                let barrier = Arc::clone(&barrier);
                scope.spawn(move || {
                    barrier.wait();
                    for submission in planned_submissions_for_submitter(submitter) {
                        lane.submit(
                            submission.ordinal,
                            hostile_insert_task_command(&submission.identity, &submission.title),
                        );
                        tx.send(submission.ordinal)
                            .expect("submission observation should send");
                    }
                });
            }
            drop(tx);
            let observed_submission_count = rx.into_iter().count();
            let reader_results = reader_handles
                .into_iter()
                .map(|handle| handle.join().expect("reader should complete"))
                .collect::<Vec<_>>();
            (observed_submission_count, reader_results)
        });
        assert_eq!(
            reader_results.0,
            PHASE_SIXTEEN_SUBMITTERS * PHASE_SIXTEEN_SUBMISSION_ROUNDS
        );
        self.reader_result_digests.append(&mut reader_results.1);
        for record in intake.drain_ordered() {
            self.submit_concurrent_record(record);
        }
    }

    fn execute_serialized_reader_matrix(&mut self) {
        for _ in 0..PHASE_SIXTEEN_READERS {
            let artifact = self
                .workspace
                .shared_read_context()
                .expect("shared read context should mint")
                .published_derived_artifact(&self.derived)
                .expect("serialized reader artifact should mint");
            self.reader_result_digests
                .push(consume_artifact_title(&artifact));
        }
    }

    fn submit_planned(&mut self, submission: PlannedSubmission) {
        self.submit_command(hostile_insert_task_command(
            &submission.identity,
            &submission.title,
        ));
    }

    fn submit_concurrent_record(&mut self, record: WorthQueryConcurrentSubmissionRecord) {
        assert!(
            record.submitter_thread_ordinal() < PHASE_SIXTEEN_SUBMITTERS,
            "submitter ordinal should be part of the declared topology"
        );
        assert!(
            record.submission_ordinal()
                < PHASE_SIXTEEN_SUBMITTERS * PHASE_SIXTEEN_SUBMISSION_ROUNDS,
            "submission ordinal should be part of the declared topology"
        );
        self.submit_command(record.into_command());
    }

    fn submit_command(&mut self, command: WorthQueryWriteCommand) {
        let receipt = self
            .workspace
            .submissions()
            .expect("submission lane should mint")
            .submit(command)
            .expect("submission should succeed");
        self.receipt_digests
            .push(hostile_write_receipt_digest(&receipt));
        self.receipts.push(receipt);
        let artifact = self
            .workspace
            .shared_read_context()
            .expect("shared read context should mint")
            .published_derived_artifact(&self.derived)
            .expect("published artifact should mint");
        let consumed_title = consume_artifact_title(&artifact);
        self.reader_result_digests.push(consumed_title.clone());
        self.published_artifact_digests
            .push(hostile_published_artifact_digest(
                &artifact,
                Some(consumed_title.as_str()),
            ));
        self.last_artifact = Some(artifact);
    }

    fn reconsume_published_artifacts(&mut self) {
        let artifact = self
            .last_artifact
            .as_ref()
            .expect("latest artifact should exist")
            .clone();
        let consumed_title = consume_artifact_title(&artifact);
        self.reader_result_digests.push(consumed_title.clone());
        self.published_artifact_digests
            .push(hostile_published_artifact_digest(
                &artifact,
                Some(consumed_title.as_str()),
            ));
    }

    fn record_preview_discard(&mut self) {
        let discarded = {
            let mut preview = self
                .workspace
                .preview(test_session_label("phase16-preview-discard"))
                .expect("preview should admit");
            preview
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("phase16-preview-discard"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Preview Discard"),
                    )
                })
                .expect("preview insert should stage");
            preview.discard()
        };
        self.delivery_residue_count += hostile_preview_delivery_residue_count(&discarded);
        self.preview_closeout_digests
            .push(hostile_preview_closeout_digest(&discarded));
    }

    fn record_preview_promotion(&mut self) {
        let promoted = {
            let mut preview = self
                .workspace
                .preview(test_session_label("phase16-preview-promote"))
                .expect("preview should admit");
            preview
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("phase16-preview-promote"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Preview Promote"),
                    )
                })
                .expect("preview insert should stage");
            preview.promote().expect("preview should promote")
        };
        self.delivery_residue_count += hostile_preview_delivery_residue_count(&promoted);
        self.preview_closeout_digests
            .push(hostile_preview_closeout_digest(&promoted));
    }

    fn record_branch_basis(&mut self, label: &'static str) {
        let branch = self
            .workspace
            .branch(test_session_label(label))
            .expect("branch should admit");
        self.branch_basis_digests
            .push(hostile_branch_basis_digest(&branch));
    }

    fn finish(mut self) -> ConcurrentMatrixRun {
        let first = self.receipts.first().expect("first receipt should exist");
        let last = self.receipts.last().expect("last receipt should exist");
        let segment = WorthQueryJournalSegmentIdentity::between(
            first.journal_position(),
            last.journal_position(),
        )
        .expect("committed segment should build");
        let outcome = self
            .workspace
            .replay_journal_segment(WorthQueryJournalReplayRequest::new(segment))
            .expect("replay should succeed");
        let replay_digest = outcome.outcome_digest().to_string();
        self.last_artifact = None;
        let counters = WorthQueryConcurrentHostileMatrixCounterSnapshot::from_runtime(
            &self.workspace.runtime,
            self.delivery_residue_count,
        );
        let topology = WorthQueryConcurrentHostileMatrixTopology::new(
            PHASE_SIXTEEN_READERS,
            PHASE_SIXTEEN_SUBMITTERS,
            PHASE_SIXTEEN_SUBMISSION_ROUNDS,
        );
        let raw_digest = raw_matrix_digest(
            topology,
            &self.receipt_digests,
            &self.reader_result_digests,
            &self.published_artifact_digests,
            &self.preview_closeout_digests,
            &self.branch_basis_digests,
            &replay_digest,
            &counters,
        );
        ConcurrentMatrixRun {
            topology,
            receipt_digests: self.receipt_digests,
            reader_result_digests: self.reader_result_digests,
            published_artifact_digests: self.published_artifact_digests,
            preview_closeout_digests: self.preview_closeout_digests,
            branch_basis_digests: self.branch_basis_digests,
            replay_digest,
            counters,
            raw_digest,
        }
    }
}
