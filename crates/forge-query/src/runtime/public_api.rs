use crate::identity::hash_parts;

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeSupportProfile,
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeStateKind {
    Ready,
    Pending,
    Stale,
    Failed,
    Cancelled,
    Superseded,
    Denied,
    Unsupported,
}

impl ForgeQueryRuntimeStateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Pending => "pending",
            Self::Stale => "stale",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
            Self::Unsupported => "unsupported",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeStateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeStateSnapshot {
    kind: ForgeQueryRuntimeStateKind,
    basis_digest: String,
    result_shape_digest: String,
    authority_lane: ForgeQueryAuthorityLane,
    explanation: String,
    state_digest: String,
}

impl ForgeQueryRuntimeStateSnapshot {
    pub fn ready(
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryRuntimeStateKind::Ready,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
        )
    }

    pub fn deferred(
        kind: ForgeQueryRuntimeStateKind,
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        debug_assert!(
            kind != ForgeQueryRuntimeStateKind::Ready,
            "ready state should use ForgeQueryRuntimeStateSnapshot::ready"
        );
        Self::new(
            kind,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
        )
    }

    fn new(
        kind: ForgeQueryRuntimeStateKind,
        basis_digest: impl Into<String>,
        result_shape_digest: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        explanation: impl Into<String>,
    ) -> Self {
        let basis_digest = basis_digest.into();
        let result_shape_digest = result_shape_digest.into();
        let explanation = explanation.into();
        let state_digest = hash_parts(&[
            format!("kind:{}", kind.as_str()),
            format!("basis:{basis_digest}"),
            format!("result_shape:{result_shape_digest}"),
            format!("lane:{}", authority_lane.as_str()),
            format!("explanation:{explanation}"),
        ]);
        Self {
            kind,
            basis_digest,
            result_shape_digest,
            authority_lane,
            explanation,
            state_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryRuntimeStateKind {
        self.kind
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    pub fn state_digest(&self) -> &str {
        &self.state_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiFamilyContract {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    authority_lanes: Vec<ForgeQueryAuthorityLane>,
    evidence: Vec<String>,
    reason: Option<String>,
    contract_digest: String,
}

impl ForgeQueryRuntimePublicApiFamilyContract {
    fn from_support_row(row: &super::ForgeQueryRuntimeFamilySupport) -> Self {
        let family = row.family();
        let status = row.status();
        let authority_lanes = row.authority_lanes().to_vec();
        let evidence = row.evidence().to_vec();
        let reason = row.denial_reason().map(str::to_string);
        let mut parts = vec![
            format!("family:{}", family.as_str()),
            format!("status:{}", status.as_str()),
        ];
        parts.extend(
            authority_lanes
                .iter()
                .map(|lane| format!("lane:{}", lane.as_str())),
        );
        parts.extend(evidence.iter().map(|item| format!("evidence:{item}")));
        if let Some(reason) = &reason {
            parts.push(format!("reason:{reason}"));
        }
        let contract_digest = hash_parts(&parts);
        Self {
            family,
            status,
            authority_lanes,
            evidence,
            reason,
            contract_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn authority_lanes(&self) -> &[ForgeQueryAuthorityLane] {
        &self.authority_lanes
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimePublicApiContract {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    families: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
    stable_family_count: usize,
    deferred_family_count: usize,
    unsupported_family_count: usize,
    contract_digest: String,
}

impl ForgeQueryRuntimePublicApiContract {
    pub fn from_support_profile(profile: &ForgeQueryRuntimeSupportProfile) -> Self {
        let families: Vec<_> = profile
            .rows()
            .map(ForgeQueryRuntimePublicApiFamilyContract::from_support_row)
            .collect();
        let stable_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::Supported)
            .count();
        let deferred_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::DeferredDebt)
            .count();
        let unsupported_family_count = families
            .iter()
            .filter(|family| family.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported)
            .count();
        let mut parts = vec![format!("posture:{}", profile.posture().as_str())];
        parts.extend(
            families
                .iter()
                .map(|family| format!("family:{}", family.contract_digest())),
        );
        parts.push(format!("stable:{stable_family_count}"));
        parts.push(format!("deferred:{deferred_family_count}"));
        parts.push(format!("unsupported:{unsupported_family_count}"));
        let contract_digest = hash_parts(&parts);
        Self {
            backend_posture: profile.posture(),
            families,
            stable_family_count,
            deferred_family_count,
            unsupported_family_count,
            contract_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn families(&self) -> &[ForgeQueryRuntimePublicApiFamilyContract] {
        &self.families
    }

    pub fn stable_family_count(&self) -> usize {
        self.stable_family_count
    }

    pub fn deferred_family_count(&self) -> usize {
        self.deferred_family_count
    }

    pub fn unsupported_family_count(&self) -> usize {
        self.unsupported_family_count
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Option<&ForgeQueryRuntimePublicApiFamilyContract> {
        self.families.iter().find(|row| row.family() == family)
    }
}
