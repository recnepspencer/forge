mod declarations;
mod execution;

pub use declarations::mutations;

use super::BankMutationControls;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

pub struct BankMutation<'runtime, Mutation> {
    runtime: &'runtime BankIdentityRuntime,
    mutation: Mutation,
}

pub struct BankMutationForPrincipal<'runtime, 'principal, Mutation> {
    runtime: &'runtime BankIdentityRuntime,
    mutation: Mutation,
    principal: &'principal BankAuthenticatedPrincipal,
}

pub struct BankReadyMutation<'runtime, 'principal, Mutation> {
    runtime: &'runtime BankIdentityRuntime,
    mutation: Mutation,
    principal: &'principal BankAuthenticatedPrincipal,
    controls: BankMutationControls,
}

impl BankIdentityRuntime {
    pub const fn mutate<Mutation>(&self, mutation: Mutation) -> BankMutation<'_, Mutation> {
        BankMutation {
            runtime: self,
            mutation,
        }
    }
}

impl<'runtime, Mutation> BankMutation<'runtime, Mutation> {
    pub fn as_principal<'principal>(
        self,
        principal: &'principal BankAuthenticatedPrincipal,
    ) -> BankMutationForPrincipal<'runtime, 'principal, Mutation> {
        BankMutationForPrincipal {
            runtime: self.runtime,
            mutation: self.mutation,
            principal,
        }
    }
}

impl<'runtime, 'principal, Mutation> BankMutationForPrincipal<'runtime, 'principal, Mutation> {
    pub fn controls(
        self,
        controls: BankMutationControls,
    ) -> BankReadyMutation<'runtime, 'principal, Mutation> {
        BankReadyMutation {
            runtime: self.runtime,
            mutation: self.mutation,
            principal: self.principal,
            controls,
        }
    }
}
