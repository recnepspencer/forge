use bank_domain::{
    estate::{BankEstateWorld, EstateCaseId},
    model::BankPrincipalId,
};

pub(super) fn install_present_beneficiary(
    estate: BankEstateWorld,
    estate_id: EstateCaseId,
    beneficiary: BankPrincipalId,
) -> BankEstateWorld {
    estate.with_beneficiary(estate_id, beneficiary)
}
