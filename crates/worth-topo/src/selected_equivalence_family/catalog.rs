use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::declaration::TopologySelectedEquivalenceFamilyDeclaration;
use super::family_identity::TopologySelectedEquivalenceFamilyIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedEquivalenceFamilyCatalog {
    declarations: Vec<TopologySelectedEquivalenceFamilyDeclaration>,
    catalog_digest: String,
}

impl TopologySelectedEquivalenceFamilyCatalog {
    pub(crate) fn current() -> Self {
        let declarations =
            vec![TopologySelectedEquivalenceFamilyDeclaration::derived_topology_semantic_parity()];
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:selected-equivalence-family-catalog:v1".to_string(),
                declarations[0].family_digest().to_string(),
            ],
        );
        Self {
            declarations,
            catalog_digest,
        }
    }

    pub fn declaration(
        &self,
        identity: TopologySelectedEquivalenceFamilyIdentity,
    ) -> Option<&TopologySelectedEquivalenceFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }

    pub fn family_for_compiled_product(
        &self,
        compiled_product_family_identity: crate::compiled_product_family::TopologyCompiledProductFamilyIdentity,
    ) -> Option<&TopologySelectedEquivalenceFamilyDeclaration> {
        self.declarations.iter().find(|declaration| {
            declaration.compiled_product_family_identity() == compiled_product_family_identity
        })
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

pub fn current_topology_selected_equivalence_family_catalog(
) -> TopologySelectedEquivalenceFamilyCatalog {
    TopologySelectedEquivalenceFamilyCatalog::current()
}
