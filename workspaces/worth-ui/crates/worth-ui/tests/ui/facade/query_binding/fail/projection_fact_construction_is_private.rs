use worth_ui_query_binding::UiScalarProjectionFactReceipt;

mod binding_affinity {
    include!("projection_binding_is_affine.rs");
}

mod fact_reassembly {
    include!("projection_fact_reassembly_is_private.rs");
}

mod shape_crossing {
    include!("collection_fact_cannot_become_scalar_observation.rs");
}

fn invalid() -> UiScalarProjectionFactReceipt {
    UiScalarProjectionFactReceipt {}
}

fn main() {}
