use crate::identity::hash_parts;
use crate::identity_evolution::{InspectorIdentityClassification, InspectorIdentityDigest};

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
