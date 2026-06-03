use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackCausalityIdentityTag};

pub type BridgeWritebackCausalityIdentity = BridgeIdentity<WritebackCausalityIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackCausalityEvidence {
    truth_trigger_basis: Arc<str>,
    route_basis: Arc<str>,
    evaluation_basis: Arc<str>,
    truth_view_basis: Arc<str>,
}

impl BridgeWritebackCausalityEvidence {
    pub fn from_native_bases(
        truth_trigger_basis: impl Into<Arc<str>>,
        route_basis: impl Into<Arc<str>>,
        evaluation_basis: impl Into<Arc<str>>,
        truth_view_basis: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            truth_trigger_basis: truth_trigger_basis.into(),
            route_basis: route_basis.into(),
            evaluation_basis: evaluation_basis.into(),
            truth_view_basis: truth_view_basis.into(),
        }
    }

    pub fn truth_trigger_basis(&self) -> &str {
        self.truth_trigger_basis.as_ref()
    }

    pub fn route_basis(&self) -> &str {
        self.route_basis.as_ref()
    }

    pub fn evaluation_basis(&self) -> &str {
        self.evaluation_basis.as_ref()
    }

    pub fn truth_view_basis(&self) -> &str {
        self.truth_view_basis.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackCausalityBasis {
    causality_identity: BridgeWritebackCausalityIdentity,
    truth_trigger_digest: Arc<str>,
    route_digest: Arc<str>,
    evaluation_surface_digest: Arc<str>,
    truth_view_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackCausalityBasis {
    pub fn from_evidence(
        causality_identity: BridgeWritebackCausalityIdentity,
        evidence: BridgeWritebackCausalityEvidence,
    ) -> Self {
        let truth_trigger_digest = digest_truth_trigger_evidence(evidence.truth_trigger_basis());
        let route_digest = digest_route_evidence(evidence.route_basis());
        let evaluation_surface_digest =
            digest_evaluation_surface_evidence(evidence.evaluation_basis());
        let truth_view_digest = digest_truth_view_evidence(evidence.truth_view_basis());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-causality|id={}|truth-trigger={}|route={}|evaluation={}|truth-view={}",
            causality_identity.as_str(),
            truth_trigger_digest.as_ref(),
            route_digest.as_ref(),
            evaluation_surface_digest.as_ref(),
            truth_view_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            causality_identity,
            truth_trigger_digest,
            route_digest,
            evaluation_surface_digest,
            truth_view_digest,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-causality:sha256:{digest:x}")),
        }
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn truth_trigger_digest(&self) -> &str {
        self.truth_trigger_digest.as_ref()
    }

    pub fn route_digest(&self) -> &str {
        self.route_digest.as_ref()
    }

    pub fn evaluation_surface_digest(&self) -> &str {
        self.evaluation_surface_digest.as_ref()
    }

    pub fn truth_view_digest(&self) -> &str {
        self.truth_view_digest.as_ref()
    }
}

fn digest_truth_trigger_evidence(basis: &str) -> Arc<str> {
    digest_native_causality_basis(CausalityEvidenceDigestDomain::TruthTrigger, basis)
}

fn digest_route_evidence(basis: &str) -> Arc<str> {
    digest_native_causality_basis(CausalityEvidenceDigestDomain::Route, basis)
}

fn digest_evaluation_surface_evidence(basis: &str) -> Arc<str> {
    digest_native_causality_basis(CausalityEvidenceDigestDomain::EvaluationSurface, basis)
}

fn digest_truth_view_evidence(basis: &str) -> Arc<str> {
    digest_native_causality_basis(CausalityEvidenceDigestDomain::TruthView, basis)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CausalityEvidenceDigestDomain {
    TruthTrigger,
    Route,
    EvaluationSurface,
    TruthView,
}

impl CausalityEvidenceDigestDomain {
    const fn prefix(self) -> &'static str {
        match self {
            Self::TruthTrigger => "bridge-writeback-truth-trigger",
            Self::Route => "bridge-writeback-route",
            Self::EvaluationSurface => "bridge-writeback-evaluation",
            Self::TruthView => "bridge-writeback-truth-view",
        }
    }
}

fn digest_native_causality_basis(domain: CausalityEvidenceDigestDomain, basis: &str) -> Arc<str> {
    let prefix = domain.prefix();
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    hasher.update(b"\0");
    hasher.update(basis.as_bytes());
    Arc::from(format!("{prefix}:sha256:{:x}", hasher.finalize()))
}
