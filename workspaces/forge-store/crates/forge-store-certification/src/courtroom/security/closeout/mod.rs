mod api_adoption;
mod denial;
mod input;

pub use crate::evidence::security::{
    S51CertificationCloseoutEvidence, S51CloseoutBoundaryEvidencePublication,
    S51CloseoutCounterMatrix, S51CloseoutFoundationalBoundaryPackage, S51CloseoutFoundationalLane,
    S51CloseoutPerformanceReceipts, S51CloseoutPerformanceRows,
};
pub use api_adoption::S51CloseoutApiAdoptionEvidence;
pub use denial::S51CertificationCloseoutDenial;
pub use input::{S51CertificationCloseoutInput, S51CertificationEvidencePolicy};

pub fn certify_security_scope_closeout(
    input: S51CertificationCloseoutInput,
) -> Result<S51CertificationCloseoutEvidence, S51CertificationCloseoutDenial> {
    let counter_matrix = S51CloseoutCounterMatrix::from_input(&input)?;
    let performance_receipts =
        S51CloseoutPerformanceReceipts::from_counter_matrix(&counter_matrix)?;
    let boundary_evidence = S51CloseoutBoundaryEvidencePublication::from_input_and_counter_matrix(
        &input,
        &counter_matrix,
        &performance_receipts,
    )?;
    let api_adoption = S51CloseoutApiAdoptionEvidence::from_boundary_publication(
        &boundary_evidence,
        &performance_receipts,
    )?;
    Ok(S51CertificationCloseoutEvidence::new(
        input,
        counter_matrix,
        performance_receipts,
        boundary_evidence,
        api_adoption,
    ))
}
