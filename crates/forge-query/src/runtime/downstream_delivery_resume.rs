use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportPosture;

use super::evidence_identities::runtime_downstream_resume_posture_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeDownstreamResumePostureKind {
    RuntimeBackedAdmitted,
    MissingBasisDenied,
    StaleBasisDenied,
    SupportGateDeferred,
    SupportGateDenied,
    DurableDeferredDebt,
}

impl ForgeQueryRuntimeDownstreamResumePostureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeBackedAdmitted => "runtime-backed-admitted",
            Self::MissingBasisDenied => "missing-basis-denied",
            Self::StaleBasisDenied => "stale-basis-denied",
            Self::SupportGateDeferred => "support-gate-deferred",
            Self::SupportGateDenied => "support-gate-denied",
            Self::DurableDeferredDebt => "durable-deferred-debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeDownstreamResumePosture {
    kind: ForgeQueryRuntimeDownstreamResumePostureKind,
    required_basis_identity: Option<ForgeQueryEvidenceIdentity>,
    support_posture: ForgeQueryLowerRuntimeSupportPosture,
    support_identity: ForgeQueryEvidenceIdentity,
    posture_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeDownstreamResumePosture {
    pub(crate) fn new(
        kind: ForgeQueryRuntimeDownstreamResumePostureKind,
        required_basis_identity: Option<ForgeQueryEvidenceIdentity>,
        support_posture: ForgeQueryLowerRuntimeSupportPosture,
        support_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        let posture_identity = runtime_downstream_resume_posture_identity(
            kind,
            required_basis_identity.as_ref(),
            support_posture,
            &support_identity,
        );
        Self {
            kind,
            required_basis_identity,
            support_posture,
            support_identity,
            posture_identity,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeDownstreamResumePostureKind {
        self.kind
    }

    pub fn required_basis_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.required_basis_identity.as_ref()
    }

    pub fn required_basis_for_reporting(&self) -> Option<&str> {
        self.required_basis_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.support_posture
    }

    pub fn support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_identity
    }

    pub fn support_for_reporting(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.posture_identity
    }

    pub fn posture_for_reporting(&self) -> &str {
        self.posture_identity.as_str()
    }
}

pub(crate) fn aggregate_support_posture(
    postures: impl IntoIterator<Item = ForgeQueryLowerRuntimeSupportPosture>,
) -> ForgeQueryLowerRuntimeSupportPosture {
    postures
        .into_iter()
        .max_by_key(|posture| support_posture_rank(*posture))
        .unwrap_or(ForgeQueryLowerRuntimeSupportPosture::Deferred)
}

pub(crate) fn support_gate_resume_kind(
    posture: ForgeQueryLowerRuntimeSupportPosture,
) -> ForgeQueryRuntimeDownstreamResumePostureKind {
    match posture {
        ForgeQueryLowerRuntimeSupportPosture::Admitted => {
            ForgeQueryRuntimeDownstreamResumePostureKind::RuntimeBackedAdmitted
        }
        ForgeQueryLowerRuntimeSupportPosture::Deferred => {
            ForgeQueryRuntimeDownstreamResumePostureKind::SupportGateDeferred
        }
        ForgeQueryLowerRuntimeSupportPosture::CompatibilityDebt
        | ForgeQueryLowerRuntimeSupportPosture::SeamEliminated
        | ForgeQueryLowerRuntimeSupportPosture::Forbidden => {
            ForgeQueryRuntimeDownstreamResumePostureKind::SupportGateDenied
        }
    }
}

fn support_posture_rank(posture: ForgeQueryLowerRuntimeSupportPosture) -> u8 {
    match posture {
        ForgeQueryLowerRuntimeSupportPosture::Admitted => 0,
        ForgeQueryLowerRuntimeSupportPosture::CompatibilityDebt => 1,
        ForgeQueryLowerRuntimeSupportPosture::SeamEliminated => 2,
        ForgeQueryLowerRuntimeSupportPosture::Deferred => 3,
        ForgeQueryLowerRuntimeSupportPosture::Forbidden => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        forge_query_lower_runtime_support_matrix, ForgeQueryLowerRuntimeSeamKey,
    };
    use crate::runtime::evidence_identities::lower_runtime_support_row_identity;

    #[test]
    fn aggregate_support_posture_uses_strictest_row() {
        assert_eq!(
            aggregate_support_posture([
                ForgeQueryLowerRuntimeSupportPosture::Deferred,
                ForgeQueryLowerRuntimeSupportPosture::Admitted,
                ForgeQueryLowerRuntimeSupportPosture::Forbidden,
            ]),
            ForgeQueryLowerRuntimeSupportPosture::Forbidden
        );
    }

    #[test]
    fn support_gate_resume_kind_fails_closed_for_non_admitted_runtime_support() {
        assert_eq!(
            support_gate_resume_kind(ForgeQueryLowerRuntimeSupportPosture::Deferred),
            ForgeQueryRuntimeDownstreamResumePostureKind::SupportGateDeferred
        );
        assert_eq!(
            support_gate_resume_kind(ForgeQueryLowerRuntimeSupportPosture::CompatibilityDebt),
            ForgeQueryRuntimeDownstreamResumePostureKind::SupportGateDenied
        );
        assert_eq!(
            support_gate_resume_kind(ForgeQueryLowerRuntimeSupportPosture::Forbidden),
            ForgeQueryRuntimeDownstreamResumePostureKind::SupportGateDenied
        );
    }

    #[test]
    fn resume_posture_carries_typed_support_identity() {
        let matrix = forge_query_lower_runtime_support_matrix();
        let support = matrix
            .support_for(ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence)
            .expect("support row must exist");
        let support_identity = lower_runtime_support_row_identity(support);
        let posture = ForgeQueryRuntimeDownstreamResumePosture::new(
            ForgeQueryRuntimeDownstreamResumePostureKind::SupportGateDeferred,
            None,
            support.posture(),
            support_identity,
        );
        assert_eq!(posture.support_identity(), posture.support_identity());
    }
}
