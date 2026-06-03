use std::collections::BTreeMap;

use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::entities::TopologyEntityKind;

use super::mutation_records::TopologyDeclaredMutationRecord;
use super::{
    NamingMutationContinuityMatrix, TopologyDeclaredMutationActionRef,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest, TopologyMutationFamily,
    TopologyMutationNamingOutcome, TopologyMutationNamingReport, TopologyMutationNamingRow,
    TopologyMutationSequenceDigest,
};

#[derive(Clone)]
pub(crate) struct TopologyDeclaredMutationSequence {
    records: Vec<TopologyDeclaredMutationRecord>,
    created_entity_kinds: BTreeMap<String, TopologyEntityKind>,
    families: Vec<TopologyMutationFamily>,
    topology_mutation_digest: TopologyMutationDigest,
    naming_continuity_matrix: NamingMutationContinuityMatrix,
}

#[derive(Clone, Copy)]
pub(crate) struct TopologyDeclaredMutationMember<'a> {
    record: &'a TopologyDeclaredMutationRecord,
}

impl TopologyDeclaredMutationSequence {
    pub(crate) fn new(records: Vec<TopologyDeclaredMutationRecord>) -> Self {
        let naming_report = topology_mutation_naming_report_for_records(&records);
        let families = topology_mutation_families_for_records(&records);
        Self {
            created_entity_kinds: created_entity_kinds_for_records(&records),
            topology_mutation_digest: topology_mutation_digest_for_records(&records),
            naming_continuity_matrix: naming_mutation_continuity_matrix_from_rows(
                naming_report.rows.clone(),
            ),
            families,
            records,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_iter(
        records: impl IntoIterator<Item = TopologyDeclaredMutationRecord>,
    ) -> Self {
        Self::new(records.into_iter().collect())
    }

    #[cfg(test)]
    pub(crate) fn concatenate(
        sequences: impl IntoIterator<Item = TopologyDeclaredMutationSequence>,
    ) -> Self {
        Self::from_iter(
            sequences
                .into_iter()
                .flat_map(|sequence| sequence.records.into_iter()),
        )
    }

    pub(crate) fn members(&self) -> impl Iterator<Item = TopologyDeclaredMutationMember<'_>> {
        self.records
            .iter()
            .map(|record| TopologyDeclaredMutationMember { record })
    }

    pub(crate) fn created_entity_kinds(&self) -> &BTreeMap<String, TopologyEntityKind> {
        &self.created_entity_kinds
    }

    pub(crate) fn families(&self) -> &[TopologyMutationFamily] {
        &self.families
    }

    pub(crate) fn topology_mutation_digest(&self) -> &TopologyMutationDigest {
        &self.topology_mutation_digest
    }

    pub(crate) fn naming_continuity_matrix(&self) -> &NamingMutationContinuityMatrix {
        &self.naming_continuity_matrix
    }

    #[cfg(test)]
    pub(crate) fn naming_report(&self) -> TopologyMutationNamingReport {
        TopologyMutationNamingReport {
            rows: self.naming_continuity_matrix.rows.clone(),
        }
    }

    pub(crate) fn strictest_fallback_policy(&self) -> TopologyMutationDerivedFallbackPolicy {
        if self.members().any(|member| {
            member.record().derived_fallback_policy()
                == TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
        }) {
            TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
        } else {
            TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback
        }
    }
}

impl<'a> TopologyDeclaredMutationMember<'a> {
    pub(crate) fn record(&self) -> &'a TopologyDeclaredMutationRecord {
        self.record
    }

    pub(crate) fn action_ref(&self) -> TopologyDeclaredMutationActionRef<'a> {
        self.record.action_ref()
    }

    pub(crate) fn touched_aspects(&self) -> &'a std::collections::BTreeSet<Aspect> {
        self.record.touched_aspects()
    }

    pub(crate) fn lowered_mutations(
        &self,
    ) -> &'a [schema::facade::platform::authority::TopologyMutation] {
        self.record.lowered_mutations()
    }
}

pub(crate) fn topology_mutation_naming_report_for_records(
    records: &[TopologyDeclaredMutationRecord],
) -> TopologyMutationNamingReport {
    let rows = records
        .iter()
        .flat_map(|record| record.naming_report().rows)
        .collect();
    TopologyMutationNamingReport { rows }
}

pub(crate) fn topology_mutation_families_for_records(
    records: &[TopologyDeclaredMutationRecord],
) -> Vec<TopologyMutationFamily> {
    records.iter().map(|record| record.family).collect()
}

pub(crate) fn topology_mutation_digest_for_records(
    records: &[TopologyDeclaredMutationRecord],
) -> TopologyMutationDigest {
    let rows = records.iter().map(record_digest_row);
    let changed_scope_count = records
        .iter()
        .map(|record| record.changed_scopes().len())
        .sum();
    let naming_scope_count = records
        .iter()
        .map(|record| record.naming_scopes().len())
        .sum();
    let derived_region_count = records
        .iter()
        .map(|record| record.derived_regions().len())
        .sum();
    let fallback_policy_count = records.len();
    let fallback_rejection_policy_count = records
        .iter()
        .filter(|record| {
            record.derived_fallback_policy()
                == super::TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
        })
        .count();
    TopologyMutationDigest {
        digest: digest_rows(rows),
        mutation_record_count: records.len(),
        family_count: records.len(),
        changed_scope_count,
        naming_scope_count,
        derived_region_count,
        fallback_policy_count,
        fallback_rejection_policy_count,
    }
}

pub(crate) fn naming_mutation_continuity_matrix_from_rows(
    rows: Vec<TopologyMutationNamingRow>,
) -> NamingMutationContinuityMatrix {
    let preserved_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyMutationNamingOutcome::Preserved)
        .count();
    let ambiguous_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyMutationNamingOutcome::Ambiguous)
        .count();
    let rejected_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyMutationNamingOutcome::Rejected)
        .count();
    NamingMutationContinuityMatrix {
        rows,
        preserved_count,
        ambiguous_count,
        rejected_count,
    }
}

fn record_digest_row(record: &TopologyDeclaredMutationRecord) -> String {
    serde_json::to_string(record).expect(" topology mutation records should serialize")
}

fn created_entity_kinds_for_records(
    records: &[TopologyDeclaredMutationRecord],
) -> BTreeMap<String, TopologyEntityKind> {
    records
        .iter()
        .filter_map(|record| match record.action_ref() {
            TopologyDeclaredMutationActionRef::CreateTopologyEntity { create_key, kind } => {
                Some((create_key.to_string(), kind))
            }
            _ => None,
        })
        .collect()
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> TopologyMutationSequenceDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    TopologyMutationSequenceDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
