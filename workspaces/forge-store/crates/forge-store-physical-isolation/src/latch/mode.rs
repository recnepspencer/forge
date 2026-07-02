#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalLatchMode {
    Shared,
    Exclusive,
}

impl PhysicalLatchMode {
    pub const fn permits_upgrade_to(self, requested: Self) -> bool {
        matches!((self, requested), (Self::Shared, Self::Exclusive))
    }
}
