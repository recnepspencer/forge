macro_rules! domain_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u64);

        impl $name {
            pub const fn new(value: u64) -> Option<Self> {
                if value == 0 {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

domain_id!(AccountId);
domain_id!(AccountAuthorizationId);
domain_id!(BankPrincipalId);
domain_id!(BusinessId);
domain_id!(InstitutionId);
domain_id!(JournalEntryId);
domain_id!(PaymentId);
