use crate::identity::hash_parts;
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InspectorIdentityDigest(String);

impl InspectorIdentityDigest {
    pub fn from_parts(parts: &[String]) -> Self {
        Self(crate::identity::hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InspectorIdentityClassification {
    IdentitySummary,
    AuthoritativeContinuity,
    AdvisoryCandidates,
    Ambiguity,
    IdentityBreak,
    Denied,
}

impl InspectorIdentityClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IdentitySummary => "identity_summary",
            Self::AuthoritativeContinuity => "authoritative_continuity",
            Self::AdvisoryCandidates => "advisory_candidates",
            Self::Ambiguity => "ambiguity",
            Self::IdentityBreak => "identity_break",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeIdentityConsumption {
    None,
    InspectorIdentitySummary,
    FocusedInspectorIdentityClassification(InspectorIdentityClassification),
}

impl ViewShapeIdentityConsumption {
    pub fn none() -> Self {
        Self::None
    }

    pub fn inspector_identity_summary() -> Self {
        Self::InspectorIdentitySummary
    }

    pub fn focused_inspector_identity_classification(
        classification: InspectorIdentityClassification,
    ) -> Self {
        Self::FocusedInspectorIdentityClassification(classification)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::InspectorIdentitySummary => "inspector_identity_summary",
            Self::FocusedInspectorIdentityClassification(_) => {
                "focused_inspector_identity_classification"
            }
        }
    }

    pub fn classification(&self) -> Option<InspectorIdentityClassification> {
        match self {
            Self::FocusedInspectorIdentityClassification(classification) => Some(*classification),
            Self::None | Self::InspectorIdentitySummary => None,
        }
    }

    pub fn digest(&self) -> InspectorIdentityDigest {
        let mut parts = vec![format!("consumption:{}", self.as_str())];
        if let Some(classification) = self.classification() {
            parts.push(format!("classification:{}", classification.as_str()));
        }
        InspectorIdentityDigest::from_parts(&parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeIdentityBinding {
    digest: String,
    identity_consumption: ViewShapeIdentityConsumption,
}

impl ViewShapeIdentityBinding {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn identity_consumption(&self) -> &ViewShapeIdentityConsumption {
        &self.identity_consumption
    }

    pub(crate) fn new(identity_consumption: ViewShapeIdentityConsumption) -> Self {
        let digest = hash_parts(&[format!(
            "identity_consumption_digest:{}",
            identity_consumption.digest().as_str()
        )]);
        Self {
            digest,
            identity_consumption,
        }
    }
}
