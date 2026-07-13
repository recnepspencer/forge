use super::{
    access_planning, admitted_store_wal_checkpoint_security_scope_for_layout_partition_test,
    execute_baseline_lsm_persisted_fixture, layout_declarations,
};
use forge_store_contracts::DurableArtifactFamilyId;

#[test]
fn equal_looking_publications_from_independent_stores_do_not_share_materialization_authority() {
    let first = execute_baseline_lsm_persisted_fixture().publication_execution();
    let second = execute_baseline_lsm_persisted_fixture().publication_execution();
    let catalog = super::super::layout::admitted_layout_bootstrap_catalog();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let declarations = layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PublicationWalIntent)
        .unwrap();
    let family = declarations
        .admit_physical_artifact_family(declaration, security.witnesses())
        .unwrap();

    let first_materialization = access_planning()
        .admit_lsm_publication_materialization(family, &catalog, &first)
        .unwrap();
    let second_materialization = access_planning()
        .admit_lsm_publication_materialization(family, &catalog, &second)
        .unwrap();

    assert_eq!(
        first.wal_publication().identity(),
        second.wal_publication().identity()
    );
    assert_ne!(
        first_materialization.source(),
        second_materialization.source()
    );
}
