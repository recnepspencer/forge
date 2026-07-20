use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::harness::physical_isolation::publication::{
    admitted_copy_on_write_plan, publication_inputs_for_store, PublicationInputs,
};

use crate::maintenance::{
    copy_on_write_layout_mutation_execution, layout_mutation_admission,
    CopyOnWriteLayoutMutationRequest, IndexMaintenanceMode, PhysicalMutationShape,
};
use crate::strategy::registry::{
    layout_admission_registry, LayoutAdmissionRequest, LayoutRequestedCapability,
    LayoutStrategyRegistrySnapshot,
};
use crate::strategy::tests_support::{
    admit_strategy_scope, strategy_test_security_scope, strategy_test_store_identity,
};
use crate::{ArtifactFamilyAccessLane, LayoutStrategyFamily};

pub(super) fn btree_strategy(
    mutation_shape: PhysicalMutationShape,
) -> LayoutStrategyRegistrySnapshot {
    btree_strategy_with_mode(mutation_shape, IndexMaintenanceMode::SynchronousExact)
}

pub(super) fn btree_strategy_with_mode(
    mutation_shape: PhysicalMutationShape,
    mode: IndexMaintenanceMode,
) -> LayoutStrategyRegistrySnapshot {
    let (family, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    layout_admission_registry()
        .admit(
            LayoutAdmissionRequest::from_admitted(
                family,
                key_domain,
                LayoutStrategyFamily::BaselineBTreeRange,
                LayoutRequestedCapability::point_lookup(),
                match mode {
                    IndexMaintenanceMode::RebuildOnly | IndexMaintenanceMode::MigrationOnly => {
                        ArtifactFamilyAccessLane::MaintenancePath
                    }
                    IndexMaintenanceMode::AdvisoryOnly | IndexMaintenanceMode::VerifierOnly => {
                        ArtifactFamilyAccessLane::TerminalPath
                    }
                    _ => ArtifactFamilyAccessLane::HotPath,
                },
            )
            .for_mutation_shape(mutation_shape)
            .under_maintenance_mode(mode),
        )
        .into_result()
        .unwrap()
}

pub(super) fn current_security_scope() -> worth_store_security::StoreAdmittedSecurityScope {
    strategy_test_security_scope(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
    )
}

pub(super) fn executed_btree_mutation(
    mode: IndexMaintenanceMode,
    operation_digest: &str,
    generation: u64,
) -> crate::CopyOnWriteLayoutMutationReceipt {
    let strategy = btree_strategy_with_mode(PhysicalMutationShape::PointRewrite, mode);
    let inputs = publication_inputs_for_store(
        &strategy_test_store_identity(),
        operation_digest,
        generation,
    );
    let source_materialization =
        source_materialization(strategy.admitted_strategy().admitted_family(), &inputs);
    let plan = layout_mutation_admission()
        .admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            strategy,
            admitted_copy_on_write_plan(&inputs),
            &source_materialization,
            current_security_scope().witnesses(),
        ))
        .into_planned()
        .expect("ordinary B-tree mutation should plan")
        .into_copy_on_write()
        .expect("ordinary B-tree mutation should retain copy-on-write authority");
    let mut runtime =
        worth_store_test_support::harness::physical_isolation::PhysicalRootPublicationFixture::open(
            inputs.old_root,
        )
        .unwrap();
    copy_on_write_layout_mutation_execution()
        .execute(&mut runtime, plan)
        .into_published()
        .expect("ordinary B-tree mutation should publish")
}

pub(super) fn source_materialization(
    family: crate::AdmittedPhysicalArtifactFamily,
    inputs: &PublicationInputs,
) -> crate::AdmittedLayoutMaterialization {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    crate::access_planning()
        .admit_btree_publication_materialization(
            family,
            &catalog,
            inputs.old_candidate.validation(),
        )
        .into_result()
        .expect("physical mutation source must admit exact B-tree materialization")
}
