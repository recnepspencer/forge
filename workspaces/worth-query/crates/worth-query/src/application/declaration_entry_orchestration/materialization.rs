use worth_foundational::facade::{
    plan_foundational_profile_materialization_with_elision, profiles, AdmissionReadinessProfile,
    BoundaryArtifactTarget, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile,
    FoundationalBoundaryEvidenceMaterializationProfile, FoundationalDescriptiveElisionProfile,
    FoundationalMaterializationCost, MaterializedFoundationalProfileSet,
    ObservationActivationProfile, RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_proof::TransitionOutcome;

use crate::application::declaration_publication::declaration_publication_for_tier;
use crate::application::{
    route_scoped_declaration_aspect_contract, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectPublication,
};

use super::artifacts::{
    WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationMaterializationTier {
    OperationalLean,
    SupportReady,
    FullDescriptive,
}

impl WorthQueryDeclarationEntryOrchestrationMaterializationTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperationalLean => "operational_lean",
            Self::SupportReady => "support_ready",
            Self::FullDescriptive => "full_descriptive",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationCostPosture {
    OrdinaryDefault,
    ExplicitlyLean,
    ExplicitlyRich,
    PreparedButNotExecuted,
    ExpensiveByDefault,
}

impl WorthQueryDeclarationEntryOrchestrationCostPosture {
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
pub enum WorthQueryDeclarationEntryOrchestrationMaterializationGate {
    AdmittedByDefault,
    ExplicitRequestRequired,
    ForbiddenOnOrdinaryLane,
    PreparedOnly,
    UnsupportedForCurrentArtifactSet,
}

impl WorthQueryDeclarationEntryOrchestrationMaterializationGate {
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

pub struct WorthQueryDeclarationEntryOrchestrationMaterializationPolicy {
    foundational_evidence_profile: FoundationalBoundaryEvidenceMaterializationProfile,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
    envelope_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
    foundational_aspect_publication: WorthQueryDeclarationAspectPublication,
    receipt_aspect_publication: WorthQueryDeclarationAspectPublication,
    envelope_aspect_publication: WorthQueryDeclarationAspectPublication,
    support_rich_publication_admitted: bool,
    diagnostic_rich_publication_admitted: bool,
    cost_posture: WorthQueryDeclarationEntryOrchestrationCostPosture,
    materialization_gate: WorthQueryDeclarationEntryOrchestrationMaterializationGate,
}

impl WorthQueryDeclarationEntryOrchestrationMaterializationPolicy {
    pub(crate) fn default_for_lane(
        exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
        artifact_policy: WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
        aspect_contract: &WorthQueryDeclarationAspectContract,
        aspect_coverage: &WorthQueryDeclarationAspectCoverage,
    ) -> Self {
        let (
            foundational_evidence_profile,
            receipt_tier,
            envelope_tier,
            support_rich_publication_admitted,
            diagnostic_rich_publication_admitted,
            cost_posture,
            materialization_gate,
        ) = match (exposure_level, artifact_policy) {
            (
                WorthQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
                WorthQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
            ) => (
                foundational_profile_for_tier(
                    WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                ),
                WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
                WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
                true,
                false,
                WorthQueryDeclarationEntryOrchestrationCostPosture::OrdinaryDefault,
                WorthQueryDeclarationEntryOrchestrationMaterializationGate::AdmittedByDefault,
            ),
            (
                WorthQueryDeclarationEntryOrchestrationExposureLevel::Checked,
                WorthQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
            ) => (
                foundational_profile_for_tier(
                    WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                ),
                WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
                false,
                false,
                WorthQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyLean,
                WorthQueryDeclarationEntryOrchestrationMaterializationGate::AdmittedByDefault,
            ),
            (
                WorthQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
                WorthQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
            ) => (
                foundational_profile_for_tier(
                    WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
                ),
                WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
                WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
                true,
                true,
                WorthQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyRich,
                WorthQueryDeclarationEntryOrchestrationMaterializationGate::ExplicitRequestRequired,
            ),
            _ => panic!("unsupported orchestration exposure/artifact policy pairing"),
        };
        let crossing_aspect_contract = route_scoped_declaration_aspect_contract(aspect_contract);

        Self {
            foundational_evidence_profile,
            receipt_tier,
            envelope_tier,
            foundational_aspect_publication: declaration_publication_for_tier(
                aspect_contract,
                aspect_coverage,
                foundational_materialization_tier(foundational_evidence_profile),
            ),
            receipt_aspect_publication: declaration_publication_for_tier(
                &crossing_aspect_contract,
                aspect_coverage,
                receipt_tier,
            ),
            envelope_aspect_publication: declaration_publication_for_tier(
                &crossing_aspect_contract,
                aspect_coverage,
                envelope_tier,
            ),
            support_rich_publication_admitted,
            diagnostic_rich_publication_admitted,
            cost_posture,
            materialization_gate,
        }
    }

    pub fn foundational_evidence_profile(
        &self,
    ) -> FoundationalBoundaryEvidenceMaterializationProfile {
        self.foundational_evidence_profile
    }

    pub fn receipt_tier(&self) -> WorthQueryDeclarationEntryOrchestrationMaterializationTier {
        self.receipt_tier
    }

    pub fn envelope_tier(&self) -> WorthQueryDeclarationEntryOrchestrationMaterializationTier {
        self.envelope_tier
    }

    pub fn foundational_aspect_publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.foundational_aspect_publication
    }

    pub fn receipt_aspect_publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.receipt_aspect_publication
    }

    pub fn envelope_aspect_publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.envelope_aspect_publication
    }

    pub fn support_rich_publication_admitted(&self) -> bool {
        self.support_rich_publication_admitted
    }

    pub fn diagnostic_rich_publication_admitted(&self) -> bool {
        self.diagnostic_rich_publication_admitted
    }

    pub fn cost_posture(&self) -> WorthQueryDeclarationEntryOrchestrationCostPosture {
        self.cost_posture
    }

    pub fn materialization_gate(
        &self,
    ) -> WorthQueryDeclarationEntryOrchestrationMaterializationGate {
        self.materialization_gate
    }

    pub fn materialization_tier(
        &self,
    ) -> WorthQueryDeclarationEntryOrchestrationMaterializationTier {
        self.envelope_tier
    }
}

pub(crate) fn foundational_profile_for_tier(
    tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> FoundationalBoundaryEvidenceMaterializationProfile {
    match tier {
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics
        }
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady => {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
        }
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => {
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness
        }
    }
}

pub(crate) fn foundational_materialization_tier(
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> WorthQueryDeclarationEntryOrchestrationMaterializationTier {
    match profile {
        FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => {
            WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean
        }
        FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics => {
            WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady
        }
        FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness => {
            WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
        }
    }
}

pub(crate) fn materialized_profile_for_tier(
    tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> MaterializedFoundationalProfileSet {
    let requested = match tier {
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => profiles()
            .set()
            .diagnostic_richness(DiagnosticRichnessProfile::OperationalMinimal)
            .support_posture(SupportPostureProfile::InternalOnly)
            .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
            .execution_objective(ExecutionObjectiveProfile::Throughput)
            .observation_activation(ObservationActivationProfile::Continuous)
            .admission_readiness(AdmissionReadinessProfile::Admitted)
            .retention_delivery(RetentionDeliveryProfile::Retained)
            .certification_posture(CertificationPostureProfile::Uncertified)
            .request()
            .expect("operational-lean materialization profile should compose"),
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady => profiles()
            .set()
            .diagnostic_richness(DiagnosticRichnessProfile::Standard)
            .support_posture(SupportPostureProfile::SupportReady)
            .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
            .execution_objective(ExecutionObjectiveProfile::Balanced)
            .observation_activation(ObservationActivationProfile::Continuous)
            .admission_readiness(AdmissionReadinessProfile::Admitted)
            .retention_delivery(RetentionDeliveryProfile::Retained)
            .certification_posture(CertificationPostureProfile::Uncertified)
            .request()
            .expect("support-ready materialization profile should compose"),
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => profiles()
            .set()
            .diagnostic_richness(DiagnosticRichnessProfile::Forensic)
            .support_posture(SupportPostureProfile::CertificationReady)
            .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
            .execution_objective(ExecutionObjectiveProfile::Balanced)
            .observation_activation(ObservationActivationProfile::Continuous)
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
    tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> FoundationalMaterializationCost {
    let elision = match tier {
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => {
            FoundationalDescriptiveElisionProfile::OperationalSummary
        }
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady
        | WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => {
            FoundationalDescriptiveElisionProfile::FullFidelity
        }
    };
    let profile = materialized_profile_for_tier(tier);
    plan_foundational_profile_materialization_with_elision::<BoundaryArtifactTarget>(
        &profile, elision,
    )
    .expect("declaration-entry materialization planning should succeed")
    .cost()
}
