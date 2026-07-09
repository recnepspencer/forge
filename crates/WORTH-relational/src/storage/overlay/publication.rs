use crate::replay::data::RelationalReplayRecord;

#[derive(Debug, Clone)]
pub(crate) struct PublicationArtifacts {
    pub(crate) bundle: crate::publication::bundle::PublicationBundle<RelationalReplayRecord>,
}
