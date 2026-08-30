use worth_store::physical_runtime::{ObservedRecoveryArtifact, ObservedWalArtifact};
use worth_store_physical_integrity::{
    BootstrapCatalogIntegrityValidation, CheckpointBindingCompactionIntegrityValidation,
    CheckpointBindingIntegrityValidation, CheckpointDirtyBasisIntegrityValidation,
    CheckpointFooterIntegrityValidation, CheckpointStreamHeaderIntegrityValidation,
    CurrentRootSelectorIntegrityValidation, ExtentChunkIntegrityValidation,
    ExtentManifestIntegrityValidation, FreeSpaceHeaderIntegrityValidation,
    FreeSpaceMembershipBlockIntegrityValidation, InlinePageIntegrityValidation,
    PhysicalArtifactScope, PhysicalByteRange, PreviousRootSelectorIntegrityValidation,
    RootManifestIntegrityValidation, RootRoutingBlockIntegrityValidation,
    SegmentMembershipBlockIntegrityValidation, WalFrameIntegrityValidation,
};

use super::super::{
    IntegrityAdmittedRecoveryArtifact, RecoveryIntegrityIngressAttempt,
    RecoveryIntegrityIngressCounters,
};

macro_rules! complete_route_contract {
    ($contract:ident, $route:ident, $validation:ty) => {
        fn $contract<'media>(
            observed: &'media ObservedRecoveryArtifact,
            expected_scope: PhysicalArtifactScope,
            validation: $validation,
            counters: &mut RecoveryIntegrityIngressCounters,
        ) -> RecoveryIntegrityIngressAttempt<'media> {
            IntegrityAdmittedRecoveryArtifact::$route(
                observed,
                expected_scope,
                validation,
                counters,
            )
        }
    };
}

macro_rules! ranged_route_contract {
    ($contract:ident, $route:ident, $validation:ty) => {
        fn $contract<'media>(
            observed: &'media ObservedRecoveryArtifact,
            expected_scope: PhysicalArtifactScope,
            relative_range: PhysicalByteRange,
            validation: $validation,
            counters: &mut RecoveryIntegrityIngressCounters,
        ) -> RecoveryIntegrityIngressAttempt<'media> {
            IntegrityAdmittedRecoveryArtifact::$route(
                observed,
                expected_scope,
                relative_range,
                validation,
                counters,
            )
        }
    };
}

complete_route_contract!(
    bootstrap,
    bind_bootstrap_catalog,
    BootstrapCatalogIntegrityValidation<'media>
);
complete_route_contract!(
    current_selector,
    bind_current_selector,
    CurrentRootSelectorIntegrityValidation<'media>
);
complete_route_contract!(
    previous_selector,
    bind_previous_selector,
    PreviousRootSelectorIntegrityValidation<'media>
);
complete_route_contract!(
    root_manifest,
    bind_root_manifest,
    RootManifestIntegrityValidation<'media>
);
complete_route_contract!(
    root_routing,
    bind_root_routing_block,
    RootRoutingBlockIntegrityValidation<'media>
);
complete_route_contract!(
    segment_membership,
    bind_segment_membership_block,
    SegmentMembershipBlockIntegrityValidation<'media>
);
complete_route_contract!(page, bind_page_frame, InlinePageIntegrityValidation<'media>);
complete_route_contract!(
    extent_manifest,
    bind_extent_manifest,
    ExtentManifestIntegrityValidation<'media>
);
complete_route_contract!(
    extent_chunk,
    bind_extent_chunk,
    ExtentChunkIntegrityValidation<'media>
);
complete_route_contract!(
    free_space_header,
    bind_free_space_header,
    FreeSpaceHeaderIntegrityValidation<'media>
);
complete_route_contract!(
    free_space_membership,
    bind_free_space_membership_block,
    FreeSpaceMembershipBlockIntegrityValidation<'media>
);
ranged_route_contract!(
    checkpoint_header,
    bind_checkpoint_stream_header,
    CheckpointStreamHeaderIntegrityValidation<'media>
);
ranged_route_contract!(
    checkpoint_dirty,
    bind_checkpoint_dirty_basis,
    CheckpointDirtyBasisIntegrityValidation<'media>
);
ranged_route_contract!(
    checkpoint_compaction,
    bind_checkpoint_binding_compaction,
    CheckpointBindingCompactionIntegrityValidation<'media>
);
ranged_route_contract!(
    checkpoint_binding,
    bind_checkpoint_binding,
    CheckpointBindingIntegrityValidation<'media>
);
ranged_route_contract!(
    checkpoint_footer,
    bind_checkpoint_footer,
    CheckpointFooterIntegrityValidation<'media>
);

fn wal<'media>(
    owner: &worth_store::physical_runtime::PhysicalRecoveryCoordination,
    observed: &'media ObservedWalArtifact,
    expected_scope: PhysicalArtifactScope,
    relative_range: PhysicalByteRange,
    validation: WalFrameIntegrityValidation<'media>,
    counters: &mut RecoveryIntegrityIngressCounters,
) -> RecoveryIntegrityIngressAttempt<'media> {
    IntegrityAdmittedRecoveryArtifact::bind_wal_frame(
        owner,
        observed,
        expected_scope,
        relative_range,
        validation,
        counters,
    )
}

#[test]
fn every_phase_five_family_has_the_real_owner_route_signature() {
    let _ = (
        bootstrap,
        current_selector,
        previous_selector,
        root_manifest,
        root_routing,
        segment_membership,
        page,
        extent_manifest,
        extent_chunk,
        free_space_header,
        free_space_membership,
        checkpoint_header,
        checkpoint_dirty,
        checkpoint_compaction,
        checkpoint_binding,
        checkpoint_footer,
        wal,
    );
}
