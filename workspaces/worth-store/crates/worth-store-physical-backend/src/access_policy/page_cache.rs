#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageCachePolicyKind {
    StoreAdmittedVisibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageCachePolicyProof {
    kind: PageCachePolicyKind,
    _seal: PageCachePolicySeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PageCachePolicySeal;

impl PageCachePolicyProof {
    pub(crate) const fn store_admitted_visibility() -> Self {
        Self {
            kind: PageCachePolicyKind::StoreAdmittedVisibility,
            _seal: PageCachePolicySeal,
        }
    }

    pub const fn kind(self) -> PageCachePolicyKind {
        self.kind
    }
}
