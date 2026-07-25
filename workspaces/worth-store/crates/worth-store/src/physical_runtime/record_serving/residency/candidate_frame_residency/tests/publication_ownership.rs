use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::*;

#[test]
fn residency_covers_store_write() {
    let dirty = Arc::new(AtomicBool::new(false));
    let port = PublicationOrderPublisher {
        dirty: Arc::clone(&dirty),
    };
    let declaration = CandidateFrameSet::new(
        1,
        vec![CandidateFrameDeclaration::new(
            CandidateFrameRole::InlinePage,
            segment_coordinate(0),
            3,
        )
        .unwrap()],
    )
    .unwrap();
    let mut session = StoreCandidateFramePublicationSession::begin(&port, declaration).unwrap();

    session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![1, 2, 3],
            ),
            &mut |_| {
                assert!(
                    dirty.load(Ordering::Acquire),
                    "C5_PREDICATE:publication-ownership store write escaped dirty residency"
                );
                Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test())
            },
        )
        .unwrap();

    assert!(!dirty.load(Ordering::Acquire));
}

struct PublicationOrderPublisher {
    dirty: Arc<AtomicBool>,
}

impl CandidateFramePublicationPort for PublicationOrderPublisher {
    fn begin(
        &self,
        _: &CandidateFrameSet,
    ) -> Result<Box<dyn CandidateFrameResidencySession>, RecordAppendDenial> {
        Ok(Box::new(PublicationOrderSession {
            dirty: Arc::clone(&self.dirty),
        }))
    }
}

struct PublicationOrderSession {
    dirty: Arc<AtomicBool>,
}

impl CandidateFrameResidencySession for PublicationOrderSession {
    fn retain(
        &mut self,
        frame: CandidateFrame,
    ) -> Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial> {
        assert!(!self.dirty.swap(true, Ordering::AcqRel));
        Ok(Box::new(PublicationOrderResident {
            frame,
            dirty: Arc::clone(&self.dirty),
        }))
    }

    fn prepare_catalog_cutover(
        &mut self,
        _: CandidateFrameCoordinate,
        _: u32,
    ) -> Result<(), RecordAppendDenial> {
        Ok(())
    }
}

struct PublicationOrderResident {
    frame: CandidateFrame,
    dirty: Arc<AtomicBool>,
}

impl ResidentCandidateFrame for PublicationOrderResident {
    fn role(&self) -> CandidateFrameRole {
        self.frame.role()
    }

    fn coordinate(&self) -> CandidateFrameCoordinate {
        self.frame.coordinate()
    }

    fn bytes(&self) -> &[u8] {
        self.frame.bytes()
    }

    fn discard(self: Box<Self>) -> Result<(), RecordAppendDenial> {
        assert!(self.dirty.swap(false, Ordering::AcqRel));
        Ok(())
    }

    fn publish_clean(
        self: Box<Self>,
        _physical: &CandidateFramePhysicalWrite,
    ) -> Result<CandidateFrameWriteCompletion, RecordAppendDenial> {
        assert!(self.dirty.swap(false, Ordering::AcqRel));
        Ok(CandidateFrameWriteCompletion::for_contract_test(
            self.frame.bytes().len() as u64,
        ))
    }
}
