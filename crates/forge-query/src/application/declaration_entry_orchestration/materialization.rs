use forge_foundational::facade::{
    plan_foundational_profile_materialization_with_elision, profiles, AdmissionReadinessProfile,
    BoundaryArtifactTarget, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalDescriptiveElisionProfile, FoundationalMaterializationCost,
    MaterializedFoundationalProfileSet, RetentionDeliveryProfile, SupportPostureProfile,
};
use forge_proof::TransitionOutcome;

use super::artifacts::{
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
    OperationalLean,
    SupportReady,
    FullDescriptive,
}

impl ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperationalLean => "operational_lean",
            Self::SupportReady => "support_ready",
            Self::FullDescriptive => "full_descriptive",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationCostPosture {
    OrdinaryDefault,
    ExplicitlyLean,
    ExplicitlyRich,
    PreparedButNotExecuted,
    ExpensiveByDefault,
}

impl ForgeQueryDeclarationEntryOrchestrationCostPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryDefault => "ordinary_default",
            Self::ExplicitlyLean => "explicitly_lean",
            Self::ExplicitlyRich => "explicitly_rich",
            Self::PreparedButNotExecuted => "prepared_but_not_executed",
            Self::ExpensiveByDefault => "expensive_by_default",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationMaterializationGate {
    AdmittedByDefault,
    ExplicitRequestRequired,
    ForbiddenOnOrdinaryLane,
    PreparedOnly,
    UnsupportedForCurrentArtifactSet,
}

impl ForgeQueryDeclarationEntryOrchestrationMaterializationGate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedByDefault => "admitted_by_default",
            Self::ExplicitRequestRequired => "explicit_request_required",
            Self::ForbiddenOnOrdinaryLane => "forbidden_on_ordinary_lane",
            Self::PreparedOnly => "prepared_only",
            Self::UnsupportedForCurrentArtifactSet => "unsupported_for_current_artifact_set",
        }
    }
}

pub struct ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy {
    foundational_evidence_profile: FoundationalBoundaryEvidenceMaterializationProfile,
    receipt_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    envelope_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    support_rich_publication_admitted: bool,
    diagnostic_rich_publication_admitted: bool,
    cost_posture: ForgeQueryDeclarationEntryOrchestrationCostPosture,
    materialization_gate: ForgeQueryDeclarationEntryOrchestrationMaterializationGate,
}

impl ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy {
    pub(crate) fn default_for_lane(
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
        artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ) -> Self {
        match (exposure_level, artifact_policy) {
            (
                ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
                ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
            ) => Self {
                foundational_evidence_profile: foundational_profile_for_tier(
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                ),
                receipt_tier:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
                envelope_tier:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
                support_rich_publication_admitted: true,
                diagnostic_rich_publication_admitted: false,
                cost_posture: ForgeQueryDeclarationEntryOrchestrationCostPosture::OrdinaryDefault,
                materialization_gate:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationGate::AdmittedByDefault,
            },
            (
                ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
                ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
            ) => Self {
                foundational_evidence_profile: foundational_profile_for_tier(
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                ),
                receipt_tier:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                envelope_tier:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                support_rich_publication_admitted: false,
                diagnostic_rich_publication_admitted: false,
                cost_posture: ForgeQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyLean,
                materialization_gate:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationGate::AdmittedByDefault,
            },
            (
                ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
                ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
            ) => Self {
                foundational_evidence_profile: foundational_profile_for_tier(
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
                ),
                receipt_tier:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
                envelope_tier:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
                support_rich_publication_admitted: true,
                diagnostic_rich_publication_admitted: true,
                cost_posture: ForgeQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyRich,
                materialization_gate:
                    ForgeQueryDeclarationEntryOrchestrationMaterializationGate::ExplicitRequestRequired,
            },
            _ => panic!("unsupported orchestration exposure/artifact policy pairing"),
        }
    }

    pub fn foundational_evidence_profile(
        &self,
    ) -> FoundationalBoundaryEvidenceMaterializationProfile {
        self.foundational_evidence_profile
    }

    pub fn receipt_tier(&self) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
        self.receipt_tier
    }

    pub fn envelope_tier(&self) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
        self.envelope_tier
    }

    pub fn support_rich_publication_admitted(&self) -> bool {
        self.support_rich_publication_admitted
    }

    pub fn diagnostic_rich_publication_admitted(&self) -> bool {
        self.diagnostic_rich_publication_admitted
    }

    pub fn cost_posture(&self) -> ForgeQueryDeclarationEntryOrchestrationCostPosture {
        self.cost_posture
    }

    pub fn materialization_gate(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationMaterializationGate {
        self.materialization_gate
    }

    pub fn materialization_tier(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
        self.envelope_tier
    }
}

pub(crate) fn foundational_profile_for_tier(
    tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
) -> FoundationalBoundaryEvidenceMaterializationProfile {
    match tier {
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics
        }
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady => {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
        }
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => {
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness
        }
    }
}

pub(crate) fn foundational_materialization_tier(
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
    match profile {
        FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean
        }
        FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady
        }
        FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
        }
    }
}

pub(crate) fn materialized_profile_for_tier(
    tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
) -> MaterializedFoundationalProfileSet {
    let requested = match tier {
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => profiles()
            .set()
            .diagnostic_richness(DiagnosticRichnessProfile::OperationalMinimal)
            .support_posture(SupportPostureProfile::InternalOnly)
            .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
            .admission_readiness(AdmissionReadinessProfile::Admitted)
            .retention_delivery(RetentionDeliveryProfile::Retained)
            .certification_posture(CertificationPostureProfile::Uncertified)
            .request()
            .expect("operational-lean materialization profile should compose"),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady => profiles()
            .set()
            .diagnostic_richness(DiagnosticRichnessProfile::Standard)
            .support_posture(SupportPostureProfile::SupportReady)
            .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
            .admission_readiness(AdmissionReadinessProfile::Admitted)
            .retention_delivery(RetentionDeliveryProfile::Retained)
            .certification_posture(CertificationPostureProfile::Uncertified)
            .request()
            .expect("support-ready materialization profile should compose"),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => profiles()
            .set()
            .diagnostic_richness(DiagnosticRichnessProfile::Forensic)
            .support_posture(SupportPostureProfile::CertificationReady)
            .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
            .admission_readiness(AdmissionReadinessProfile::Admitted)
            .retention_delivery(RetentionDeliveryProfile::Retained)
            .certification_posture(CertificationPostureProfile::EvidenceBacked)
            .request()
            .expect("full-descriptive materialization profile should compose"),
    };
    let admitted = match profiles().progression().admit_same(requested) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("materialization profile admission should succeed: {outcome:?}"),
    };
    match profiles().progression().materialize_same(admitted) {
        TransitionOutcome::Success(value) => *value.payload(),
        outcome => panic!("materialization profile materialization should succeed: {outcome:?}"),
    }
}

pub(crate) fn descriptive_materialization_cost_for_tier(
    tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
) -> FoundationalMaterializationCost {
    let elision = match tier {
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => {
            FoundationalDescriptiveElisionProfile::OperationalSummary
        }
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady
        | ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => {
            FoundationalDescriptiveElisionProfile::FullFidelity
        }
    };
    let profile = materialized_profile_for_tier(tier);
    plan_foundational_profile_materialization_with_elision::<BoundaryArtifactTarget>(
        &profile, elision,
    )
    .cost()
}
