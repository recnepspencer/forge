use crate::publication::bundle::PublicationBundle;
use crate::runtime::{RelationalReplayRecord, RelationalRuntime};

pub(crate) trait PublicationBundleSource {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>>;
}

impl PublicationBundleSource for RelationalRuntime {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.publication.latest_bundle.as_ref()
    }
}
