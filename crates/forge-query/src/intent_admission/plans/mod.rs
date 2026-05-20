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
use crate::runtime::ForgeQueryIntentDeclaration;

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeIntentExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryEffectTriggeredIntentExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
}

pub use inspection::{
    ForgeQueryDerivedInspectionExecutionPlan, ForgeQueryDerivedMaterializationExecutionPlan,
};
pub use mutation::{
    ForgeQueryAuthoritativeMutationBatchExecutionPlan, ForgeQueryAuthoritativeMutationExecutionPlan,
};
pub use read::{ForgeQueryLiveReadExecutionPlan, ForgeQueryReadExecutionPlan};
pub use routing::ForgeQueryExistingTruthProbeExecutionPlan;
pub use unified_inspection::ForgeQueryUnifiedInspectionExecutionPlan;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryBasisObservationPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    capability: AdmittedBasisCapability<ObservationLaneWitness>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryProjectionConsumptionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    admitted: AdmittedProjectionConsumption,
    warnings: Option<ProjectionConsumptionWarnings>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryAdmittedIntentPlan {
    Authoritative(ForgeQueryAuthoritativeIntentExecutionPlan),
    EffectTriggered(ForgeQueryEffectTriggeredIntentExecutionPlan),
    AuthoritativeMutation(ForgeQueryAuthoritativeMutationExecutionPlan),
    AuthoritativeMutationBatch(ForgeQueryAuthoritativeMutationBatchExecutionPlan),
    ReadExecution(ForgeQueryReadExecutionPlan),
    LiveReadExecution(ForgeQueryLiveReadExecutionPlan),
    DerivedMaterialization(ForgeQueryDerivedMaterializationExecutionPlan),
    DerivedInspection(ForgeQueryDerivedInspectionExecutionPlan),
    UnifiedInspection(ForgeQueryUnifiedInspectionExecutionPlan),
    ExistingTruthProbeRouting(ForgeQueryExistingTruthProbeExecutionPlan),
    BasisObservation(ForgeQueryBasisObservationPlan),
    ProjectionConsumption(ForgeQueryProjectionConsumptionPlan),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ForgeQueryAdmittedIntentPlanCore {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: Option<ForgeQueryIntentAdmissionExecutionSeam>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    intent_name: String,
    declaration: Option<ForgeQueryIntentDeclaration>,
    decision_digest: String,
}

impl ForgeQueryAuthoritativeIntentExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        self.inner.runtime_declaration()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl ForgeQueryEffectTriggeredIntentExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        self.inner.runtime_declaration()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl ForgeQueryBasisObservationPlan {
    pub(crate) fn from_eligibility(eligibility: ForgeQueryIntentAdmissionEligibility) -> Self {
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
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(eligibility, None),
            capability: admitted,
        }
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn scope(self) -> ScopedObservationBasis {
        scope_basis_for_observation(self.capability)
    }
}

impl ForgeQueryProjectionConsumptionPlan {
    pub(crate) fn from_eligibility(eligibility: ForgeQueryIntentAdmissionEligibility) -> Self {
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
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(eligibility, None),
            admitted,
            warnings,
        }
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
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

impl ForgeQueryAdmittedIntentPlan {
    pub(crate) fn from_eligibility(eligibility: ForgeQueryIntentAdmissionEligibility) -> Self {
        match eligibility.request().family() {
            ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent => Self::Authoritative(
                ForgeQueryAuthoritativeIntentExecutionPlan::from_eligibility(
                    eligibility,
                    ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
                ),
            ),
            ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent => Self::EffectTriggered(
                ForgeQueryEffectTriggeredIntentExecutionPlan::from_eligibility(
                    eligibility,
                    ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
                ),
            ),
            ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent => {
                match eligibility.request().entrypoint() {
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite => {
                        Self::AuthoritativeMutation(
                            ForgeQueryAuthoritativeMutationExecutionPlan::from_eligibility(
                                eligibility,
                                ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute,
                            ),
                        )
                    }
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite => {
                        Self::AuthoritativeMutationBatch(
                            ForgeQueryAuthoritativeMutationBatchExecutionPlan::from_eligibility(
                                eligibility,
                                ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute,
                            ),
                        )
                    }
                    other => panic!(
                        "authoritative mutation family cannot mint admitted plan for {other:?}"
                    ),
                }
            }
            ForgeQueryIntentAdmissionFamily::ReadExecutionIntent => {
                match eligibility.request().entrypoint() {
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead => {
                        Self::LiveReadExecution(ForgeQueryLiveReadExecutionPlan::from_eligibility(
                            eligibility,
                            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
                        ))
                    }
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
                    | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext => {
                        Self::ReadExecution(ForgeQueryReadExecutionPlan::from_eligibility(
                            eligibility,
                            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
                        ))
                    }
                    other => {
                        panic!("read execution family cannot mint admitted plan for {other:?}")
                    }
                }
            }
            ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent => {
                match eligibility.request().entrypoint() {
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization => {
                        Self::DerivedMaterialization(
                            ForgeQueryDerivedMaterializationExecutionPlan::from_eligibility(
                                eligibility,
                                ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
                            ),
                        )
                    }
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection => {
                        Self::DerivedInspection(
                            ForgeQueryDerivedInspectionExecutionPlan::from_eligibility(
                                eligibility,
                                ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
                            ),
                        )
                    }
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection => {
                        Self::UnifiedInspection(
                            ForgeQueryUnifiedInspectionExecutionPlan::from_eligibility(
                                eligibility,
                                ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
                            ),
                        )
                    }
                    other => panic!(
                        "inspection-materialization family cannot mint admitted plan for {other:?}"
                    ),
                }
            }
            ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent => {
                Self::ExistingTruthProbeRouting(
                    ForgeQueryExistingTruthProbeExecutionPlan::from_eligibility(
                        eligibility,
                        ForgeQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute,
                    ),
                )
            }
            ForgeQueryIntentAdmissionFamily::BasisUseIntent => Self::BasisObservation(
                ForgeQueryBasisObservationPlan::from_eligibility(eligibility),
            ),
            ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent => {
                Self::ProjectionConsumption(ForgeQueryProjectionConsumptionPlan::from_eligibility(
                    eligibility,
                ))
            }
        }
    }

    fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.core().family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.core().entrypoint
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.core().execution_seam
    }

    pub fn request_digest(&self) -> &str {
        &self.core().request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.core().eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.core().eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.core().decision_digest
    }
}

impl ForgeQueryAdmittedIntentPlanCore {
    fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: Option<ForgeQueryIntentAdmissionExecutionSeam>,
    ) -> Self {
        let decision_digest = hash_parts(&[
            "forge_query_intent_admission_decision_v1".to_string(),
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

    fn runtime_declaration(&self) -> &ForgeQueryIntentDeclaration {
        self.declaration.as_ref().unwrap_or_else(|| {
            panic!(
                "runtime declaration requested from non-runtime admitted plan: {}",
                self.intent_name
            )
        })
    }
}
