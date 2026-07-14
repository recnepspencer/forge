#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5CloseoutReservedScope {
    S6IoQosIsolation,
    BlobLifecycle,
    LayoutIndexes,
    S10Repair,
    S11Security,
    S12Certification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5CloseoutReservationSet {
    scopes: [S5CloseoutReservedScope; 6],
}

impl S5CloseoutReservationSet {
    pub const fn physical_isolation_closeout_reservations() -> Self {
        Self {
            scopes: [
                S5CloseoutReservedScope::S6IoQosIsolation,
                S5CloseoutReservedScope::BlobLifecycle,
                S5CloseoutReservedScope::LayoutIndexes,
                S5CloseoutReservedScope::S10Repair,
                S5CloseoutReservedScope::S11Security,
                S5CloseoutReservedScope::S12Certification,
            ],
        }
    }

    pub const fn scopes(&self) -> &[S5CloseoutReservedScope; 6] {
        &self.scopes
    }

    pub fn contains(&self, scope: S5CloseoutReservedScope) -> bool {
        self.scopes.contains(&scope)
    }

    pub fn reserves_only_future_work(&self) -> bool {
        self.scopes.len() == 6
            && self.contains(S5CloseoutReservedScope::S6IoQosIsolation)
            && self.contains(S5CloseoutReservedScope::BlobLifecycle)
            && self.contains(S5CloseoutReservedScope::LayoutIndexes)
            && self.contains(S5CloseoutReservedScope::S10Repair)
            && self.contains(S5CloseoutReservedScope::S11Security)
            && self.contains(S5CloseoutReservedScope::S12Certification)
    }
}
