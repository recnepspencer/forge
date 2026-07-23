use worth_foundational::facade::RetentionDeliveryProfile;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactRedactionPosture {
    NotRequired,
    CanonicalProjectionOnly,
    DomainRedactorRequired,
    NeverDisclose,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactDeletionPosture {
    DeleteWithRun,
    DeleteAfterRetention,
    DomainControlled,
    ExternallyControlled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactLegalHoldPosture {
    NotEligible,
    DomainControlled,
    RequiredWhenDirected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactGovernanceContract {
    audiences: Vec<String>,
    classification: WorthQueryArtifactClassification,
    redaction: WorthQueryArtifactRedactionPosture,
    retention: RetentionDeliveryProfile,
    deletion: WorthQueryArtifactDeletionPosture,
    legal_hold: WorthQueryArtifactLegalHoldPosture,
}

impl WorthQueryArtifactGovernanceContract {
    pub fn new(
        audiences: impl IntoIterator<Item = impl Into<String>>,
        classification: WorthQueryArtifactClassification,
        redaction: WorthQueryArtifactRedactionPosture,
        retention: RetentionDeliveryProfile,
        deletion: WorthQueryArtifactDeletionPosture,
        legal_hold: WorthQueryArtifactLegalHoldPosture,
    ) -> Self {
        let mut contract = Self {
            audiences: audiences.into_iter().map(Into::into).collect(),
            classification,
            redaction,
            retention,
            deletion,
            legal_hold,
        };
        contract.canonicalize();
        contract
    }

    pub fn audiences(&self) -> &[String] {
        &self.audiences
    }

    pub const fn classification(&self) -> WorthQueryArtifactClassification {
        self.classification
    }

    pub const fn redaction(&self) -> WorthQueryArtifactRedactionPosture {
        self.redaction
    }

    pub const fn retention(&self) -> RetentionDeliveryProfile {
        self.retention
    }

    pub const fn deletion(&self) -> WorthQueryArtifactDeletionPosture {
        self.deletion
    }

    pub const fn legal_hold(&self) -> WorthQueryArtifactLegalHoldPosture {
        self.legal_hold
    }

    pub(crate) fn canonicalize(&mut self) {
        self.audiences.sort();
        self.audiences.dedup();
    }
}
