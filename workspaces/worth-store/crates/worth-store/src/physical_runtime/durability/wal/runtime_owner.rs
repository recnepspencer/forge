use std::sync::{Arc, Mutex};

use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    ArtifactAppendRange, ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFile,
    QualifiedFilesystemMedia,
};
use worth_store_wal::{
    plan_wal_frame_append, LogSequenceNumber, WalAppendFrontier, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};

use crate::physical_runtime::durability::AdmittedPhysicalMutation;
use crate::physical_runtime::durability::AllocatedPhysicalMutationAttemptBinding;
use crate::physical_runtime::record_serving::PreparedPhysicalMutation;
use crate::physical_runtime::{
    CanonicalRedoRecords, PhysicalSignalProfileIdentity, PhysicalWalAppendDeclaration,
    PhysicalWalMemberBasis, PhysicalWalMemberIdentity, RecordAppendBatch, RuntimeIdentity,
    WalRangeReservedPhysicalMutation,
};

use super::preparation_admission::{AdmittedWalPreparedMutation, PhysicalWalPreparationAdmission};
use super::PhysicalWalReservationDenial;

#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalWalRuntimeOwner {
    shared: Arc<Mutex<PhysicalWalRuntimeState>>,
    artifact: ArtifactTreeFile,
    preparation: Arc<PhysicalWalPreparationAdmission>,
}

struct PhysicalWalRuntimeState {
    frontier: WalAppendFrontier,
    in_flight: bool,
    sealed: bool,
    appended_frames: u64,
    appended_bytes: u64,
}

impl PhysicalWalRuntimeOwner {
    pub(in crate::physical_runtime) fn initialize(
        media: &QualifiedFilesystemMedia,
        runtime: RuntimeIdentity,
        signal_profile: PhysicalSignalProfileIdentity,
    ) -> Result<Self, ArtifactTreeFailure> {
        let segment = WalSegmentId::new(1).expect("the initial WAL segment is nonzero");
        let generation =
            WalSegmentGeneration::new(1).expect("the initial WAL generation is nonzero");
        let directory = ArtifactTreeDirectory::families()
            .child("wal")
            .expect("the Store-owned WAL directory is portable");
        let artifact = directory
            .file("segment-1-generation-1.wal")
            .expect("the Store-owned WAL artifact name is portable");
        let tree = media.artifact_tree();
        if !tree.directory_exists(&directory)? {
            tree.create_directory(&directory)?;
        }
        if !tree.file_exists(&artifact)? {
            tree.write_new(&artifact, &[])?;
        }
        let observed_bytes = tree.file_length(&artifact)?;
        let (frontier, sealed) = if observed_bytes == 0 {
            (WalAppendFrontier::empty(segment, generation), false)
        } else {
            (WalAppendFrontier::empty(segment, generation), true)
        };
        Ok(Self {
            shared: Arc::new(Mutex::new(PhysicalWalRuntimeState {
                frontier,
                in_flight: false,
                sealed,
                appended_frames: 0,
                appended_bytes: 0,
            })),
            artifact,
            preparation: Arc::new(PhysicalWalPreparationAdmission::new(
                media.store_identity(),
                runtime,
                signal_profile,
            )),
        })
    }

    pub(super) fn admit_preparation(
        &self,
        prepared: PreparedPhysicalMutation,
    ) -> Result<AdmittedWalPreparedMutation, (PreparedPhysicalMutation, PhysicalWalReservationDenial)>
    {
        self.preparation.admit(prepared)
    }

    pub(super) fn reserve(
        &self,
        prepared: AdmittedWalPreparedMutation,
    ) -> Result<
        WalRangeReservedPhysicalMutation,
        (PreparedPhysicalMutation, PhysicalWalReservationDenial),
    > {
        let prepared = prepared.into_prepared();
        if !matches!(
            prepared.disposition(),
            crate::physical_runtime::PhysicalMutationAdmissionDisposition::Fresh
        ) {
            return Err((prepared, PhysicalWalReservationDenial::DuplicateUnresolved));
        }
        let mut state = self
            .shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state.sealed {
            return Err((prepared, PhysicalWalReservationDenial::InspectionRequired));
        }
        if state.in_flight {
            return Err((prepared, PhysicalWalReservationDenial::AppendInFlight));
        }
        let start = state
            .frontier
            .last_lsn_end()
            .unwrap_or(LogSequenceNumber::new(LogSequenceNumber::GENESIS.get() + 1));
        let Some(end) = start
            .get()
            .checked_add(u64::from(prepared.resources().record_count()))
            .map(LogSequenceNumber::new)
        else {
            return Err((prepared, PhysicalWalReservationDenial::LsnExhausted));
        };
        let lsn_range = WalLsnRange::new(start, end)
            .expect("canonical redo is nonempty and therefore has a nonempty LSN range");
        let (
            admission,
            batch,
            data,
            continuation,
            placement,
            deadline,
            signal_profile,
            durability_policy_basis,
            resources,
        ) = prepared.into_parts();
        let data = match data.bind(lsn_range) {
            Ok(data) => data,
            Err((data, denial)) => {
                let prepared = PreparedPhysicalMutation::from_planned_parts(
                    admission,
                    batch,
                    data,
                    continuation,
                    placement,
                    deadline,
                    signal_profile,
                    durability_policy_basis,
                    resources,
                );
                return Err((
                    prepared,
                    PhysicalWalReservationDenial::DataPlanBinding(denial),
                ));
            }
        };
        let binding = admission
            .into_fresh_binding()
            .expect("fresh disposition carries one unallocated WAL binding");
        let redo = CanonicalRedoRecords::from_prepared_records(
            batch.into_prepared_record_bytes(),
            lsn_range,
            data.redo_targets(),
        );
        let member_identity = PhysicalWalMemberIdentity::for_mutation(binding.mutation_identity());
        let member =
            PhysicalWalMemberBasis::new(member_identity, binding.mutation_identity(), lsn_range);
        let binding = binding.allocate_wal(member, &redo);
        let payload = encode_member_payload(&binding, &redo);
        let declared_identity = declared_member_identity(&binding);
        let frame =
            match plan_wal_frame_append(state.frontier, lsn_range, &declared_identity, &payload) {
                Ok(frame) => frame,
                Err(denial) => {
                    let binding = binding.release_wal_allocation();
                    let batch = RecordAppendBatch::from_prepared_record_bytes(
                        redo.into_prepared_record_bytes(),
                    );
                    let prepared = PreparedPhysicalMutation::from_planned_parts(
                        AdmittedPhysicalMutation::Fresh(binding),
                        batch,
                        data.into_prepared(),
                        continuation,
                        placement,
                        deadline,
                        signal_profile,
                        durability_policy_basis,
                        resources,
                    );
                    return Err((
                        prepared,
                        PhysicalWalReservationDenial::FramePlanning(denial),
                    ));
                }
            };
        let artifact_range = ArtifactAppendRange::new(
            frame.frame().valid_prefix_bytes(),
            frame.frame().encoded_frame().len() as u64,
        )
        .expect("WAL framing returns one nonempty nonoverflowing frame");
        let declaration = PhysicalWalAppendDeclaration::new(
            state.frontier.segment(),
            state.frontier.generation(),
            lsn_range,
            artifact_range,
            Sha256::digest(frame.frame().encoded_frame()).into(),
        );
        state.in_flight = true;
        Ok(WalRangeReservedPhysicalMutation::new(
            binding,
            member,
            redo,
            data,
            continuation,
            frame,
            self.artifact.clone(),
            declaration,
            placement,
            deadline,
            signal_profile,
            durability_policy_basis,
            resources,
        ))
    }

    pub(in crate::physical_runtime) fn complete(&self, frontier: WalAppendFrontier, bytes: u64) {
        let mut state = self
            .shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.frontier = frontier;
        state.in_flight = false;
        state.appended_frames = state.appended_frames.saturating_add(1);
        state.appended_bytes = state.appended_bytes.saturating_add(bytes);
    }

    pub(in crate::physical_runtime) fn release_before_effect(&self) {
        self.shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .in_flight = false;
    }

    pub(in crate::physical_runtime) fn seal_for_inspection(&self) {
        let mut state = self
            .shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.in_flight = false;
        state.sealed = true;
    }

    pub(in crate::physical_runtime) fn observation(&self) -> super::PhysicalWalObservation {
        let state = self
            .shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        super::PhysicalWalObservation::new(
            state.frontier.segment().get(),
            state.frontier.generation().get(),
            state.appended_frames,
            state.appended_bytes,
            state.frontier.valid_prefix_bytes(),
            state.frontier.last_lsn_end().map(LogSequenceNumber::get),
            state.sealed,
        )
    }
}

fn encode_member_payload(
    binding: &AllocatedPhysicalMutationAttemptBinding,
    redo: &CanonicalRedoRecords,
) -> Vec<u8> {
    let persisted_binding = binding.encode_persisted();
    let mut payload = Vec::with_capacity(
        16_usize
            .saturating_add(persisted_binding.len())
            .saturating_add(redo.encoded().len()),
    );
    write_field(&mut payload, &persisted_binding);
    write_field(&mut payload, redo.encoded());
    payload
}

fn declared_member_identity(binding: &AllocatedPhysicalMutationAttemptBinding) -> String {
    let member = binding
        .member()
        .member_identity()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let fingerprint = binding
        .fingerprint()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("member-{member}-{fingerprint}")
}

fn write_field(target: &mut Vec<u8>, field: &[u8]) {
    target.extend_from_slice(&(field.len() as u64).to_le_bytes());
    target.extend_from_slice(field);
}
