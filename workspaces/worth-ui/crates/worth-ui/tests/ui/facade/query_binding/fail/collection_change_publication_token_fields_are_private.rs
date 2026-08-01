use worth_ui_query_binding::{
    WorthUiAdmittedCollectionChangePublication, WorthUiCollectionChangeConsequence,
    WorthUiCollectionChangeStagingReceipt,
};

fn invalid(
    consequence: WorthUiCollectionChangeConsequence,
    receipt: WorthUiCollectionChangeStagingReceipt,
) -> WorthUiAdmittedCollectionChangePublication {
    WorthUiAdmittedCollectionChangePublication {
        consequence,
        receipt,
    }
}

fn main() {}
