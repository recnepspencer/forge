use std::collections::BTreeMap;

use super::{
    ForgeQueryBranchBasisAdmission, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeInspectionEvidence,
};
use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeSupportProfile {
    posture: ForgeQueryRuntimeBackendPosture,
    rows: BTreeMap<ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport>,
}

impl ForgeQueryRuntimeSupportProfile {
    pub fn new(rows: impl IntoIterator<Item = ForgeQueryRuntimeFamilySupport>) -> Self {
        Self {
            posture: ForgeQueryRuntimeBackendPosture::Primary,
            rows: rows.into_iter().map(|row| (row.family(), row)).collect(),
        }
    }

    pub fn compatibility_backend() -> Self {
        Self::new([
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Read,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [],
                ["backend-live-read"],
            ),
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Live,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [],
                ["backend-live-declaration"],
            ),
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Computed,
                [ForgeQueryAuthorityLane::DerivedRuntimeState],
                [ForgeQueryEffectPolicy::DeriveOnly],
                ["query-local-derived-view-runtime"],
            ),
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Effect,
                [ForgeQueryAuthorityLane::EffectDeliveryState],
                [ForgeQueryEffectPolicy::AuthoritativeAllowed],
                ["query-local-effect-delivery-runtime"],
            ),
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                [
                    ForgeQueryAuthorityLane::PreviewTruth,
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                ],
                [
                    ForgeQueryEffectPolicy::DeriveOnly,
                    ForgeQueryEffectPolicy::Muted,
                    ForgeQueryEffectPolicy::Redirected,
                    ForgeQueryEffectPolicy::SandboxedWriteIntent,
                    ForgeQueryEffectPolicy::AuthoritativeAllowed,
                ],
                ["preview-session-admission"],
            ),
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Write,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [],
                ["backend-write-authority", "authoritative-mutation-evidence"],
            ),
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Intent,
                "intent commit strategies are not admitted by this runtime batch",
            ),
            ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Inspect,
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                    ForgeQueryAuthorityLane::DerivedRuntimeState,
                    ForgeQueryAuthorityLane::EffectDeliveryState,
                    ForgeQueryAuthorityLane::PendingWriteIntent,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                [],
                ["retained-runtime-artifact-inspection"],
            ),
            ForgeQueryRuntimeFamilySupport::deferred(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                "temporal query basis is deferred to Milestone 9.4",
            ),
            ForgeQueryRuntimeFamilySupport::deferred(
                ForgeQueryRuntimeFacadeFamily::AsyncResource,
                "async/resource query families are deferred to Milestone 9.5",
            ),
            ForgeQueryRuntimeFamilySupport::deferred(
                ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
                "mixed truth/time/async delivery is deferred to Milestone 9.6",
            ),
            ForgeQueryRuntimeFamilySupport::deferred(
                ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
                "store-backed execution parity is deferred to Milestone 10",
            ),
            ForgeQueryRuntimeFamilySupport::deferred(
                ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
                "durable restart and artifact reload are deferred to Milestone 11",
            ),
        ])
        .with_posture(ForgeQueryRuntimeBackendPosture::Compatibility)
    }

    pub fn bridge_backed(
        subscription_activation_evidence: impl Into<String>,
        preview_basis_evidence: impl Into<String>,
        inspector_evidence: impl Into<String>,
    ) -> Self {
        let subscription_activation_evidence = subscription_activation_evidence.into();
        let preview_basis_evidence = preview_basis_evidence.into();
        let inspector_evidence = inspector_evidence.into();
        Self::compatibility_backend()
            .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Live,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [],
                [
                    "backend-live-declaration".to_string(),
                    subscription_activation_evidence,
                ],
            ))
            .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                [
                    ForgeQueryAuthorityLane::PreviewTruth,
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                ],
                [
                    ForgeQueryEffectPolicy::DeriveOnly,
                    ForgeQueryEffectPolicy::Muted,
                    ForgeQueryEffectPolicy::Redirected,
                    ForgeQueryEffectPolicy::SandboxedWriteIntent,
                    ForgeQueryEffectPolicy::AuthoritativeAllowed,
                ],
                [
                    "preview-session-admission".to_string(),
                    preview_basis_evidence,
                ],
            ))
            .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
                ForgeQueryRuntimeFacadeFamily::Inspect,
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                    ForgeQueryAuthorityLane::DerivedRuntimeState,
                    ForgeQueryAuthorityLane::EffectDeliveryState,
                    ForgeQueryAuthorityLane::PendingWriteIntent,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                [],
                [
                    "retained-runtime-artifact-inspection".to_string(),
                    inspector_evidence,
                ],
            ))
            .with_posture(ForgeQueryRuntimeBackendPosture::Primary)
    }

    pub fn support_for(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Option<&ForgeQueryRuntimeFamilySupport> {
        self.rows.get(&family)
    }

    pub fn posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.posture
    }

    pub fn with_posture(mut self, posture: ForgeQueryRuntimeBackendPosture) -> Self {
        self.posture = posture;
        self
    }

    pub fn with_family_support(mut self, row: ForgeQueryRuntimeFamilySupport) -> Self {
        self.rows.insert(row.family(), row);
        self
    }

    pub fn rows(&self) -> impl Iterator<Item = &ForgeQueryRuntimeFamilySupport> {
        self.rows.values()
    }

    pub fn admit(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<(), ForgeQueryRuntimeSupportDenial> {
        let Some(row) = self.support_for(family) else {
            return Err(ForgeQueryRuntimeSupportDenial {
                family,
                reason: "backend support profile does not declare this facade family".to_string(),
            });
        };

        match row.status() {
            ForgeQueryRuntimeFamilySupportStatus::Supported => Ok(()),
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt => {
                Err(ForgeQueryRuntimeSupportDenial {
                    family,
                    reason: row
                        .denial_reason()
                        .unwrap_or("backend support profile marks this facade family deferred")
                        .to_string(),
                })
            }
            ForgeQueryRuntimeFamilySupportStatus::Unsupported => {
                Err(ForgeQueryRuntimeSupportDenial {
                    family,
                    reason: row
                        .denial_reason()
                        .unwrap_or("backend support profile marks this facade family unsupported")
                        .to_string(),
                })
            }
        }
    }

    pub(crate) fn validate_backend_claims(
        &self,
        has_intent_authority: bool,
    ) -> Result<(), ForgeQueryRuntimeSupportDenial> {
        if self
            .support_for(ForgeQueryRuntimeFacadeFamily::Intent)
            .is_some_and(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::Supported)
            && !has_intent_authority
        {
            return Err(ForgeQueryRuntimeSupportDenial::new(
                ForgeQueryRuntimeFacadeFamily::Intent,
                "intent support requires an executable intent authority adapter",
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeSupportDenial {
    family: ForgeQueryRuntimeFacadeFamily,
    reason: String,
}

impl ForgeQueryRuntimeSupportDenial {
    pub(crate) fn new(family: ForgeQueryRuntimeFacadeFamily, reason: impl Into<String>) -> Self {
        Self {
            family,
            reason: reason.into(),
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl std::fmt::Display for ForgeQueryRuntimeSupportDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "runtime backend does not admit `{}` facade family: {}",
            self.family, self.reason
        )
    }
}

#[allow(dead_code)]
fn _keep_types_visible(
    _authority: &ForgeQueryRuntimeEvidenceAuthority,
    _preview: &ForgeQueryPreviewBasisAdmission,
    _branch: &ForgeQueryBranchBasisAdmission,
    _inspection: &ForgeQueryRuntimeInspectionEvidence,
) {
}
