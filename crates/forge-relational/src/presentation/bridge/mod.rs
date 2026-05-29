#[cfg(test)]
mod bridge_snapshot_reader_tests;
#[cfg(test)]
mod bridge_source_tests;
mod identities;
mod patch_envelopes;
mod runtime_source;
#[cfg(test)]
mod snapshot_catalog_tests;
mod snapshot_reading;
#[cfg(test)]
mod snapshot_reading_tests;
mod snapshot_values;
#[cfg(test)]
mod test_catalog;

pub use identities::{bridge_snapshot_identity_for_commit, bridge_snapshot_identity_for_handle};
pub use patch_envelopes::{
    commit_envelope_to_bridge_envelope, publication_bundle_to_bridge_envelope,
    publication_patch_to_bridge_envelope,
};
pub use runtime_source::RuntimeBridgeRelationalSource;
#[cfg(test)]
pub use test_catalog::{PublicationBridgeCatalog, PublicationBridgeSnapshot};
