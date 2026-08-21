use worth_foundational::{
    profiles, request_foundational_profile_set, FoundationalMaterializationCost,
    FoundationalProfileMaterializationPlan, FoundationalProfileSet,
    FoundationalSurfaceAvailabilityDecision, SupportArtifactTarget, SupportProfiledArtifact,
};
use worth_proof::TransitionOutcome;

use super::{OperationalAuditRecord, OperationalAuditTransitionKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalAuditSupportPayload {
    operation_id: String,
    transition_id: String,
    sequence: u64,
    transition_kind: OperationalAuditTransitionKind,
    source_artifact_identity: [u8; 32],
    record_identity: [u8; 32],
}

#[derive(Debug)]
pub struct RequestedOperationalAuditSupport {
    requested: worth_foundational::RequestedFoundationalProfileArtifact,
    payload: OperationalAuditSupportPayload,
}

#[derive(Debug)]
pub struct OperationalAuditSupportMaterializationPlan {
    artifact: SupportProfiledArtifact<OperationalAuditSupportPayload>,
    plan: FoundationalProfileMaterializationPlan<SupportArtifactTarget>,
}

#[derive(Debug)]
pub struct MaterializedOperationalAuditSupport {
    artifact: SupportProfiledArtifact<OperationalAuditSupportPayload>,
    plan: FoundationalProfileMaterializationPlan<SupportArtifactTarget>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalAuditSupportDenial {
    ProfileAdmissionDenied,
    ProfileAdmissionDeferred,
    ProfileAdmissionStale,
    ProfileAdmissionRequiresRebind,
    ProfileAdmissionFailed,
    SupportAttachmentDenied,
    SupportAttachmentDeferred,
    SupportAttachmentStale,
    SupportAttachmentRequiresRebind,
    SupportAttachmentFailed,
    MaterializationPlanningDenied,
}

impl OperationalAuditRecord {
    pub fn request_support_projection(
        &self,
        requested_profile: FoundationalProfileSet,
    ) -> RequestedOperationalAuditSupport {
        RequestedOperationalAuditSupport {
            requested: request_foundational_profile_set(requested_profile),
            payload: OperationalAuditSupportPayload {
                operation_id: self.operation_id().as_str().to_owned(),
                transition_id: self.transition_id().as_str().to_owned(),
                sequence: self.sequence().get(),
                transition_kind: self.transition_kind(),
                source_artifact_identity: self.source_artifact_identity(),
                record_identity: self.record_identity(),
            },
        }
    }
}

impl RequestedOperationalAuditSupport {
    pub fn plan_materialization(
        self,
    ) -> Result<OperationalAuditSupportMaterializationPlan, OperationalAuditSupportDenial> {
        let admitted_profile = *self.requested.payload().requested();
        let admitted = match profiles().progression().admit_same(self.requested) {
            TransitionOutcome::Success(admitted) => admitted,
            TransitionOutcome::Denied(_) => {
                return Err(OperationalAuditSupportDenial::ProfileAdmissionDenied)
            }
            TransitionOutcome::Deferred(_) => {
                return Err(OperationalAuditSupportDenial::ProfileAdmissionDeferred)
            }
            TransitionOutcome::Stale(_) => {
                return Err(OperationalAuditSupportDenial::ProfileAdmissionStale)
            }
            TransitionOutcome::RebindRequired(_) => {
                return Err(OperationalAuditSupportDenial::ProfileAdmissionRequiresRebind)
            }
            TransitionOutcome::Failed(_) => {
                return Err(OperationalAuditSupportDenial::ProfileAdmissionFailed)
            }
        };
        let artifact = match profiles().attach().to_support_artifact(
            admitted,
            admitted_profile,
            None,
            self.payload,
        ) {
            TransitionOutcome::Success(artifact) => artifact,
            TransitionOutcome::Denied(_) => {
                return Err(OperationalAuditSupportDenial::SupportAttachmentDenied)
            }
            TransitionOutcome::Deferred(_) => {
                return Err(OperationalAuditSupportDenial::SupportAttachmentDeferred)
            }
            TransitionOutcome::Stale(_) => {
                return Err(OperationalAuditSupportDenial::SupportAttachmentStale)
            }
            TransitionOutcome::RebindRequired(_) => {
                return Err(OperationalAuditSupportDenial::SupportAttachmentRequiresRebind)
            }
            TransitionOutcome::Failed(_) => {
                return Err(OperationalAuditSupportDenial::SupportAttachmentFailed)
            }
        };
        let plan = profiles()
            .materialization()
            .for_support_artifact(&artifact)
            .full_fidelity()
            .map_err(|_| OperationalAuditSupportDenial::MaterializationPlanningDenied)?;
        Ok(OperationalAuditSupportMaterializationPlan { artifact, plan })
    }
}

impl OperationalAuditSupportMaterializationPlan {
    pub const fn cost(&self) -> FoundationalMaterializationCost {
        self.plan.cost()
    }

    pub fn availability_decisions(&self) -> &[FoundationalSurfaceAvailabilityDecision] {
        self.plan.decisions()
    }

    pub fn materialize(self) -> MaterializedOperationalAuditSupport {
        MaterializedOperationalAuditSupport {
            artifact: self.artifact,
            plan: self.plan,
        }
    }
}

impl MaterializedOperationalAuditSupport {
    pub fn payload(&self) -> &OperationalAuditSupportPayload {
        self.artifact.payload().payload()
    }

    pub fn availability_decisions(&self) -> &[FoundationalSurfaceAvailabilityDecision] {
        self.plan.decisions()
    }

    pub fn prepare_foundational_boundary_bundle(
        self,
    ) -> SupportProfiledArtifact<OperationalAuditSupportPayload> {
        self.artifact
    }
}

impl OperationalAuditSupportPayload {
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }
    pub fn transition_id(&self) -> &str {
        &self.transition_id
    }
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
    pub const fn transition_kind(&self) -> OperationalAuditTransitionKind {
        self.transition_kind
    }
    pub const fn source_artifact_identity(&self) -> [u8; 32] {
        self.source_artifact_identity
    }
    pub const fn record_identity(&self) -> [u8; 32] {
        self.record_identity
    }
}
