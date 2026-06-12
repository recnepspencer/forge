use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationFamily,
    ForgeQueryContinuityOutcomeClass, ForgeQueryMutationAuthorityIdentity,
    ForgeQueryMutationEvidenceDigest, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionLineageEntry {
    component_index: usize,
    family: ForgeQueryContinuityMutationFamily,
    outcome_class: ForgeQueryContinuityOutcomeClass,
    prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    successor_authoritative_identities: Vec<ForgeQueryMutationAuthorityIdentity>,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    lineage_digest: ForgeQueryMutationEvidenceDigest,
    continuity_resolution_digest: ForgeQueryMutationEvidenceDigest,
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
            prior_authoritative_identity: evidence.prior_authoritative_identity().clone(),
            successor_authoritative_identities: evidence
                .successor_authoritative_identities()
                .to_vec(),
            target_collection: receipt
                .target_collection()
                .map(|collection| {
                    ForgeQueryMutationTargetCollectionIdentity::new(
                        "graph-lineage-receipt-target",
                        collection,
                    )
                })
                .or_else(|| {
                    receipt.declared_collection().map(|collection| {
                        ForgeQueryMutationTargetCollectionIdentity::new(
                            "graph-lineage-receipt-declared",
                            collection,
                        )
                    })
                })
                .or_else(|| evidence.target_collection().cloned()),
            lineage_digest: evidence.lineage_digest().clone(),
            continuity_resolution_digest: evidence.continuity_resolution_digest().clone(),
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

    pub fn prior_authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identities(&self) -> &[ForgeQueryMutationAuthorityIdentity] {
        &self.successor_authoritative_identities
    }

    pub fn target_collection(&self) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }

    pub fn lineage_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.lineage_digest
    }

    pub fn continuity_resolution_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.continuity_resolution_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionLineageSummary {
    entries: Vec<ForgeQueryGraphCompositionLineageEntry>,
    counter_snapshot: String,
    aggregate_lineage_digest: ForgeQueryEvidenceIdentity,
    aggregate_continuity_resolution_digest: ForgeQueryEvidenceIdentity,
    lineage_summary_digest: ForgeQueryEvidenceIdentity,
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
        let counter_snapshot = diagnostic_counter_snapshot(&[
            ("continuity_entries", entries.len()),
            ("single_successors", single_successor_count),
            ("split_successors", split_successor_count),
            ("merge_successors", merge_successor_count),
            ("rejections", rejected_count),
        ]);
        let aggregate_lineage_digest = aggregate_digest(
            "forge_query_graph_composition_lineage_digest_v1",
            entries
                .iter()
                .map(|entry| entry.lineage_digest().evidence_identity()),
        );
        let aggregate_continuity_resolution_digest = aggregate_digest(
            "forge_query_graph_composition_lineage_resolution_digest_v1",
            entries
                .iter()
                .map(|entry| entry.continuity_resolution_digest().evidence_identity()),
        );
        let entry_digests = entries
            .iter()
            .map(graph_composition_lineage_entry_digest)
            .collect::<Vec<_>>();
        let aggregate_entry_digest = aggregate_digest(
            "forge_query_graph_composition_lineage_entry_digest_v1",
            entry_digests.iter(),
        );
        let lineage_summary_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-lineage-summary",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("lineage"),
                    &aggregate_lineage_digest,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolution"),
                    &aggregate_continuity_resolution_digest,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("continuity_entry_count"),
                    entries.len(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("single_successor_count"),
                    single_successor_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("split_successor_count"),
                    split_successor_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("merge_successor_count"),
                    merge_successor_count,
                )
                .field_usize(ForgeQueryEvidenceTag::new("rejected_count"), rejected_count)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("entries"),
                    &aggregate_entry_digest,
                )
                .seal();

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
        self.aggregate_lineage_digest.as_str()
    }

    pub fn aggregate_continuity_resolution_digest(&self) -> &str {
        self.aggregate_continuity_resolution_digest.as_str()
    }

    pub fn lineage_summary_digest(&self) -> &str {
        self.lineage_summary_digest.as_str()
    }

    pub fn lineage_summary_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.lineage_summary_digest
    }
}

fn diagnostic_counter_snapshot(fields: &[(&str, usize)]) -> String {
    let mut snapshot = String::new();
    for (index, (label, value)) in fields.iter().enumerate() {
        if index > 0 {
            snapshot.push(';');
        }
        snapshot.push_str(label);
        snapshot.push('=');
        snapshot.push_str(&value.to_string());
    }
    snapshot
}

fn aggregate_digest<'a>(
    label: &str,
    digests: impl IntoIterator<Item = &'a ForgeQueryEvidenceIdentity>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(ForgeQueryEvidenceTag::new("role"), label)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("digest"), digests)
        .seal()
}

fn graph_composition_lineage_entry_digest(
    entry: &ForgeQueryGraphCompositionLineageEntry,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("role"),
                "graph-composition-lineage-entry",
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("component"),
                entry.component_index(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                entry.family().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("outcome"),
                entry.outcome_class().as_str(),
            );
    if let Some(collection) = entry.target_collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_collection"),
            collection.evidence_identity(),
        );
    }
    identity = identity
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("prior_authority"),
            &entry.prior_authoritative_identity().evidence_identity(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("successor_authority"),
            entry
                .successor_authoritative_identities()
                .iter()
                .map(|identity| identity.evidence_identity()),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("lineage"),
            &entry.lineage_digest().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_resolution"),
            &entry.continuity_resolution_digest().evidence_identity(),
        );
    identity.seal()
}
