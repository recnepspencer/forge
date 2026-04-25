#![allow(invalid_value)]

use forge_store::{
    RawSupportTrustRequest, SupportTrustReceiptBundle, SupportTrustRequestAdmitted,
};

fn main() {
    let _ = SupportTrustRequestAdmitted {
        request: unsafe { std::mem::zeroed::<RawSupportTrustRequest>() },
        receipt_bundle: unsafe { std::mem::zeroed::<SupportTrustReceiptBundle>() },
    };
}
