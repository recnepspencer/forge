use worth_ui::facade::WorthUiMeasuredProductViewReceipt;

fn main() {
    let _receipt = WorthUiMeasuredProductViewReceipt {
        mounted_product_view: panic!("mounted product view is runtime-admitted"),
        host_observations: panic!("host observations are runtime-admitted"),
        consumed_facts: Vec::new(),
        receipt_digest: 1,
    };
}
