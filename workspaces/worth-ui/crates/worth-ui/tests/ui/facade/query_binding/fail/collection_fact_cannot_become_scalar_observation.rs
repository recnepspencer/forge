use worth_ui::facade::query_binding::{
    UiCollectionProjectionFactReceipt, UiScalarProjectionObservation,
};

fn invalid(receipt: UiCollectionProjectionFactReceipt) -> UiScalarProjectionObservation {
    receipt.into_observation()
}

fn main() {}
