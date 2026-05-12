use std::collections::BTreeMap;

use crate::identity::hash_parts;

use super::inventory::{CausalEvidenceFamily, CausalEvidenceOwner};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceIndex {
    records: BTreeMap<CausalEvidenceFamily, BTreeMap<String, CausalEvidenceReferenceIndexRecord>>,
    index_digest: String,
}

impl CausalEvidenceReferenceIndex {
    pub(in crate::runtime) fn new(records: Vec<CausalEvidenceReferenceIndexRecord>) -> Self {
        let mut indexed_records = BTreeMap::<
            CausalEvidenceFamily,
            BTreeMap<String, CausalEvidenceReferenceIndexRecord>,
        >::new();
        for record in records {
            indexed_records
                .entry(record.family())
                .or_default()
                .insert(record.reference_digest().to_string(), record);
        }
        let index_digest = index_digest(&indexed_records);
        Self {
            records: indexed_records,
            index_digest,
        }
    }

    pub(super) fn record_for_reference(
        &self,
        family: CausalEvidenceFamily,
        reference_digest: &str,
    ) -> Option<&CausalEvidenceReferenceIndexRecord> {
        self.records
            .get(&family)
            .and_then(|family_records| family_records.get(reference_digest))
    }

    pub fn record_count(&self) -> usize {
        self.records.values().map(BTreeMap::len).sum()
    }

    pub fn family_count(&self) -> usize {
        self.records.len()
    }

    pub fn index_digest(&self) -> &str {
        &self.index_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceIndexRecord {
    owner: CausalEvidenceOwner,
    family: CausalEvidenceFamily,
    reference_digest: String,
    record_digest: String,
}

impl CausalEvidenceReferenceIndexRecord {
    pub(in crate::runtime) fn new(
        owner: CausalEvidenceOwner,
        family: CausalEvidenceFamily,
        reference_digest: String,
    ) -> Self {
        let record_digest = hash_parts(&[
            "causal_evidence_reference_index_record_v1".to_string(),
            format!("owner:{}", owner.as_str()),
            format!("family:{}", family.as_str()),
            format!("reference:{reference_digest}"),
        ]);
        Self {
            owner,
            family,
            reference_digest,
            record_digest,
        }
    }

    pub fn owner(&self) -> CausalEvidenceOwner {
        self.owner
    }

    pub fn family(&self) -> CausalEvidenceFamily {
        self.family
    }

    pub fn reference_digest(&self) -> &str {
        &self.reference_digest
    }

    pub fn record_digest(&self) -> &str {
        &self.record_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalEvidenceReferenceIndexErrorKind {
    EmptyReferenceDigest,
    EvidenceOwnerMismatch,
}

impl CausalEvidenceReferenceIndexErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyReferenceDigest => "empty_reference_digest",
            Self::EvidenceOwnerMismatch => "evidence_owner_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceIndexError {
    kind: CausalEvidenceReferenceIndexErrorKind,
    family: CausalEvidenceFamily,
    supplied_owner: CausalEvidenceOwner,
    expected_owner: CausalEvidenceOwner,
    failure_digest: String,
}

impl CausalEvidenceReferenceIndexError {
    fn new(
        kind: CausalEvidenceReferenceIndexErrorKind,
        family: CausalEvidenceFamily,
        supplied_owner: CausalEvidenceOwner,
        expected_owner: CausalEvidenceOwner,
    ) -> Self {
        let failure_digest = hash_parts(&[
            "causal_evidence_reference_index_error_v1".to_string(),
            kind.as_str().to_string(),
            format!("family:{}", family.as_str()),
            format!("supplied-owner:{}", supplied_owner.as_str()),
            format!("expected-owner:{}", expected_owner.as_str()),
        ]);
        Self {
            kind,
            family,
            supplied_owner,
            expected_owner,
            failure_digest,
        }
    }

    pub fn kind(&self) -> CausalEvidenceReferenceIndexErrorKind {
        self.kind
    }

    pub fn family(&self) -> CausalEvidenceFamily {
        self.family
    }

    pub fn supplied_owner(&self) -> CausalEvidenceOwner {
        self.supplied_owner
    }

    pub fn expected_owner(&self) -> CausalEvidenceOwner {
        self.expected_owner
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

pub(in crate::runtime) fn causal_evidence_reference_index(
    records: impl IntoIterator<Item = CausalEvidenceReferenceIndexRecord>,
) -> CausalEvidenceReferenceIndex {
    CausalEvidenceReferenceIndex::new(records.into_iter().collect())
}

pub(in crate::runtime) fn causal_evidence_reference_index_record(
    owner: CausalEvidenceOwner,
    family: CausalEvidenceFamily,
    reference_digest: impl Into<String>,
) -> Result<CausalEvidenceReferenceIndexRecord, CausalEvidenceReferenceIndexError> {
    let expected_owner = owner_for_family(family);
    if owner != expected_owner {
        return Err(CausalEvidenceReferenceIndexError::new(
            CausalEvidenceReferenceIndexErrorKind::EvidenceOwnerMismatch,
            family,
            owner,
            expected_owner,
        ));
    }
    let reference_digest = reference_digest.into();
    if reference_digest.is_empty() {
        return Err(CausalEvidenceReferenceIndexError::new(
            CausalEvidenceReferenceIndexErrorKind::EmptyReferenceDigest,
            family,
            owner,
            expected_owner,
        ));
    }
    Ok(CausalEvidenceReferenceIndexRecord::new(
        owner,
        family,
        reference_digest,
    ))
}

pub(super) fn owner_for_family(family: CausalEvidenceFamily) -> CausalEvidenceOwner {
    match family {
        CausalEvidenceFamily::QueryInspection
        | CausalEvidenceFamily::QueryMutationCausality
        | CausalEvidenceFamily::QueryMutationProvenance
        | CausalEvidenceFamily::Policy
        | CausalEvidenceFamily::Redaction
        | CausalEvidenceFamily::Provenance => CausalEvidenceOwner::Query,
        CausalEvidenceFamily::RelationalAuthority | CausalEvidenceFamily::RelationalDecision => {
            CausalEvidenceOwner::Relational
        }
        CausalEvidenceFamily::SignalInvalidation
        | CausalEvidenceFamily::SignalEvaluation
        | CausalEvidenceFamily::SignalForensicAvailability
        | CausalEvidenceFamily::SignalReplayCursor
        | CausalEvidenceFamily::SignalLineage
        | CausalEvidenceFamily::SignalProvenance
        | CausalEvidenceFamily::Lineage => CausalEvidenceOwner::Signal,
        CausalEvidenceFamily::BridgeRoute
        | CausalEvidenceFamily::BridgeEvaluation
        | CausalEvidenceFamily::BridgeSourceMaterialization
        | CausalEvidenceFamily::BridgeSourceFailure
        | CausalEvidenceFamily::BridgeContinuity
        | CausalEvidenceFamily::BridgeMerge
        | CausalEvidenceFamily::BridgeStructural
        | CausalEvidenceFamily::BridgeStream
        | CausalEvidenceFamily::BridgePreview
        | CausalEvidenceFamily::BridgeWriteback
        | CausalEvidenceFamily::BridgeMapper
        | CausalEvidenceFamily::BridgeReplay => CausalEvidenceOwner::RuntimeBridge,
    }
}

fn index_digest(
    records: &BTreeMap<CausalEvidenceFamily, BTreeMap<String, CausalEvidenceReferenceIndexRecord>>,
) -> String {
    let record_part = records
        .values()
        .flat_map(|family_records| family_records.values())
        .map(CausalEvidenceReferenceIndexRecord::record_digest)
        .collect::<Vec<_>>()
        .join("|");
    hash_parts(&[
        "causal_evidence_reference_index_v1".to_string(),
        format!("records:{record_part}"),
    ])
}
