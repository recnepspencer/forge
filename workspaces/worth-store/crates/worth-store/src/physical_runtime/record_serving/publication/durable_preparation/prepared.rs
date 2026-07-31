use crate::physical_runtime::{
    durability::{AdmittedPhysicalMutation, PreparedPhysicalDataPlan},
    record_serving::planning::prepared_payload::PreparedRecordPayloadPlan,
    AdmittedRecordPlacementPolicy, PhysicalMutationDeadline,
    PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdempotencyLease,
    PhysicalMutationIdentity, PhysicalMutationRequestFingerprint, PhysicalSignalProfileIdentity,
    PhysicalWorkSemanticBasis, RecordAppendBatch,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationAdmissionDisposition {
    Fresh,
    DuplicateUnresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationResourceShape {
    record_count: u32,
    payload_bytes: u64,
    prepared_payload_bytes: u64,
}

pub struct PreparedPhysicalMutation {
    admission: AdmittedPhysicalMutation,
    data: PreparedPhysicalMutationData,
    placement: AdmittedRecordPlacementPolicy,
    deadline: PhysicalMutationDeadline,
    signal_profile: PhysicalSignalProfileIdentity,
    durability_policy_basis: PhysicalWorkSemanticBasis,
    resources: PhysicalMutationResourceShape,
}

pub(in crate::physical_runtime) struct PreparedRecordPublicationContinuation(
    PreparedRecordPayloadPlan,
);

enum PreparedPhysicalMutationData {
    Unplanned {
        batch: RecordAppendBatch,
    },
    Planned {
        batch: RecordAppendBatch,
        data: PreparedPhysicalDataPlan,
        continuation: PreparedRecordPublicationContinuation,
    },
}

impl PhysicalMutationResourceShape {
    pub(in crate::physical_runtime::record_serving) const fn prepared(
        record_count: u32,
        payload_bytes: u64,
    ) -> Self {
        Self {
            record_count,
            payload_bytes,
            prepared_payload_bytes: payload_bytes,
        }
    }

    pub const fn record_count(self) -> u32 {
        self.record_count
    }

    pub const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }

    pub const fn prepared_payload_bytes(self) -> u64 {
        self.prepared_payload_bytes
    }
}

impl PreparedPhysicalMutation {
    pub(in crate::physical_runtime) const fn new(
        admission: AdmittedPhysicalMutation,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        deadline: PhysicalMutationDeadline,
        signal_profile: PhysicalSignalProfileIdentity,
        durability_policy_basis: PhysicalWorkSemanticBasis,
        resources: PhysicalMutationResourceShape,
    ) -> Self {
        Self {
            admission,
            data: PreparedPhysicalMutationData::Unplanned { batch },
            placement,
            deadline,
            signal_profile,
            durability_policy_basis,
            resources,
        }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.admission.mutation_identity()
    }

    pub const fn idempotency_identity(&self) -> PhysicalMutationIdempotencyKeyIdentity {
        self.admission.idempotency_identity()
    }

    pub const fn idempotency_lease(&self) -> PhysicalMutationIdempotencyLease {
        self.admission.lease()
    }

    pub const fn request_fingerprint(&self) -> PhysicalMutationRequestFingerprint {
        self.admission.fingerprint()
    }

    pub const fn deadline(&self) -> PhysicalMutationDeadline {
        self.deadline
    }

    pub const fn signal_profile(&self) -> PhysicalSignalProfileIdentity {
        self.signal_profile
    }

    pub(in crate::physical_runtime) const fn placement(&self) -> AdmittedRecordPlacementPolicy {
        self.placement
    }

    pub fn durability_policy_basis(&self) -> PhysicalWorkSemanticBasis {
        self.durability_policy_basis.clone()
    }

    pub const fn resources(&self) -> PhysicalMutationResourceShape {
        self.resources
    }

    pub const fn disposition(&self) -> PhysicalMutationAdmissionDisposition {
        if self.admission.is_fresh() {
            PhysicalMutationAdmissionDisposition::Fresh
        } else {
            PhysicalMutationAdmissionDisposition::DuplicateUnresolved
        }
    }

    pub(in crate::physical_runtime) const fn data_is_planned(&self) -> bool {
        matches!(self.data, PreparedPhysicalMutationData::Planned { .. })
    }

    pub(in crate::physical_runtime) fn duplicate_prepared_batch(&self) -> RecordAppendBatch {
        match &self.data {
            PreparedPhysicalMutationData::Unplanned { batch }
            | PreparedPhysicalMutationData::Planned { batch, .. } => batch.duplicate_prepared(),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn attach_data_plan(
        mut self,
        data: PreparedPhysicalDataPlan,
        continuation: PreparedRecordPayloadPlan,
    ) -> Self {
        let batch = match self.data {
            PreparedPhysicalMutationData::Unplanned { batch } => batch,
            PreparedPhysicalMutationData::Planned { .. } => {
                unreachable!("a prepared mutation receives one immutable data plan")
            }
        };
        self.data = PreparedPhysicalMutationData::Planned {
            batch,
            data,
            continuation: PreparedRecordPublicationContinuation(continuation),
        };
        self
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        AdmittedPhysicalMutation,
        RecordAppendBatch,
        PreparedPhysicalDataPlan,
        PreparedRecordPublicationContinuation,
        AdmittedRecordPlacementPolicy,
        PhysicalMutationDeadline,
        PhysicalSignalProfileIdentity,
        PhysicalWorkSemanticBasis,
        PhysicalMutationResourceShape,
    ) {
        let PreparedPhysicalMutationData::Planned {
            batch,
            data,
            continuation,
        } = self.data
        else {
            unreachable!("fresh WAL reservation requires the director-attached data plan")
        };
        (
            self.admission,
            batch,
            data,
            continuation,
            self.placement,
            self.deadline,
            self.signal_profile,
            self.durability_policy_basis,
            self.resources,
        )
    }

    pub(in crate::physical_runtime) const fn from_planned_parts(
        admission: AdmittedPhysicalMutation,
        batch: RecordAppendBatch,
        data: PreparedPhysicalDataPlan,
        continuation: PreparedRecordPublicationContinuation,
        placement: AdmittedRecordPlacementPolicy,
        deadline: PhysicalMutationDeadline,
        signal_profile: PhysicalSignalProfileIdentity,
        durability_policy_basis: PhysicalWorkSemanticBasis,
        resources: PhysicalMutationResourceShape,
    ) -> Self {
        Self {
            admission,
            data: PreparedPhysicalMutationData::Planned {
                batch,
                data,
                continuation,
            },
            placement,
            deadline,
            signal_profile,
            durability_policy_basis,
            resources,
        }
    }
}

impl PreparedRecordPublicationContinuation {
    pub(in crate::physical_runtime::record_serving) fn into_payload(
        self,
    ) -> PreparedRecordPayloadPlan {
        self.0
    }
}
