mod execution_receipt;
mod family_catalog;
mod family_declaration;
mod grouped_input;
mod plan_lowering;
mod selected_plan;

pub use execution_receipt::{
    execute_selected_batch_admission_plan, BatchAdmissionExecutionCounters,
    BatchAdmissionExecutionReceipt,
};
pub use family_catalog::{
    current_batch_admission_family_catalog_closeout, BatchAdmissionFamilyCatalog,
    BatchAdmissionFamilyCatalogCloseout,
};
pub use family_declaration::{
    BatchAdmissionAdvisoryWitnessShape, BatchAdmissionFamilyDeclaration,
    BatchAdmissionFamilyDeclarationInput, BatchAdmissionFamilyIdentity,
    BatchAdmissionFamilyPosture, BatchAdmissionIndependenceRequirement,
};
pub use grouped_input::{
    admit_batch_admission_grouped_input, AdmittedBatchAdmissionGroupedInput,
    BatchAdmissionCandidate, BatchAdmissionGroupedInput, BatchAdmissionGroupedInputAdmissionError,
    BatchAdmissionGroupedInputAdmissionErrorKind, BatchAdmissionPairwiseIndependenceProof,
};
pub use plan_lowering::lower_selected_batch_admission_plan;
pub use selected_plan::{
    BatchAdmissionPlanAdvisory, BatchAdmissionPlanDenial, BatchAdmissionPlanDenialKind,
    BatchAdmissionSelectedFamilyRow, BatchAdmissionSupportingConflictFamilyRow,
    BatchAdmissionSupportingConflictLane, SelectedBatchAdmissionPlan,
};

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_execution_receipt;
#[cfg(test)]
mod tests_hardening;
#[cfg(test)]
mod tests_selected_plan;
