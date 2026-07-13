use forge_store_layout_indexes::access_planning::{
    AccessAuthorityPosture, AccessLaneClassification, AccessPlanIdentity, AccessShape,
    AccessStaleDisposition,
};
use forge_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;

fn main() {
    let _ = AccessPlanIdentity::new(
        LayoutStrategyFamily::ExactScan,
        AccessShape::DegradedExactScan,
        AccessLaneClassification::Foreground,
        AccessAuthorityPosture::ExplicitDegradedExactScan,
        AccessStaleDisposition::ExplicitDegradedFallback,
        todo!(),
    );
}
