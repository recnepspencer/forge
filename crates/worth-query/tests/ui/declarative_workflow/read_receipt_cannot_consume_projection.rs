use worth_query::facade::read::{project_facts, WorthQueryReadReceipt};

fn attempt(receipt: &WorthQueryReadReceipt) {
    let _ = receipt.consume_projection(project_facts().entity_identities());
}

fn main() {}
