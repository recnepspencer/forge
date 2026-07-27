use worth_proof::TransitionOutcome;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use crate::physical_runtime::{
    PhysicalMutationWorkRequest, PhysicalWorkAdmission, PhysicalWorkReadiness, PhysicalWorkScope,
};
#[cfg(feature = "certification-test-authority")]
use crate::physical_runtime::{PhysicalWorkDurabilityRequirement, ReadyPhysicalWork};

use super::{
    super::{
        failure::{PhysicalWritebackFailureCause, PhysicalWritebackTransitionFailure},
        AdmittedDirtyFrame,
    },
    FrameWritebackPort, PreparedPhysicalWriteback, ReadyPhysicalWriteback,
};

impl FrameWritebackPort {
    pub(in crate::physical_runtime::record_serving) fn prepare(
        &self,
        dirty: AdmittedDirtyFrame,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> Result<PreparedPhysicalWriteback, PhysicalWritebackTransitionFailure> {
        self.frame_ports.observe_writeback_attempt();
        let claim = match self.frame_ports.claim_writeback(dirty.coordinate()) {
            Ok(claim) => claim,
            Err(denial) => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::Residency(denial),
                    dirty,
                ))
            }
        };
        let request = match PhysicalMutationWorkRequest::exact_write(
            PhysicalWorkScope::one(dirty.coordinate()),
            self.record.frame_writeback_basis(),
            self.record.security(),
            durability,
        ) {
            Ok(request) => request,
            Err(_) => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::SubmissionRejected,
                    dirty,
                ))
            }
        };
        let receipt = match self.submission.submit(request).into_raw() {
            TransitionOutcome::Success(receipt) => receipt,
            _ => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::SubmissionRejected,
                    dirty,
                ))
            }
        };
        Ok(PreparedPhysicalWriteback {
            receipt,
            claim,
            dirty,
            durability,
        })
    }

    pub(in crate::physical_runtime::record_serving) fn request_ready(
        &self,
        prepared: PreparedPhysicalWriteback,
    ) -> Result<ReadyPhysicalWriteback, PhysicalWritebackTransitionFailure> {
        let PreparedPhysicalWriteback {
            receipt,
            claim,
            dirty,
            durability,
        } = prepared;
        let Some(runtime) = self.runtime.upgrade() else {
            return Err(PhysicalWritebackTransitionFailure::new(
                PhysicalWritebackFailureCause::RuntimeReleased,
                dirty,
            ));
        };
        let admitted = match PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.physical,
            &runtime.health,
        ) {
            Ok(admitted) => admitted,
            Err(denial) => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::PreEffect(denial),
                    dirty,
                ))
            }
        };
        let readiness = match runtime.signal.request(admitted) {
            Ok(readiness) => readiness,
            Err(denial) => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::PreEffect(denial),
                    dirty,
                ))
            }
        };
        match readiness {
            PhysicalWorkReadiness::Ready(ready) => Ok(ReadyPhysicalWriteback {
                ready,
                claim,
                dirty,
                durability,
            }),
            PhysicalWorkReadiness::Blocked(_) => Err(PhysicalWritebackTransitionFailure::new(
                PhysicalWritebackFailureCause::DependencyBlocked,
                dirty,
            )),
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn bind_retry_ready(
        &self,
        ready: ReadyPhysicalWork,
        dirty: AdmittedDirtyFrame,
    ) -> Result<ReadyPhysicalWriteback, PhysicalWritebackTransitionFailure> {
        let [coordinate] = ready.intent().scope().coordinates() else {
            return Err(PhysicalWritebackTransitionFailure::new(
                PhysicalWritebackFailureCause::SubmissionRejected,
                dirty,
            ));
        };
        if *coordinate != dirty.coordinate() {
            return Err(PhysicalWritebackTransitionFailure::new(
                PhysicalWritebackFailureCause::SubmissionRejected,
                dirty,
            ));
        }
        let PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(durability) =
            ready.intent().durability()
        else {
            return Err(PhysicalWritebackTransitionFailure::new(
                PhysicalWritebackFailureCause::SubmissionRejected,
                dirty,
            ));
        };
        self.frame_ports.observe_writeback_attempt();
        let claim = match self.frame_ports.claim_writeback(dirty.coordinate()) {
            Ok(claim) => claim,
            Err(denial) => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::Residency(denial),
                    dirty,
                ))
            }
        };
        Ok(ReadyPhysicalWriteback {
            ready,
            claim,
            dirty,
            durability,
        })
    }
}
