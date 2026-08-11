//! Whether an installed operation still matches one re-presented candidate.

use worth_query_declaration::facade::application_schema::ApplicationSchemaMember;

use crate::installed_index::WorthQueryInstalledPackageAuthority;

use super::super::installed_contract_support::authority_transcript;
use super::operation_compilation::WorthQueryApplicationOperationCompilation;
use super::WorthQueryInstalledApplicationOperation;

impl<Schema, Operation, Input> WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    pub(crate) fn meaning_matches(&self, members: &[ApplicationSchemaMember]) -> bool {
        self.recompiled_contracts_match(members)
    }

    fn recompiled_contracts_match(&self, members: &[ApplicationSchemaMember]) -> bool {
        let Ok(compilation) = WorthQueryApplicationOperationCompilation::resolve(
            self.binding_identity.clone(),
            members,
            &self.operation,
            &self.input_type,
        ) else {
            return false;
        };
        let Ok(candidate_contracts) =
            compilation.compile_contracts(self.contracts.ability_requirements().to_vec())
        else {
            return false;
        };
        candidate_contracts == self.contracts
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        authority_transcript(
            &package.authority_key,
            &self.binding_identity,
            &self.operation,
            &self.input_type,
            self.obligations.identity(),
        )
        .verifies(&self.authority_identity)
    }
}
