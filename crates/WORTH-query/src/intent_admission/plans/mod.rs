mod inspection;
mod mutation;
mod read;
mod routing;
mod unified_inspection;

use crate::basis_lifecycle::{
    admit_basis_capability, scope_basis_for_observation, AdmittedBasisCapability,
    ObservationLaneWitness, ScopedObservationBasis,
};
use crate::identity::hash_parts;
use crate::projection_consumption::{
    AdmittedProjectionConsumption, MaterializedProjectionContract,
    ProjectionConsumptionEligibility, ProjectionConsumptionWarnings,
};
use crate::runtime::WorthQueryIntentDeclaration;

use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionEligibility,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
    WorthQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeIntentExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryEffectTriggeredIntentExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
}

pub use inspection::{
    WorthQueryDerivedInspectionExecutionPlan, WorthQueryDerivedMaterializationExecutionPlan,
};
pub use mutation::{
    WorthQueryAuthoritativeMutationBatchExecutionPlan, WorthQueryAuthoritativeMutationExecutionPlan,
};
pub use read::{WorthQueryLiveReadExecutionPlan, WorthQueryReadExecutionPlan};
pub use routing::WorthQueryExistingTruthProbeExecutionPlan;
pub use unified_inspection::WorthQueryUnifiedInspectionExecutionPlan;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryBasisObservationPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    capability: AdmittedBasisCapability<ObservationLaneWitness>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryProjectionConsumptionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    admitted: AdmittedProjectionConsumption,
    warnings: Option<ProjectionConsumptionWarnings>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryAdmittedIntentPlan {
    Authoritative(WorthQueryAuthoritativeIntentExecutionPlan),
    EffectTriggered(WorthQueryEffectTriggeredIntentExecutionPlan),
    AuthoritativeMutation(WorthQueryAuthoritativeMutationExecutionPlan),
    AuthoritativeMutationBatch(WorthQueryAuthoritativeMutationBatchExecutionPlan),
    ReadExecution(WorthQueryReadExecutionPlan),
    LiveReadExecution(WorthQueryLiveReadExecutionPlan),
    DerivedMaterialization(WorthQueryDerivedMaterializationExecutionPlan),
    DerivedInspection(WorthQueryDerivedInspectionExecutionPlan),
    UnifiedInspection(WorthQueryUnifiedInspectionExecutionPlan),
    ExistingTruthProbeRouting(WorthQueryExistingTruthProbeExecutionPlan),
    BasisObservation(WorthQueryBasisObservationPlan),
    ProjectionConsumption(WorthQueryProjectionConsumptionPlan),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WorthQueryAdmittedIntentPlanCore {
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: Option<WorthQueryIntentAdmissionExecutionSeam>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    intent_name: String,
    declaration: Option<WorthQueryIntentDeclaration>,
    decision_digest: String,
}

impl WorthQueryAuthoritativeIntentExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
        }
    }

    pub(crate) fn core(&self) -> &WorthQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        self.inner.runtime_declaration()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl WorthQueryEffectTriggeredIntentExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
        }
    }

    pub(crate) fn core(&self) -> &WorthQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        self.inner.runtime_declaration()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl WorthQueryBasisObservationPlan {
    pub(crate) fn from_eligibility(eligibility: WorthQueryIntentAdmissionEligibility) -> Self {
        let normalized = eligibility
            .request()
            .basis_observation()
            .expect("basis observation plan requires basis payload")
            .clone();
        let admitted = admit_basis_capability(
            crate::basis_lifecycle::evaluate_basis_observation_eligibility(normalized)
                .expect("basis observation eligibility should admit before plan construction"),
        );
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(eligibility, None),
            capability: admitted,
        }
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn scope(self) -> ScopedObservationBasis {
        scope_basis_for_observation(self.capability)
    }
}

impl WorthQueryProjectionConsumptionPlan {
    pub(crate) fn from_eligibility(eligibility: WorthQueryIntentAdmissionEligibility) -> Self {
        let declaration = eligibility
            .request()
            .projection_consumption_declaration()
            .expect("projection plan requires projection declaration");
        let (admitted, warnings) =
            match crate::projection_consumption::evaluate_projection_consumption_eligibility(
                declaration,
            ) {
                ProjectionConsumptionEligibility::Admitted(admitted) => (admitted, None),
                ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, warnings) => {
                    (admitted, Some(warnings))
                }
                other => panic!(
                    "projection plan requires admitted projection eligibility, got {other:?}"
                ),
            };
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(eligibility, None),
            admitted,
            warnings,
        }
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn bind_contract(&self) -> MaterializedProjectionContract {
        self.admitted.bind_contract()
    }

    pub fn warning_kinds(&self) -> Option<&ProjectionConsumptionWarnings> {
        self.warnings.as_ref()
    }
}

impl WorthQueryAdmittedIntentPlan {
    pub(crate) fn from_eligibility(eligibility: WorthQueryIntentAdmissionEligibility) -> Self {
        match eligibility.request().family() {
            WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent => Self::Authoritative(
                WorthQueryAuthoritativeIntentExecutionPlan::from_eligibility(
                    eligibility,
                    WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
                ),
            ),
            WorthQueryIntentAdmissionFamily::EffectTriggeredWriteIntent => Self::EffectTriggered(
                WorthQueryEffectTriggeredIntentExecutionPlan::from_eligibility(
                    eligibility,
                    WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
                ),
            ),
            WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent => {
                match eligibility.request().entrypoint() {
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite => {
                        Self::AuthoritativeMutation(
                            WorthQueryAuthoritativeMutationExecutionPlan::from_eligibility(
                                eligibility,
                                WorthQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute,
                            ),
                        )
                    }
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite => {
                        Self::AuthoritativeMutationBatch(
                            WorthQueryAuthoritativeMutationBatchExecutionPlan::from_eligibility(
                                eligibility,
                                WorthQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute,
                            ),
                        )
                    }
                    other => panic!(
                        "authoritative mutation family cannot mint admitted plan for {other:?}"
                    ),
                }
            }
            WorthQueryIntentAdmissionFamily::ReadExecutionIntent => {
                match eligibility.request().entrypoint() {
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead => {
                        Self::LiveReadExecution(WorthQueryLiveReadExecutionPlan::from_eligibility(
                            eligibility,
                            WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
                        ))
                    }
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
                    | WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext => {
                        Self::ReadExecution(WorthQueryReadExecutionPlan::from_eligibility(
                            eligibility,
                            WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
                        ))
                    }
                    other => {
                        panic!("read execution family cannot mint admitted plan for {other:?}")
                    }
                }
            }
            WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent => {
                match eligibility.request().entrypoint() {
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization => {
                        Self::DerivedMaterialization(
                            WorthQueryDerivedMaterializationExecutionPlan::from_eligibility(
                                eligibility,
                                WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
                            ),
                        )
                    }
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection => {
                        Self::DerivedInspection(
                            WorthQueryDerivedInspectionExecutionPlan::from_eligibility(
                                eligibility,
                                WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
                            ),
                        )
                    }
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection => {
                        Self::UnifiedInspection(
                            WorthQueryUnifiedInspectionExecutionPlan::from_eligibility(
                                eligibility,
                                WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
                            ),
                        )
                    }
                    other => panic!(
                        "inspection-materialization family cannot mint admitted plan for {other:?}"
                    ),
                }
            }
            WorthQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent => {
                Self::ExistingTruthProbeRouting(
                    WorthQueryExistingTruthProbeExecutionPlan::from_eligibility(
                        eligibility,
                        WorthQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute,
                    ),
                )
            }
            WorthQueryIntentAdmissionFamily::BasisUseIntent => Self::BasisObservation(
                WorthQueryBasisObservationPlan::from_eligibility(eligibility),
            ),
            WorthQueryIntentAdmissionFamily::ProjectionConsumptionIntent => {
                Self::ProjectionConsumption(WorthQueryProjectionConsumptionPlan::from_eligibility(
                    eligibility,
                ))
            }
        }
    }

    fn core(&self) -> &WorthQueryAdmittedIntentPlanCore {
        match self {
            Self::Authoritative(plan) => plan.core(),
            Self::EffectTriggered(plan) => plan.core(),
            Self::AuthoritativeMutation(plan) => plan.core(),
            Self::AuthoritativeMutationBatch(plan) => plan.core(),
            Self::ReadExecution(plan) => plan.core(),
            Self::LiveReadExecution(plan) => plan.core(),
            Self::DerivedMaterialization(plan) => &plan.inner,
            Self::DerivedInspection(plan) => &plan.inner,
            Self::UnifiedInspection(plan) => &plan.inner,
            Self::ExistingTruthProbeRouting(plan) => plan.core(),
            Self::BasisObservation(plan) => &plan.inner,
            Self::ProjectionConsumption(plan) => &plan.inner,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.core().family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.core().entrypoint
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.core().execution_seam
    }

    pub fn request_digest(&self) -> &str {
        &self.core().request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.core().eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.core().eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.core().decision_digest
    }
}

impl WorthQueryAdmittedIntentPlanCore {
    fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: Option<WorthQueryIntentAdmissionExecutionSeam>,
    ) -> Self {
        let decision_digest = hash_parts(&[
            "worth_query_intent_admission_decision_v1".to_string(),
            format!("family:{}", eligibility.request().family().as_str()),
            format!("entrypoint:{}", eligibility.request().entrypoint().as_str()),
            format!("request:{}", eligibility.request().request_digest()),
            format!("eligibility:{}", eligibility.eligibility_digest()),
            "decision:admitted".to_string(),
        ]);
        Self {
            family: eligibility.request().family(),
            entrypoint: eligibility.request().entrypoint(),
            execution_seam,
            request_digest: eligibility.request().request_digest().to_string(),
            eligibility_digest: eligibility.eligibility_digest().to_string(),
            eligibility_trace: eligibility.trace_evidence(),
            intent_name: eligibility.request().intent_name().to_string(),
            declaration: eligibility.request().runtime_declaration().cloned(),
            decision_digest,
        }
    }

    fn runtime_declaration(&self) -> &WorthQueryIntentDeclaration {
        self.declaration.as_ref().unwrap_or_else(|| {
            panic!(
                "runtime declaration requested from non-runtime admitted plan: {}",
                self.intent_name
            )
        })
    }
}
