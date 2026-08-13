use bank_domain::queries::EstateEmergencyAccessActivityRequest;

use crate::{
    BankApprovedEstateElevation, BankAuthenticatedPrincipal, BankIdentityRuntime, BankReadControls,
};

pub(crate) struct BankEstateEmergencyAccessActivityAdmission<
    'runtime,
    'principal,
    'approved,
    'controls,
> {
    pub(super) runtime: &'runtime BankIdentityRuntime,
    pub(super) principal: &'principal BankAuthenticatedPrincipal,
    pub(super) request: EstateEmergencyAccessActivityRequest,
    pub(super) approved: &'approved BankApprovedEstateElevation,
    pub(super) controls: &'controls BankReadControls,
}

impl<'runtime, 'principal, 'approved, 'controls>
    BankEstateEmergencyAccessActivityAdmission<'runtime, 'principal, 'approved, 'controls>
{
    pub(crate) const fn new(
        runtime: &'runtime BankIdentityRuntime,
        principal: &'principal BankAuthenticatedPrincipal,
        request: EstateEmergencyAccessActivityRequest,
        approved: &'approved BankApprovedEstateElevation,
        controls: &'controls BankReadControls,
    ) -> Self {
        Self {
            runtime,
            principal,
            request,
            approved,
            controls,
        }
    }
}
