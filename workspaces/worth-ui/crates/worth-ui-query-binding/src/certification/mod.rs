mod collection_patch_attack;
mod installed_projection_fixture;
mod operation_live_fixture;
mod operation_semantic_facts;
mod product_support_contract;
mod projection_world_fixture;
mod query_row_reference_fixture;
mod scalar_native_authority_attack;

pub use collection_patch_attack::{
    certify_collection_patch_attack, WorthUiCollectionPatchAttack,
    WorthUiCollectionPatchAttackReport,
};
pub use installed_projection_fixture::{
    worth_ui_installed_test_domain, WorthUiInstalledQueryTestFixture,
};
pub use operation_live_fixture::WorthUiOperationLiveTestFixture;
pub use operation_semantic_facts::WorthUiInstalledOperationCertificationFacts;
pub use product_support_contract::certify_product_projection_support_contract;
pub use projection_world_fixture::{
    collection_projection_workspace, collection_projection_workspace_without_dependency_impact,
    collection_projection_workspace_without_entity_lookup, insert_projection_status,
    partial_collection_projection_workspace, remasked_scalar_projection_workspace,
    scalar_projection_workspace, seeded_collection_projection_workspace,
    update_projection_identity, update_projection_status, update_projection_status_batch,
    WorthUiCollectionProjectionSeedPosture,
};
pub use query_row_reference_fixture::query_row_reference_fixture;
pub use scalar_native_authority_attack::{
    certify_scalar_native_authority_attack, WorthUiScalarNativeAuthorityAttackReport,
    WorthUiScalarNativeKeyReport,
};
