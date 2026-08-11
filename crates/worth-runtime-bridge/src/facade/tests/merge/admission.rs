use super::*;

#[test]
fn runtime_admits_registered_merge_declaration() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:analysis"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());

    let contract = runtime
        .admit_merge_history(declaration)
        .expect("registered merge declaration should be admitted");
    assert_eq!(
        contract
            .validated_declaration()
            .declaration()
            .bridge_class(),
        BridgeMergeConsumptionClass::AspectReconciliationMerge
    );
}
