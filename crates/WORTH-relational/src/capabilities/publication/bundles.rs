use crate::history::data::CommitReference;
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime};
use crate::publication::bundle::PublicationBundle;

pub(crate) trait PublicationBundleSource {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>>;

    fn latest_published_commit_ref(&self) -> Option<&CommitReference> {
        self.latest_publication_bundle()
            .map(|bundle| &bundle.commit)
    }
}

impl PublicationBundleSource for RelationalRuntime {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.publication.latest_bundle.as_ref()
    }
}
