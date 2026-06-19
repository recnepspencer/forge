use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitPersistentNameRow, PlanarBooleanSplitPersistentNamingReceipt,
    PlanarBooleanSplitSubshapeSignatureRow,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanSourceLoopSplitAttribution,
};

use super::counters::PlanarBooleanLoopIdentityMintingCounters;
use super::denial::{
    PlanarBooleanLoopIdentityMintingDenial, PlanarBooleanLoopIdentityMintingDenialKind,
};
use super::identity::naming_support_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopNamingAuthoritySupport {
    support_identity: String,
    request_identity: String,
    split_ledger_receipt_identity: String,
    split_persistent_naming_receipt_identity: String,
    persistent_name_rows: Vec<PlanarBooleanSplitPersistentNameRow>,
    subshape_signature_rows: Vec<PlanarBooleanSplitSubshapeSignatureRow>,
    source_edges_by_source_loop: BTreeMap<String, BTreeSet<String>>,
}

impl PlanarBooleanLoopNamingAuthoritySupport {
    pub fn admit_from_split_receipt_and_provenance(
        split_persistent_naming: &PlanarBooleanSplitPersistentNamingReceipt,
        source_provenance: &PlanarBooleanLoopSourceProvenanceBundle,
        split_attribution: &PlanarBooleanSourceLoopSplitAttribution,
    ) -> Result<Self, PlanarBooleanLoopIdentityMintingDenial> {
        let counters = PlanarBooleanLoopIdentityMintingCounters::default();
        if source_provenance.request_identity() != split_attribution.request_identity() {
            return Err(PlanarBooleanLoopIdentityMintingDenial::new(
                PlanarBooleanLoopIdentityMintingDenialKind::RequestIdentityMismatch,
                split_attribution.request_identity().to_string(),
                counters,
                "loop naming authority support requires provenance and split attribution to share one request identity",
            ));
        }
        let source_edges_by_source_loop =
            source_provenance.source_loop_carriers().rows().iter().fold(
                BTreeMap::<String, BTreeSet<String>>::new(),
                |mut acc, row| {
                    acc.entry(row.source_loop_identity().to_string())
                        .or_default()
                        .insert(row.source_edge_identity().to_string());
                    acc
                },
            );
        Ok(Self {
            support_identity: naming_support_identity(
                source_provenance.request_identity(),
                source_provenance.split_ledger_receipt_identity(),
                split_persistent_naming.receipt_identity(),
            ),
            request_identity: source_provenance.request_identity().to_string(),
            split_ledger_receipt_identity: source_provenance
                .split_ledger_receipt_identity()
                .to_string(),
            split_persistent_naming_receipt_identity: split_persistent_naming
                .receipt_identity()
                .to_string(),
            persistent_name_rows: split_persistent_naming.persistent_name_rows().to_vec(),
            subshape_signature_rows: split_persistent_naming.subshape_signature_rows().to_vec(),
            source_edges_by_source_loop,
        })
    }

    pub fn support_identity(&self) -> &str {
        &self.support_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }

    pub fn split_persistent_naming_receipt_identity(&self) -> &str {
        &self.split_persistent_naming_receipt_identity
    }

    pub fn persistent_name_rows(&self) -> &[PlanarBooleanSplitPersistentNameRow] {
        &self.persistent_name_rows
    }

    pub fn subshape_signature_rows(&self) -> &[PlanarBooleanSplitSubshapeSignatureRow] {
        &self.subshape_signature_rows
    }

    pub(crate) fn source_edges_for_source_loop(
        &self,
        source_loop_identity: &str,
    ) -> Option<&BTreeSet<String>> {
        self.source_edges_by_source_loop.get(source_loop_identity)
    }
}
