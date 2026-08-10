#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionEligibilityAuthority {
    RuntimeIntentAuthorityAdapter,
    RuntimeWriteAuthorityAdapter,
    BasisLifecycleObservationAuthority,
    ProjectionConsumptionEligibilityAuthority,
    ReadCompositionExecutionAuthority,
    InspectionMaterializationExecutionAuthority,
    DeferredInspectionMaterializationAuthority,
    LowerRuntimeCapabilityRoutingAuthority,
}

impl WorthQueryIntentAdmissionEligibilityAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeIntentAuthorityAdapter => "runtime-intent-authority-adapter",
            Self::RuntimeWriteAuthorityAdapter => "runtime-write-authority-adapter",
            Self::BasisLifecycleObservationAuthority => "basis-lifecycle-observation-authority",
            Self::ProjectionConsumptionEligibilityAuthority => {
                "projection-consumption-eligibility-authority"
            }
            Self::ReadCompositionExecutionAuthority => "read-composition-execution-authority",
            Self::InspectionMaterializationExecutionAuthority => {
                "inspection-materialization-execution-authority"
            }
            Self::DeferredInspectionMaterializationAuthority => {
                "deferred-inspection-materialization-authority"
            }
            Self::LowerRuntimeCapabilityRoutingAuthority => {
                "lower-runtime-capability-routing-authority"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionPlanKind {
    AuthoritativeIntentExecutionPlan,
    EffectTriggeredIntentExecutionPlan,
    AuthoritativeMutationExecutionPlan,
    AuthoritativeMutationBatchExecutionPlan,
    BasisObservationPlan,
    ProjectionConsumptionPlan,
    ReadExecutionPlan,
    UnifiedInspectionExecutionPlan,
    InspectionMaterializationExecutionPlan,
    DeferredInspectionMaterializationPlan,
    ExistingTruthProbeRoutingPlan,
}

impl WorthQueryIntentAdmissionPlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeIntentExecutionPlan => "authoritative-intent-execution-plan",
            Self::EffectTriggeredIntentExecutionPlan => "effect-triggered-intent-execution-plan",
            Self::AuthoritativeMutationExecutionPlan => "authoritative-mutation-execution-plan",
            Self::AuthoritativeMutationBatchExecutionPlan => {
                "authoritative-mutation-batch-execution-plan"
            }
            Self::BasisObservationPlan => "basis-observation-plan",
            Self::ProjectionConsumptionPlan => "projection-consumption-plan",
            Self::ReadExecutionPlan => "read-execution-plan",
            Self::UnifiedInspectionExecutionPlan => "unified-inspection-execution-plan",
            Self::InspectionMaterializationExecutionPlan => {
                "inspection-materialization-execution-plan"
            }
            Self::DeferredInspectionMaterializationPlan => {
                "deferred-inspection-materialization-plan"
            }
            Self::ExistingTruthProbeRoutingPlan => "existing-truth-probe-routing-plan",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionDecisionClass {
    AdvisoryNotYetExercisedOnCoveredEntrypoint,
    ProjectionWarningBearingAdmission,
    InspectionDetailRedactionAdvisory,
    DeferredNeighborSupport,
    AdmissionOrExecutionViolation,
    NeighborUnsupportedUntilCoverage,
}

impl WorthQueryIntentAdmissionDecisionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryNotYetExercisedOnCoveredEntrypoint => {
                "advisory-not-yet-exercised-on-covered-entrypoint"
            }
            Self::ProjectionWarningBearingAdmission => "projection-warning-bearing-admission",
            Self::InspectionDetailRedactionAdvisory => "inspection-detail-redaction-advisory",
            Self::DeferredNeighborSupport => "deferred-neighbor-support",
            Self::AdmissionOrExecutionViolation => "admission-or-execution-violation",
            Self::NeighborUnsupportedUntilCoverage => "neighbor-unsupported-until-coverage",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionResultArtifact {
    WorthQueryIntentReceipt,
    WorthQueryEffectIntentReceipt,
    WorthQueryWriteReceipt,
    WorthQueryBatchWriteReceipt,
    WorthQueryReadResult,
    WorthQueryLiveReadResult,
    WorthQueryUnifiedInspectionResult,
    WorthQueryDerivedMaterializationResult,
    WorthQueryDerivedInspectionResult,
    WorthQueryExistingTruthProbeResult,
    ScopedObservationBasis,
    MaterializedProjectionContract,
    DeferredInspectionMaterializationArtifact,
}

impl WorthQueryIntentAdmissionResultArtifact {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorthQueryIntentReceipt => "WorthQueryIntentReceipt",
            Self::WorthQueryEffectIntentReceipt => "WorthQueryEffectIntentReceipt",
            Self::WorthQueryWriteReceipt => "WorthQueryWriteReceipt",
            Self::WorthQueryBatchWriteReceipt => "WorthQueryBatchWriteReceipt",
            Self::WorthQueryReadResult => "WorthQueryReadResult",
            Self::WorthQueryLiveReadResult => "WorthQueryLiveReadResult",
            Self::WorthQueryUnifiedInspectionResult => "WorthQueryUnifiedInspectionResult",
            Self::WorthQueryDerivedMaterializationResult => {
                "WorthQueryDerivedMaterializationResult"
            }
            Self::WorthQueryDerivedInspectionResult => "WorthQueryDerivedInspectionResult",
            Self::WorthQueryExistingTruthProbeResult => "WorthQueryExistingTruthProbeResult",
            Self::ScopedObservationBasis => "ScopedObservationBasis",
            Self::MaterializedProjectionContract => "MaterializedProjectionContract",
            Self::DeferredInspectionMaterializationArtifact => {
                "deferred-inspection-materialization-artifact"
            }
        }
    }
}
