use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerExplanationStopFamily {
    CheckerRejection,
    AspectAuthority,
    ProofClaimBlocked,
    QueryRecovery,
    QueryDeclarationEntry,
    QueryContributionComposition,
    QueryBinding,
    QueryContinuation,
    QueryGroupedNeighborhood,
    QueryRouteOrReceipt,
    QuerySignalCompatibility,
    ConservativeEscalation,
    FutureLowerRuntimeCompatibility,
}

impl HadwigerExplanationStopFamily {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::CheckerRejection => "checker_rejection",
            Self::AspectAuthority => "aspect_authority",
            Self::ProofClaimBlocked => "proof_claim_blocked",
            Self::QueryRecovery => "query_recovery",
            Self::QueryDeclarationEntry => "query_declaration_entry",
            Self::QueryContributionComposition => "query_contribution_composition",
            Self::QueryBinding => "query_binding",
            Self::QueryContinuation => "query_continuation",
            Self::QueryGroupedNeighborhood => "query_grouped_neighborhood",
            Self::QueryRouteOrReceipt => "query_route_or_receipt",
            Self::QuerySignalCompatibility => "query_signal_compatibility",
            Self::ConservativeEscalation => "conservative_escalation",
            Self::FutureLowerRuntimeCompatibility => "future_lower_runtime_compatibility",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerExplanationAuthoritySurface {
    CheckerArtifact,
    HadwigerAspectAuthority,
    HadwigerProofAuthority,
    QueryRecovery,
    QueryDeclarationProgression,
    QueryContributionComposition,
    QueryGroupedNeighborhood,
    ProjectionConsumption,
    LowerRuntimeCompatibility,
    ConservativeInvalidation,
}

impl HadwigerExplanationAuthoritySurface {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::CheckerArtifact => "checker_artifact",
            Self::HadwigerAspectAuthority => "hadwiger_aspect_authority",
            Self::HadwigerProofAuthority => "hadwiger_proof_authority",
            Self::QueryRecovery => "query_recovery",
            Self::QueryDeclarationProgression => "query_declaration_progression",
            Self::QueryContributionComposition => "query_contribution_composition",
            Self::QueryGroupedNeighborhood => "query_grouped_neighborhood",
            Self::ProjectionConsumption => "projection_consumption",
            Self::LowerRuntimeCompatibility => "lower_runtime_compatibility",
            Self::ConservativeInvalidation => "conservative_invalidation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerRepairObligation {
    core: HadwigerArtifactCore,
    detail: String,
}

impl HadwigerRepairObligation {
    pub fn new(detail: impl Into<String>) -> Result<Self, HadwigerArtifactShapeError> {
        let detail = require_non_empty(detail, "repair_obligation")?;
        let core = artifact_core(
            HadwigerArtifactKind::RepairObligation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "hadwiger_repair_obligation".to_string(),
            },
            Vec::new(),
            vec![HadwigerArtifactPayloadEntry::text("detail", detail.clone())],
        )?;
        Ok(Self { core, detail })
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl_hadwiger_artifact!(HadwigerRepairObligation, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerReusableNegativeEvidence {
    core: HadwigerArtifactCore,
    failure_basis: String,
    scope: String,
    reactivation_or_repair_hint: String,
}

impl HadwigerReusableNegativeEvidence {
    pub(crate) fn new(
        affected_artifact: HadwigerArtifactReference,
        failure_basis: impl Into<String>,
        scope: impl Into<String>,
        reactivation_or_repair_hint: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let failure_basis = require_non_empty(failure_basis, "failure_basis")?;
        let scope = require_non_empty(scope, "scope")?;
        let reactivation_or_repair_hint =
            require_non_empty(reactivation_or_repair_hint, "reactivation_or_repair_hint")?;
        let core = artifact_core(
            HadwigerArtifactKind::ReusableNegativeEvidence,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "hadwiger_reusable_negative_evidence".to_string(),
            },
            vec![affected_artifact],
            vec![
                HadwigerArtifactPayloadEntry::text("failure_basis", failure_basis.clone()),
                HadwigerArtifactPayloadEntry::text("scope", scope.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "reactivation_or_repair_hint",
                    reactivation_or_repair_hint.clone(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            failure_basis,
            scope,
            reactivation_or_repair_hint,
        })
    }

    pub fn failure_basis(&self) -> &str {
        &self.failure_basis
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn reactivation_or_repair_hint(&self) -> &str {
        &self.reactivation_or_repair_hint
    }
}

impl_hadwiger_artifact!(HadwigerReusableNegativeEvidence, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerSurvivingEvidenceReport {
    surviving_artifacts: Vec<HadwigerArtifactReference>,
}

impl HadwigerSurvivingEvidenceReport {
    pub(crate) fn new(mut surviving_artifacts: Vec<HadwigerArtifactReference>) -> Self {
        surviving_artifacts.sort_by_key(HadwigerArtifactReference::stable_token);
        surviving_artifacts.dedup();
        Self {
            surviving_artifacts,
        }
    }

    pub fn surviving_artifacts(&self) -> &[HadwigerArtifactReference] {
        &self.surviving_artifacts
    }
}
