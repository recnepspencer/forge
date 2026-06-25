use worth_ui::facade::{
    WorthUiAllocatedChildReceipt, WorthUiLayoutAllocationContainerPolicyReceipt,
    WorthUiLayoutAllocationReceipt,
};

fn main() {
    let _allocation = WorthUiLayoutAllocationReceipt {
        root_node_id: "root".to_owned(),
        host_measurement_basis_digest: 1,
        container_policies: Vec::<WorthUiLayoutAllocationContainerPolicyReceipt>::new(),
        children: Vec::<WorthUiAllocatedChildReceipt>::new(),
        consumed_facts: Vec::new(),
        counters: panic!("layout counters are runtime-admitted"),
        receipt_digest: 1,
    };
}
