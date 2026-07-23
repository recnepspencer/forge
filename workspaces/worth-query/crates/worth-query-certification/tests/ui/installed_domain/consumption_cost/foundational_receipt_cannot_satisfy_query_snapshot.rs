use worth_query::facade::domain::{
    WorthQueryConsumptionCostSnapshot, WorthQueryFoundationalConsumptionCostReceipt,
};

fn require_query_snapshot(_: &WorthQueryConsumptionCostSnapshot) {}

fn cannot_substitute(receipt: &WorthQueryFoundationalConsumptionCostReceipt) {
    require_query_snapshot(receipt);
}

fn main() {}
