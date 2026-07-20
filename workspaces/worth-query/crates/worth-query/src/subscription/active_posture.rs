#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveSubscriptionLifecyclePosture {
    SingleConsumer,
    SharedEquivalent,
    PreviewScoped,
    ContinuationRemapped,
    DeniedMeaningMismatch,
    DeniedDurableOverclaim,
}

impl ActiveSubscriptionLifecyclePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleConsumer => "single_consumer",
            Self::SharedEquivalent => "shared_equivalent",
            Self::PreviewScoped => "preview_scoped",
            Self::ContinuationRemapped => "continuation_remapped",
            Self::DeniedMeaningMismatch => "denied_meaning_mismatch",
            Self::DeniedDurableOverclaim => "denied_durable_overclaim",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveSubscriptionDeliveryPosture {
    QueryShapedPatch,
    GroupedPatch,
    FocusedInspectorPatch,
    BoundedMaterializationPatch,
    DeniedRawCdcFallback,
    DebtExplicit,
}

impl ActiveSubscriptionDeliveryPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryShapedPatch => "query_shaped_patch",
            Self::GroupedPatch => "grouped_patch",
            Self::FocusedInspectorPatch => "focused_inspector_patch",
            Self::BoundedMaterializationPatch => "bounded_materialization_patch",
            Self::DeniedRawCdcFallback => "denied_raw_cdc_fallback",
            Self::DebtExplicit => "debt_explicit",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveLaneLookupClass {
    EquivalenceIndex,
    DirectIndexGeneration,
    LinearScanDebtExplicit,
    LinearScanDenied,
}

impl ActiveLaneLookupClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EquivalenceIndex => "equivalence_index",
            Self::DirectIndexGeneration => "direct_index_generation",
            Self::LinearScanDebtExplicit => "linear_scan_debt_explicit",
            Self::LinearScanDenied => "linear_scan_denied",
        }
    }
}
