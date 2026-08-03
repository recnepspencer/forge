use worth_foundational::{
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceBundleConstructionDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalPerformanceEvidenceDenial {
    PerformanceBundleDenied(FoundationalPerformanceBundleConstructionDenial),
    PerformanceReceiptDenied(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}
