use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::*;
use crate::physical_runtime::record_serving::residency::candidate_frame_publishers::{
    BoundedCandidateFramePublisher, CandidateFrameCounterCells,
};
use worth_store_buffer_pool::{PhysicalResidencyLimits, PhysicalResidencyPool};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};

#[path = "tests/effect_fate_cleanup.rs"]
mod effect_fate_cleanup;
#[path = "tests/exact_receipt.rs"]
mod exact_receipt;
#[path = "tests/publication_ownership.rs"]
mod publication_ownership;

fn segment_coordinate(offset: u64) -> CandidateFrameCoordinate {
    CandidateFrameCoordinate::new(
        RecordArtifactFile::Segment {
            segment: 1,
            generation: 1,
        },
        offset,
    )
}

fn session(declaration: CandidateFrameSet) -> StoreCandidateFramePublicationSession {
    let port = RetainingPublisher {
        active: Arc::new(AtomicBool::new(false)),
        coordinates: Arc::new(std::sync::Mutex::new(Vec::new())),
    };
    StoreCandidateFramePublicationSession::begin(&port, declaration).unwrap()
}

fn declared_inline_frames(frames: &[(u64, u32)]) -> CandidateFrameSet {
    CandidateFrameSet::new(
        1,
        frames
            .iter()
            .map(|(offset, length)| {
                CandidateFrameDeclaration::new(
                    CandidateFrameRole::InlinePage,
                    segment_coordinate(*offset),
                    *length,
                )
                .unwrap()
            })
            .collect(),
    )
    .unwrap()
}

#[test]
fn store_writes_each_coordinate_and_requires_the_complete_declaration() {
    let mut session = session(
        CandidateFrameSet::new(
            7,
            vec![
                CandidateFrameDeclaration::new(
                    CandidateFrameRole::InlinePage,
                    segment_coordinate(0),
                    2,
                )
                .unwrap(),
                CandidateFrameDeclaration::new(
                    CandidateFrameRole::InlinePage,
                    segment_coordinate(2),
                    2,
                )
                .unwrap(),
            ],
        )
        .unwrap(),
    );
    let mut written = Vec::new();
    let first = session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![1, 2],
            ),
            &mut |bytes| {
                written.extend_from_slice(bytes);
                Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test())
            },
        )
        .unwrap();
    assert!(first.into_reusable_bytes().is_none());
    assert_eq!(
        session.require_complete(),
        Err(CandidateFrameContractViolation::IncompleteFrameSet)
    );
    session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(2),
                vec![3, 4],
            ),
            &mut |bytes| {
                written.extend_from_slice(bytes);
                Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test())
            },
        )
        .unwrap();
    assert_eq!(session.require_complete(), Ok(()));
    assert_eq!(written, [1, 2, 3, 4]);
}

#[test]
fn declaration_and_coordinate_violations_precede_store_writes() {
    let mut oversized = session(declared_inline_frames(&[(0, 1)]));
    let mut writes = 0;
    let failure = oversized
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![1, 2],
            ),
            &mut |_| {
                writes += 1;
                Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test())
            },
        )
        .unwrap_err();
    assert!(matches!(
        failure,
        CandidateFrameWriteFailure::Contract(CandidateFrameContractViolation::UnexpectedFrame)
    ));
    let mut mismatched = session(declared_inline_frames(&[(0, 1)]));
    let failure = mismatched
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::CatalogCandidate,
                segment_coordinate(0),
                vec![1],
            ),
            &mut |_| {
                writes += 1;
                Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test())
            },
        )
        .unwrap_err();
    assert!(matches!(
        failure,
        CandidateFrameWriteFailure::Contract(
            CandidateFrameContractViolation::CoordinateRoleMismatch
        )
    ));
    assert_eq!(writes, 0);
}

#[test]
fn residency_can_keep_identified_dirty_frames_until_session_release() {
    let active = Arc::new(AtomicBool::new(false));
    let coordinates = Arc::new(std::sync::Mutex::new(Vec::new()));
    let port = RetainingPublisher {
        active: Arc::clone(&active),
        coordinates: Arc::clone(&coordinates),
    };
    let mut session =
        StoreCandidateFramePublicationSession::begin(&port, declared_inline_frames(&[(64, 1)]))
            .unwrap();
    let completion = session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(64),
                vec![7],
            ),
            &mut |_| Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test()),
        )
        .unwrap();
    assert!(completion.into_reusable_bytes().is_none());
    assert!(active.load(Ordering::Acquire));
    assert_eq!(
        coordinates.lock().unwrap().as_slice(),
        &[segment_coordinate(64)]
    );
    drop(session);
    assert!(!active.load(Ordering::Acquire));
}

#[test]
fn retained_bytes_must_still_be_the_declared_candidate_before_any_store_effect() {
    let port = MutatingPublisher;
    let mut session =
        StoreCandidateFramePublicationSession::begin(&port, declared_inline_frames(&[(0, 3)]))
            .unwrap();
    let mut writes = 0;
    let failure = session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![1, 2, 3],
            ),
            &mut |_| {
                writes += 1;
                Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test())
            },
        )
        .unwrap_err();
    assert!(matches!(
        failure,
        CandidateFrameWriteFailure::Contract(
            CandidateFrameContractViolation::RetainedFrameBytesChanged
        )
    ));
    assert_eq!(writes, 0);
}

struct MutatingPublisher;

impl CandidateFramePublicationPort for MutatingPublisher {
    fn begin(
        &self,
        _: &CandidateFrameSet,
    ) -> Result<Box<dyn CandidateFrameResidencySession>, RecordAppendDenial> {
        Ok(Box::new(MutatingSession))
    }
}

struct MutatingSession;

impl CandidateFrameResidencySession for MutatingSession {
    fn retain(
        &mut self,
        mut frame: CandidateFrame,
    ) -> Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial> {
        frame.bytes[0] ^= 0xff;
        Ok(Box::new(MutatingResident(frame)))
    }

    fn prepare_catalog_cutover(
        &mut self,
        _: CandidateFrameCoordinate,
        _: u32,
    ) -> Result<(), RecordAppendDenial> {
        Ok(())
    }
}

struct MutatingResident(CandidateFrame);

impl ResidentCandidateFrame for MutatingResident {
    fn role(&self) -> CandidateFrameRole {
        self.0.role()
    }
    fn coordinate(&self) -> CandidateFrameCoordinate {
        self.0.coordinate()
    }
    fn bytes(&self) -> &[u8] {
        self.0.bytes()
    }
    fn discard(self: Box<Self>) -> Result<(), RecordAppendDenial> {
        Ok(())
    }
    fn publish_clean(
        self: Box<Self>,
        _physical: &CandidateFramePhysicalWrite,
    ) -> Result<CandidateFrameWriteCompletion, RecordAppendDenial> {
        Ok(CandidateFrameWriteCompletion::for_contract_test(
            self.0.bytes().len() as u64,
        ))
    }
}

struct RetainingPublisher {
    active: Arc<AtomicBool>,
    coordinates: Arc<std::sync::Mutex<Vec<CandidateFrameCoordinate>>>,
}

impl CandidateFramePublicationPort for RetainingPublisher {
    fn begin(
        &self,
        _: &CandidateFrameSet,
    ) -> Result<Box<dyn CandidateFrameResidencySession>, RecordAppendDenial> {
        Ok(Box::new(RetainingSession {
            frames: Arc::new(std::sync::Mutex::new(Vec::new())),
            active: Arc::clone(&self.active),
            coordinates: Arc::clone(&self.coordinates),
        }))
    }
}

struct RetainingSession {
    frames: Arc<std::sync::Mutex<Vec<CandidateFrame>>>,
    active: Arc<AtomicBool>,
    coordinates: Arc<std::sync::Mutex<Vec<CandidateFrameCoordinate>>>,
}

impl CandidateFrameResidencySession for RetainingSession {
    fn retain(
        &mut self,
        frame: CandidateFrame,
    ) -> Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial> {
        self.active.store(true, Ordering::Release);
        self.coordinates.lock().unwrap().push(frame.coordinate());
        Ok(Box::new(RetainingResidentFrame {
            frame: Some(frame),
            retained: Arc::clone(&self.frames),
        }))
    }

    fn prepare_catalog_cutover(
        &mut self,
        _target: CandidateFrameCoordinate,
        _length: u32,
    ) -> Result<(), RecordAppendDenial> {
        Ok(())
    }
}

struct RetainingResidentFrame {
    frame: Option<CandidateFrame>,
    retained: Arc<std::sync::Mutex<Vec<CandidateFrame>>>,
}

impl ResidentCandidateFrame for RetainingResidentFrame {
    fn role(&self) -> CandidateFrameRole {
        self.frame.as_ref().unwrap().role()
    }
    fn coordinate(&self) -> CandidateFrameCoordinate {
        self.frame.as_ref().unwrap().coordinate()
    }
    fn bytes(&self) -> &[u8] {
        self.frame.as_ref().unwrap().bytes()
    }

    fn discard(self: Box<Self>) -> Result<(), RecordAppendDenial> {
        Ok(())
    }

    fn publish_clean(
        mut self: Box<Self>,
        _physical: &CandidateFramePhysicalWrite,
    ) -> Result<CandidateFrameWriteCompletion, RecordAppendDenial> {
        let frame = self.frame.take().unwrap();
        let bytes = frame.bytes().len() as u64;
        self.retained.lock().unwrap().push(frame);
        Ok(CandidateFrameWriteCompletion::for_contract_test(bytes))
    }
}

impl Drop for RetainingSession {
    fn drop(&mut self) {
        self.active.store(false, Ordering::Release);
    }
}
