use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    OfflineMediaClosureEntry, OfflineMediaConsistencyBasis, OfflineMediaReadDenial,
};

use crate::{
    compose_operational_truth, OfflineAuthorityClass, OfflineFileTruthEvidence,
    OfflineInspectionBudget, OfflineInspectionDenial, OfflineRecoveryAvailability,
    OfflineSecurityEvidencePosture, OfflineStoreInspection, OfflineTruthEvidenceSet,
    OperationalTruthCompositionBudget, OperationalTruthRegion, UntrustedOfflineMediaSet,
};

#[test]
fn real_media_walk_stays_within_buffer_budget_and_composes_canonical_truth() {
    let directory = tempfile::tempdir().expect("temp directory");
    let page = directory.path().join("primary.page");
    let wal = directory.path().join("tail.wal");
    let page_bytes = vec![7u8; 16 * 1024];
    let wal_bytes = vec![9u8; 9 * 1024];
    std::fs::write(&page, &page_bytes).expect("page media");
    std::fs::write(&wal, &wal_bytes).expect("wal media");
    let budget = OfflineInspectionBudget::bounded(1024, 64 * 1024).expect("budget");
    let walked = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        content_closure(
            "closed-producer",
            [(&page, &page_bytes), (&wal, &wal_bytes)],
        ),
    ))
    .budget(budget)
    .start()
    .expect("start independent inspection")
    .finish()
    .expect("finish");

    assert_eq!(
        walked.admitted_bytes(),
        (page_bytes.len() + wal_bytes.len()) as u64
    );
    assert!(walked.counters().peak_buffer_bytes() <= 1024);
    assert!(
        walked.counters().peak_owned_allocation_bytes() <= budget.maximum_owned_allocation_bytes()
    );
    assert_eq!(walked.counters().bytes_read(), walked.admitted_bytes());
    assert!(walked.counters().chunk_touches() > walked.counters().file_touches());

    let confirmed = |path, bytes: &[u8]| {
        OfflineFileTruthEvidence::new(path)
            .with_expected_digest(Sha256::digest(bytes).into())
            .with_authenticity(OfflineSecurityEvidencePosture::Confirmed)
            .with_custody(OfflineSecurityEvidencePosture::Confirmed)
    };
    let evidence = OfflineTruthEvidenceSet::from_entries(
        [confirmed(page, &page_bytes), confirmed(wal, &wal_bytes)],
        1024 * 1024,
    )
    .expect("unique evidence sources");
    let report = compose_operational_truth(walked, &evidence, truth_composition_budget())
        .expect("truth composition");
    assert_eq!(
        report.coverage().covered_bytes(),
        (page_bytes.len() + wal_bytes.len()) as u64
    );
    assert!(report
        .regions()
        .iter()
        .all(|region| matches!(region, OperationalTruthRegion::IndeterminateTruthRegion(_))));
    assert!(report.recovery_candidates().candidates().is_empty());
}

#[test]
fn mutation_between_bounded_reads_is_indeterminate_not_silently_mixed() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("moving.extent");
    std::fs::write(&path, vec![1u8; 4096]).expect("source media");
    let mut session = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        content_closure("generation-1", [(&path, &vec![1u8; 4096])]),
    ))
    .budget(OfflineInspectionBudget::bounded(512, 8192).expect("budget"))
    .start()
    .expect("start");
    session
        .advance()
        .expect("first bounded read")
        .expect("progress");
    std::fs::write(&path, vec![2u8; 4097]).expect("concurrent mutation");
    assert!(matches!(
        session.advance(),
        Err(OfflineInspectionDenial::Media(
            OfflineMediaReadDenial::ConcurrentMutationIndeterminate { .. }
        ))
    ));
}

#[test]
fn same_length_substitution_cannot_satisfy_a_content_closure() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("authority.page");
    let expected = vec![1u8; 4096];
    std::fs::write(&path, &expected).expect("source media");
    let media = UntrustedOfflineMediaSet::from_root(
        directory.path(),
        content_closure("authority-closure", [(&path, &expected)]),
    );
    std::fs::write(&path, vec![2u8; expected.len()]).expect("same-length substitution");
    let denial = OfflineStoreInspection::open(media)
        .budget(OfflineInspectionBudget::bounded(257, 8192).expect("budget"))
        .start()
        .expect("length still matches closure")
        .finish()
        .expect_err("digest closure must reject substitution");
    assert!(matches!(
        denial,
        OfflineInspectionDenial::Media(
            OfflineMediaReadDenial::ContentClosureArtifactMismatch { .. }
        )
    ));
}

#[test]
fn undeclared_residue_cannot_enter_a_content_closed_walk() {
    let directory = tempfile::tempdir().expect("temp directory");
    let expected_path = directory.path().join("authority.page");
    let expected = b"authority".to_vec();
    std::fs::write(&expected_path, &expected).expect("source media");
    let media = UntrustedOfflineMediaSet::from_root(
        directory.path(),
        content_closure("authority-closure", [(&expected_path, &expected)]),
    );
    std::fs::write(directory.path().join("residue.page"), b"residue").expect("residue");
    assert!(matches!(
        OfflineStoreInspection::open(media).start(),
        Err(crate::OfflineMediaAcquisitionDenial::Media(
            OfflineMediaReadDenial::ContentClosureUnexpectedArtifact { .. }
        ))
    ));
}

#[test]
fn caller_declarations_cannot_override_disk_bytes() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("authority.manifest");
    std::fs::write(&path, b"actual-disk-bytes").expect("media");
    let walked = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        content_closure("clone", [(&path, b"actual-disk-bytes".as_slice())]),
    ))
    .budget(OfflineInspectionBudget::bounded(64, 1024).expect("budget"))
    .start()
    .expect("start")
    .finish()
    .expect("finish");
    let false_digest = Sha256::digest(b"caller-declared-different-bytes").into();
    let evidence = OfflineTruthEvidenceSet::from_entries(
        [OfflineFileTruthEvidence::new(path)
            .with_expected_digest(false_digest)
            .with_authenticity(OfflineSecurityEvidencePosture::Confirmed)
            .with_custody(OfflineSecurityEvidencePosture::Confirmed)],
        1024 * 1024,
    )
    .expect("unique evidence source");
    let report = compose_operational_truth(walked, &evidence, truth_composition_budget())
        .expect("truth report");
    assert!(matches!(
        report.regions(),
        [OperationalTruthRegion::QuarantinedRegion(_)]
    ));
}

#[test]
fn filename_authority_hint_cannot_promote_damage_to_unrecoverable_authority() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("authority.manifest");
    std::fs::write(&path, b"damaged-authority").expect("media");
    let walked = inspect(directory.path());
    let evidence = OfflineTruthEvidenceSet::from_entries(
        [OfflineFileTruthEvidence::new(path)
            .with_expected_digest(Sha256::digest(b"expected-authority").into())
            .with_recovery_availability(OfflineRecoveryAvailability::Unavailable)],
        1024 * 1024,
    )
    .expect("unique evidence source");
    let report = compose_operational_truth(walked, &evidence, truth_composition_budget())
        .expect("truth report");
    let [OperationalTruthRegion::QuarantinedRegion(region)] = report.regions() else {
        panic!("unowned damaged bytes must remain quarantined");
    };
    assert_eq!(region.authority_class(), OfflineAuthorityClass::Unknown);
}

#[test]
fn hard_link_aliases_are_one_physical_region_with_all_claimants() {
    let directory = tempfile::tempdir().expect("temp directory");
    let original = directory.path().join("primary.page");
    let alias = directory.path().join("secondary.page");
    std::fs::write(&original, b"one-physical-allocation").expect("media");
    std::fs::hard_link(&original, &alias).expect("hard-link alias");
    let report = compose_operational_truth(
        inspect(directory.path()),
        &OfflineTruthEvidenceSet::default(),
        truth_composition_budget(),
    )
    .expect("truth report");
    assert_eq!(
        report.coverage().covered_bytes(),
        b"one-physical-allocation".len() as u64
    );
    assert!(matches!(
        report.regions(),
        [OperationalTruthRegion::AliasGroup { claimants, .. }] if claimants.len() == 2
    ));
}

#[test]
fn distinct_files_cannot_be_promoted_to_an_overlap_conflict_by_a_caller() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("primary.extent");
    std::fs::write(&path, b"extent").expect("media");
    let competing = directory.path().join("competing.extent");
    std::fs::write(&competing, b"other-extent").expect("competing media");
    let report = compose_operational_truth(
        inspect(directory.path()),
        &OfflineTruthEvidenceSet::default(),
        truth_composition_budget(),
    )
    .expect("truth report");
    assert_eq!(report.regions().len(), 2);
    assert!(report
        .regions()
        .iter()
        .all(|region| !matches!(region, OperationalTruthRegion::OverlapConflict { .. })));
}

fn truth_composition_budget() -> OperationalTruthCompositionBudget {
    OperationalTruthCompositionBudget::bounded(16 * 1024 * 1024)
        .expect("test truth-composition budget")
}

#[test]
fn checkpoint_substitution_cannot_skip_inspection_bytes() {
    let first = tempfile::tempdir().expect("first directory");
    std::fs::write(first.path().join("first.page"), vec![1u8; 128]).expect("first media");
    let mut partial = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        first.path(),
        content_closure(
            "first",
            [(&first.path().join("first.page"), &vec![1u8; 128])],
        ),
    ))
    .budget(OfflineInspectionBudget::bounded(16, 1024).expect("budget"))
    .start()
    .expect("start");
    partial.advance().expect("advance").expect("progress");
    let checkpoint = partial.checkpoint().expect("checkpoint");

    let second = tempfile::tempdir().expect("second directory");
    std::fs::write(second.path().join("second.page"), vec![2u8; 96]).expect("second media");
    let walked = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        second.path(),
        content_closure(
            "second",
            [(&second.path().join("second.page"), &vec![2u8; 96])],
        ),
    ))
    .budget(OfflineInspectionBudget::bounded(16, 1024).expect("budget"))
    .resume_from_checkpoint(&checkpoint)
    .expect("safe restart")
    .finish()
    .expect("finish");
    assert_eq!(walked.counters().bytes_read(), 96);
}

#[test]
fn cancellation_is_observed_before_any_media_byte_is_admitted() {
    let directory = tempfile::tempdir().expect("directory");
    let path = directory.path().join("cancelled.page");
    let bytes = vec![7_u8; 128];
    std::fs::write(&path, &bytes).expect("media");
    let cancellation = crate::OfflineInspectionCancellation::new();
    let mut session = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        content_closure("cancelled", [(&path, &bytes)]),
    ))
    .cancellation(cancellation.clone())
    .budget(OfflineInspectionBudget::bounded(16, 1024).expect("budget"))
    .start()
    .expect("acquisition");
    cancellation.cancel();
    assert!(matches!(
        session.advance(),
        Err(OfflineInspectionDenial::Cancelled)
    ));
    assert_eq!(
        session.checkpoint().expect("checkpoint").observed_bytes(),
        0
    );
}

#[test]
fn high_cardinality_media_is_walked_without_resident_handle_per_file() {
    let directory = tempfile::tempdir().expect("temp directory");
    const FILES: usize = 1_200;
    let mut closure = Vec::with_capacity(FILES);
    for index in 0..FILES {
        let path = directory.path().join(format!("{index:04}.page"));
        let bytes = [index as u8];
        std::fs::write(&path, bytes).expect("media file");
        closure.push(
            OfflineMediaClosureEntry::new(path, 1, Sha256::digest(bytes).into())
                .expect("closure row"),
        );
    }
    let walked = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        OfflineMediaConsistencyBasis::content_addressed_closure("many-files", closure)
            .expect("content closure"),
    ))
    .budget(OfflineInspectionBudget::bounded(1, FILES as u64).expect("budget"))
    .start()
    .expect("acquire without descriptor exhaustion")
    .finish()
    .expect("walk all media");
    assert_eq!(walked.files().len(), FILES);
    assert_eq!(walked.counters().file_touches(), FILES as u64);
    assert_eq!(walked.counters().peak_buffer_bytes(), 1);
}

fn inspect(root: &std::path::Path) -> crate::StructurallyWalkedMedia {
    let entries = std::fs::read_dir(root)
        .expect("test media directory")
        .map(|entry| {
            let path = entry.expect("directory entry").path();
            let bytes = std::fs::read(&path).expect("test media");
            OfflineMediaClosureEntry::new(path, bytes.len() as u64, Sha256::digest(bytes).into())
                .expect("closure row")
        })
        .collect::<Vec<_>>();
    OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        root,
        OfflineMediaConsistencyBasis::content_addressed_closure("test-closure", entries)
            .expect("content closure"),
    ))
    .budget(OfflineInspectionBudget::bounded(64, 4096).expect("budget"))
    .start()
    .expect("start")
    .finish()
    .expect("finish")
}

fn content_closure<'a, B: AsRef<[u8]> + ?Sized + 'a>(
    identity: &str,
    entries: impl IntoIterator<Item = (&'a std::path::PathBuf, &'a B)>,
) -> OfflineMediaConsistencyBasis {
    OfflineMediaConsistencyBasis::content_addressed_closure(
        identity,
        entries.into_iter().map(|(path, bytes)| {
            let bytes = bytes.as_ref();
            OfflineMediaClosureEntry::new(path, bytes.len() as u64, Sha256::digest(bytes).into())
                .expect("closure row")
        }),
    )
    .expect("content-addressed closure")
}
