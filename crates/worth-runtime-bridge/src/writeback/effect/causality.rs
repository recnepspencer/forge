use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, BridgeIdentityEvidence, WritebackCausalityIdentityTag};
use crate::input::envelope::TruthCommitIdentity;
use crate::routing::BridgeRouteIdentity;
use crate::snapshot::TruthSnapshotIdentity;
use crate::writeback::{BridgeMutationSubject, BridgeWritebackEffectIntent};

pub type BridgeWritebackCausalityIdentity = BridgeIdentity<WritebackCausalityIdentityTag>;

impl BridgeWritebackCausalityIdentity {
    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self::admit_bridge_owned(format!(
            "bridge-writeback-causality:external-authority-evidence:{}",
            evidence_identity.as_str()
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackNativeCausalityInputs {
    causality_identity: BridgeWritebackCausalityIdentity,
    truth_trigger_identity: TruthCommitIdentity,
    route_identity: BridgeRouteIdentity,
    evaluation_snapshot_identity: TruthSnapshotIdentity,
    truth_view_snapshot_identity: TruthSnapshotIdentity,
    mutation_subject: Option<BridgeMutationSubject>,
    causality_digest: Arc<str>,
    truth_trigger_digest: Arc<str>,
    route_digest: Arc<str>,
    evaluation_surface_digest: Arc<str>,
    truth_view_digest: Arc<str>,
}

impl BridgeWritebackNativeCausalityInputs {
    pub fn new(
        causality_identity: BridgeWritebackCausalityIdentity,
        truth_trigger_identity: TruthCommitIdentity,
        route_identity: BridgeRouteIdentity,
        evaluation_snapshot_identity: TruthSnapshotIdentity,
        truth_view_snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        let basis = BridgeWritebackCausalityBasis::from_evidence(
            causality_identity.clone(),
            BridgeWritebackCausalityEvidence::from_native_bases(
                truth_trigger_identity.as_str(),
                route_identity.as_str(),
                evaluation_snapshot_identity.as_str(),
                truth_view_snapshot_identity.as_str(),
            ),
        );
        Self {
            causality_identity,
            truth_trigger_identity,
            route_identity,
            evaluation_snapshot_identity,
            truth_view_snapshot_identity,
            mutation_subject: None,
            causality_digest: Arc::from(basis.digest().to_owned()),
            truth_trigger_digest: Arc::from(basis.truth_trigger_digest().to_owned()),
            route_digest: Arc::from(basis.route_digest().to_owned()),
            evaluation_surface_digest: Arc::from(basis.evaluation_surface_digest().to_owned()),
            truth_view_digest: Arc::from(basis.truth_view_digest().to_owned()),
        }
    }

    pub fn causality_identity(&self) -> &BridgeWritebackCausalityIdentity {
        &self.causality_identity
    }

    pub fn digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn truth_trigger_digest(&self) -> &str {
        self.truth_trigger_digest.as_ref()
    }

    pub(crate) fn truth_trigger_identity(&self) -> &TruthCommitIdentity {
        &self.truth_trigger_identity
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

    pub(crate) fn truth_view_snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.truth_view_snapshot_identity
    }

    pub fn bind_mutation_subject(mut self, subject: BridgeMutationSubject) -> Self {
        let basis = BridgeWritebackCausalityBasis::from_evidence(
            self.causality_identity.clone(),
            BridgeWritebackCausalityEvidence::from_native_bases(
                self.truth_trigger_identity.as_str(),
                self.route_identity.as_str(),
                self.evaluation_snapshot_identity.as_str(),
                self.truth_view_snapshot_identity.as_str(),
            )
            .with_mutation_subject_digest(subject.digest()),
        );
        self.causality_digest = Arc::from(basis.digest().to_owned());
        self.mutation_subject = Some(subject);
        self
    }

    pub(crate) fn mutation_subject(&self) -> Option<&BridgeMutationSubject> {
        self.mutation_subject.as_ref()
    }

    pub(crate) fn mutation_subject_matches_effect_intent(
        &self,
        effect_intent: &BridgeWritebackEffectIntent,
    ) -> bool {
        self.mutation_subject
            .as_ref()
            .is_none_or(|subject| subject.matches_effect_intent(effect_intent))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BridgeWritebackCausalityEvidence {
    truth_trigger_basis: Arc<str>,
    route_basis: Arc<str>,
    evaluation_basis: Arc<str>,
    truth_view_basis: Arc<str>,
    mutation_subject_digest: Option<Arc<str>>,
}

impl BridgeWritebackCausalityEvidence {
    pub(crate) fn from_native_bases(
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
            mutation_subject_digest: None,
        }
    }

    pub(crate) fn with_mutation_subject_digest(mut self, digest: &str) -> Self {
        self.mutation_subject_digest = Some(Arc::from(digest.to_owned()));
        self
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

    pub(crate) fn mutation_subject_digest(&self) -> Option<&str> {
        self.mutation_subject_digest.as_deref()
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
    pub(crate) fn from_evidence(
        causality_identity: BridgeWritebackCausalityIdentity,
        evidence: BridgeWritebackCausalityEvidence,
    ) -> Self {
        let truth_trigger_digest = digest_truth_trigger_evidence(evidence.truth_trigger_basis());
        let route_digest = digest_route_evidence(evidence.route_basis());
        let evaluation_surface_digest =
            digest_evaluation_surface_evidence(evidence.evaluation_basis());
        let truth_view_digest = digest_truth_view_evidence(evidence.truth_view_basis());
        let mut canonical_basis = format!(
            "bridge-writeback-causality|id={}|truth-trigger={}|route={}|evaluation={}|truth-view={}",
            causality_identity.as_str(),
            truth_trigger_digest.as_ref(),
            route_digest.as_ref(),
            evaluation_surface_digest.as_ref(),
            truth_view_digest.as_ref(),
        );
        if let Some(subject_digest) = evidence.mutation_subject_digest() {
            canonical_basis.push_str("|mutation-subject=");
            canonical_basis.push_str(subject_digest);
        }
        let canonical_basis = Arc::<str>::from(canonical_basis);
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
