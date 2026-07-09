use worth_store::{AdmittedSubscriptionSupportDeclaration, RawSubscriptionSupportDeclaration};

fn main() {
    let raw: RawSubscriptionSupportDeclaration =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = AdmittedSubscriptionSupportDeclaration::new(raw);
}
