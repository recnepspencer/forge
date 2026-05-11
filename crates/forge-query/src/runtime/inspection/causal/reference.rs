use crate::identity::hash_parts;

use super::anchor::{CausalObservationAnchor, CausalObservationAnchorDigest};
use super::inventory::{CausalEvidenceFamily, CausalEvidenceOwner};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceDigest {
    digest: String,
}

impl CausalEvidenceReferenceDigest {
    pub(super) fn new(digest: String) -> Self {
        Self { digest }
    }

    pub fn as_str(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReference {
    owner: CausalEvidenceOwner,
    family: CausalEvidenceFamily,
    reference_digest: CausalEvidenceReferenceDigest,
}

impl CausalEvidenceReference {
    pub(super) fn new(
        owner: CausalEvidenceOwner,
        family: CausalEvidenceFamily,
        reference_digest: &str,
    ) -> Self {
        Self {
            owner,
            family,
            reference_digest: CausalEvidenceReferenceDigest::new(reference_digest.to_string()),
        }
    }

    pub fn owner(&self) -> CausalEvidenceOwner {
        self.owner
    }

    pub fn family(&self) -> CausalEvidenceFamily {
        self.family
    }

    pub fn reference_digest(&self) -> &CausalEvidenceReferenceDigest {
        &self.reference_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceReceipt {
    receipt_digest: String,
    anchor_digest: CausalObservationAnchorDigest,
    reference_set_digest: CausalEvidenceReferenceDigest,
    resolved_reference_count: usize,
    missing_reference_family_count: usize,
}

impl CausalEvidenceReferenceReceipt {
    pub(super) fn new(
        anchor_digest: CausalObservationAnchorDigest,
        reference_set_digest: CausalEvidenceReferenceDigest,
        resolved_reference_count: usize,
        missing_reference_family_count: usize,
    ) -> Self {
        let receipt_digest = hash_parts(&[
            "causal_evidence_reference_receipt_v1".to_string(),
            format!("anchor:{}", anchor_digest.as_str()),
            format!("reference-set:{}", reference_set_digest.as_str()),
            format!("resolved:{resolved_reference_count}"),
            format!("missing:{missing_reference_family_count}"),
        ]);
        Self {
            receipt_digest,
            anchor_digest,
            reference_set_digest,
            resolved_reference_count,
            missing_reference_family_count,
        }
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn anchor_digest(&self) -> &CausalObservationAnchorDigest {
        &self.anchor_digest
    }

    pub fn reference_set_digest(&self) -> &CausalEvidenceReferenceDigest {
        &self.reference_set_digest
    }

    pub fn resolved_reference_count(&self) -> usize {
        self.resolved_reference_count
    }

    pub fn missing_reference_family_count(&self) -> usize {
        self.missing_reference_family_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceSet {
    anchor: CausalObservationAnchor,
    references: Vec<CausalEvidenceReference>,
    reference_set_digest: CausalEvidenceReferenceDigest,
    receipt: CausalEvidenceReferenceReceipt,
}

impl CausalEvidenceReferenceSet {
    pub(super) fn new(
        anchor: CausalObservationAnchor,
        references: Vec<CausalEvidenceReference>,
        reference_set_digest: CausalEvidenceReferenceDigest,
        receipt: CausalEvidenceReferenceReceipt,
    ) -> Self {
        Self {
            anchor,
            references,
            reference_set_digest,
            receipt,
        }
    }

    pub fn anchor(&self) -> &CausalObservationAnchor {
        &self.anchor
    }

    pub fn references(&self) -> &[CausalEvidenceReference] {
        &self.references
    }

    pub fn reference_set_digest(&self) -> &CausalEvidenceReferenceDigest {
        &self.reference_set_digest
    }

    pub fn receipt(&self) -> &CausalEvidenceReferenceReceipt {
        &self.receipt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceResolutionCounters {
    requested_family_count: usize,
    anchor_reference_width: usize,
    indexed_record_count: usize,
    index_lookup_count: usize,
    resolved_reference_count: usize,
    missing_required_reference_count: usize,
    bridge_record_scan_fallback_count: usize,
    retained_record_scan_count: usize,
    runtime_graph_scan_count: usize,
    counter_snapshot: String,
}

impl CausalEvidenceReferenceResolutionCounters {
    pub(super) fn new(
        requested_family_count: usize,
        anchor_reference_width: usize,
        consulted_indexed_record_count: usize,
        index_lookup_count: usize,
        resolved_reference_count: usize,
        missing_required_reference_count: usize,
    ) -> Self {
        let counter_snapshot = hash_parts(&[
            "causal_evidence_reference_resolution_counters_v1".to_string(),
            format!("requested_family_count:{requested_family_count}"),
            format!("anchor_reference_width:{anchor_reference_width}"),
            format!("indexed_record_count:{consulted_indexed_record_count}"),
            format!("index_lookup_count:{index_lookup_count}"),
            format!("resolved_reference_count:{resolved_reference_count}"),
            format!("missing_required_reference_count:{missing_required_reference_count}"),
            "bridge_record_scan_fallback_count:0".to_string(),
            "retained_record_scan_count:0".to_string(),
            "runtime_graph_scan_count:0".to_string(),
        ]);
        Self {
            requested_family_count,
            anchor_reference_width,
            indexed_record_count: consulted_indexed_record_count,
            index_lookup_count,
            resolved_reference_count,
            missing_required_reference_count,
            bridge_record_scan_fallback_count: 0,
            retained_record_scan_count: 0,
            runtime_graph_scan_count: 0,
            counter_snapshot,
        }
    }

    pub fn requested_family_count(&self) -> usize {
        self.requested_family_count
    }

    pub fn anchor_reference_width(&self) -> usize {
        self.anchor_reference_width
    }

    pub fn indexed_record_count(&self) -> usize {
        self.indexed_record_count
    }

    pub fn index_lookup_count(&self) -> usize {
        self.index_lookup_count
    }

    pub fn resolved_reference_count(&self) -> usize {
        self.resolved_reference_count
    }

    pub fn missing_required_reference_count(&self) -> usize {
        self.missing_required_reference_count
    }

    pub fn bridge_record_scan_fallback_count(&self) -> usize {
        self.bridge_record_scan_fallback_count
    }

    pub fn retained_record_scan_count(&self) -> usize {
        self.retained_record_scan_count
    }

    pub fn runtime_graph_scan_count(&self) -> usize {
        self.runtime_graph_scan_count
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalEvidenceReferenceResolutionDenial {
    anchor_digest: CausalObservationAnchorDigest,
    missing_families: Vec<CausalEvidenceFamily>,
    missing_indexed_reference_count: usize,
    failure_digest: String,
}

impl CausalEvidenceReferenceResolutionDenial {
    pub(super) fn new(
        anchor_digest: CausalObservationAnchorDigest,
        missing_families: Vec<CausalEvidenceFamily>,
        missing_indexed_reference_count: usize,
    ) -> Self {
        let missing_part = missing_families
            .iter()
            .map(CausalEvidenceFamily::as_str)
            .collect::<Vec<_>>()
            .join("|");
        let failure_digest = hash_parts(&[
            "causal_evidence_reference_resolution_denial_v1".to_string(),
            format!("anchor:{}", anchor_digest.as_str()),
            format!("missing:{missing_part}"),
            format!("missing-indexed-reference:{missing_indexed_reference_count}"),
        ]);
        Self {
            anchor_digest,
            missing_families,
            missing_indexed_reference_count,
            failure_digest,
        }
    }

    pub fn anchor_digest(&self) -> &CausalObservationAnchorDigest {
        &self.anchor_digest
    }

    pub fn missing_families(&self) -> &[CausalEvidenceFamily] {
        &self.missing_families
    }

    pub fn missing_indexed_reference_count(&self) -> usize {
        self.missing_indexed_reference_count
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CausalEvidenceReferenceResolution {
    Resolved {
        reference_set: CausalEvidenceReferenceSet,
        counters: CausalEvidenceReferenceResolutionCounters,
    },
    MissingRequiredEvidence {
        denial: CausalEvidenceReferenceResolutionDenial,
        counters: CausalEvidenceReferenceResolutionCounters,
    },
}

impl CausalEvidenceReferenceResolution {
    pub fn counters(&self) -> &CausalEvidenceReferenceResolutionCounters {
        match self {
            Self::Resolved { counters, .. } | Self::MissingRequiredEvidence { counters, .. } => {
                counters
            }
        }
    }

    pub fn resolved_reference_set(&self) -> Option<&CausalEvidenceReferenceSet> {
        match self {
            Self::Resolved { reference_set, .. } => Some(reference_set),
            Self::MissingRequiredEvidence { .. } => None,
        }
    }

    pub fn denial(&self) -> Option<&CausalEvidenceReferenceResolutionDenial> {
        match self {
            Self::MissingRequiredEvidence { denial, .. } => Some(denial),
            Self::Resolved { .. } => None,
        }
    }
}
