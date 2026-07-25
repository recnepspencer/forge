mod labels;
mod projection;

pub(crate) use projection::{hex, mutant_value, source_value};
pub(crate) use projection::{process_value, run_environment_value};
pub use projection::{
    project_physical_work_courtroom_evidence, PhysicalWorkCourtroomTerminalProjection,
    PHYSICAL_WORK_COURTROOM_EVIDENCE_SCHEMA,
};

pub fn decode_physical_work_mutant_localization(
    encoded: &str,
) -> Result<worth_store::physical_runtime::PhysicalWorkMutantLocalization, String> {
    crate::mutation_campaign::evidence::decode_physical_work_localization(encoded)
}
