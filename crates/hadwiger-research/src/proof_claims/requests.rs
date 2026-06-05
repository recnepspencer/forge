use crate::aspect_authority::{ColorabilityAspectRecord, UnitDistanceAspectRecord};
use crate::domain_artifacts::{
    ColorabilityVerification, GraphVersion, HadwigerCanonicalArtifact, ProofClaim,
    RetainedBackgroundTheorem, UnitDistanceVerification,
};
use crate::mathematical_verification::WholePlaneColoringVerification;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneLowerBoundClaimRequest {
    claim_id: String,
    graph_version: GraphVersion,
    unit_distance_verification: Option<UnitDistanceVerification>,
    unit_distance_aspect: Option<UnitDistanceAspectRecord>,
    not_k_colorable_verification: Option<ColorabilityVerification>,
    not_k_colorable_aspect: Option<ColorabilityAspectRecord>,
}

impl PlaneLowerBoundClaimRequest {
    pub fn new(claim_id: impl Into<String>, graph_version: &GraphVersion) -> Self {
        Self {
            claim_id: claim_id.into(),
            graph_version: graph_version.clone(),
            unit_distance_verification: None,
            unit_distance_aspect: None,
            not_k_colorable_verification: None,
            not_k_colorable_aspect: None,
        }
    }

    pub fn with_unit_distance_verification(
        mut self,
        verification: &UnitDistanceVerification,
    ) -> Self {
        self.unit_distance_verification = Some(verification.clone());
        self
    }

    pub fn with_unit_distance_aspect(mut self, aspect: &UnitDistanceAspectRecord) -> Self {
        self.unit_distance_aspect = Some(aspect.clone());
        self
    }

    pub fn with_not_k_colorable_verification(
        mut self,
        verification: &ColorabilityVerification,
    ) -> Self {
        self.not_k_colorable_verification = Some(verification.clone());
        self
    }

    pub fn with_not_k_colorable_aspect(mut self, aspect: &ColorabilityAspectRecord) -> Self {
        self.not_k_colorable_aspect = Some(aspect.clone());
        self
    }

    pub(crate) fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub(crate) fn graph_version(&self) -> &GraphVersion {
        &self.graph_version
    }

    pub(crate) fn unit_distance_verification(&self) -> Option<&UnitDistanceVerification> {
        self.unit_distance_verification.as_ref()
    }

    pub(crate) fn unit_distance_aspect(&self) -> Option<&UnitDistanceAspectRecord> {
        self.unit_distance_aspect.as_ref()
    }

    pub(crate) fn not_k_colorable_verification(&self) -> Option<&ColorabilityVerification> {
        self.not_k_colorable_verification.as_ref()
    }

    pub(crate) fn not_k_colorable_aspect(&self) -> Option<&ColorabilityAspectRecord> {
        self.not_k_colorable_aspect.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneUpperBoundClaimRequest {
    claim_id: String,
    checked_upper_bound: WholePlaneColoringVerification,
}

impl PlaneUpperBoundClaimRequest {
    pub fn from_checked_upper_bound(
        claim_id: impl Into<String>,
        verification: &WholePlaneColoringVerification,
    ) -> Self {
        Self {
            claim_id: claim_id.into(),
            checked_upper_bound: verification.clone(),
        }
    }

    pub(crate) fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub(crate) fn checked_upper_bound(&self) -> &WholePlaneColoringVerification {
        &self.checked_upper_bound
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneExactValueClaimRequest {
    claim_id: String,
    lower_bound_claim: ProofClaim,
    checked_upper_bound: Option<WholePlaneColoringVerification>,
    background_upper_bound: Option<RetainedBackgroundTheorem>,
}

impl PlaneExactValueClaimRequest {
    pub fn from_checked_upper_bound(
        claim_id: impl Into<String>,
        lower_bound_claim: &ProofClaim,
        checked_upper_bound: &WholePlaneColoringVerification,
    ) -> Self {
        Self {
            claim_id: claim_id.into(),
            lower_bound_claim: lower_bound_claim.clone(),
            checked_upper_bound: Some(checked_upper_bound.clone()),
            background_upper_bound: None,
        }
    }

    pub fn from_background_upper_bound(
        claim_id: impl Into<String>,
        lower_bound_claim: &ProofClaim,
        background_upper_bound: &RetainedBackgroundTheorem,
    ) -> Self {
        Self {
            claim_id: claim_id.into(),
            lower_bound_claim: lower_bound_claim.clone(),
            checked_upper_bound: None,
            background_upper_bound: Some(background_upper_bound.clone()),
        }
    }

    pub fn with_background_upper_bound(
        mut self,
        background_upper_bound: &RetainedBackgroundTheorem,
    ) -> Self {
        self.background_upper_bound = Some(background_upper_bound.clone());
        self
    }

    pub(crate) fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub(crate) fn lower_bound_claim(&self) -> &ProofClaim {
        &self.lower_bound_claim
    }

    pub(crate) fn checked_upper_bound(&self) -> Option<&WholePlaneColoringVerification> {
        self.checked_upper_bound.as_ref()
    }

    pub(crate) fn background_upper_bound(&self) -> Option<&RetainedBackgroundTheorem> {
        self.background_upper_bound.as_ref()
    }

    pub(crate) fn upper_bound_source_token(&self) -> String {
        if let Some(checked) = self.checked_upper_bound() {
            return checked.reference().stable_token();
        }
        self.background_upper_bound()
            .map(|theorem| theorem.reference().stable_token())
            .unwrap_or_else(|| "missing-upper-bound".to_string())
    }
}
