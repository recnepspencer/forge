use worth_query::facade::certification::{
    WorthQueryPublicBridgeReaderLaneCertification, WorthQueryPublicBridgeReaderLanePosture,
};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicBridgeReaderLaneHonestyArtifact {
    legacy_digest: WorthQueryEvidenceIdentity,
    reader_lane: WorthQueryPublicBridgeReaderLaneCertification,
}

impl PublicBridgeReaderLaneHonestyArtifact {
    pub fn new(
        legacy_digest: WorthQueryEvidenceIdentity,
        reader_lane: WorthQueryPublicBridgeReaderLaneCertification,
    ) -> Self {
        Self {
            legacy_digest,
            reader_lane,
        }
    }

    #[allow(dead_code)]
    pub fn digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.legacy_digest
    }

    #[allow(dead_code)]
    pub fn reader_lane(&self) -> &WorthQueryPublicBridgeReaderLaneCertification {
        &self.reader_lane
    }

    #[allow(dead_code)]
    pub fn posture(&self) -> WorthQueryPublicBridgeReaderLanePosture {
        self.reader_lane.posture()
    }
}
