mod atomic_record_append;
#[cfg(test)]
mod concurrent_append_tests;
mod control_media_fault;
mod control_media_identity;
mod control_media_location;
mod control_tail_state;
mod durable_prefix_recovery;
#[cfg(test)]
mod media_mutation_tests;
mod recovery_object;
#[cfg(test)]
mod recovery_object_tests;
#[cfg(test)]
mod tests;
mod transition_receipt_index;

pub use atomic_record_append::{
    PhysicalControlAppendReceipt, PhysicalControlStoreInspection, PhysicalControlStoreSummary,
    PhysicalOperationalControlStore,
};
pub use control_media_fault::ControlMediaFault;
pub use control_media_identity::ControlMediaIdentity;
pub use control_media_location::ControlMediaLocation;
pub use durable_prefix_recovery::DurableControlRecordBytes;
pub use recovery_object::ControlRecoveryObjectHandle;

pub const MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES: usize =
    durable_prefix_recovery::MAX_CONTROL_PAYLOAD_BYTES;

pub(crate) use recovery_object::PhysicalControlRecoveryObjectStore;

pub(crate) use durable_prefix_recovery::{
    encode_record, extend_prefix_digest, scan_durable_prefix, scan_durable_suffix,
    validate_record_lengths,
};
