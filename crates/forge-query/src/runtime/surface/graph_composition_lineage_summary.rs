use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationFamily,
    ForgeQueryContinuityOutcomeClass, ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionLineageEntry {
    component_index: usize,
    family: ForgeQueryContinuityMutationFamily,
    outcome_class: ForgeQueryContinuityOutcomeClass,
    prior_authoritative_identity: String,
    successor_authoritative_identities: Vec<String>,
    target_collection: Option<String>,
    lineage_digest: String,
    continuity_resolution_digest: String,
}

impl ForgeQueryGraphCompositionLineageEntry {
    fn new(
        component_index: usize,
        receipt: &ForgeQueryWriteReceipt,
        evidence: &ForgeQueryContinuityMutationEvidence,
    ) -> Self {
        Self {
            component_index,
            family: evidence.family(),
            outcome_class: evidence.outcome_class(),
            prior_authoritative_identity: evidence.prior_authoritative_identity().to_string(),
            successor_authoritative_identities: evidence
                .successor_authoritative_identities()
                .to_vec(),
            target_collection: receipt
                .target_collection()
                .or(receipt.declared_collection())
                .or(evidence.target_collection())
                .map(str::to_string),
            lineage_digest: evidence.lineage_digest().to_string(),
            continuity_resolution_digest: evidence.continuity_resolution_digest().to_string(),
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn family(&self) -> ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> ForgeQueryContinuityOutcomeClass {
        self.outcome_class
    }

    pub fn prior_authoritative_identity(&self) -> &str {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identities(&self) -> &[String] {
        &self.successor_authoritative_identities
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn lineage_digest(&self) -> &str {
        &self.lineage_digest
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        &self.continuity_resolution_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionLineageSummary {
    entries: Vec<ForgeQueryGraphCompositionLineageEntry>,
    counter_snapshot: String,
    aggregate_lineage_digest: String,
    aggregate_continuity_resolution_digest: String,
    lineage_summary_digest: String,
}

impl ForgeQueryGraphCompositionLineageSummary {
    pub(in crate::runtime) fn derive(write_receipts: &[ForgeQueryWriteReceipt]) -> Option<Self> {
        let entries = write_receipts
            .iter()
            .enumerate()
            .filter_map(|(component_index, receipt)| {
                receipt.continuity_mutation_evidence().map(|evidence| {
                    ForgeQueryGraphCompositionLineageEntry::new(component_index, receipt, evidence)
                })
            })
            .collect::<Vec<_>>();
        if entries.is_empty() {
            return None;
        }

        let single_successor_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            })
            .count();
        let split_successor_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
            })
            .count();
        let merge_successor_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
            })
            .count();
        let rejected_count =
            entries.len() - single_successor_count - split_successor_count - merge_successor_count;
        let counter_snapshot = format!(
            "continuity_entries={};single_successors={};split_successors={};merge_successors={};rejections={}",
            entries.len(),
            single_successor_count,
            split_successor_count,
            merge_successor_count,
            rejected_count,
        );
        let aggregate_lineage_digest = aggregate_digest(
            "forge_query_graph_composition_lineage_digest_v1",
            entries.iter().map(|entry| entry.lineage_digest()),
        );
        let aggregate_continuity_resolution_digest = aggregate_digest(
            "forge_query_graph_composition_lineage_resolution_digest_v1",
            entries
                .iter()
                .map(|entry| entry.continuity_resolution_digest()),
        );
        let lineage_summary_digest = hash_parts(&[
            "forge_query_graph_composition_lineage_summary_v1".to_string(),
            format!("lineage:{aggregate_lineage_digest}"),
            format!("resolution:{aggregate_continuity_resolution_digest}"),
            format!("counters:{counter_snapshot}"),
            format!(
                "entries:{}",
                aggregate_digest(
                    "forge_query_graph_composition_lineage_entry_digest_v1",
                    entries.iter().map(|entry| {
                        format!(
                            "{}:{:?}:{:?}:{}:{}:{}:{}",
                            entry.component_index(),
                            entry.family(),
                            entry.outcome_class(),
                            entry.target_collection().unwrap_or("none"),
                            entry.prior_authoritative_identity(),
                            entry.successor_authoritative_identities().join("|"),
                            entry.lineage_digest(),
                        )
                    })
                )
            ),
        ]);

        Some(Self {
            entries,
            counter_snapshot,
            aggregate_lineage_digest,
            aggregate_continuity_resolution_digest,
            lineage_summary_digest,
        })
    }

    pub fn entries(&self) -> &[ForgeQueryGraphCompositionLineageEntry] {
        &self.entries
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn aggregate_lineage_digest(&self) -> &str {
        &self.aggregate_lineage_digest
    }

    pub fn aggregate_continuity_resolution_digest(&self) -> &str {
        &self.aggregate_continuity_resolution_digest
    }

    pub fn lineage_summary_digest(&self) -> &str {
        &self.lineage_summary_digest
    }
}

fn aggregate_digest(label: &str, digests: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    hash_parts(
        &std::iter::once(label.to_string())
            .chain(
                digests
                    .into_iter()
                    .map(|digest| format!("digest:{}", digest.as_ref())),
            )
            .collect::<Vec<_>>(),
    )
}
