use worth_store_operations::certification_scenario::{
    execute_scenario_authority_affecting_repair, execute_scenario_derived_repair,
    OwnerBackedBackupScenario,
};
use worth_store_operations::{OperationalCounterReceipt, OperationalSecurityScope};
use worth_store_physical_certification::DrivenOperationalControlStore;

pub(super) fn execute(
    identity: &str,
    scenario: &OwnerBackedBackupScenario,
    control: &worth_store_operations::OperationalControlStore,
    driven: &DrivenOperationalControlStore<'_, '_>,
    counters: &mut Vec<OperationalCounterReceipt>,
) -> worth_store_physical_integrity::IntegrityRepairClassificationReceipt {
    let repair_basis = scenario.execute_named(identity, "repair-basis", driven);
    counters.push(repair_basis.counters());
    let repair_source = repair_basis.into_restore_source();
    let security_scope =
        OperationalSecurityScope::from_admission(repair_source.custody().custody_receipt());
    let repair_target = scenario.workspace_root().join("damaged.index");
    let repair_replacement = scenario.workspace_root().join("replacement.index");
    std::fs::write(&repair_target, b"damaged-derived-index").unwrap();
    std::fs::write(&repair_replacement, b"rebuilt-derived-index").unwrap();
    let repair = execute_scenario_derived_repair(
        &format!("{identity}/repair"),
        &repair_target,
        &repair_replacement,
        security_scope,
        scenario.authority(),
        control,
        driven,
    );
    counters.push(OperationalCounterReceipt::from_repair(&repair));

    let authority_repair_basis = scenario.execute_named(identity, "authority-repair-basis", driven);
    counters.push(authority_repair_basis.counters());
    let authority_repair = execute_scenario_authority_affecting_repair(
        &format!("{identity}/authority-repair"),
        authority_repair_basis.into_restore_source(),
        &scenario.workspace_root().join("authority-repair"),
        scenario.authority(),
        control,
        driven,
    );
    counters.push(OperationalCounterReceipt::from_authority_affecting_repair(
        &authority_repair,
    ));
    let classification = authority_repair.integrity();
    super::super::recovery_publication::publish_authority_repair(
        identity,
        authority_repair,
        scenario,
        control,
        driven,
    );
    classification
}
