#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanRegionIdentity {
    exact_basis: String,
    routing_fingerprint: u64,
}

impl WorthUiPlanRegionIdentity {
    pub(crate) fn from_exact_basis(exact_basis: impl Into<String>) -> Self {
        let exact_basis = exact_basis.into();
        let routing_fingerprint = route_exact_basis(&exact_basis);
        Self {
            exact_basis,
            routing_fingerprint,
        }
    }

    pub fn exact_basis(&self) -> &str {
        &self.exact_basis
    }

    pub fn routing_fingerprint(&self) -> u64 {
        self.routing_fingerprint
    }
}

impl Ord for WorthUiPlanRegionIdentity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.routing_fingerprint
            .cmp(&other.routing_fingerprint)
            .then_with(|| self.exact_basis.cmp(&other.exact_basis))
    }
}

impl PartialOrd for WorthUiPlanRegionIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn route_exact_basis(exact_basis: &str) -> u64 {
    exact_basis
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}
