use std::collections::HashSet;
use std::sync::Arc;

use super::super::authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
use super::super::counters::BridgeCausalEnvelopeCounters;
use super::super::denial::{BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind};
use super::super::digest;
use super::super::evidence_reference::BridgeCausalEvidenceReference;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeCausalInspectionAdmissionSummaryKind {
    Admitted,
    Advisory,
}

impl BridgeCausalInspectionAdmissionSummaryKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalInspectionAdmissionSummary {
    kind: BridgeCausalInspectionAdmissionSummaryKind,
    query_admission_digest: Arc<str>,
    causal_observation_anchor_digest: Arc<str>,
    summary_digest: Arc<str>,
}

impl BridgeCausalInspectionAdmissionSummary {
    pub fn admitted(
        query_admission_digest: impl Into<Arc<str>>,
        causal_observation_anchor_digest: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        Self::new(
            BridgeCausalInspectionAdmissionSummaryKind::Admitted,
            query_admission_digest,
            causal_observation_anchor_digest,
        )
    }

    pub fn advisory(
        query_admission_digest: impl Into<Arc<str>>,
        causal_observation_anchor_digest: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        Self::new(
            BridgeCausalInspectionAdmissionSummaryKind::Advisory,
            query_admission_digest,
            causal_observation_anchor_digest,
        )
    }

    fn new(
        kind: BridgeCausalInspectionAdmissionSummaryKind,
        query_admission_digest: impl Into<Arc<str>>,
        causal_observation_anchor_digest: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        let query_admission_digest = query_admission_digest.into();
        let causal_observation_anchor_digest = causal_observation_anchor_digest.into();
        validate_admission_summary_inputs(
            query_admission_digest.as_ref(),
            causal_observation_anchor_digest.as_ref(),
        )?;
        let summary_digest = digest(
            "bridge-causal-inspection-admission-summary",
            &[
                kind.as_str(),
                query_admission_digest.as_ref(),
                causal_observation_anchor_digest.as_ref(),
            ],
        );
        Ok(Self {
            kind,
            query_admission_digest,
            causal_observation_anchor_digest,
            summary_digest: Arc::from(summary_digest),
        })
    }

    pub fn kind(&self) -> BridgeCausalInspectionAdmissionSummaryKind {
        self.kind
    }

    pub fn query_admission_digest(&self) -> &str {
        self.query_admission_digest.as_ref()
    }

    pub fn causal_observation_anchor_digest(&self) -> &str {
        self.causal_observation_anchor_digest.as_ref()
    }

    pub fn summary_digest(&self) -> &str {
        self.summary_digest.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEnvelopeAssemblyRequest {
    admission_summary: BridgeCausalInspectionAdmissionSummary,
    references: Arc<[BridgeCausalEvidenceReference]>,
    request_digest: Arc<str>,
}

impl BridgeCausalEnvelopeAssemblyRequest {
    pub fn from_query_admission(
        admission_summary: BridgeCausalInspectionAdmissionSummary,
        references: Vec<BridgeCausalEvidenceReference>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        validate_request_inputs(&references)?;
        let reference_part = references
            .iter()
            .map(BridgeCausalEvidenceReference::reference_digest)
            .collect::<Vec<_>>()
            .join("|");
        let request_digest = digest(
            "bridge-causal-envelope-assembly-request",
            &[admission_summary.summary_digest(), &reference_part],
        );
        Ok(Self {
            admission_summary,
            references: Arc::from(references),
            request_digest: Arc::from(request_digest),
        })
    }

    pub fn admission_summary(&self) -> &BridgeCausalInspectionAdmissionSummary {
        &self.admission_summary
    }

    pub fn query_admission_digest(&self) -> &str {
        self.admission_summary.query_admission_digest()
    }

    pub fn causal_observation_anchor_digest(&self) -> &str {
        self.admission_summary.causal_observation_anchor_digest()
    }

    pub fn references(&self) -> &[BridgeCausalEvidenceReference] {
        &self.references
    }

    pub fn request_digest(&self) -> &str {
        self.request_digest.as_ref()
    }
}

fn validate_admission_summary_inputs(
    query_admission_digest: &str,
    causal_observation_anchor_digest: &str,
) -> Result<(), BridgeCausalEnvelopeDenial> {
    if query_admission_digest.is_empty() || causal_observation_anchor_digest.is_empty() {
        return Err(BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::EmptyAssemblyRequestDigest,
            BridgeCausalEvidenceFamily::QueryObservation,
            BridgeCausalEvidenceOwner::Query,
            BridgeCausalEvidenceOwner::Query,
            Arc::from("empty-query-admission-or-anchor"),
            BridgeCausalEnvelopeCounters::empty(),
        ));
    }
    Ok(())
}

fn validate_request_inputs(
    references: &[BridgeCausalEvidenceReference],
) -> Result<(), BridgeCausalEnvelopeDenial> {
    if references.is_empty() {
        return Err(BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::MissingEvidenceReference,
            BridgeCausalEvidenceFamily::BridgeRoute,
            BridgeCausalEvidenceOwner::RuntimeBridge,
            BridgeCausalEvidenceOwner::RuntimeBridge,
            Arc::from("missing-evidence-reference"),
            BridgeCausalEnvelopeCounters::empty(),
        ));
    }
    validate_unique_references(references)?;
    validate_query_observation_anchor(references)?;
    Ok(())
}

fn validate_unique_references(
    references: &[BridgeCausalEvidenceReference],
) -> Result<(), BridgeCausalEnvelopeDenial> {
    let mut seen_references = HashSet::new();
    for reference in references {
        let reference_key = (
            reference.owner(),
            reference.family(),
            reference.reference_identity(),
        );
        if !seen_references.insert(reference_key) {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::DuplicateEvidenceReference,
                reference.family(),
                reference.owner(),
                reference.family().expected_owner(),
                Arc::from(reference.reference_identity()),
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
    }
    Ok(())
}

fn validate_query_observation_anchor(
    references: &[BridgeCausalEvidenceReference],
) -> Result<(), BridgeCausalEnvelopeDenial> {
    let query_observation_anchor_count = references
        .iter()
        .filter(|reference| {
            reference.owner() == BridgeCausalEvidenceOwner::Query
                && reference.family() == BridgeCausalEvidenceFamily::QueryObservation
        })
        .count();
    match query_observation_anchor_count {
        1 => Ok(()),
        0 => Err(BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::MissingQueryObservationAnchor,
            BridgeCausalEvidenceFamily::QueryObservation,
            BridgeCausalEvidenceOwner::Query,
            BridgeCausalEvidenceOwner::Query,
            Arc::from("missing-query-observation-anchor"),
            BridgeCausalEnvelopeCounters::empty(),
        )),
        count => Err(BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::QueryObservationAnchorOverclaim,
            BridgeCausalEvidenceFamily::QueryObservation,
            BridgeCausalEvidenceOwner::Query,
            BridgeCausalEvidenceOwner::Query,
            Arc::from(format!("query-observation-anchor-count:{count}")),
            BridgeCausalEnvelopeCounters::empty(),
        )),
    }
}
