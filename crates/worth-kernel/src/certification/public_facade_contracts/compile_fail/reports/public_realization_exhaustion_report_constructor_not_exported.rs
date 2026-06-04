use worth_kernel::facade::{
    diagnostics::realization::PrimitiveConstructionRealizationExhaustionReport,
    PrimitiveConstructionFamily, PrimitiveConstructionRealizationExhaustionStatus,
};

fn main() {
    let _ = PrimitiveConstructionRealizationExhaustionReport {
        family: PrimitiveConstructionFamily::Orthotope,
        status: PrimitiveConstructionRealizationExhaustionStatus::NotApplicable,
        attempted_strategies: Vec::new(),
        stability_class: None,
        exhaustion_reason: None,
        conditioning_witness: None,
        report_digest: String::new(),
    };
}
