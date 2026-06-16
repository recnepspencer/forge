use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{AdmittedSourceContract, SourceDeclaration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSourceDeclaration {
    declaration: SourceDeclaration,
    contract_identity: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ValidatedSourceDeclaration {
    pub(crate) fn from_contract(contract: &AdmittedSourceContract) -> Self {
        let declaration = contract.declaration().clone();
        let canonical_basis = Arc::<str>::from(format!(
            "validated-source-declaration|contract={}|declaration={}",
            contract.digest(),
            declaration.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration,
            contract_identity: Arc::from(contract.contract_identity().as_str()),
            canonical_basis,
            digest: Arc::from(format!("validated-source-declaration:sha256:{digest:x}")),
        }
    }

    pub fn declaration(&self) -> &SourceDeclaration {
        &self.declaration
    }

    pub fn contract_identity(&self) -> &str {
        self.contract_identity.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::ValidatedSourceDeclaration;

    use crate::snapshot::BridgeTruthViewSelector;
    use crate::source::{
        AdmittedSourceRegistry, BridgeSourceCapability, BridgeSourceCapabilitySet,
        SourceDeclaration, SourceDeclarationIdentity,
    };

    #[test]
    fn validated_source_declaration_is_canonical_for_same_inputs() {
        let declaration = SourceDeclaration::new(
            SourceDeclarationIdentity::admit_bridge_owned("source:profile"),
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            ),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
        );
        let registry = AdmittedSourceRegistry::freeze(vec![declaration.clone()])
            .expect("source registry should freeze");
        let contract = registry
            .contract_for_declaration(&declaration)
            .expect("contract should exist")
            .clone();

        let left = ValidatedSourceDeclaration::from_contract(&contract);
        let right = ValidatedSourceDeclaration::from_contract(&contract);

        assert_eq!(left, right);
        assert_eq!(
            left.canonical_basis(),
            format!(
                "validated-source-declaration|contract={}|declaration={}",
                contract.digest(),
                declaration.digest(),
            )
        );
        assert_eq!(
            left.contract_identity(),
            contract.contract_identity().as_str()
        );
    }
}
