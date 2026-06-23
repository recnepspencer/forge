use worth_ui::facade::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiRuntimeChangeFamilyRow,
    WorthUiRuntimeInstanceWitness,
};

fn main() {
    let _witness = WorthUiRuntimeInstanceWitness { raw: 1 };
    let _row = WorthUiRuntimeChangeFamilyRow {
        runtime_instance: _witness,
        family: unreachable!(),
        status: unreachable!(),
        changed_facts: unreachable!(),
        denial_detail: None,
        payload_digest: 0,
        component_reload_receipt: None,
    };
    let _evidence = WorthUiAdmittedRuntimeChangeEvidence {
        runtime_instance: _witness,
        posture: unreachable!(),
        family_rows: vec![_row],
        digest: unreachable!(),
        counters: unreachable!(),
    };
}
