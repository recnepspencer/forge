#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalBranchTransactionAdmissionDenial {
    ForeignRuntime {
        expected_runtime_instance_id: u64,
        actual_runtime_instance_id: u64,
    },
    BasisIdentityMismatch,
}

impl crate::runtime::RelationalRuntime {
    pub fn begin_branch_transaction(
        &self,
        basis: &crate::branch::AdmittedRelationalBranchBasis,
        intent: super::RelationalTransactionIntent,
    ) -> Result<super::BranchBoundRelationalTransaction, RelationalBranchTransactionAdmissionDenial>
    {
        if basis.identity().runtime_instance_id() != self.runtime_instance_id() {
            return Err(RelationalBranchTransactionAdmissionDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: basis.identity().runtime_instance_id(),
            });
        }
        if basis.descriptor().runtime_instance_id() != self.runtime_instance_id()
            || basis.descriptor().branch_id() != basis.identity().branch_id()
            || basis.descriptor().root_identity() != basis.inner.root.id()
        {
            return Err(RelationalBranchTransactionAdmissionDenial::BasisIdentityMismatch);
        }

        let footprint = super::RelationalTransactionFootprint::for_basis(basis);
        Ok(super::BranchBoundRelationalTransaction {
            basis: basis.clone(),
            mutation_authority: crate::branch::issue_relational_branch_mutation_authority(),
            transaction_id: self.services.next_transaction_id(),
            intent,
            merge_parent_bases: Vec::new(),
            schema_authority_input: None,
            schema_authority: basis.inner.root.retained_schema_authority(),
            overlay: super::DetachedRelationalTransactionOverlay::default(),
            footprint,
            savepoints: Vec::new(),
            next_savepoint_ordinal: 1,
            last_merged_plan: None,
            client_key_symbol_policy: self.config.identity.client_key_symbol_policy,
        })
    }

    pub(crate) fn begin_branch_transaction_with_owner_inputs(
        &self,
        owner_inputs: crate::mvcc::RelationalTransactionValidationInput,
    ) -> Result<super::BranchBoundRelationalTransaction, RelationalBranchTransactionAdmissionDenial>
    {
        let mut transaction =
            self.begin_branch_transaction(owner_inputs.basis(), owner_inputs.intent().clone())?;
        owner_inputs.apply_owner_inputs_to(&mut transaction);
        Ok(transaction)
    }
}
