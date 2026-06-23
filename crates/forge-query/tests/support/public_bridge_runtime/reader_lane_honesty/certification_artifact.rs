use forge_query::{
    facade::ForgeQueryEvidenceIdentity, ForgeQueryPublicBridgeReaderLaneCertification,
    ForgeQueryPublicBridgeReaderLanePosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicBridgeReaderLaneHonestyArtifact {
    legacy_digest: ForgeQueryEvidenceIdentity,
    reader_lane: ForgeQueryPublicBridgeReaderLaneCertification,
}

impl PublicBridgeReaderLaneHonestyArtifact {
    pub fn new(
        legacy_digest: ForgeQueryEvidenceIdentity,
        reader_lane: ForgeQueryPublicBridgeReaderLaneCertification,
    ) -> Self {
        Self {
            legacy_digest,
            reader_lane,
        }
    }

    #[allow(dead_code)]
    pub fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.legacy_digest
    }

    #[allow(dead_code)]
    pub fn reader_lane(&self) -> &ForgeQueryPublicBridgeReaderLaneCertification {
        &self.reader_lane
    }

    #[allow(dead_code)]
    pub fn posture(&self) -> ForgeQueryPublicBridgeReaderLanePosture {
        self.reader_lane.posture()
    }
}
