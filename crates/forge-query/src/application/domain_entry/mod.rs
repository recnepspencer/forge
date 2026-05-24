mod checked_outcome;
mod domain_marker;
mod entry_root;
mod front_door;
mod support_snapshot;

pub use checked_outcome::{
    ForgeQueryDomainEntryChecked, ForgeQueryDomainEntryDeferred, ForgeQueryDomainEntryUnsupported,
};
pub use domain_marker::ForgeQueryDomainEntryMarker;
pub use entry_root::{ForgeQueryDomainEntryProofRoot, ForgeQueryDomainEntryRoot};
pub use support_snapshot::ForgeQueryDomainEntrySupportSnapshot;

pub(crate) use front_door::{
    forge_query_checked_domain_entry, forge_query_domain_entry,
    forge_query_domain_entry_support_snapshot, forge_query_domain_proof_root,
};

#[cfg(test)]
mod tests;
