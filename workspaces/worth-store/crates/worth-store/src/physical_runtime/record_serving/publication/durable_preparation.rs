mod outcome;
mod payload;
mod prepared;
mod scope;

pub use outcome::{
    PhysicalMutationPreparationDeferred, PhysicalMutationPreparationDenial,
    PhysicalMutationPreparationFailure, PhysicalMutationPreparationOutcome,
    PhysicalMutationPreparationRebindRequired, PhysicalMutationPreparationStale,
};
pub(in crate::physical_runtime::record_serving) use payload::{
    prepare_canonical_payload, CanonicalPayloadPreparationError, CanonicalRecordAppendPayload,
};
pub(in crate::physical_runtime) use prepared::PreparedRecordPublicationContinuation;
pub use prepared::{
    PhysicalMutationAdmissionDisposition, PhysicalMutationResourceShape, PreparedPhysicalMutation,
};
pub(in crate::physical_runtime::record_serving) use scope::record_append_scope_identity;
