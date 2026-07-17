use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    OfflineMediaClosureEntry, OfflineMediaConsistencyBasis, ReadOnlyOfflineMediaCapability,
};

use super::{
    ForensicAcquisitionDenial, ForensicAcquisitionIntent, ForensicAcquisitionProgress,
    ForensicAcquisitionSession, ForensicRangePosture,
};

#[test]
fn crash_after_one_source_reopens_the_durable_plan_bound_session() {
    let fixture = Fixture::new("forensic-resume", 2);
    let source_before = fixture.source_digests();
    let plan = fixture.plan();
    let mut first = ForensicAcquisitionSession::open(plan.clone(), fixture.media()).unwrap();
    assert_eq!(
        first.acquire_next().unwrap(),
        ForensicAcquisitionProgress::SourceRecorded { source_index: 0 }
    );
    drop(first);

    let reopened = ForensicAcquisitionSession::open(plan, fixture.media()).unwrap();
    let (bundle, counters) = reopened.acquire(20).unwrap();

    assert_eq!(counters.recovered_source_records(), 1);
    assert_eq!(bundle.ranges().len(), 2);
    assert!(bundle
        .ranges()
        .iter()
        .all(|range| range.posture() == ForensicRangePosture::Acquired));
    assert_eq!(fixture.source_digests(), source_before);
    assert!(bundle.root().join("forensic.manifest").is_file());
}

#[test]
fn damaged_recovered_output_is_not_silently_trusted() {
    let fixture = Fixture::new("forensic-damaged-resume", 1);
    let plan = fixture.plan();
    let mut first = ForensicAcquisitionSession::open(plan.clone(), fixture.media()).unwrap();
    first.acquire_next().unwrap();
    drop(first);
    std::fs::write(fixture.target.join("evidence-00000000.bin"), b"substituted").unwrap();

    assert!(matches!(
        ForensicAcquisitionSession::open(plan, fixture.media()),
        Err(ForensicAcquisitionDenial::DamagedAcquisitionJournal)
    ));
}

#[test]
fn finalization_is_idempotent_for_the_same_completed_evidence() {
    let fixture = Fixture::new("forensic-finalization", 1);
    let plan = fixture.plan();
    let (first, _) = ForensicAcquisitionSession::open(plan.clone(), fixture.media())
        .unwrap()
        .acquire(20)
        .unwrap();
    let (reopened, _) = ForensicAcquisitionSession::open(plan, fixture.media())
        .unwrap()
        .acquire(20)
        .unwrap();

    assert_eq!(first.bundle_identity(), reopened.bundle_identity());
}

struct Fixture {
    _directory: tempfile::TempDir,
    sources: Vec<std::path::PathBuf>,
    target: std::path::PathBuf,
    basis: OfflineMediaConsistencyBasis,
}

impl Fixture {
    fn new(label: &str, count: usize) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let source_root = directory.path().join("source");
        let target = directory.path().join("evidence");
        std::fs::create_dir_all(&source_root).unwrap();
        let mut sources = Vec::new();
        let mut closure = Vec::new();
        for index in 0..count {
            let path = source_root.join(format!("source-{index}.bin"));
            let bytes = vec![(index + 1) as u8; 4096 + index * 733];
            std::fs::write(&path, &bytes).unwrap();
            closure.push(
                OfflineMediaClosureEntry::new(
                    path.clone(),
                    bytes.len() as u64,
                    Sha256::digest(&bytes).into(),
                )
                .unwrap(),
            );
            sources.push(path);
        }
        let basis =
            OfflineMediaConsistencyBasis::content_addressed_closure(label, closure).unwrap();
        Self {
            _directory: directory,
            sources,
            target,
            basis,
        }
    }

    fn media(&self) -> ReadOnlyOfflineMediaCapability {
        ReadOnlyOfflineMediaCapability::open(self.sources.clone(), self.basis.clone()).unwrap()
    }

    fn plan(&self) -> super::ForensicAcquisitionPlan {
        ForensicAcquisitionIntent::new(
            &self.target,
            "observer-1",
            "read-only-os-handle",
            "test-monotonic-clock",
            10,
            257,
        )
        .unwrap()
        .plan(&self.media())
        .unwrap()
    }

    fn source_digests(&self) -> Vec<[u8; 32]> {
        self.sources
            .iter()
            .map(|path| Sha256::digest(std::fs::read(path).unwrap()).into())
            .collect()
    }
}
