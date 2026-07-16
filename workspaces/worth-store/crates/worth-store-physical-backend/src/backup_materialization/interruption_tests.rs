use sha2::{Digest, Sha256};

use super::{
    observe_physical_backup_artifact, PhysicalBackupMaterializationCancellation,
    PhysicalBackupMaterializationDenial, PhysicalBackupMaterializationSession,
    PhysicalBackupSource,
};

const MANIFEST: &[u8] = b"cancellable-materialization-manifest";

#[test]
fn cancellation_before_a_copy_boundary_performs_no_source_read_or_output_write() {
    let fixture = Fixture::new();
    let target = fixture.directory.path().join("copy-cancel");
    let mut session = fixture.session(&target, "copy-cancel");
    let cancellation = PhysicalBackupMaterializationCancellation::new();
    cancellation.cancel();

    assert!(matches!(
        session.advance_with_cancellation(&cancellation),
        Err(PhysicalBackupMaterializationDenial::Cancelled)
    ));
    assert_eq!(session.counters().source_bytes_read(), 0);
    assert_eq!(session.counters().output_bytes_written(), 0);
    assert_eq!(
        std::fs::metadata(target.join(".incomplete-copy-cancel").join("artifact.bin"))
            .expect("empty staged artifact")
            .len(),
        0
    );
}

#[test]
fn every_publication_transition_observes_cancellation_before_its_effect() {
    let fixture = Fixture::new();
    for completed_transitions in 0..6 {
        let identity = format!("publication-cancel-{completed_transitions}");
        let target = fixture.directory.path().join(&identity);
        let mut publication = fixture
            .copied_session(&target, &identity)
            .begin_publication(MANIFEST.to_vec())
            .expect("publication session");
        for _ in 0..completed_transitions {
            publication
                .advance()
                .expect("publication transition")
                .expect("transition remains");
        }
        let counters_before = publication.counters();
        let cancellation = PhysicalBackupMaterializationCancellation::new();
        cancellation.cancel();
        assert!(matches!(
            publication.advance_with_cancellation(&cancellation),
            Err(PhysicalBackupMaterializationDenial::Cancelled)
        ));
        assert_eq!(publication.counters(), counters_before);
        drop(publication);

        let mut resumed = fixture.session(&target, &identity);
        while resumed.advance().expect("resumed copy") {}
        let bundle = resumed
            .begin_publication(MANIFEST.to_vec())
            .expect("resumed publication")
            .finish()
            .expect("converged publication");
        assert_eq!(
            std::fs::read(bundle.root().join("artifact.bin")).expect("artifact"),
            fixture.bytes
        );
    }
}

#[test]
fn cancellation_after_parent_durability_cannot_reclassify_completed_publication() {
    let fixture = Fixture::new();
    let target = fixture.directory.path().join("completed-cancel");
    let mut publication = fixture
        .copied_session(&target, "completed-cancel")
        .begin_publication(MANIFEST.to_vec())
        .expect("publication session");
    while publication
        .advance()
        .expect("publication transition")
        .is_some()
    {}
    let cancellation = PhysicalBackupMaterializationCancellation::new();
    cancellation.cancel();
    assert_eq!(
        publication
            .advance_with_cancellation(&cancellation)
            .expect("completed publication remains complete"),
        None
    );
}

#[test]
fn abandonment_removes_only_incomplete_output_and_releases_session_ownership() {
    let fixture = Fixture::new();
    let target = fixture.directory.path().join("abandon");
    let mut session = fixture.session(&target, "abandon");
    session.advance().expect("partial copy");
    let staging = target.join(".incomplete-abandon");
    assert!(staging.exists());

    let receipt = session.abandon().expect("physical abandonment");
    assert!(receipt.incomplete_output_removed());
    assert!(!receipt.completed_bundle_retained());
    assert_eq!(receipt.directory_sync_operations(), 1);
    assert!(!staging.exists());

    fixture
        .session(&target, "abandon")
        .abandon()
        .expect("same identity is available after abandonment");
}

struct Fixture {
    directory: tempfile::TempDir,
    source_path: std::path::PathBuf,
    bytes: Vec<u8>,
    physical_identity: [u8; 32],
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().expect("directory");
        let source_path = directory.path().join("source.bin");
        let bytes = (0..=255).cycle().take(641).collect::<Vec<_>>();
        std::fs::write(&source_path, &bytes).expect("source");
        let physical_identity = observe_physical_backup_artifact(&source_path, 23)
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
        PhysicalBackupSource::new(
            &self.source_path,
            "artifact.bin",
            self.bytes.len() as u64,
            Sha256::digest(&self.bytes).into(),
            self.physical_identity,
        )
        .expect("source")
    }

    fn session(
        &self,
        target: &std::path::Path,
        identity: &str,
    ) -> PhysicalBackupMaterializationSession {
        PhysicalBackupMaterializationSession::open_or_resume(target, identity, [self.source()], 23)
            .expect("materialization session")
    }

    fn copied_session(
        &self,
        target: &std::path::Path,
        identity: &str,
    ) -> PhysicalBackupMaterializationSession {
        let mut session = self.session(target, identity);
        while session.advance().expect("copy") {}
        session
    }
}
