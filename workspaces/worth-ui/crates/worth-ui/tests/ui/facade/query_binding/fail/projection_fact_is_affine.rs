use worth_ui_query_binding::UiScalarProjectionFactReceipt;

fn invalid(fact: &UiScalarProjectionFactReceipt) {
    let _copy: UiScalarProjectionFactReceipt = fact.clone();
}

fn main() {}
