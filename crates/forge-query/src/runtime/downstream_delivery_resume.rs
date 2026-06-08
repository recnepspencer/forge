use crate::identity::hash_parts;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportPosture;

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
    required_basis_digest: Option<String>,
    support_posture: ForgeQueryLowerRuntimeSupportPosture,
    support_digest: String,
    posture_digest: String,
}

impl ForgeQueryRuntimeDownstreamResumePosture {
    pub(crate) fn new(
        kind: ForgeQueryRuntimeDownstreamResumePostureKind,
        required_basis_digest: Option<String>,
        support_posture: ForgeQueryLowerRuntimeSupportPosture,
        support_digest: impl Into<String>,
    ) -> Self {
        let support_digest = support_digest.into();
        let posture_digest = hash_parts(&[
            "forge_query_runtime_downstream_resume_posture_v2".to_string(),
            format!("kind:{}", kind.as_str()),
            format!(
                "required_basis:{}",
                required_basis_digest.as_deref().unwrap_or("none")
            ),
            format!("support_posture:{}", support_posture.as_str()),
            format!("support:{support_digest}"),
        ]);
        Self {
            kind,
            required_basis_digest,
            support_posture,
            support_digest,
            posture_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeDownstreamResumePostureKind {
        self.kind
    }

    pub fn required_basis_digest(&self) -> Option<&str> {
        self.required_basis_digest.as_deref()
    }

    pub fn support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.support_posture
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn posture_digest(&self) -> &str {
        &self.posture_digest
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
}
