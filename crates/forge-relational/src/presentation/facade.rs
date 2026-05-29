pub mod bridge {
    pub use crate::presentation::bridge::{
        bridge_snapshot_identity_for_commit, bridge_snapshot_identity_for_handle,
        commit_envelope_to_bridge_envelope, publication_bundle_to_bridge_envelope,
        publication_patch_to_bridge_envelope, RuntimeBridgeRelationalSource,
    };
}

pub mod runtime {
    pub use crate::presentation::api::RelationalRuntimeApi;
    pub use crate::presentation::contracts::{
        ImmutableReadContract, RelationalBoundaryContract, SerializedAuthorityContract,
    };
}

#[cfg(test)]
pub mod harness {
    pub use crate::presentation::harness::{
        default_harness_expectations, FixtureEntity, FixtureRelation, RelationalFixture,
        RelationalHarnessAdapter, RelationalHarnessError, RelationalHarnessExpectations,
        RelationalHarnessPlan,
    };
}
