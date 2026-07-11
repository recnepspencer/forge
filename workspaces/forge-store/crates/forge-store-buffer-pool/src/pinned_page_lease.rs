use crate::{
    AccessPolicyBufferLifecycle, PageLeaseId, PinnedFrameView, RecordViewAccess, RecordViewDenial,
    RecordViewMaterializationProfile, ResidentFrameDenial, ResidentFrameIdentity,
    ResidentFrameTable, ResidentFrameToken, UnpinnedPageReceipt, ZeroCopyRecordView,
};
use forge_store_physical_format::{FramedRecordView, PhysicalReference};

#[derive(Debug)]
pub struct PinnedPageLease<'table> {
    table: &'table mut ResidentFrameTable,
    lease_id: PageLeaseId,
    identity: ResidentFrameIdentity,
    closed: bool,
}

impl<'table> PinnedPageLease<'table> {
    pub(crate) const fn new(
        table: &'table mut ResidentFrameTable,
        lease_id: PageLeaseId,
        identity: ResidentFrameIdentity,
    ) -> Self {
        Self {
            table,
            lease_id,
            identity,
            closed: false,
        }
    }

    pub fn view(&self) -> Result<PinnedFrameView<'_>, ResidentFrameDenial> {
        self.table.pinned_frame_view(self.identity)
    }

    pub fn zero_copy_record_view(
        &mut self,
        framed_record: FramedRecordView<'_>,
        profile: RecordViewMaterializationProfile,
    ) -> Result<ZeroCopyRecordView<'_>, RecordViewDenial> {
        let admission =
            self.table
                .admit_record_view_basis(self.identity, framed_record, profile)?;
        let pinned = self.view().map_err(|denial| {
            RecordViewDenial::from_resident(denial, self.table.record_view_counters())
        })?;
        Ok(ZeroCopyRecordView::new(
            pinned.as_bytes(),
            admission,
            RecordViewAccess::Immutable,
        ))
    }

    pub fn mutable_zero_copy_record_view(
        &mut self,
        _framed_record: FramedRecordView<'_>,
        _profile: RecordViewMaterializationProfile,
    ) -> Result<ZeroCopyRecordView<'_>, RecordViewDenial> {
        Err(self
            .table
            .deny_mutable_record_view_without_exclusive_lease())
    }

    pub fn unpin(mut self) -> Result<UnpinnedPageReceipt, ResidentFrameDenial> {
        let receipt = self.table.explicit_unpin(self.lease_id, self.identity)?;
        self.closed = true;
        Ok(receipt)
    }

    pub const fn lease_id(&self) -> PageLeaseId {
        self.lease_id
    }

    pub const fn resident_frame_token(&self) -> ResidentFrameToken {
        self.identity.token()
    }

    pub fn physical_reference(&self) -> Result<PhysicalReference, ResidentFrameDenial> {
        self.table.resident_physical_reference(self.identity)
    }

    pub const fn access_policy_lifecycle_proof(&self) -> AccessPolicyBufferLifecycle {
        AccessPolicyBufferLifecycle::pinned_physical_substrate_lease()
    }
}

impl Drop for PinnedPageLease<'_> {
    fn drop(&mut self) {
        if !self.closed {
            self.table.defensive_drop_pin(self.identity);
            self.closed = true;
        }
    }
}
