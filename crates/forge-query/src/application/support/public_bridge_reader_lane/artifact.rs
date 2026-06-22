use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryPublicBridgeProjectionConsumptionEvidence, ForgeQueryPublicBridgeReaderLaneInventory,
    ForgeQueryPublicBridgeReaderLanePosture, ForgeQueryPublicBridgeReaderLaneSabotage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublicBridgeReaderLaneCertification {
    posture: ForgeQueryPublicBridgeReaderLanePosture,
    projection_reads: Vec<ForgeQueryPublicBridgeProjectionConsumptionEvidence>,
    projection_receipt_digests: Vec<String>,
    published_artifact_digests: Vec<String>,
    inventory: ForgeQueryPublicBridgeReaderLaneInventory,
    direct_materialization_read_count: usize,
    sabotage: ForgeQueryPublicBridgeReaderLaneSabotage,
    digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPublicBridgeReaderLaneCertification {
    pub fn certify(
        projection_reads: Vec<ForgeQueryPublicBridgeProjectionConsumptionEvidence>,
        published_artifact_digests: Vec<String>,
        inventory: ForgeQueryPublicBridgeReaderLaneInventory,
        sabotage: ForgeQueryPublicBridgeReaderLaneSabotage,
    ) -> Self {
        let direct_materialization_read_count = inventory.direct_materialization_read_count();
        let has_projection_receipts = !projection_reads.is_empty()
            && projection_reads.iter().all(|read| {
                !read.receipt_digest().is_empty() && !read.fact_set_digest().is_empty()
            });
        let has_published_artifacts = !published_artifact_digests.is_empty()
            && published_artifact_digests
                .iter()
                .all(|digest| !digest.is_empty());
        let projection_receipt_digests = projection_reads
            .iter()
            .map(|read| read.receipt_digest().to_string())
            .collect::<Vec<_>>();
        let posture = if has_projection_receipts
            && has_published_artifacts
            && direct_materialization_read_count == 0
            && sabotage.rejected()
        {
            ForgeQueryPublicBridgeReaderLanePosture::Closed
        } else {
            ForgeQueryPublicBridgeReaderLanePosture::Open
        };
        let digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("projection_receipt_digest"),
            projection_reads
                .iter()
                .map(ForgeQueryPublicBridgeProjectionConsumptionEvidence::receipt_digest),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("projection_read_digest"),
            projection_reads
                .iter()
                .map(|read| read.digest().terminal_projection_for_reporting()),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("published_artifact_digest"),
            published_artifact_digests.iter().map(String::as_str),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("inventory_digest"),
            inventory.digest().terminal_projection_for_reporting(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("direct_materialization_read_count"),
            direct_materialization_read_count,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("sabotage_kind"),
            sabotage.kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("sabotage_pattern"),
            sabotage.localized_pattern(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("sabotage_rejected"),
            if sabotage.rejected() { "true" } else { "false" },
        )
        .seal();
        Self {
            posture,
            projection_reads,
            projection_receipt_digests,
            published_artifact_digests,
            inventory,
            direct_materialization_read_count,
            sabotage,
            digest,
        }
    }

    pub fn posture(&self) -> ForgeQueryPublicBridgeReaderLanePosture {
        self.posture
    }

    pub fn projection_receipt_digests(&self) -> &[String] {
        &self.projection_receipt_digests
    }

    pub fn projection_reads(&self) -> &[ForgeQueryPublicBridgeProjectionConsumptionEvidence] {
        &self.projection_reads
    }

    pub fn published_artifact_digests(&self) -> &[String] {
        &self.published_artifact_digests
    }

    pub fn direct_materialization_read_count(&self) -> usize {
        self.direct_materialization_read_count
    }

    pub fn inventory(&self) -> &ForgeQueryPublicBridgeReaderLaneInventory {
        &self.inventory
    }

    pub fn sabotage(&self) -> &ForgeQueryPublicBridgeReaderLaneSabotage {
        &self.sabotage
    }

    pub fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.digest
    }
}
