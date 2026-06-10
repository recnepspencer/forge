use std::collections::BTreeMap;

use super::bridge_backed_verification_profile::{
    default_bridge_backed_verification_support_rows,
    ForgeQueryBridgeBackedVerificationSupportProfileRow,
};
use super::{
    default_graph_composition_capability_support_rows, ForgeQueryBranchBasisAdmission,
    ForgeQueryGraphCompositionCapabilitySupportRow, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeFamilyTeachingPosture,
    ForgeQueryRuntimeInspectionEvidence,
};
use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeSupportProfile {
    posture: ForgeQueryRuntimeBackendPosture,
    rows: BTreeMap<ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport>,
    bridge_backed_verification_support_rows:
        Vec<ForgeQueryBridgeBackedVerificationSupportProfileRow>,
    graph_composition_capability_support_rows: Vec<ForgeQueryGraphCompositionCapabilitySupportRow>,
}

impl ForgeQueryRuntimeSupportProfile {
    pub fn new(rows: impl IntoIterator<Item = ForgeQueryRuntimeFamilySupport>) -> Self {
        Self {
            posture: ForgeQueryRuntimeBackendPosture::Primary,
            rows: rows.into_iter().map(|row| (row.family(), row)).collect(),
            bridge_backed_verification_support_rows:
                default_bridge_backed_verification_support_rows(),
            graph_composition_capability_support_rows:
                default_graph_composition_capability_support_rows(),
        }
    }

    pub fn scaffold_backend_profile() -> Self {
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
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                super::ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
            ),
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture(
                ForgeQueryRuntimeFacadeFamily::AsyncResource,
                super::ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::AsyncResourceState],
                [],
                ["runtime-backed-async-resource-state-inspection"],
            ),
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture(
                ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
                super::ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::BridgeExternalState],
                [],
                ["runtime-backed-mixed-cause-delivery-state-inspection"],
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
        .with_posture(ForgeQueryRuntimeBackendPosture::Scaffold)
    }

    pub fn bridge_backed(
        subscription_activation_evidence: impl Into<String>,
        preview_basis_evidence: impl Into<String>,
        inspector_evidence: impl Into<String>,
    ) -> Self {
        let subscription_activation_evidence = subscription_activation_evidence.into();
        let preview_basis_evidence = preview_basis_evidence.into();
        let inspector_evidence = inspector_evidence.into();
        Self::scaffold_backend_profile()
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

    pub(crate) fn bridge_backed_verification_support_rows(
        &self,
    ) -> &[ForgeQueryBridgeBackedVerificationSupportProfileRow] {
        &self.bridge_backed_verification_support_rows
    }

    pub(crate) fn graph_composition_capability_support_rows(
        &self,
    ) -> &[ForgeQueryGraphCompositionCapabilitySupportRow] {
        &self.graph_composition_capability_support_rows
    }

    pub(crate) fn with_bridge_backed_verification_support_row(
        mut self,
        row: ForgeQueryBridgeBackedVerificationSupportProfileRow,
    ) -> Self {
        let rows = &mut self.bridge_backed_verification_support_rows;
        if let Some(index) = rows.iter().position(|candidate| {
            candidate.operation_family() == row.operation_family()
                && candidate.target_binding_family() == row.target_binding_family()
        }) {
            rows[index] = row;
        } else {
            rows.push(row);
        }
        self
    }

    pub fn with_bridge_backed_verification_support(
        self,
        operation_family: impl Into<String>,
        target_binding_family: impl Into<String>,
        scaffold_profile_supported: bool,
        primary_bridge_backed_runtime_supported: bool,
        denial_class_when_primary_unsupported: Option<&str>,
    ) -> Self {
        self.with_bridge_backed_verification_support_row(
            ForgeQueryBridgeBackedVerificationSupportProfileRow::new(
                operation_family,
                target_binding_family,
                scaffold_profile_supported,
                primary_bridge_backed_runtime_supported,
                denial_class_when_primary_unsupported,
            ),
        )
    }

    pub fn with_graph_composition_capability_support_row(
        mut self,
        row: ForgeQueryGraphCompositionCapabilitySupportRow,
    ) -> Self {
        if let Some(index) = self
            .graph_composition_capability_support_rows
            .iter()
            .position(|candidate| {
                candidate.capability_family() == row.capability_family()
                    && candidate.capability_class() == row.capability_class()
            })
        {
            self.graph_composition_capability_support_rows[index] = row;
        } else {
            self.graph_composition_capability_support_rows.push(row);
        }
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
            return Err(ForgeQueryRuntimeSupportDenial::unsupported(
                family,
                "backend support profile does not declare this facade family",
            ));
        };

        match row.status() {
            ForgeQueryRuntimeFamilySupportStatus::Supported => Ok(()),
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt => {
                Err(ForgeQueryRuntimeSupportDenial::new(
                    family,
                    ForgeQueryRuntimeFamilySupportStatus::DeferredDebt,
                    Some(row.teaching_posture()),
                    row.denial_reason()
                        .unwrap_or("backend support profile marks this facade family deferred"),
                ))
            }
            ForgeQueryRuntimeFamilySupportStatus::Unsupported => {
                Err(ForgeQueryRuntimeSupportDenial::new(
                    family,
                    ForgeQueryRuntimeFamilySupportStatus::Unsupported,
                    Some(row.teaching_posture()),
                    row.denial_reason()
                        .unwrap_or("backend support profile marks this facade family unsupported"),
                ))
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
                ForgeQueryRuntimeFamilySupportStatus::Supported,
                self.support_for(ForgeQueryRuntimeFacadeFamily::Intent)
                    .map(ForgeQueryRuntimeFamilySupport::teaching_posture),
                "intent support requires an executable intent authority adapter",
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeSupportDenial {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
    reason: String,
}

impl ForgeQueryRuntimeSupportDenial {
    pub(crate) fn new(
        family: ForgeQueryRuntimeFacadeFamily,
        status: ForgeQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            family,
            status,
            teaching_posture,
            reason: reason.into(),
        }
    }

    pub(crate) fn unsupported(
        family: ForgeQueryRuntimeFacadeFamily,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            family,
            ForgeQueryRuntimeFamilySupportStatus::Unsupported,
            None,
            reason,
        )
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> Option<ForgeQueryRuntimeFamilyTeachingPosture> {
        self.teaching_posture
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

fn _keep_types_visible(
    _authority: &ForgeQueryRuntimeEvidenceAuthority,
    _preview: &ForgeQueryPreviewBasisAdmission,
    _branch: &ForgeQueryBranchBasisAdmission,
    _inspection: &ForgeQueryRuntimeInspectionEvidence,
) {
}
