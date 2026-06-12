use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::packet_artifacts::TilingIterationPacket;
use super::packet_counters::TilingIterationCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingIterationReplayReport {
    core: HadwigerArtifactCore,
    packet_digest: String,
    counters: TilingIterationCounters,
}

impl TilingIterationReplayReport {
    pub(crate) fn checked(
        packet: &TilingIterationPacket,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let packet_digest = packet.packet_digest().to_string();
        let counters = packet.counters().clone();
        let core = artifact_core(
            HadwigerArtifactKind::TilingIterationReplayReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "tiling_iteration_packet_replay".to_string(),
            },
            vec![packet.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text("packet_digest", &packet_digest),
                HadwigerArtifactPayloadEntry::unsigned(
                    "action_count",
                    packet.actions().len() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "query_readiness_rows",
                    counters.query_readiness_rows() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "blocked_actions",
                    counters.blocked_actions() as u128,
                ),
            ],
        )?;
        Ok(Self {
            core,
            packet_digest,
            counters,
        })
    }

    pub fn packet_digest(&self) -> &str {
        &self.packet_digest
    }

    pub fn counters(&self) -> &TilingIterationCounters {
        &self.counters
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingIterationReplayReport, core);
