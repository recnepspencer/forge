use std::sync::Arc;

use worth_store_buffer_pool::ForegroundWriteAllocationGrant;

use super::RecordPublicationDirector;
use crate::physical_runtime::durability::{
    PhysicalRootPublicationPreparationFailure as RootPublicationPreparationFailure,
    RootCandidateSynchronizationFailure,
};
use crate::physical_runtime::record_serving::{
    publication::{write_root_candidate_artifacts, RootCandidateWriteFailure},
    residency::publication_artifacts::PublicationRecordArtifacts,
    RecordAppendDenial, RecordAppendError,
};
use crate::physical_runtime::{
    PhysicalRootPublicationWorkAction, PhysicalRootPublicationWorkScope,
    PhysicalWorkSettlementEvidence, RootPublicationCandidatePlan,
    RootPublicationPreparedPhysicalMutationMembers,
};

impl RecordPublicationDirector {
    #[cfg_attr(not(feature = "certification-test-authority"), allow(dead_code))]
    pub(super) fn continue_root_publication_candidate(
        &self,
        candidate: RootPublicationCandidatePlan,
    ) -> Result<RootPublicationPreparedPhysicalMutationMembers, RootPublicationPreparationFailure>
    {
        let Some(runtime) = self.runtime.upgrade() else {
            return Err(
                RootPublicationPreparationFailure::CandidateAuthorityReleased { candidate },
            );
        };
        let allocation = match self
            .residency
            .begin_foreground_write_operation(candidate.allocation_bytes())
        {
            Ok(allocation) => allocation,
            Err(denial) => {
                return Err(RootPublicationPreparationFailure::CandidateAdmission {
                    candidate,
                    cause: RecordAppendError::Denied(RecordAppendDenial::from_residency(denial)),
                })
            }
        };
        self.prepare_root_publication_candidate(runtime, candidate, allocation)
    }

    pub(super) fn prepare_root_publication_candidate(
        &self,
        runtime: Arc<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
        candidate: RootPublicationCandidatePlan,
        allocation: ForegroundWriteAllocationGrant,
    ) -> Result<RootPublicationPreparedPhysicalMutationMembers, RootPublicationPreparationFailure>
    {
        let declaration = match candidate.frame_set() {
            Ok(declaration) => declaration,
            Err(denial) => {
                return Err(RootPublicationPreparationFailure::CandidateAdmission {
                    candidate,
                    cause: RecordAppendError::Denied(denial),
                })
            }
        };
        let mut residency = match self
            .residency
            .begin_candidate_publication(&allocation, declaration)
        {
            Ok(residency) => residency,
            Err(denial) => {
                return Err(RootPublicationPreparationFailure::CandidateAdmission {
                    candidate,
                    cause: RecordAppendError::Denied(denial),
                })
            }
        };
        let (mut candidate_basis, plan) = candidate.into_write_parts();
        candidate_basis.mark_effect_started();
        let artifacts = PublicationRecordArtifacts::new(&self.mutation);
        let written = match write_root_candidate_artifacts(&artifacts, plan, &mut residency) {
            Ok(written) => written,
            Err(RootCandidateWriteFailure::RetryableNoEffect {
                plan,
                failed_artifact,
                cause,
            }) => {
                let candidate = candidate_basis.restore_proven_no_effect(plan);
                return Err(
                    RootPublicationPreparationFailure::CandidateWriteNotStarted {
                        candidate,
                        failed_artifact,
                        cause,
                    },
                );
            }
            Err(RootCandidateWriteFailure::InspectionRequired {
                plan,
                completed_artifacts,
                failed_artifact,
                cause,
            }) => {
                candidate_basis.require_inspection();
                runtime.health.revoke();
                let candidate = candidate_basis.restore_inspection_required(plan);
                return Err(RootPublicationPreparationFailure::CandidateWrite {
                    candidate,
                    completed_artifacts,
                    failed_artifact,
                    cause,
                });
            }
        };
        let mut candidate = candidate_basis.complete_write(written.plan, written.artifacts);
        if let Err(violation) = residency.require_complete() {
            candidate.require_inspection();
            runtime.health.revoke();
            return Err(
                RootPublicationPreparationFailure::CandidateFrameSetIncomplete {
                    candidate,
                    violation,
                },
            );
        }
        self.synchronize_root_publication_candidate(runtime, candidate)
    }

    fn synchronize_root_publication_candidate(
        &self,
        runtime: Arc<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
        mut candidate: crate::physical_runtime::WrittenRootPublicationCandidate,
    ) -> Result<RootPublicationPreparedPhysicalMutationMembers, RootPublicationPreparationFailure>
    {
        let mut synchronized = Vec::with_capacity(candidate.candidate().artifacts().len());
        for index in 0..candidate.candidate().artifacts().len() {
            let artifact = candidate.candidate().artifacts()[index];
            let scope = PhysicalRootPublicationWorkScope::new(
                candidate.identity(),
                PhysicalRootPublicationWorkAction::SynchronizeCandidateArtifact { artifact },
            )
            .expect("a root candidate manifest never targets the bootstrap catalog");
            let settlement = match self.root_work.execute(scope) {
                Ok(settlement) => settlement,
                Err(cause) => {
                    candidate.require_inspection();
                    runtime.health.revoke();
                    return Err(
                        RootPublicationPreparationFailure::CandidateSynchronization {
                            candidate,
                            completed: synchronized.into_boxed_slice(),
                            artifact,
                            cause: RootCandidateSynchronizationFailure::Work(cause),
                        },
                    );
                }
            };
            if !matches!(
                settlement.evidence(),
                PhysicalWorkSettlementEvidence::PublicationEffect { .. }
            ) || settlement.evidence().fate()
                != crate::physical_runtime::PhysicalWorkEffectFate::PublicationCompleted
            {
                let fate = settlement.evidence().fate();
                let recovery = settlement.recovery_disposition();
                candidate.require_inspection();
                runtime.health.revoke();
                return Err(
                    RootPublicationPreparationFailure::CandidateSynchronization {
                        candidate,
                        completed: synchronized.into_boxed_slice(),
                        artifact,
                        cause: RootCandidateSynchronizationFailure::Settlement { fate, recovery },
                    },
                );
            }
            synchronized.push(settlement);
        }
        let (group, members, candidate, transition) = candidate.into_parts();
        Ok(RootPublicationPreparedPhysicalMutationMembers::new(
            transition.identity(),
            group,
            members,
            candidate,
            synchronized.into_boxed_slice(),
            transition,
        ))
    }
}
