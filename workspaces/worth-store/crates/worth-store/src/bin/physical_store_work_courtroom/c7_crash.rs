use std::num::NonZeroU64;
use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::{
    CertificationMediaFaultActivation, CertificationPhysicalMutationCheckpoint,
    MediaFaultDirective, MediaPauseGate,
};
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, FilesystemMediaAdmission, PhysicalCheckpointDeadline,
    PhysicalCheckpointIdempotencyKey, PhysicalCheckpointOutcome, PhysicalCheckpointRequest,
    PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial, PhysicalMutationOutcome,
    PhysicalMutationPreparationSuccess, PhysicalMutationRequest, PhysicalRecordOpen,
    PreparedPhysicalMutation, RecordAppendBatch, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultSchedule, MediaOperationRole,
};

use super::arguments::{
    C7CrashInvocation, C7CrashSeamInvocation, C7DurabilityCheckpointOrderInvocation,
};

const CHECKPOINT_MARKER: &str = "C7_COURTROOM_CRASH_CHECKPOINT";

pub(super) fn run(invocation: C7CrashInvocation) -> Result<(), String> {
    let configuration =
        super::bounded_residency::configuration::BoundedResidencyConfiguration::read(
            &invocation.configuration,
        )?;
    let control = CrashControl::for_seam(invocation.seam)?;
    let (format, placement, access) = super::configuration::record_configuration();
    let policy = configuration
        .serving_policy(format)
        .into_result()
        .map_err(|denial| format!("C7 crash residency policy denied: {denial:?}"))?;
    let media = super::admission::admit_media(&invocation.root, control.schedule().cloned())?;
    let durability = super::admission::admit_durability_with_checkpoint_memory(
        &media,
        configuration.checkpoint_memory_limit(),
    )?;
    let serving = super::admission::require_serving(
        media.open_record_store(
            PhysicalRecordOpen::new(format, access, durability).with_residency_policy(policy),
        ),
        "C7 crash Store open",
    )?;
    let outcome = match invocation.checkpoint_order {
        C7DurabilityCheckpointOrderInvocation::CheckpointBeforeTarget => {
            complete_checkpoint(&serving, invocation.seam.identity_byte())?;
            let prepared = prepare_mutation(&serving, configuration, invocation.seam, placement)?;
            control.install(&serving, invocation.seam)?;
            prepared.execute()
        }
        C7DurabilityCheckpointOrderInvocation::TargetSealedBeforeCheckpoint => {
            let prepared = prepare_mutation(&serving, configuration, invocation.seam, placement)?;
            let ordering_gate = serving.certification_pause_physical_mutation_at(
                CertificationPhysicalMutationCheckpoint::AfterGroupSeal,
            );
            let handle = prepared.start();
            if !ordering_gate.await_arrival() {
                return Err("C7 target did not reach the post-group-seal schedule gate".into());
            }
            complete_checkpoint(&serving, invocation.seam.identity_byte())?;
            control.install(&serving, invocation.seam)?;
            ordering_gate.release();
            handle.wait()
        }
    };
    Err(describe_returned_outcome(invocation.seam, outcome))
}

fn describe_returned_outcome(
    seam: C7CrashSeamInvocation,
    outcome: PhysicalMutationOutcome,
) -> String {
    match outcome {
        PhysicalMutationOutcome::Completed(_) => {
            format!(
                "C7 mutation crossed crash seam {} as completed",
                seam.label()
            )
        }
        PhysicalMutationOutcome::ProvenNoEffect(fact) => format!(
            "C7 mutation crossed crash seam {} as proven no effect: {fact:?}",
            seam.label()
        ),
        PhysicalMutationOutcome::Indeterminate(fact) => format!(
            "C7 mutation crossed crash seam {} as indeterminate: {fact:?}",
            seam.label()
        ),
    }
}

fn prepare_mutation(
    serving: &ServingPhysicalRuntime,
    configuration: super::bounded_residency::configuration::BoundedResidencyConfiguration,
    seam: C7CrashSeamInvocation,
    placement: AdmittedRecordPlacementPolicy,
) -> Result<PreparedPhysicalMutation, String> {
    let ordinal = configuration.serving_append_ordinals()[0];
    let payload = vec![seam.identity_byte(); configuration.record_bytes(ordinal).unwrap()];
    let submission = serving.record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new(
            [seam.identity_byte(); 32],
        ))
        .map_err(|denial| format!("C7 crash idempotency issuance denied: {denial:?}"))?;
    match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([payload.as_slice()])
                .map_err(|denial| format!("C7 crash batch denied: {denial:?}"))?,
            placement,
            PhysicalMutationRequest::platform_durable(
                key,
                PhysicalMutationDeadline::at(
                    TemporalDuration::temporal_duration(1_000_000)
                        .expect("C7 crash deadline is nonzero"),
                ),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
            Ok(prepared)
        }
        _ => Err("C7 crash mutation preparation did not succeed".to_owned()),
    }
}

fn complete_checkpoint(serving: &ServingPhysicalRuntime, identity_byte: u8) -> Result<(), String> {
    let request = PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new([identity_byte ^ 0x2F; 32]),
        PhysicalCheckpointDeadline::at(
            TemporalDuration::temporal_duration(1_000_000)
                .expect("C7 checkpoint deadline is nonzero"),
        ),
    );
    let handle = match serving.checkpoints().start(request).into_raw() {
        TransitionOutcome::Success(handle) => handle,
        _ => return Err("C7 schedule checkpoint was not admitted".to_owned()),
    };
    match handle.wait() {
        PhysicalCheckpointOutcome::Completed(_) => Ok(()),
        other => Err(format!(
            "C7 schedule checkpoint did not complete: {other:?}"
        )),
    }
}

enum CrashControl {
    Media {
        schedule: MediaFaultSchedule,
        gate: MediaPauseGate,
        activation: CertificationMediaFaultActivation,
    },
    Mutation(CertificationPhysicalMutationCheckpoint),
}

impl CrashControl {
    fn for_seam(seam: C7CrashSeamInvocation) -> Result<Self, String> {
        let mutation_checkpoint = match seam {
            C7CrashSeamInvocation::AfterDataSettlementBeforeRootPublication => {
                Some(CertificationPhysicalMutationCheckpoint::AfterDataSettlement)
            }
            C7CrashSeamInvocation::AfterPhysicalDurabilityBeforeAcknowledgment => {
                Some(CertificationPhysicalMutationCheckpoint::BeforeTerminalFinalization)
            }
            _ => None,
        };
        if let Some(checkpoint) = mutation_checkpoint {
            return Ok(Self::Mutation(checkpoint));
        }
        let admission = FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        );
        let authority = admission.fault_schedule_authority();
        let gate = authority.pause_gate();
        let activation = authority.one_shot_activation();
        let (role, match_ordinal, directive) = seam.media_fault(gate.clone())?;
        let rule = authority
            .rule(role, 1, directive)
            .for_nth_identified_operation_after_activation(activation.clone(), match_ordinal);
        let schedule = authority
            .schedule(vec![rule])
            .map_err(|denial| format!("C7 crash media schedule denied: {denial:?}"))?;
        Ok(Self::Media {
            schedule,
            gate,
            activation,
        })
    }

    const fn schedule(&self) -> Option<&MediaFaultSchedule> {
        match self {
            Self::Media { schedule, .. } => Some(schedule),
            Self::Mutation(_) => None,
        }
    }

    fn install(
        self,
        serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
        seam: C7CrashSeamInvocation,
    ) -> Result<(), String> {
        match self {
            Self::Media {
                gate, activation, ..
            } => {
                super::checkpoint::watch_c7_media(
                    CHECKPOINT_MARKER,
                    seam.label(),
                    gate,
                    activation.clone(),
                );
                activation
                    .arm()
                    .map_err(|denial| format!("C7 crash media activation denied: {denial:?}"))
            }
            Self::Mutation(checkpoint) => {
                let gate = serving.certification_pause_physical_mutation_at(checkpoint);
                super::checkpoint::watch_mutation(CHECKPOINT_MARKER, seam.label(), gate);
                Ok(())
            }
        }
    }
}

impl C7CrashSeamInvocation {
    const fn identity_byte(self) -> u8 {
        0xD0 + self as u8
    }

    fn media_fault(
        self,
        gate: MediaPauseGate,
    ) -> Result<(MediaOperationRole, NonZeroU64, MediaFaultDirective), String> {
        let first = NonZeroU64::MIN;
        let second = NonZeroU64::new(2).expect("two is nonzero");
        let fault = match self {
            Self::BeforeWalAppend => (
                MediaOperationRole::PositionedWrite,
                first,
                MediaFaultDirective::PauseBefore(gate),
            ),
            Self::DuringWalAppendPrefix => (
                MediaOperationRole::PositionedWrite,
                first,
                MediaFaultDirective::AllowPrefixThenPause { bytes: 1, gate },
            ),
            Self::AfterWalWriteBeforeBarrier => (
                MediaOperationRole::PositionedWrite,
                first,
                MediaFaultDirective::PauseAfter(gate),
            ),
            Self::AfterWalBarrierBeforeDataDispatch => (
                MediaOperationRole::SynchronizeFileState,
                first,
                MediaFaultDirective::PauseAfter(gate),
            ),
            Self::DuringDataWritePrefix => (
                MediaOperationRole::PositionedWrite,
                second,
                MediaFaultDirective::AllowPrefixThenPause { bytes: 1, gate },
            ),
            Self::AfterRootReplacementBeforeNamespaceDurability => (
                MediaOperationRole::AtomicReplace,
                first,
                MediaFaultDirective::PauseAfter(gate),
            ),
            Self::AfterDataSettlementBeforeRootPublication
            | Self::AfterPhysicalDurabilityBeforeAcknowledgment => {
                return Err("mutation checkpoint seam has no media fault".to_owned())
            }
        };
        Ok(fault)
    }
}
