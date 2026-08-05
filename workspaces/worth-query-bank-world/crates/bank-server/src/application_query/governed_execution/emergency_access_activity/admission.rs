use bank_domain::queries::EstateEmergencyAccessActivityRequest;
use worth_query_host::facade::primary_graph::WorthQueryApprovedElevation;

use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankReadControls};

pub(crate) struct BankEstateEmergencyAccessActivityAdmission<
    'runtime,
    'principal,
    'approved,
    'controls,
> {
    pub(super) runtime: &'runtime BankIdentityRuntime,
    pub(super) principal: &'principal BankAuthenticatedPrincipal,
    pub(super) request: EstateEmergencyAccessActivityRequest,
    pub(super) approved: &'approved WorthQueryApprovedElevation,
    pub(super) controls: &'controls BankReadControls,
}

impl<'runtime, 'principal, 'approved, 'controls>
    BankEstateEmergencyAccessActivityAdmission<'runtime, 'principal, 'approved, 'controls>
{
    pub(crate) const fn new(
        runtime: &'runtime BankIdentityRuntime,
        principal: &'principal BankAuthenticatedPrincipal,
        request: EstateEmergencyAccessActivityRequest,
        approved: &'approved WorthQueryApprovedElevation,
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
