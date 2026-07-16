use worth_store_authority::StoreCurrentAuthorityWitness;

use super::bundle::BlobExportPublishedBundle;
use super::canonical::prepare_export_artifact;
use super::chunk_bytes::BlobExportedChunkBytes;
use super::counters::BlobExportBundleCounters;
use super::custody_receipt::BlobExportCustodyEvidence;
use super::denial::BlobExportBundleDenial;
use super::evidence_bundle::BlobExportDigestEvidence;
use super::intent::BlobExportIntent;
use super::manifest::BlobExportManifest;
use super::transition_verification::verify_export_transition;
use crate::{BlobChunkByteWindow, BlobChunkProofLeaf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

impl BlobExportAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub fn collect_exported_chunk_bytes<'a>(
        &self,
        leaf: &BlobChunkProofLeaf,
        bytes: BlobChunkByteWindow<'a>,
    ) -> Result<BlobExportedChunkBytes<'a>, BlobExportBundleDenial> {
        let _physical = self.current_authority.current_physical_authority();
        BlobExportedChunkBytes::collect_from_leaf(leaf, bytes)
    }

    pub fn publish_export_bundle(
        &self,
        intent: BlobExportIntent<'_>,
    ) -> Result<BlobExportPublishedBundle, BlobExportBundleDenial> {
        let _physical = self.current_authority.current_physical_authority();
        let verified = verify_export_transition(&intent)?;
        let classification = verified.classification();

        let manifest = BlobExportManifest::new(
            intent.export_name().to_owned(),
            classification.manifest_rows().to_vec(),
        );
        let (canonical_export, export_digest) =
            prepare_export_artifact(intent.export_name(), intent.publication().canonical_basis())?;
        let offline_declarations = classification.offline_declarations().to_vec();
        let counters = BlobExportBundleCounters::start().with_evidence(classification.counts());
        Ok(BlobExportPublishedBundle::new(
            super::BlobExportPublishedBundleParts {
                object_id: intent.lifecycle().declaration().object_id().clone(),
                generation: intent.lifecycle().declaration().generation(),
                chunk_tree_root: intent.publication().chunk_tree_root().clone(),
                security_metadata: intent.lifecycle().declaration().security_metadata(),
                manifest,
                custody: BlobExportCustodyEvidence::new(
                    intent.custody().identity(),
                    intent.custody().purpose(),
                ),
                digest_evidence: BlobExportDigestEvidence::new(
                    intent
                        .lifecycle()
                        .declaration()
                        .logical_content_digest()
                        .clone(),
                    export_digest,
                    &offline_declarations,
                ),
                offline_declarations,
                canonical_export,
                counters,
            },
        ))
    }
}
