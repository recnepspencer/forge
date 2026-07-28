mod account_access;
mod account_creation;
mod business_payment;
mod money_movement;
mod reversal;

use bank_domain::proposals::BankInvariantApprovedProposal;

use crate::BankAdmittedOperation;

/// Typed phase progression retaining both installed operation admission and
/// the bank-domain invariant witness for the future compare-and-commit phase.
///
/// ```compile_fail
/// use bank_server::BankAuthorizedProposal;
///
/// let _ = BankAuthorizedProposal::<(), (), (), u64> {
///     admission: todo!(),
///     invariant: todo!(),
/// };
/// ```
pub struct BankAuthorizedProposal<Operation, Input, Scope, ScopeIdentity> {
    admission: BankAdmittedOperation<Operation, Input, Scope, ScopeIdentity>,
    invariant: BankInvariantApprovedProposal,
}

impl<Operation, Input, Scope, ScopeIdentity>
    BankAuthorizedProposal<Operation, Input, Scope, ScopeIdentity>
{
    pub(crate) const fn new(
        admission: BankAdmittedOperation<Operation, Input, Scope, ScopeIdentity>,
        invariant: BankInvariantApprovedProposal,
    ) -> Self {
        Self {
            admission,
            invariant,
        }
    }

    pub const fn admission(
        &self,
    ) -> &BankAdmittedOperation<Operation, Input, Scope, ScopeIdentity> {
        &self.admission
    }

    pub const fn invariant(&self) -> &BankInvariantApprovedProposal {
        &self.invariant
    }
}

pub struct BankOperationProposals;
