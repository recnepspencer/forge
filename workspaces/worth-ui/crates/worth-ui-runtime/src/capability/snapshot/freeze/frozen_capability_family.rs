/// Family-level structural fact carried by the frozen snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCapabilityFamily {
    family_name: &'static str,
    width: usize,
    digest_basis: u64,
}

impl FrozenCapabilityFamily {
    pub(crate) fn new(family_name: &'static str, width: usize, digest_basis: u64) -> Self {
        Self {
            family_name,
            width,
            digest_basis,
        }
    }

    pub fn family_name(&self) -> &'static str {
        self.family_name
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn digest_basis(&self) -> u64 {
        self.digest_basis
    }
}
