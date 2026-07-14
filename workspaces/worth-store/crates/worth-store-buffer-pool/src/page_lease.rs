use crate::{
    PageLeaseId, PinnedPageLease, ResidentFrameDenial, ResidentFrameIdentity, ResidentFrameTable,
};

#[derive(Debug)]
pub struct PageLease<'table> {
    table: &'table mut ResidentFrameTable,
    lease_id: PageLeaseId,
    identity: ResidentFrameIdentity,
}

impl<'table> PageLease<'table> {
    pub(crate) const fn new(
        table: &'table mut ResidentFrameTable,
        lease_id: PageLeaseId,
        identity: ResidentFrameIdentity,
    ) -> Self {
        Self {
            table,
            lease_id,
            identity,
        }
    }

    pub fn pin(self) -> Result<PinnedPageLease<'table>, ResidentFrameDenial> {
        self.table.begin_pin(self.lease_id, self.identity)?;
        Ok(PinnedPageLease::new(
            self.table,
            self.lease_id,
            self.identity,
        ))
    }

    pub const fn lease_id(&self) -> PageLeaseId {
        self.lease_id
    }
}
