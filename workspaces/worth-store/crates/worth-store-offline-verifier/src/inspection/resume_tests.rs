use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    OfflineMediaClosureEntry, OfflineMediaConsistencyBasis, OfflineMediaReadDenial,
};

use crate::{
    OfflineInspectionBudget, OfflineInspectionCancellation, OfflineInspectionCheckpoint,
    OfflineInspectionDenial, OfflineMediaAcquisitionDenial, OfflineStoreInspection,
    UntrustedOfflineMediaSet,
};

#[test]
fn every_chunk_boundary_crash_and_cancellation_converges_across_buffer_widths() {
    let fixture = ResumeFixture::new();
    for width in [1, 7, 16, 31] {
        let budget = OfflineInspectionBudget::bounded(width, 256).expect("budget");
        let expected = fixture
            .inspection(budget)
            .start()
            .expect("start")
            .finish()
            .expect("uninterrupted");
        let expected_files = file_facts(&expected);
        let chunks_per_file = 64_usize.div_ceil(width);
        for boundary in 0..=(chunks_per_file * 2) {
            let cancellation = OfflineInspectionCancellation::new();
            let mut interrupted = fixture
                .inspection(budget)
                .cancellation(cancellation.clone())
                .start()
                .expect("start");
            for _ in 0..boundary {
                interrupted.advance().expect("advance").expect("progress");
            }
            let checkpoint = interrupted.checkpoint().expect("checkpoint");
            let observed_before_crash = checkpoint.observed_bytes();
            cancellation.cancel();
            assert!(matches!(
                interrupted.advance(),
                Err(OfflineInspectionDenial::Cancelled)
            ));
            let encoded = checkpoint.encode().expect("persisted checkpoint");
            let resumed = fixture
                .inspection(budget)
                .resume_from_checkpoint_bytes(&encoded)
                .expect("fresh-process resume")
                .finish()
                .expect("resumed completion");
            assert_eq!(file_facts(&resumed), expected_files);
            assert_eq!(resumed.counters().bytes_read(), 128);
            assert_eq!(
                resumed.counters().checkpoint_revalidated_bytes(),
                observed_before_crash
            );
        }
    }
}

#[test]
fn absolute_deadline_cannot_be_reset_by_persisted_resume() {
    let fixture = ResumeFixture::new();
    let deadline = std::time::SystemTime::now() + std::time::Duration::from_millis(40);
    let budget = OfflineInspectionBudget::bounded(16, 256)
        .expect("budget")
        .with_deadline(deadline)
        .expect("deadline");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    interrupted.advance().expect("read").expect("progress");
    let encoded = interrupted
        .checkpoint()
        .expect("checkpoint")
        .encode()
        .expect("persisted checkpoint");
    std::thread::sleep(std::time::Duration::from_millis(60));

    let denial = match fixture
        .inspection(budget)
        .resume_from_checkpoint_bytes(&encoded)
    {
        Err(denial) => denial,
        Ok(_) => panic!("expired work must stop during reacquisition"),
    };
    assert!(matches!(
        denial,
        OfflineMediaAcquisitionDenial::Interrupted(
            OfflineInspectionDenial::AbsoluteDeadlineReached {
            deadline: observed
        }) if observed == deadline));
}

#[test]
fn persisted_checkpoint_revalidates_completed_files_before_reusing_observations() {
    let fixture = ResumeFixture::new();
    let budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    for _ in 0..4 {
        interrupted.advance().expect("read").expect("progress");
    }
    let checkpoint = interrupted.checkpoint().expect("checkpoint");
    assert_eq!(checkpoint.file_index(), 1);
    assert_eq!(checkpoint.offset(), 0);
    let encoded = checkpoint.encode().expect("persisted checkpoint");
    drop(interrupted);

    let walked = fixture
        .inspection(budget)
        .resume_from_checkpoint_bytes(&encoded)
        .expect("reacquire")
        .finish()
        .expect("resume");
    assert_eq!(walked.files().len(), 2);
    assert_eq!(walked.admitted_bytes(), 128);
    assert_eq!(walked.counters().bytes_read(), 128);
    assert_eq!(walked.counters().file_touches(), 2);
    assert_eq!(walked.counters().checkpoint_revalidated_files(), 1);
    assert_eq!(walked.counters().checkpoint_revalidated_bytes(), 64);
    assert_eq!(walked.counters().checkpoint_rejections(), 0);
}

#[test]
fn partial_file_checkpoint_rewalks_and_reports_its_revalidated_prefix() {
    let fixture = ResumeFixture::new();
    let budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    interrupted.advance().expect("read").expect("progress");
    let checkpoint = interrupted.checkpoint().expect("checkpoint");
    assert_eq!(checkpoint.file_index(), 0);
    assert_eq!(checkpoint.offset(), 16);

    let walked = fixture
        .inspection(budget)
        .resume_from_checkpoint(&checkpoint)
        .expect("reacquire")
        .finish()
        .expect("resume with bounded rewalk");
    assert_eq!(walked.admitted_bytes(), 128);
    assert_eq!(walked.counters().bytes_read(), 128);
    assert_eq!(walked.counters().checkpoint_revalidated_files(), 0);
    assert_eq!(walked.counters().checkpoint_revalidated_bytes(), 16);
}

#[test]
fn malformed_persisted_checkpoint_restarts_without_skipping_media() {
    let fixture = ResumeFixture::new();
    let budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    for _ in 0..4 {
        interrupted.advance().expect("read").expect("progress");
    }
    let encoded = interrupted
        .checkpoint()
        .expect("checkpoint")
        .encode()
        .expect("encoded");
    for boundary in 0..encoded.len() {
        assert!(OfflineInspectionCheckpoint::decode(&encoded[..boundary]).is_err());
    }
    let mut corrupted = encoded;
    corrupted[12] ^= 0x80;

    let walked = fixture
        .inspection(budget)
        .resume_from_checkpoint_bytes(&corrupted)
        .expect("safe fresh acquisition")
        .finish()
        .expect("full restart");
    assert_eq!(walked.counters().bytes_read(), 128);
    assert_eq!(walked.counters().checkpoint_revalidated_files(), 0);
    assert_eq!(walked.counters().checkpoint_rejections(), 1);
}

#[test]
fn persisted_checkpoint_decode_cannot_escape_the_session_owned_memory_budget() {
    let fixture = ResumeFixture::new_with_basis_identity("x".repeat(4096));
    let broad_budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture
        .inspection(broad_budget)
        .start()
        .expect("start inspection");
    for _ in 0..4 {
        interrupted.advance().expect("advance").expect("progress");
    }
    let checkpoint = interrupted.checkpoint().expect("checkpoint");
    let encoded = checkpoint.encode().expect("encode checkpoint");
    let tight_owned_limit = fixture
        .inspection(broad_budget)
        .start()
        .expect("baseline start")
        .finish()
        .expect("baseline finish")
        .counters()
        .peak_owned_allocation_bytes();
    let tight_budget = OfflineInspectionBudget::bounded(16, 256)
        .expect("budget")
        .with_maximum_owned_allocation_bytes(tight_owned_limit)
        .expect("session resident allocation remains admissible");

    let walked = fixture
        .inspection(tight_budget)
        .resume_from_checkpoint_bytes(&encoded)
        .expect("oversized checkpoint safely becomes a fresh session")
        .finish()
        .expect("fresh walk");

    assert_eq!(walked.counters().checkpoint_revalidated_files(), 0);
    assert_eq!(walked.counters().checkpoint_rejections(), 1);
    assert_eq!(walked.counters().bytes_read(), 128);
    assert!(walked.counters().peak_owned_allocation_bytes() <= tight_owned_limit);
}

#[test]
fn self_consistent_hostile_checkpoint_encodings_fail_closed() {
    let fixture = ResumeFixture::new();
    let budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    for _ in 0..4 {
        interrupted.advance().expect("read").expect("progress");
    }
    let encoded = interrupted
        .checkpoint()
        .expect("checkpoint")
        .encode()
        .expect("encoded");
    let body = &encoded[..encoded.len() - 32];
    let basis_length = u32::from_le_bytes(body[8..12].try_into().expect("basis length")) as usize;
    let observation_start = 12 + basis_length + 8 + 8 + (10 * 8) + 8 + 4 + 4;

    let mut unknown_version = body.to_vec();
    unknown_version[7] = b'9';
    assert!(OfflineInspectionCheckpoint::decode(&reseal(unknown_version)).is_err());

    let mut invalid_utf8 = body.to_vec();
    invalid_utf8[12] = u8::MAX;
    assert!(OfflineInspectionCheckpoint::decode(&reseal(invalid_utf8)).is_err());

    let mut impossible_basis_length = body.to_vec();
    impossible_basis_length[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(OfflineInspectionCheckpoint::decode(&reseal(impossible_basis_length)).is_err());

    let mut unknown_family = body.to_vec();
    unknown_family[observation_start + 88] = u8::MAX;
    assert!(OfflineInspectionCheckpoint::decode(&reseal(unknown_family)).is_err());

    let mut trailing = body.to_vec();
    trailing.push(0);
    assert!(OfflineInspectionCheckpoint::decode(&reseal(trailing)).is_err());
}

#[test]
fn media_identity_change_invalidates_completed_checkpoint_observations() {
    let fixture = ResumeFixture::new();
    let budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    for _ in 0..4 {
        interrupted.advance().expect("read").expect("progress");
    }
    let checkpoint = interrupted.checkpoint().expect("checkpoint");
    std::fs::write(&fixture.page, vec![9_u8; 64]).expect("same-length mutation");

    let resumed = fixture
        .inspection(budget)
        .resume_from_checkpoint(&checkpoint)
        .expect("safe fresh acquisition");
    assert_eq!(
        resumed
            .checkpoint()
            .expect("fresh checkpoint")
            .observed_bytes(),
        0
    );
    assert!(matches!(
        resumed.finish(),
        Err(OfflineInspectionDenial::Media(
            OfflineMediaReadDenial::ContentClosureArtifactMismatch { .. }
        ))
    ));
}

#[test]
fn same_metadata_content_substitution_is_rehashed_before_checkpoint_observation_reuse() {
    let fixture = ResumeFixture::new();
    let original_metadata = std::fs::metadata(&fixture.page).expect("metadata");
    let original_mtime = filetime::FileTime::from_last_modification_time(&original_metadata);
    let budget = OfflineInspectionBudget::bounded(16, 256).expect("budget");
    let mut interrupted = fixture.inspection(budget).start().expect("start");
    for _ in 0..4 {
        interrupted.advance().expect("read").expect("progress");
    }
    let checkpoint = interrupted.checkpoint().expect("checkpoint");
    std::fs::write(&fixture.page, vec![9_u8; 64]).expect("same-length mutation");
    filetime::set_file_mtime(&fixture.page, original_mtime).expect("restore observed timestamp");

    let mut resumed = fixture
        .inspection(budget)
        .resume_from_checkpoint(&checkpoint)
        .expect("resume acquisition");
    for _ in 0..4 {
        resumed
            .advance()
            .expect("revalidation read")
            .expect("progress");
    }
    let counters = resumed.checkpoint().expect("checkpoint").counters();
    assert_eq!(counters.checkpoint_revalidated_files(), 0);
    assert_eq!(counters.checkpoint_rejections(), 1);
    assert!(matches!(
        resumed.finish(),
        Err(OfflineInspectionDenial::Media(
            OfflineMediaReadDenial::ContentClosureArtifactMismatch { .. }
        ))
    ));
}

struct ResumeFixture {
    _directory: tempfile::TempDir,
    page: std::path::PathBuf,
    wal: std::path::PathBuf,
    basis: OfflineMediaConsistencyBasis,
}

impl ResumeFixture {
    fn new() -> Self {
        Self::new_with_basis_identity("resume-fixture")
    }

    fn new_with_basis_identity(identity: impl Into<String>) -> Self {
        let directory = tempfile::tempdir().expect("directory");
        let page = directory.path().join("00-primary.page");
        let wal = directory.path().join("01-tail.wal");
        let page_bytes = vec![1_u8; 64];
        let wal_bytes = vec![2_u8; 64];
        std::fs::write(&page, &page_bytes).expect("page");
        std::fs::write(&wal, &wal_bytes).expect("wal");
        let basis = OfflineMediaConsistencyBasis::content_addressed_closure(
            identity,
            [(&page, &page_bytes), (&wal, &wal_bytes)]
                .into_iter()
                .map(|(path, bytes)| {
                    OfflineMediaClosureEntry::new(
                        path,
                        bytes.len() as u64,
                        Sha256::digest(bytes).into(),
                    )
                    .expect("closure row")
                }),
        )
        .expect("content closure");
        Self {
            _directory: directory,
            page,
            wal,
            basis,
        }
    }

    fn inspection(&self, budget: OfflineInspectionBudget) -> OfflineStoreInspection {
        let root = self
            .page
            .parent()
            .expect("fixture root is retained by temp directory");
        debug_assert_eq!(self.wal.parent(), Some(root));
        OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
            root,
            self.basis.clone(),
        ))
        .budget(budget)
    }
}

fn reseal(mut body: Vec<u8>) -> Vec<u8> {
    let checksum: [u8; 32] = Sha256::digest(&body).into();
    body.extend_from_slice(&checksum);
    body
}

fn file_facts(
    walked: &crate::StructurallyWalkedMedia,
) -> Vec<(
    worth_store_physical_format::OfflinePhysicalArtifactFamily,
    u64,
    [u8; 32],
)> {
    walked
        .files()
        .iter()
        .map(|file| (file.family(), file.length(), file.content_digest()))
        .collect()
}
