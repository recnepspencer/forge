use std::collections::BTreeSet;

use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceAuthorityKind, UiEvidenceFamily,
    UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
};

use crate::declaration::stable_text_digest;
use crate::evidence::{
    evidence_authority_binding, evidence_handle, evidence_identity, evidence_ref,
    UiEvidenceIdentity, UiEvidenceRef,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAspectEvidenceLane {
    Published,
    Consumed,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAspectEvidenceSubjectKind {
    GraphNode,
    MountedReceipt,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAspectEvidenceRefProjection {
    lane: UiAspectEvidenceLane,
    subject_kind: UiAspectEvidenceSubjectKind,
    subject_digest: u64,
    evidence_identity_digest: u64,
}

impl UiAspectEvidenceRefProjection {
    pub const fn new(
        lane: UiAspectEvidenceLane,
        subject_kind: UiAspectEvidenceSubjectKind,
        subject_digest: u64,
        evidence_identity_digest: u64,
    ) -> Self {
        Self {
            lane,
            subject_kind,
            subject_digest,
            evidence_identity_digest,
        }
    }

    pub const fn lane(self) -> UiAspectEvidenceLane {
        self.lane
    }

    pub const fn subject_kind(self) -> UiAspectEvidenceSubjectKind {
        self.subject_kind
    }

    pub const fn subject_digest(self) -> u64 {
        self.subject_digest
    }

    pub const fn evidence_identity_digest(self) -> u64 {
        self.evidence_identity_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiAspectEvidenceRecordKind {
    PublishedGraphNode(u64),
    PublishedMountedReceipt(u64),
    ConsumedGraphNode(u64),
    ConsumedMountedReceipt(u64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiAspectEvidenceRecord {
    identity: UiEvidenceIdentity,
    authority_generation: UiEvidenceAuthorityGeneration,
}

impl UiAspectEvidenceRecord {
    pub(crate) fn new(
        canonical_label: &str,
        kind: UiAspectEvidenceRecordKind,
        authority_generation: UiEvidenceAuthorityGeneration,
    ) -> Self {
        let identity = match kind {
            UiAspectEvidenceRecordKind::PublishedGraphNode(graph_node_digest) => {
                aspect_identity("aspect:published:graph-node", canonical_label, graph_node_digest)
            }
            UiAspectEvidenceRecordKind::PublishedMountedReceipt(receipt_digest) => aspect_identity(
                "aspect:published:mounted-receipt",
                canonical_label,
                receipt_digest,
            ),
            UiAspectEvidenceRecordKind::ConsumedGraphNode(graph_node_digest) => {
                aspect_identity("aspect:consumed:graph-node", canonical_label, graph_node_digest)
            }
            UiAspectEvidenceRecordKind::ConsumedMountedReceipt(receipt_digest) => aspect_identity(
                "aspect:consumed:mounted-receipt",
                canonical_label,
                receipt_digest,
            ),
        };

        Self {
            identity,
            authority_generation,
        }
    }

    pub(crate) fn reference(self) -> UiEvidenceRef {
        evidence_ref(
            UiEvidenceFamily::Aspect,
            self.identity,
            evidence_authority_binding(
                UiEvidenceAuthorityKind::AspectAuthority,
                self.identity.digest(),
                self.authority_generation,
                None,
            ),
            UiEvidenceMaterializationPosture::RefsOnly,
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            evidence_handle(UiEvidenceFamily::Aspect, self.identity, self.identity.digest()),
        )
    }
}

pub fn project_aspect_evidence_ref(
    evidence_ref: UiEvidenceRef,
    canonical_label: &str,
    graph_node_digests: &[u64],
    mounted_receipt_digests: &[u64],
) -> Option<UiAspectEvidenceRefProjection> {
    if evidence_ref.family() != UiEvidenceFamily::Aspect {
        return None;
    }

    for graph_node_digest in graph_node_digests {
        for (kind, lane) in [
            (
                UiAspectEvidenceRecordKind::PublishedGraphNode(*graph_node_digest),
                UiAspectEvidenceLane::Published,
            ),
            (
                UiAspectEvidenceRecordKind::ConsumedGraphNode(*graph_node_digest),
                UiAspectEvidenceLane::Consumed,
            ),
        ] {
            if evidence_ref.identity()
                == identity_for_kind(canonical_label, kind)
            {
                return Some(UiAspectEvidenceRefProjection::new(
                    lane,
                    UiAspectEvidenceSubjectKind::GraphNode,
                    *graph_node_digest,
                    evidence_ref.identity().digest(),
                ));
            }
        }
    }

    for mounted_receipt_digest in mounted_receipt_digests {
        for (kind, lane) in [
            (
                UiAspectEvidenceRecordKind::PublishedMountedReceipt(*mounted_receipt_digest),
                UiAspectEvidenceLane::Published,
            ),
            (
                UiAspectEvidenceRecordKind::ConsumedMountedReceipt(*mounted_receipt_digest),
                UiAspectEvidenceLane::Consumed,
            ),
        ] {
            if evidence_ref.identity()
                == identity_for_kind(canonical_label, kind)
            {
                return Some(UiAspectEvidenceRefProjection::new(
                    lane,
                    UiAspectEvidenceSubjectKind::MountedReceipt,
                    *mounted_receipt_digest,
                    evidence_ref.identity().digest(),
                ));
            }
        }
    }

    None
}

pub fn project_aspect_evidence_refs(
    refs: &[UiEvidenceRef],
    canonical_label: &str,
    graph_node_digests: &[u64],
    mounted_receipt_digests: &[u64],
) -> BTreeSet<UiAspectEvidenceRefProjection> {
    refs.iter()
        .filter_map(|evidence_ref| {
            project_aspect_evidence_ref(
                *evidence_ref,
                canonical_label,
                graph_node_digests,
                mounted_receipt_digests,
            )
        })
        .collect()
}

fn identity_for_kind(
    canonical_label: &str,
    kind: UiAspectEvidenceRecordKind,
) -> UiEvidenceIdentity {
    match kind {
        UiAspectEvidenceRecordKind::PublishedGraphNode(graph_node_digest) => {
            aspect_identity("aspect:published:graph-node", canonical_label, graph_node_digest)
        }
        UiAspectEvidenceRecordKind::PublishedMountedReceipt(receipt_digest) => {
            aspect_identity("aspect:published:mounted-receipt", canonical_label, receipt_digest)
        }
        UiAspectEvidenceRecordKind::ConsumedGraphNode(graph_node_digest) => {
            aspect_identity("aspect:consumed:graph-node", canonical_label, graph_node_digest)
        }
        UiAspectEvidenceRecordKind::ConsumedMountedReceipt(receipt_digest) => {
            aspect_identity("aspect:consumed:mounted-receipt", canonical_label, receipt_digest)
        }
    }
}

fn aspect_identity(prefix: &str, canonical_label: &str, lane_digest: u64) -> UiEvidenceIdentity {
    evidence_identity(
        UiEvidenceFamily::Aspect,
        stable_text_digest(&format!("{prefix}:{canonical_label}:{lane_digest}")),
    )
}
