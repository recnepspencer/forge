#![allow(invalid_value)]

use forge_store::{SupportTrustDriftChecked, SupportTrustDriftReport, SupportTrustTranslatedInputs};

fn main() {
    let _ = SupportTrustDriftChecked {
        translated: unsafe { std::mem::zeroed::<SupportTrustTranslatedInputs>() },
        drift_report: unsafe { std::mem::zeroed::<SupportTrustDriftReport>() },
    };
}
