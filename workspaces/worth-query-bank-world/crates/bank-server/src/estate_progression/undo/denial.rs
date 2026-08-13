use worth_query_host::facade::provisional_aftermath::WorthQueryUndoDenial;

use crate::estate_progression::BankEstateProgressionDenial;
use crate::operation_admission::BankOperationAdmissionError;
use crate::operation_proposals::BankOperationProposalError;

pub(super) fn map_admission(denial: BankOperationAdmissionError) -> BankEstateProgressionDenial {
    match denial {
        BankOperationAdmissionError::Authorization(denial) => {
            BankEstateProgressionDenial::Authorization(denial)
        }
        _ => BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::current_policy_denied()),
    }
}

pub(super) fn map_proposal(denial: BankOperationProposalError) -> BankEstateProgressionDenial {
    let _ = denial;
    BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::conflicted())
}
