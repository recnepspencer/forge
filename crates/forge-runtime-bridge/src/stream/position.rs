use std::sync::Arc;

use crate::identity::{BridgeIdentity, StreamPositionIdentityTag};
use crate::routing::canonicalization::digest_string;

use super::member::CanonicalStreamMember;
use super::protocol::StreamProtocolIdentity;

type StreamPositionIdentity = BridgeIdentity<StreamPositionIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalStreamPosition {
    stream_position_identity: StreamPositionIdentity,
    stream_protocol_identity: StreamProtocolIdentity,
    canonical_stream_member_identity: Arc<str>,
    ordinal_position: usize,
    digest: Arc<str>,
}

impl CanonicalStreamPosition {
    pub(crate) fn new(
        stream_protocol_identity: StreamProtocolIdentity,
        member: &CanonicalStreamMember,
        ordinal_position: usize,
    ) -> Self {
        let basis = format!(
            "canonical-stream-position|protocol={}|member={}|ordinal-position={}",
            stream_protocol_identity.as_str(),
            member.stream_member_identity(),
            ordinal_position,
        );
        let digest = digest_string("canonical-stream-position", &basis);
        Self {
            stream_position_identity: StreamPositionIdentity::new(digest.clone()),
            stream_protocol_identity,
            canonical_stream_member_identity: Arc::from(member.stream_member_identity()),
            ordinal_position,
            digest,
        }
    }

    pub fn stream_position_identity(&self) -> &str {
        self.stream_position_identity.as_str()
    }

    pub fn stream_protocol_identity(&self) -> &StreamProtocolIdentity {
        &self.stream_protocol_identity
    }

    pub fn canonical_stream_member_identity(&self) -> &str {
        self.canonical_stream_member_identity.as_ref()
    }

    pub fn ordinal_position(&self) -> usize {
        self.ordinal_position
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
