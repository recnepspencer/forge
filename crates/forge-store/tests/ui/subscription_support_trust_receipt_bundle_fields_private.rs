#![allow(invalid_value)]

use forge_store::{
    SupportBasisReceipt, SupportCompatibilityReceipt, SupportCursorCheckpointReceipt,
    SupportFamilyRoleReceipt, SupportOperationalVerdictReceipt, SupportPortabilityReceipt,
    SupportResumeClassificationReceipt, SupportTrustReceiptBundle,
};

fn main() {
    let _ = SupportTrustReceiptBundle {
        resume: unsafe { std::mem::zeroed::<SupportResumeClassificationReceipt>() },
        operational: unsafe { std::mem::zeroed::<SupportOperationalVerdictReceipt>() },
        family_role: unsafe { std::mem::zeroed::<SupportFamilyRoleReceipt>() },
        basis: unsafe { std::mem::zeroed::<SupportBasisReceipt>() },
        cursor_checkpoint: unsafe { std::mem::zeroed::<SupportCursorCheckpointReceipt>() },
        compatibility: unsafe { std::mem::zeroed::<SupportCompatibilityReceipt>() },
        portability: unsafe { std::mem::zeroed::<SupportPortabilityReceipt>() },
        retention: None,
        maintenance: None,
        import_admission: None,
    };
}
