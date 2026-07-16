use sha2::{Digest, Sha256};

use super::{
    observe_physical_backup_artifact, PhysicalBackupMaterializationDenial,
    PhysicalBackupMaterializationSession, PhysicalBackupPublicationProgress, PhysicalBackupSource,
};

const MANIFEST: &[u8] = b"owner-bound-binary-manifest";

#[test]
fn every_publication_durability_cut_reopens_and_converges() {
    let fixture = PublicationFixture::new();
    let expected_progress = [
        PhysicalBackupPublicationProgress::PendingManifestDurable,
        PhysicalBackupPublicationProgress::ManifestPublished,
        PhysicalBackupPublicationProgress::SessionDescriptorRemoved,
        PhysicalBackupPublicationProgress::StagingDirectoryDurable,
        PhysicalBackupPublicationProgress::FinalRootRenamed,
        PhysicalBackupPublicationProgress::ParentDirectoryDurable,
    ];

    for cut in 0..=expected_progress.len() {
        let target = fixture.directory.path().join(format!("target-{cut}"));
        let identity = format!("publication-cut-{cut}");
        let mut publication = fixture
            .copied_session(&target, &identity)
            .begin_publication(MANIFEST.to_vec())
            .expect("begin publication");
        for expected in expected_progress.iter().take(cut) {
            assert_eq!(
                publication.advance().expect("publication transition"),
                Some(*expected)
            );
        }
        drop(publication);

        let mut resumed = PhysicalBackupMaterializationSession::open_or_resume(
            &target,
            &identity,
            [fixture.source()],
            31,
        )
        .expect("reopen from durable filesystem state");
        while resumed.advance().expect("resume copy boundary") {}
        let bundle = resumed
            .begin_publication(MANIFEST.to_vec())
            .expect("resume publication")
            .finish()
            .expect("finish resumed publication");

        assert_eq!(
            std::fs::read(bundle.root().join("artifact.bin")).expect("artifact"),
            fixture.bytes
        );
        assert_eq!(
            std::fs::read(bundle.root().join("backup.manifest")).expect("manifest"),
            MANIFEST
        );
    }
}

#[test]
fn fresh_process_publication_cut_matrix_reopens_from_filesystem_truth() {
    const CHILD_SOURCE: &str = "WORTH_STORE_PUBLICATION_CUT_SOURCE";
    const CHILD_TARGET: &str = "WORTH_STORE_PUBLICATION_CUT_TARGET";
    const CHILD_IDENTITY: &str = "WORTH_STORE_PUBLICATION_CUT_IDENTITY";
    const CHILD_CUT: &str = "WORTH_STORE_PUBLICATION_CUT_INDEX";
    const CHILD_EXIT: i32 = 74;

    if let (Some(source_path), Some(target), Some(identity), Some(cut)) = (
        std::env::var_os(CHILD_SOURCE),
        std::env::var_os(CHILD_TARGET),
        std::env::var_os(CHILD_IDENTITY),
        std::env::var_os(CHILD_CUT),
    ) {
        let source_path = std::path::PathBuf::from(source_path);
        let target = std::path::PathBuf::from(target);
        let identity = identity.to_string_lossy();
        let cut = cut
            .to_string_lossy()
            .parse::<usize>()
            .expect("child cut index");
        let bytes = std::fs::read(&source_path).expect("child source");
        let physical_identity = observe_physical_backup_artifact(&source_path, 31)
            .expect("child observation")
            .physical_identity();
        let source = PhysicalBackupSource::new(
            source_path,
            "artifact.bin",
            bytes.len() as u64,
            Sha256::digest(bytes).into(),
            physical_identity,
        )
        .expect("child source declaration");
        let mut materialization =
            PhysicalBackupMaterializationSession::open_or_resume(target, &identity, [source], 31)
                .expect("child materialization");
        while materialization.advance().expect("child copy") {}
        let mut publication = materialization
            .begin_publication(MANIFEST.to_vec())
            .expect("child publication");
        for _ in 0..cut {
            publication
                .advance()
                .expect("child publication cut")
                .expect("cut precedes completion");
        }
        std::process::exit(CHILD_EXIT);
    }

    let fixture = PublicationFixture::new();
    for cut in 0..=6 {
        let target = fixture
            .directory
            .path()
            .join(format!("fresh-process-target-{cut}"));
        let identity = format!("fresh-process-cut-{cut}");
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .arg("--exact")
            .arg(
                "backup_materialization::publication_tests::fresh_process_publication_cut_matrix_reopens_from_filesystem_truth",
            )
            .arg("--nocapture")
            .env(CHILD_SOURCE, &fixture.source_path)
            .env(CHILD_TARGET, &target)
            .env(CHILD_IDENTITY, &identity)
            .env(CHILD_CUT, cut.to_string())
            .status()
            .expect("crashing child");
        assert_eq!(status.code(), Some(CHILD_EXIT));

        let mut resumed = PhysicalBackupMaterializationSession::open_or_resume(
            &target,
            &identity,
            [fixture.source()],
            31,
        )
        .expect("fresh parent resume");
        while resumed.advance().expect("parent resume copy") {}
        let bundle = resumed
            .begin_publication(MANIFEST.to_vec())
            .expect("parent resume publication")
            .finish()
            .expect("parent finish");
        assert_eq!(
            std::fs::read(bundle.root().join("artifact.bin")).expect("artifact"),
            fixture.bytes
        );
    }
}

#[test]
fn completed_publication_is_revalidated_instead_of_trusted_or_overwritten() {
    let fixture = PublicationFixture::new();
    let target = fixture.directory.path().join("target-corrupt-final");
    let identity = "corrupt-final";
    let bundle = fixture
        .copied_session(&target, identity)
        .begin_publication(MANIFEST.to_vec())
        .expect("begin")
        .finish()
        .expect("publish");
    std::fs::write(
        bundle.root().join("artifact.bin"),
        vec![0x99; fixture.bytes.len()],
    )
    .expect("same-length corruption");

    let resumed = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        identity,
        [fixture.source()],
        31,
    )
    .expect("published recovery session");
    let denial = resumed
        .begin_publication(MANIFEST.to_vec())
        .expect("publication state discovery")
        .finish()
        .expect_err("corrupt final bytes cannot satisfy idempotent recovery");

    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { .. }
    ));
}

#[test]
fn sealed_staging_resume_does_not_recreate_the_removed_session_descriptor() {
    let fixture = PublicationFixture::new();
    let target = fixture.directory.path().join("sealed-staging");
    let identity = "sealed-staging";
    let staging = target.join(".incomplete-sealed-staging");
    let descriptor = staging.join("materialization.session");
    let mut publication = fixture
        .copied_session(&target, identity)
        .begin_publication(MANIFEST.to_vec())
        .expect("publication");
    for _ in 0..3 {
        publication
            .advance()
            .expect("publication transition")
            .expect("transition");
    }
    assert!(!descriptor.exists());
    drop(publication);

    let resumed = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        identity,
        [fixture.source()],
        31,
    )
    .expect("resume sealed staging");
    assert!(!descriptor.exists());
    resumed
        .begin_publication(MANIFEST.to_vec())
        .expect("resume publication")
        .finish()
        .expect("publish sealed staging");
}

#[test]
fn recovered_manifest_cannot_publish_unvalidated_staging_payloads() {
    let fixture = PublicationFixture::new();
    let target = fixture
        .directory
        .path()
        .join("hostile-recovered-publication");
    let identity = "hostile-recovered-publication";
    let staging = target.join(".incomplete-hostile-recovered-publication");
    std::fs::create_dir_all(&staging).expect("hostile staging root");
    std::fs::write(staging.join("backup.manifest"), MANIFEST).expect("published manifest");
    let wrong_payload = vec![0x91; fixture.bytes.len()];
    std::fs::write(staging.join("artifact.bin"), &wrong_payload).expect("wrong artifact payload");

    let resumed = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        identity,
        [fixture.source()],
        31,
    )
    .expect("publication recovery is discovered before manifest bytes are supplied");
    let denial = resumed
        .begin_publication(MANIFEST.to_vec())
        .err()
        .expect("recovered payload must be independently revalidated");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { .. }
    ));
    assert!(!target.join("backup-hostile-recovered-publication").exists());
    assert_eq!(
        std::fs::read(staging.join("artifact.bin")).expect("hostile payload remains quarantined"),
        wrong_payload
    );
}

#[test]
fn duplicate_and_reserved_output_names_fail_before_staging_creation() {
    let fixture = PublicationFixture::new();
    let duplicate_target = fixture.directory.path().join("duplicate-target");
    let duplicate = PhysicalBackupMaterializationSession::open_or_resume(
        &duplicate_target,
        "duplicate",
        [fixture.source(), fixture.source_named("Artifact.bin")],
        31,
    );
    assert!(matches!(
        duplicate,
        Err(PhysicalBackupMaterializationDenial::DuplicateOutputName { .. })
    ));
    assert!(!duplicate_target.exists());

    let reserved_target = fixture.directory.path().join("reserved-target");
    let reserved_source = fixture.source_named("backup.manifest");
    let reserved = PhysicalBackupMaterializationSession::open_or_resume(
        &reserved_target,
        "reserved",
        [reserved_source],
        31,
    );
    assert!(matches!(
        reserved,
        Err(PhysicalBackupMaterializationDenial::ReservedOutputName { .. })
    ));
    assert!(!reserved_target.exists());
}

struct PublicationFixture {
    directory: tempfile::TempDir,
    source_path: std::path::PathBuf,
    bytes: Vec<u8>,
    physical_identity: [u8; 32],
}

impl PublicationFixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().expect("directory");
        let source_path = directory.path().join("source.bin");
        let bytes = (0..=255).cycle().take(513).collect::<Vec<_>>();
        std::fs::write(&source_path, &bytes).expect("source");
        let physical_identity = observe_physical_backup_artifact(&source_path, 31)
            .expect("source observation")
            .physical_identity();
        Self {
            directory,
            source_path,
            bytes,
            physical_identity,
        }
    }

    fn source(&self) -> PhysicalBackupSource {
        self.source_named("artifact.bin")
    }

    fn source_named(&self, output_name: &str) -> PhysicalBackupSource {
        PhysicalBackupSource::new(
            &self.source_path,
            output_name,
            self.bytes.len() as u64,
            Sha256::digest(&self.bytes).into(),
            self.physical_identity,
        )
        .expect("source")
    }

    fn copied_session(
        &self,
        target: &std::path::Path,
        identity: &str,
    ) -> PhysicalBackupMaterializationSession {
        let mut session = PhysicalBackupMaterializationSession::open_or_resume(
            target,
            identity,
            [self.source()],
            31,
        )
        .expect("materialization session");
        while session.advance().expect("copy") {}
        session
    }
}
