#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveDeliveryDensityPosture {
    SparseDelta,
    BurstCoalesced,
    DenseRefreshDenied,
    DenseRefreshDebtExplicit,
}

impl ActiveDeliveryDensityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SparseDelta => "sparse_delta",
            Self::BurstCoalesced => "burst_coalesced",
            Self::DenseRefreshDenied => "dense_refresh_denied",
            Self::DenseRefreshDebtExplicit => "dense_refresh_debt_explicit",
        }
    }
}
