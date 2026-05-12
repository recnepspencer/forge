use forge_query::facade::{
    ForgeQueryGraphCompositionAdmissionTrace, ForgeQueryGraphCompositionAdmissionTraceStage,
};

fn main() {
    let _ = ForgeQueryGraphCompositionAdmissionTrace {
        stages: vec![
            ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
            ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
        ],
        failure_stage: ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
        admission_trace_digest: String::new(),
    };
}
