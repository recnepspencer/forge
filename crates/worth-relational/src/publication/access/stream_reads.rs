use crate::publication::cdc::data::{
    SubscriberResumeRequest, SubscriberStreamBatch, SubscriberStreamFailure,
};
use crate::publication::patch::data::{PatchStreamBatch, PatchStreamReadError, PatchStreamRequest};

use super::{PublicationPatchStreamAccess, PublicationSubscriberStreamAccess};

impl<'runtime> PublicationPatchStreamAccess<'runtime> {
    pub fn read(
        &self,
        request: PatchStreamRequest,
    ) -> Result<PatchStreamBatch, PatchStreamReadError> {
        crate::publication::patch::read_patch_stream(self.runtime, request)
    }
}

impl<'runtime> PublicationSubscriberStreamAccess<'runtime> {
    pub fn read(
        &self,
        request: SubscriberResumeRequest,
    ) -> Result<SubscriberStreamBatch, SubscriberStreamFailure> {
        crate::publication::cdc::access::read_subscriber_stream(self.runtime, request)
    }
}
