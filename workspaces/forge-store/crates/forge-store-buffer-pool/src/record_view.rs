use crate::{
    AllocationReceipt, AllocationRequestKind, RecordCopyCounterSnapshot, RecordViewDenial,
    RecordViewDenialKind, ResidentFrameLoadRequest,
};
use forge_store_physical_format::{FramedRecordView, PhysicalReference, RecordPlacementWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordViewMaterializationProfile {
    PhysicalBytesOnly,
    BoundedPhysicalCopy,
    RichSemanticMaterialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordViewAccess {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordViewAdmission {
    request: ResidentFrameLoadRequest,
    placement: RecordPlacementWitness,
    counters: RecordCopyCounterSnapshot,
}

impl RecordViewAdmission {
    pub(crate) const fn new(
        request: ResidentFrameLoadRequest,
        placement: RecordPlacementWitness,
        counters: RecordCopyCounterSnapshot,
    ) -> Self {
        Self {
            request,
            placement,
            counters,
        }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.request.reference().reference()
    }

    pub const fn placement(self) -> RecordPlacementWitness {
        self.placement
    }

    pub const fn counters(self) -> RecordCopyCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ZeroCopyRecordView<'lease> {
    bytes: &'lease [u8],
    admission: RecordViewAdmission,
    access: RecordViewAccess,
}

impl<'lease> ZeroCopyRecordView<'lease> {
    pub(crate) const fn new(
        bytes: &'lease [u8],
        admission: RecordViewAdmission,
        access: RecordViewAccess,
    ) -> Self {
        Self {
            bytes,
            admission,
            access,
        }
    }

    pub const fn physical_record_bytes(&self) -> &'lease [u8] {
        self.bytes
    }

    pub const fn len_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub const fn access(&self) -> RecordViewAccess {
        self.access
    }

    pub const fn admission(&self) -> RecordViewAdmission {
        self.admission
    }

    pub const fn counters(&self) -> RecordCopyCounterSnapshot {
        self.admission.counters()
    }

    pub const fn proves_semantic_domain_object(&self) -> bool {
        false
    }

    pub fn bounded_copy(
        self,
        receipt: AllocationReceipt,
    ) -> Result<BoundedCopyRecordView, RecordViewDenial> {
        let counters = self.counters().with_bounded_copy_attempt();
        reject_unadmitted_copy_receipt(receipt, self.bytes.len() as u64, counters)?;
        let copied = self.bytes.to_vec();
        let counters = match receipt.kind() {
            AllocationRequestKind::CopiedPayload => counters.with_bounded_copy(receipt.bytes()),
            AllocationRequestKind::MaterializedRecordSet => {
                counters.with_materialized_copy(receipt.bytes())
            }
            _ => counters,
        };
        Ok(BoundedCopyRecordView::new(copied, counters))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundedCopyRecordView {
    bytes: Vec<u8>,
    counters: RecordCopyCounterSnapshot,
}

impl BoundedCopyRecordView {
    const fn new(bytes: Vec<u8>, counters: RecordCopyCounterSnapshot) -> Self {
        Self { bytes, counters }
    }

    pub fn physical_record_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub const fn counters(&self) -> RecordCopyCounterSnapshot {
        self.counters
    }

    pub const fn proves_semantic_domain_object(&self) -> bool {
        false
    }
}

pub(crate) fn reject_unadmitted_view_profile(
    profile: RecordViewMaterializationProfile,
    counters: RecordCopyCounterSnapshot,
) -> Result<(), RecordViewDenial> {
    match profile {
        RecordViewMaterializationProfile::PhysicalBytesOnly
        | RecordViewMaterializationProfile::BoundedPhysicalCopy => Ok(()),
        RecordViewMaterializationProfile::RichSemanticMaterialization => {
            Err(RecordViewDenial::new(
                RecordViewDenialKind::ProfileForbidsMaterialization,
                counters,
            ))
        }
    }
}

pub(crate) fn reject_mismatched_framed_record(
    request: ResidentFrameLoadRequest,
    framed_record: FramedRecordView<'_>,
    resident_len: usize,
    counters: RecordCopyCounterSnapshot,
) -> Result<(), RecordViewDenial> {
    if request.reference().reference() != framed_record.placement().reference() {
        return Err(RecordViewDenial::new(
            RecordViewDenialKind::PhysicalReferenceMismatch,
            counters,
        ));
    }
    if request.header().owner() != framed_record.placement().reference().generation_owner() {
        return Err(RecordViewDenial::new(
            RecordViewDenialKind::HeaderWitnessMismatch,
            counters,
        ));
    }
    if resident_len != framed_record.payload().as_bytes().len() {
        return Err(RecordViewDenial::new(
            RecordViewDenialKind::ResidentPayloadLengthMismatch,
            counters,
        ));
    }
    Ok(())
}

fn reject_unadmitted_copy_receipt(
    receipt: AllocationReceipt,
    expected_bytes: u64,
    counters: RecordCopyCounterSnapshot,
) -> Result<(), RecordViewDenial> {
    if !matches!(
        receipt.kind(),
        AllocationRequestKind::CopiedPayload | AllocationRequestKind::MaterializedRecordSet
    ) {
        return Err(RecordViewDenial::new(
            RecordViewDenialKind::AllocationReceiptKindMismatch,
            counters,
        ));
    }
    if receipt.bytes() != expected_bytes {
        return Err(RecordViewDenial::new(
            RecordViewDenialKind::AllocationReceiptByteMismatch,
            counters,
        ));
    }
    Ok(())
}
