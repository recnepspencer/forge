use std::collections::BTreeMap;

use forge_foundational::facade::FoundationalBoundaryEvidenceSupportTruthKind;
use forge_server::{ForgeServerDenialBoundary, ForgeServerOperatorEvidenceRecord};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCertificationBundle {
    request_context_digest: String,
    response_digest: String,
    provenance_digest: String,
    failure_digest: Option<String>,
    counter_snapshot: BTreeMap<String, u64>,
    support_truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
    support_attachment_present: bool,
}

impl ForgeServerCertificationBundle {
    pub fn from_response_and_evidence(
        request_context_digest: String,
        response: forge_server::ForgeServerResponseEnvelope,
        evidence: ForgeServerOperatorEvidenceRecord,
    ) -> Self {
        Self {
            request_context_digest,
            response_digest: response.canonical_digest().to_string(),
            provenance_digest: provenance_digest(response.provenance()),
            failure_digest: response.denial().map(failure_digest),
            counter_snapshot: counter_snapshot(&evidence),
            support_truth_kind: evidence.support_truth_kind(),
            support_attachment_present: evidence
                .materialized_attachment_bundle()
                .support()
                .is_some(),
        }
    }

    pub fn request_context_digest(&self) -> &str {
        &self.request_context_digest
    }

    pub fn response_digest(&self) -> &str {
        &self.response_digest
    }

    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }

    pub fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub fn support_truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        self.support_truth_kind
    }

    pub fn counter_snapshot(&self) -> &BTreeMap<String, u64> {
        &self.counter_snapshot
    }

    pub fn support_attachment_present(&self) -> bool {
        self.support_attachment_present
    }

    pub fn counter_value(&self, name: &str) -> Option<u64> {
        self.counter_snapshot.get(name).copied()
    }
}

fn counter_snapshot(evidence: &ForgeServerOperatorEvidenceRecord) -> BTreeMap<String, u64> {
    evidence
        .counter_receipt()
        .receipt()
        .counter_rows()
        .iter()
        .map(|row| (row.name().as_str().to_string(), row.observed_count()))
        .collect()
}

fn provenance_digest(
    provenance: &forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
) -> String {
    format!(
        "locality={:?};freshness={:?};source={:?};authority={};strategy={};profile={};comparison={};canonical={};support_contexts={}",
        provenance.locality(),
        provenance.freshness_posture(),
        provenance.source_basis().kind(),
        provenance.authority_path().is_some(),
        provenance.strategy_basis().is_some(),
        provenance.profile_basis().is_some(),
        provenance.comparison_basis().is_some(),
        provenance.canonical_digest_basis().is_some(),
        provenance.support_context_attachments().len(),
    )
}

fn failure_digest(denial: &forge_server::ForgeServerDenialEnvelope) -> String {
    match denial.cause().boundary() {
        ForgeServerDenialBoundary::RequestContext => format!(
            "request_context:{:?}:{}",
            denial
                .request_context_code()
                .expect("request-context denial code"),
            denial.cause().detail()
        ),
        ForgeServerDenialBoundary::Middleware => format!(
            "middleware:{:?}:{:?}:{:?}:{}",
            denial.middleware_code().expect("middleware denial code"),
            denial
                .middleware_priority()
                .expect("middleware denial priority"),
            denial.middleware_step().expect("middleware denial step"),
            denial.cause().detail()
        ),
        ForgeServerDenialBoundary::QueryHandoff => format!(
            "query_handoff:{:?}:{}",
            denial
                .query_handoff_code()
                .expect("query handoff denial code"),
            denial.cause().detail()
        ),
    }
}
