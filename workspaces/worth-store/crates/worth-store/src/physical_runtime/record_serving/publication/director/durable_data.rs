use std::num::NonZeroU64;

use super::RecordPublicationDirector;
use crate::physical_runtime::{
    durability::{
        DataDispatchedPhysicalMutation, IndeterminatePhysicalDataDispatch,
        PhysicalDataDispatchFailureCause, PhysicalDataDispatchOutcome,
        PhysicalDataEffectSettlement, PhysicalDataFrameKind,
    },
    record_serving::{
        residency::{
            candidate_frame_residency::{
                CandidateFrame, CandidateFrameCoordinate, CandidateFrameDeclaration,
                CandidateFrameRole, CandidateFrameSet, CandidateFrameWriteFailure,
            },
            publication_artifacts::PublicationRecordArtifacts,
        },
        RecordAppendDenial, RecordPublicationStage, RecordPublicationWorkTrace,
    },
    PhysicalWorkEffectFate, WalDurablePhysicalMutation,
};

impl RecordPublicationDirector {
    pub(super) fn dispatch_wal_durable_data(
        &self,
        durable: WalDurablePhysicalMutation,
    ) -> PhysicalDataDispatchOutcome {
        if let Some(cause) = self.dispatch_admission_failure(&durable) {
            return PhysicalDataDispatchOutcome::NotStarted { durable, cause };
        }
        let declaration =
            match candidate_declaration(&durable, self.current_root().generation()) {
                Some(declaration) => declaration,
                None => return PhysicalDataDispatchOutcome::NotStarted {
                    durable,
                    cause: PhysicalDataDispatchFailureCause::CandidateFrameContract(
                        crate::physical_runtime::CandidateFrameContractViolation::UnexpectedFrame,
                    ),
                },
            };
        let bytes = NonZeroU64::new(declaration.total_frame_bytes())
            .expect("a WAL-bound data plan has nonempty frames");
        let allocation = match self.residency.begin_foreground_write_operation(bytes) {
            Ok(allocation) => allocation,
            Err(denial) => {
                return PhysicalDataDispatchOutcome::NotStarted {
                    durable,
                    cause: PhysicalDataDispatchFailureCause::Residency(
                        RecordAppendDenial::from_residency(denial),
                    ),
                }
            }
        };
        let mut residency = match self
            .residency
            .begin_candidate_publication(&allocation, declaration)
        {
            Ok(residency) => residency,
            Err(denial) => {
                return PhysicalDataDispatchOutcome::NotStarted {
                    durable,
                    cause: PhysicalDataDispatchFailureCause::Residency(denial),
                }
            }
        };
        let artifacts = PublicationRecordArtifacts::new(&self.mutation);
        let mut work = RecordPublicationWorkTrace::default();
        let mut effects = Vec::with_capacity(durable.data_frames().len());
        for frame in durable.data_frames() {
            let basis = frame.basis().clone();
            let target = basis.target();
            let coordinate = target.coordinate();
            let candidate = CandidateFrame::new(
                candidate_role(target.kind()),
                CandidateFrameCoordinate::new(coordinate.artifact(), coordinate.offset()),
                frame.bytes().to_vec(),
            );
            let completion = {
                let mut stage = artifacts.at(RecordPublicationStage::CandidateDataWrite, &mut work);
                if coordinate.offset() == 0 {
                    stage
                        .write_new_candidate(&mut residency, candidate, coordinate.artifact())
                        .map_err(map_canonical_failure)
                } else {
                    stage
                        .write_existing_artifact_candidate(
                            &mut residency,
                            candidate,
                            self.residency.writeback(),
                        )
                        .map_err(map_writeback_failure)
                }
            };
            let completion = match completion {
                Ok(completion) => completion,
                Err(failure) => {
                    return classify_dispatch_failure(durable, effects, failure);
                }
            };
            let Some(effect) = completion.effect() else {
                return PhysicalDataDispatchOutcome::Indeterminate(
                    IndeterminatePhysicalDataDispatch::new(
                        durable,
                        effects,
                        PhysicalDataDispatchFailureCause::MissingEffectSettlement,
                    ),
                );
            };
            effects.push(PhysicalDataEffectSettlement::from_candidate(basis, effect));
        }
        if let Err(violation) = residency.require_complete() {
            return PhysicalDataDispatchOutcome::Indeterminate(
                IndeterminatePhysicalDataDispatch::new(
                    durable,
                    effects,
                    PhysicalDataDispatchFailureCause::CandidateFrameContract(violation),
                ),
            );
        }
        PhysicalDataDispatchOutcome::Dispatched(DataDispatchedPhysicalMutation::new(
            durable, effects,
        ))
    }

    fn dispatch_admission_failure(
        &self,
        durable: &WalDurablePhysicalMutation,
    ) -> Option<PhysicalDataDispatchFailureCause> {
        let identity = durable.mutation_identity();
        if identity.store_identity() != self.durability.store_identity() {
            return Some(PhysicalDataDispatchFailureCause::ForeignStore);
        }
        if identity.runtime_identity() != self.durability.runtime_identity() {
            return Some(PhysicalDataDispatchFailureCause::StaleRuntime);
        }
        if durable.appended().reserved().signal_profile() != self.signal_profile {
            return Some(PhysicalDataDispatchFailureCause::SignalProfileMismatch);
        }
        None
    }
}

fn candidate_declaration(
    durable: &WalDurablePhysicalMutation,
    root_generation: u64,
) -> Option<CandidateFrameSet> {
    let frames = durable
        .data_frames()
        .iter()
        .map(|frame| {
            let target = frame.basis().target();
            let coordinate = target.coordinate();
            CandidateFrameDeclaration::new(
                candidate_role(target.kind()),
                CandidateFrameCoordinate::new(coordinate.artifact(), coordinate.offset()),
                coordinate.length(),
            )
        })
        .collect::<Option<Vec<_>>>()?;
    CandidateFrameSet::new(root_generation, frames)
}

const fn candidate_role(kind: PhysicalDataFrameKind) -> CandidateFrameRole {
    match kind {
        PhysicalDataFrameKind::InlinePage => CandidateFrameRole::InlinePage,
        PhysicalDataFrameKind::ExtentChunk => CandidateFrameRole::ExtentChunk,
    }
}

fn map_canonical_failure(
    failure: CandidateFrameWriteFailure<
        crate::physical_runtime::record_serving::CanonicalRecordMutationFailure,
    >,
) -> DispatchFailure {
    match failure {
        CandidateFrameWriteFailure::Contract(violation) => DispatchFailure::Uncertain(
            PhysicalDataDispatchFailureCause::CandidateFrameContract(violation),
        ),
        CandidateFrameWriteFailure::Residency(denial) => {
            DispatchFailure::Uncertain(PhysicalDataDispatchFailureCause::Residency(denial))
        }
        CandidateFrameWriteFailure::Effect(failure) => {
            let fate = failure.effect_fate();
            DispatchFailure::Settled {
                cause: PhysicalDataDispatchFailureCause::Canonical(failure.evidence()),
                fate,
            }
        }
    }
}

fn map_writeback_failure(
    failure: CandidateFrameWriteFailure<
        crate::physical_runtime::PhysicalRecordWritebackFailureEvidence,
    >,
) -> DispatchFailure {
    match failure {
        CandidateFrameWriteFailure::Contract(violation) => DispatchFailure::Uncertain(
            PhysicalDataDispatchFailureCause::CandidateFrameContract(violation),
        ),
        CandidateFrameWriteFailure::Residency(denial) => {
            DispatchFailure::Uncertain(PhysicalDataDispatchFailureCause::Residency(denial))
        }
        CandidateFrameWriteFailure::Effect(failure) => DispatchFailure::Settled {
            cause: PhysicalDataDispatchFailureCause::C6Writeback(failure),
            fate: failure.effect_fate(),
        },
    }
}

enum DispatchFailure {
    Settled {
        cause: PhysicalDataDispatchFailureCause,
        fate: PhysicalWorkEffectFate,
    },
    Uncertain(PhysicalDataDispatchFailureCause),
}

fn classify_dispatch_failure(
    durable: WalDurablePhysicalMutation,
    effects: Vec<PhysicalDataEffectSettlement>,
    failure: DispatchFailure,
) -> PhysicalDataDispatchOutcome {
    match failure {
        DispatchFailure::Settled {
            cause,
            fate: PhysicalWorkEffectFate::ProvenNoEffect,
        } if effects.is_empty() => PhysicalDataDispatchOutcome::NotStarted { durable, cause },
        DispatchFailure::Settled { cause, .. } | DispatchFailure::Uncertain(cause) => {
            PhysicalDataDispatchOutcome::Indeterminate(IndeterminatePhysicalDataDispatch::new(
                durable, effects, cause,
            ))
        }
    }
}
