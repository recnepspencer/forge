mod declarations;
mod execution;

pub use declarations::queries;
pub(crate) use execution::map_admission_denial as map_read_admission_denial;

use super::BankReadControls;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

pub struct BankQuery<'runtime, Query> {
    runtime: &'runtime BankIdentityRuntime,
    query: Query,
}

pub struct BankQueryForPrincipal<'runtime, 'principal, Query> {
    runtime: &'runtime BankIdentityRuntime,
    query: Query,
    principal: &'principal BankAuthenticatedPrincipal,
}

pub struct BankReadyQuery<'runtime, 'principal, Query> {
    runtime: &'runtime BankIdentityRuntime,
    query: Query,
    principal: &'principal BankAuthenticatedPrincipal,
    controls: BankReadControls,
}

impl BankIdentityRuntime {
    pub const fn query<Query>(&self, query: Query) -> BankQuery<'_, Query> {
        BankQuery {
            runtime: self,
            query,
        }
    }
}

impl<'runtime, Query> BankQuery<'runtime, Query> {
    pub fn as_principal<'principal>(
        self,
        principal: &'principal BankAuthenticatedPrincipal,
    ) -> BankQueryForPrincipal<'runtime, 'principal, Query> {
        BankQueryForPrincipal {
            runtime: self.runtime,
            query: self.query,
            principal,
        }
    }
}

impl<'runtime, 'principal, Query> BankQueryForPrincipal<'runtime, 'principal, Query> {
    pub fn controls(
        self,
        controls: BankReadControls,
    ) -> BankReadyQuery<'runtime, 'principal, Query> {
        BankReadyQuery {
            runtime: self.runtime,
            query: self.query,
            principal: self.principal,
            controls,
        }
    }

    pub(crate) const fn runtime(&self) -> &'runtime BankIdentityRuntime {
        self.runtime
    }

    pub(crate) const fn principal(&self) -> &'principal BankAuthenticatedPrincipal {
        self.principal
    }
}

impl BankQueryForPrincipal<'_, '_, queries::AccountActivity> {
    pub(crate) const fn query_account(&self) -> bank_domain::model::AccountId {
        self.query.account()
    }
}
