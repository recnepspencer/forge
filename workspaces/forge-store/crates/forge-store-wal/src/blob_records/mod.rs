mod phase_link;
mod record_kind;

pub use phase_link::{durable_phase_for_record_kind, record_kind_admits_recovery_replay};
pub use record_kind::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind, BlobWalRecordScopeDenial,
};
