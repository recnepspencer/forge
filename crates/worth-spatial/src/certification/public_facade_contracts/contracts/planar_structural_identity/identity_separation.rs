use super::contract_subject::structural_identity_receipt;

#[test]
fn planar_structural_identity_diverges_from_topology_naming_binding_and_lineage() {
    let first = structural_identity_receipt("structural-diverge-a", "topology:stable");
    let second = structural_identity_receipt("structural-diverge-b", "topology:stable");

    assert_eq!(
        first.basis().topology_identity(),
        second.basis().topology_identity()
    );
    assert_eq!(
        first.basis().persistent_name(),
        second.basis().persistent_name()
    );
    assert_eq!(
        first.basis().binding_identity(),
        second.basis().binding_identity()
    );
    assert_eq!(
        first.basis().lineage_identity(),
        second.basis().lineage_identity()
    );
    assert_ne!(
        first.structural_identity_digest(),
        second.structural_identity_digest()
    );
}
