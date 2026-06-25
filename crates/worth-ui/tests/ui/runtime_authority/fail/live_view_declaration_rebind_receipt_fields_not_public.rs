use worth_ui::facade::WorthUiLiveViewDeclarationRebindReceipt;

fn main() {
    let _ = WorthUiLiveViewDeclarationRebindReceipt {
        live_view_id: "validation.live_view.proof".to_owned(),
        prior_declaration_digest: 1,
        next_declaration_digest: 2,
        changed_facts: changed_facts_fixture(),
        counters: counters_fixture(),
        receipt_digest: 3,
    };
}

fn changed_facts_fixture() -> worth_ui::facade::WorthUiChangedRuntimeFacts {
    panic!("fixture only checks receipt field privacy")
}

fn counters_fixture() -> worth_ui::facade::WorthUiLiveViewDeclarationRebindCounters {
    panic!("fixture only checks receipt field privacy")
}
