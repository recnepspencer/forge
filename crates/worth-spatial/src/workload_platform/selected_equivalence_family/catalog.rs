use crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::declaration::SpatialSelectedEquivalenceFamilyDeclaration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedEquivalenceFamilyCatalog {
    declarations: Vec<SpatialSelectedEquivalenceFamilyDeclaration>,
    catalog_digest: String,
}

impl SpatialSelectedEquivalenceFamilyCatalog {
    pub(crate) fn current() -> Self {
        let declarations = vec![
            SpatialSelectedEquivalenceFamilyDeclaration::evidence_lookup_semantic_parity(),
            SpatialSelectedEquivalenceFamilyDeclaration::retained_cancellation_semantic_parity(),
            SpatialSelectedEquivalenceFamilyDeclaration::retained_replay_semantic_parity(),
        ];
        let mut parts = vec!["worth-spatial:selected-equivalence-family-catalog:v1".to_string()];
        parts.extend(
            declarations
                .iter()
                .map(|declaration| declaration.family_digest().to_string()),
        );
        let catalog_digest = truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts);
        Self {
            declarations,
            catalog_digest,
        }
    }

    pub fn family_for_compiled_product(
        &self,
        compiled_product_family_identity: SpatialCompiledProductFamilyIdentity,
    ) -> Option<&SpatialSelectedEquivalenceFamilyDeclaration> {
        self.declarations.iter().find(|declaration| {
            declaration.compiled_product_family_identity() == compiled_product_family_identity
        })
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

pub fn current_spatial_selected_equivalence_family_catalog(
) -> SpatialSelectedEquivalenceFamilyCatalog {
    SpatialSelectedEquivalenceFamilyCatalog::current()
}
