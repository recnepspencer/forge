use worth_query::facade::runtime::{WorthQueryGraphCompositionAdmissionTrace, WorthQueryGraphCompositionAdmissionTraceStage};

fn main() {
    let _ = WorthQueryGraphCompositionAdmissionTrace {
        stages: vec![
            WorthQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
            WorthQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
        ],
        failure_stage: WorthQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
        admission_trace_digest: String::new(),
    };
}
