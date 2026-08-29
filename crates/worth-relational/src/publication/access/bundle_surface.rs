use crate::publication::bundle::PublicationBundle;
use crate::publication::patch::data::PublishedAuthoritativePatchEnvelope;
use crate::runtime::RelationalReplayRecord;

use super::PublicationArtifactsAccess;

impl<'runtime> PublicationArtifactsAccess<'runtime> {
    pub fn latest_bundle(
        &self,
    ) -> Option<std::sync::Arc<PublicationBundle<RelationalReplayRecord>>> {
        self.runtime.publication.latest_bundle()
    }

    pub fn latest_patch(&self) -> Option<PublishedAuthoritativePatchEnvelope> {
        self.latest_bundle().map(|bundle| bundle.patch.clone())
    }

    pub fn latest_replay(&self) -> Option<RelationalReplayRecord> {
        self.latest_bundle().map(|bundle| bundle.replay.clone())
    }
}
