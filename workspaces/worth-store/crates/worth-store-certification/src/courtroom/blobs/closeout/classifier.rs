use super::{
    denial::shortcut_rejection, BlobCloseoutCertificationInput, BlobCloseoutDenial,
    BlobCloseoutRequest,
};

#[derive(Debug)]
pub(crate) struct ClassifiedBlobCloseoutRequest {
    pub(crate) input: BlobCloseoutCertificationInput,
}

pub(crate) fn classify_blob_closeout_request(
    request: BlobCloseoutRequest,
) -> Result<ClassifiedBlobCloseoutRequest, BlobCloseoutDenial> {
    match request {
        BlobCloseoutRequest::Canonical(input) => {
            if !input.policy().is_counter_backed_foundational() {
                return Err(BlobCloseoutDenial::CounterBackedFoundationalPolicyRequired);
            }
            Ok(ClassifiedBlobCloseoutRequest { input: *input })
        }
        BlobCloseoutRequest::Shortcut(shortcut) => Err(BlobCloseoutDenial::ShortcutRejected(
            shortcut_rejection(&shortcut),
        )),
    }
}
