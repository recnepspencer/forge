use super::{
    EligiblePhysicalWalSegmentReclamation, PhysicalWalReclamationActionFailure,
    PhysicalWalReclamationObservation, PhysicalWalReclamationPlan, PhysicalWalReclamationReport,
    PhysicalWalReclamationWorkPort,
};

pub(in crate::physical_runtime) struct PhysicalWalReclamationFoundation {
    wal: super::super::PhysicalWalRuntimeOwner,
    work: PhysicalWalReclamationWorkPort,
}

#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalWalReclamationOwner {
    wal: super::super::PhysicalWalRuntimeOwner,
    work: PhysicalWalReclamationWorkPort,
}

impl PhysicalWalReclamationOwner {
    pub(in crate::physical_runtime) fn new(foundation: PhysicalWalReclamationFoundation) -> Self {
        Self {
            wal: foundation.wal,
            work: foundation.work,
        }
    }

    pub(in crate::physical_runtime::durability) fn execute(
        &self,
        plan: PhysicalWalReclamationPlan,
    ) -> PhysicalWalReclamationObservation {
        match plan {
            PhysicalWalReclamationPlan::NotRequired { checkpoint } => {
                PhysicalWalReclamationObservation::NotRequired { checkpoint }
            }
            PhysicalWalReclamationPlan::Required(eligible) => self.execute_required(eligible),
        }
    }

    pub(in crate::physical_runtime::durability) fn eligibility_denied(
        &self,
        checkpoint: worth_store_physical_format::PhysicalCheckpointIdentity,
    ) -> PhysicalWalReclamationObservation {
        self.wal.seal_for_inspection();
        PhysicalWalReclamationObservation::InspectionRequired(PhysicalWalReclamationReport::new(
            checkpoint, 0, 0, 0, None,
        ))
    }

    fn execute_required(
        &self,
        eligible: super::EligiblePhysicalWalReclamation,
    ) -> PhysicalWalReclamationObservation {
        let checkpoint = eligible.checkpoint();
        let segments = eligible.into_segments().into_vec();
        let planned_segments = u32::try_from(segments.len())
            .expect("the bounded WAL inventory fits the reclamation report");
        let mut reclaimed_segments = 0_u32;
        let mut reclaimed_bytes = 0_u64;
        for eligible in segments {
            let expected = eligible.segment();
            match self.execute_segment(eligible) {
                Ok(completed) if self.wal.complete_reclamation(expected, &completed) => {
                    reclaimed_segments = reclaimed_segments.saturating_add(1);
                    reclaimed_bytes = reclaimed_bytes.saturating_add(expected.byte_count());
                }
                Ok(_) => {
                    return self.inspection_report(
                        checkpoint,
                        planned_segments,
                        reclaimed_segments,
                        reclaimed_bytes,
                        expected.identity(),
                    )
                }
                Err(failure) if failure.requires_inspection() => {
                    return self.inspection_report(
                        checkpoint,
                        planned_segments,
                        reclaimed_segments,
                        reclaimed_bytes,
                        expected.identity(),
                    )
                }
                Err(_) => {
                    return PhysicalWalReclamationObservation::DeferredBeforeEffect(
                        PhysicalWalReclamationReport::new(
                            checkpoint,
                            planned_segments,
                            reclaimed_segments,
                            reclaimed_bytes,
                            Some(expected.identity()),
                        ),
                    )
                }
            }
        }
        PhysicalWalReclamationObservation::Reclaimed(PhysicalWalReclamationReport::new(
            checkpoint,
            planned_segments,
            reclaimed_segments,
            reclaimed_bytes,
            None,
        ))
    }

    fn execute_segment(
        &self,
        eligible: EligiblePhysicalWalSegmentReclamation,
    ) -> Result<
        crate::physical_runtime::CompletedPhysicalWalReclamationAction,
        PhysicalWalReclamationActionFailure,
    > {
        let segment = eligible.segment();
        let scope = crate::physical_runtime::work::PhysicalWalReclamationScope::new(
            eligible.checkpoint(),
            eligible.compaction_generation(),
            eligible.compaction_digest(),
            eligible.retained_boundary(),
            segment.identity(),
            segment.lsn_range(),
            segment.byte_count(),
        )
        .expect("eligibility proves a bounded obsolete WAL segment");
        self.work.execute(scope, 0)
    }

    fn inspection_report(
        &self,
        checkpoint: worth_store_physical_format::PhysicalCheckpointIdentity,
        planned_segments: u32,
        reclaimed_segments: u32,
        reclaimed_bytes: u64,
        first_unreclaimed: worth_store_wal::WalSegmentArtifactIdentity,
    ) -> PhysicalWalReclamationObservation {
        self.wal.seal_for_inspection();
        PhysicalWalReclamationObservation::InspectionRequired(PhysicalWalReclamationReport::new(
            checkpoint,
            planned_segments,
            reclaimed_segments,
            reclaimed_bytes,
            Some(first_unreclaimed),
        ))
    }
}

impl PhysicalWalReclamationFoundation {
    pub(in crate::physical_runtime) fn new(
        runtime: &std::sync::Arc<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
        generation: crate::physical_runtime::LifecycleGeneration,
        physical: crate::physical_runtime::work::PhysicalWorkAdmissionAuthority,
        scheduler: crate::physical_runtime::instance::PhysicalSchedulerAdmissionOwner,
        record: std::sync::Arc<crate::physical_runtime::record_serving::RecordWorkAdmission>,
        wal: super::super::PhysicalWalRuntimeOwner,
    ) -> Self {
        Self {
            wal,
            work: PhysicalWalReclamationWorkPort::new(
                runtime, generation, physical, scheduler, record,
            ),
        }
    }
}

impl PhysicalWalReclamationActionFailure {
    const fn requires_inspection(&self) -> bool {
        matches!(
            self,
            Self::EffectRequiresInspection | Self::StaleOrForeignSettlement
        )
    }
}
