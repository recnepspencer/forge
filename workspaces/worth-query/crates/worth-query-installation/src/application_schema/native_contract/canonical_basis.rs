use worth_foundational::facade::{
    canonical_basis_sequence_material, prepare_aspect_contract_for_canonical_basis, AspectContract,
    CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};

use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

use super::denial::{
    WorthQueryApplicationSchemaContractCatalogDenial,
    WorthQueryApplicationSchemaContractCatalogDenialKind as DenialKind,
};

const CATALOG_TO_SCHEMA_CANONICAL_WORK_FACTOR: usize = 8;

pub(super) struct CatalogCanonicalBudget {
    maximum_entries: usize,
    maximum_bytes: usize,
    consumed_entries: usize,
    consumed_bytes: usize,
    work: WorthQueryCanonicalWorkEvidence,
}

impl CatalogCanonicalBudget {
    pub(super) fn from_schema_work(
        schema_work: WorthQueryCanonicalWorkEvidence,
    ) -> Result<Self, WorthQueryApplicationSchemaContractCatalogDenial> {
        let maximum_entries = (schema_work.canonical_entries() as usize)
            .checked_mul(CATALOG_TO_SCHEMA_CANONICAL_WORK_FACTOR)
            .ok_or_else(|| entry_budget_denial("native-contract-catalog"))?;
        let maximum_bytes = schema_work
            .canonical_encoded_bytes()
            .checked_mul(CATALOG_TO_SCHEMA_CANONICAL_WORK_FACTOR)
            .ok_or_else(|| byte_budget_denial("native-contract-catalog"))?;
        Ok(Self {
            maximum_entries,
            maximum_bytes,
            consumed_entries: 0,
            consumed_bytes: 0,
            work: WorthQueryCanonicalWorkEvidence::zero(),
        })
    }

    pub(super) fn prepare(
        &mut self,
        subject: &str,
        contract: &AspectContract,
    ) -> Result<PreparedCatalogContractBasis, WorthQueryApplicationSchemaContractCatalogDenial>
    {
        let version = CanonicalizationRuleVersion::new("worth-query-native-contract-v1")
            .expect("the fixed Query native-contract canonicalization version is valid");
        let basis = prepare_aspect_contract_for_canonical_basis(version, contract.clone())
            .into_result()
            .map_err(|_| canonical_denial(subject))?;
        let material = canonical_basis_sequence_material(basis.payload());
        self.admit(subject, basis.payload().entries().len(), material.len())?;
        Ok(PreparedCatalogContractBasis { basis, material })
    }

    fn admit(
        &mut self,
        subject: &str,
        entries: usize,
        bytes: usize,
    ) -> Result<(), WorthQueryApplicationSchemaContractCatalogDenial> {
        self.consumed_entries = self
            .consumed_entries
            .checked_add(entries)
            .ok_or_else(|| entry_budget_denial(subject))?;
        self.consumed_bytes = self
            .consumed_bytes
            .checked_add(bytes)
            .ok_or_else(|| byte_budget_denial(subject))?;
        if self.consumed_entries > self.maximum_entries {
            return Err(entry_budget_denial(subject));
        }
        if self.consumed_bytes > self.maximum_bytes {
            return Err(byte_budget_denial(subject));
        }
        let entries = u32::try_from(entries).map_err(|_| entry_budget_denial(subject))?;
        self.work = self
            .work
            .combine(WorthQueryCanonicalWorkEvidence::one_basis_preparation(
                entries, bytes,
            ));
        Ok(())
    }

    pub(super) const fn work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.work
    }
}

pub(super) struct PreparedCatalogContractBasis {
    pub(super) basis: CanonicalBasisReadyArtifact,
    pub(super) material: String,
}

fn canonical_denial(subject: &str) -> WorthQueryApplicationSchemaContractCatalogDenial {
    WorthQueryApplicationSchemaContractCatalogDenial::new(
        DenialKind::CanonicalContractRejected,
        subject,
    )
}

fn entry_budget_denial(subject: &str) -> WorthQueryApplicationSchemaContractCatalogDenial {
    WorthQueryApplicationSchemaContractCatalogDenial::new(
        DenialKind::CanonicalEntryBudgetExceeded,
        subject,
    )
}

fn byte_budget_denial(subject: &str) -> WorthQueryApplicationSchemaContractCatalogDenial {
    WorthQueryApplicationSchemaContractCatalogDenial::new(
        DenialKind::CanonicalEncodedByteBudgetExceeded,
        subject,
    )
}

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{
        aspects, AspectContractRevision, AspectIdentity, AspectKey, ScalarAspectType,
    };

    use super::{CatalogCanonicalBudget, DenialKind};
    use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

    #[test]
    fn prepared_contract_entries_cannot_cross_the_schema_derived_budget() {
        let mut budget = CatalogCanonicalBudget::from_schema_work(
            WorthQueryCanonicalWorkEvidence::one_basis_preparation(0, 10_000),
        )
        .unwrap();
        let denial = budget.prepare("entry-bound", &contract()).err().unwrap();
        assert_eq!(denial.kind(), DenialKind::CanonicalEntryBudgetExceeded);
    }

    #[test]
    fn prepared_contract_bytes_cannot_cross_the_schema_derived_budget() {
        let mut budget = CatalogCanonicalBudget::from_schema_work(
            WorthQueryCanonicalWorkEvidence::one_basis_preparation(10_000, 0),
        )
        .unwrap();
        let denial = budget.prepare("byte-bound", &contract()).err().unwrap();
        assert_eq!(
            denial.kind(),
            DenialKind::CanonicalEncodedByteBudgetExceeded
        );
    }

    fn contract() -> worth_foundational::facade::AspectContract {
        let shape = aspects()
            .struct_fields()
            .required("value", ScalarAspectType::UInt64)
            .finish()
            .unwrap();
        aspects()
            .contract()
            .for_key(AspectKey::new("BudgetAspect").unwrap())
            .identified_by(AspectIdentity(1))
            .at_revision(AspectContractRevision(1))
            .struct_aspect(shape)
    }
}
