pub(super) fn branch(id: u64) -> String {
    format!("bank-branch:{id}")
}

pub(super) fn notice(id: u64) -> String {
    format!("bank-death-notice:{id}")
}

pub(super) fn estate(id: u64) -> String {
    format!("bank-estate:{id}")
}

pub(super) fn authority(id: u64) -> String {
    format!("bank-legal-authority:{id}")
}

pub(super) fn grant(id: u64) -> String {
    format!("bank-capability-grant:{id}")
}

pub(super) fn emergency(id: u64) -> String {
    format!("bank-emergency-access:{id}")
}

pub(super) fn review(id: u64) -> String {
    format!("bank-mandatory-review:{id}")
}
