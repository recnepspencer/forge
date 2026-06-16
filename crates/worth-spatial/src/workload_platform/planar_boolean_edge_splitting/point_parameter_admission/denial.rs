#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitPointAdmissionDenialKind {
    MissingExactEndpointIdentity,
    NonFiniteParameter,
    OutOfDomainParameter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitPointAdmissionDenial {
    kind: PlanarBooleanSplitPointAdmissionDenialKind,
    evidence_identity: String,
    human_reason: String,
    rejected_non_finite_points: usize,
    rejected_out_of_domain_points: usize,
    rejected_missing_endpoint_identity_points: usize,
}

impl PlanarBooleanSplitPointAdmissionDenial {
    pub(crate) fn non_finite_parameter(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitPointAdmissionDenialKind::NonFiniteParameter,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_points: 1,
            rejected_out_of_domain_points: 0,
            rejected_missing_endpoint_identity_points: 0,
        }
    }

    pub(crate) fn out_of_domain_parameter(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitPointAdmissionDenialKind::OutOfDomainParameter,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_points: 0,
            rejected_out_of_domain_points: 1,
            rejected_missing_endpoint_identity_points: 0,
        }
    }

    pub(crate) fn missing_exact_endpoint_identity(
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarBooleanSplitPointAdmissionDenialKind::MissingExactEndpointIdentity,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_non_finite_points: 0,
            rejected_out_of_domain_points: 0,
            rejected_missing_endpoint_identity_points: 1,
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitPointAdmissionDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn rejected_non_finite_points(&self) -> usize {
        self.rejected_non_finite_points
    }

    pub fn rejected_out_of_domain_points(&self) -> usize {
        self.rejected_out_of_domain_points
    }

    pub fn rejected_missing_endpoint_identity_points(&self) -> usize {
        self.rejected_missing_endpoint_identity_points
    }
}
