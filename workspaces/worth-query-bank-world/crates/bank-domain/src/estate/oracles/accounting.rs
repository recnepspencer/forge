use crate::estate::{BankEstateWorld, EstateAction, EstateDenial};

pub(super) fn validate(world: &BankEstateWorld, action: EstateAction) -> Result<(), EstateDenial> {
    let EstateAction::DisburseEstate(disbursement) = action else {
        return Ok(());
    };
    let [debit, credit] = disbursement.postings;
    if debit.account != disbursement.source_account
        || credit.account != disbursement.destination_account
        || debit.account == credit.account
    {
        return Err(EstateDenial::AccountingShapeInvalid);
    }
    let expected = disbursement.amount.minor_units();
    if debit.amount.minor_units() != -expected || credit.amount.minor_units() != expected {
        return Err(EstateDenial::AccountingShapeInvalid);
    }
    let balance = world
        .balance(disbursement.source_account)
        .ok_or(EstateDenial::InsufficientEstateFunds)?;
    if balance.minor_units() < expected {
        return Err(EstateDenial::InsufficientEstateFunds);
    }
    Ok(())
}
