use forge_store_layout_indexes::{
    S8AccessAuthorityPosture, S8AccessLaneClassification, S8AccessShape, S8AccessStaleDisposition,
    S8LayoutStrategyFamily, S8PlanFingerprint,
};

fn main() {
    let _ = S8PlanFingerprint::new(
        S8LayoutStrategyFamily::ExactScan,
        S8AccessShape::DegradedExactScan,
        S8AccessLaneClassification::Foreground,
        S8AccessAuthorityPosture::ExplicitDegradedExactScan,
        S8AccessStaleDisposition::ExplicitDegradedFallback,
        todo!(),
    );
}
