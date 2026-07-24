#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySourceOutputCorrespondence {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
    Partial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTransformationDisposition {
    Preserved,
    Normalized,
    Approximated,
    Repaired,
    Omitted,
    Unsupported,
    Quarantined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTransformationErrorPosture {
    Exact,
    Bounded,
    Estimated,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTransformationLossPosture {
    Lossless,
    DeclaredLossy,
    LossClassifiedByDomain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryImmutableSourceOccurrenceContract {
    identity_family: String,
}

impl WorthQueryImmutableSourceOccurrenceContract {
    pub fn new(identity_family: impl Into<String>) -> Self {
        Self {
            identity_family: identity_family.into(),
        }
    }

    pub fn identity_family(&self) -> &str {
        &self.identity_family
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTransformationIdentity {
    family: String,
    version: u32,
}

impl WorthQueryTransformationIdentity {
    pub fn new(family: impl Into<String>, version: u32) -> Self {
        Self {
            family: family.into(),
            version,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub const fn version(&self) -> u32 {
        self.version
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryTransformationOutcomeContract {
    correspondence: WorthQuerySourceOutputCorrespondence,
    disposition: WorthQueryTransformationDisposition,
    error: WorthQueryTransformationErrorPosture,
    loss: WorthQueryTransformationLossPosture,
}

impl WorthQueryTransformationOutcomeContract {
    pub const fn new(
        correspondence: WorthQuerySourceOutputCorrespondence,
        disposition: WorthQueryTransformationDisposition,
        error: WorthQueryTransformationErrorPosture,
        loss: WorthQueryTransformationLossPosture,
    ) -> Self {
        Self {
            correspondence,
            disposition,
            error,
            loss,
        }
    }

    pub const fn correspondence(self) -> WorthQuerySourceOutputCorrespondence {
        self.correspondence
    }

    pub const fn disposition(self) -> WorthQueryTransformationDisposition {
        self.disposition
    }

    pub const fn error(self) -> WorthQueryTransformationErrorPosture {
        self.error
    }

    pub const fn loss(self) -> WorthQueryTransformationLossPosture {
        self.loss
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryTransformationEvidenceContract {
    NotTransformation,
    Declared {
        source_occurrence: WorthQueryImmutableSourceOccurrenceContract,
        transformation: WorthQueryTransformationIdentity,
        outcome: WorthQueryTransformationOutcomeContract,
    },
}

impl WorthQueryTransformationEvidenceContract {
    pub const fn not_a_transformation() -> Self {
        Self::NotTransformation
    }

    pub fn declared(
        source_occurrence: WorthQueryImmutableSourceOccurrenceContract,
        transformation: WorthQueryTransformationIdentity,
        outcome: WorthQueryTransformationOutcomeContract,
    ) -> Self {
        Self::Declared {
            source_occurrence,
            transformation,
            outcome,
        }
    }
}
