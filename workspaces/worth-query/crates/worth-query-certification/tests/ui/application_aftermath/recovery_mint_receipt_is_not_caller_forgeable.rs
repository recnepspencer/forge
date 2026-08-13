//! R8.28 / R8.62 — mint's one argument cannot be fabricated or re-addressed.
//!
//! `recovery_mint_uses_receipt_only` already proves the signature admits nothing
//! but a receipt. That is only worth something if a receipt is itself beyond a
//! caller's reach: mint now compares the receipt's admitting Query runtime
//! against the runtime it is minting into, so a caller who could build a receipt
//! — or overwrite the authority binding on one it was handed — would choose its
//! own provenance and mint a handle for a commit it never made.

use worth_query_execution::facade::primary_graph::WorthQueryApplicationCommitReceipt;

fn caller_cannot_fabricate_a_receipt() {
    let _ = WorthQueryApplicationCommitReceipt {
        provider_runtime_instance_id: 0,
    };
}

fn caller_cannot_readdress_a_receipt_it_was_handed(
    receipt: &WorthQueryApplicationCommitReceipt,
) -> u64 {
    receipt.provider_runtime_instance_id
}

// rustc answers the struct-literal error by pointing at the associated function
// that does build one. Take it up on the offer: that constructor is confined to
// the commit path inside `primary_graph`, so the suggested route is closed too.
fn caller_cannot_take_the_constructor_rustc_suggests() {
    let _ = WorthQueryApplicationCommitReceipt::from_recovered_provider;
}

fn main() {}
