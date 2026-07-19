#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeliveryWidthClass {
    ScalarDetail,
    NarrowCollection,
    GroupedDelta,
    DiffDelta,
    DeniedWidthInflation,
}

impl DeliveryWidthClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ScalarDetail => "scalar_detail",
            Self::NarrowCollection => "narrow_collection",
            Self::GroupedDelta => "grouped_delta",
            Self::DiffDelta => "diff_delta",
            Self::DeniedWidthInflation => "denied_width_inflation",
        }
    }
    #[cfg(test)]
    pub(crate) fn budget_limit(&self) -> usize {
        match self {
            Self::ScalarDetail => 4,
            Self::NarrowCollection => 16,
            Self::GroupedDelta => 24,
            Self::DiffDelta => 24,
            Self::DeniedWidthInflation => 0,
        }
    }
}
