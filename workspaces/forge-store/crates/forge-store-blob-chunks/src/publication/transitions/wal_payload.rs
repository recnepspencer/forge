use super::super::evidence::publication_payload_frame_digest;
use super::super::types::reachability_staging::BlobReachabilityStaging;
use super::super::types::wal_types::BlobPublicationWalPayload;

pub(crate) fn from_staged_reachability(
    staged: &BlobReachabilityStaging,
) -> BlobPublicationWalPayload {
    let staging_identity = staged.staging_identity().clone();
    let frame_digest = publication_payload_frame_digest(&staging_identity);
    BlobPublicationWalPayload {
        staging_identity,
        frame_digest,
    }
}
