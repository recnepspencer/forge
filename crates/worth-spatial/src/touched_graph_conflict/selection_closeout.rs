use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::catalog::SpatialConflictFamilyCatalog;
use super::current_catalog::current_spatial_conflict_family_catalog;
use super::family_identity::SpatialConflictFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConflictFamilyCatalogCloseout {
    catalog: SpatialConflictFamilyCatalog,
    catalog_digest: String,
}

impl SpatialConflictFamilyCatalogCloseout {
    pub fn close(catalog: SpatialConflictFamilyCatalog) -> Result<Self, &'static str> {
        let mut seen = BTreeSet::new();
        for declaration in catalog.declarations() {
            if !seen.insert(declaration.identity().as_str()) {
                return Err("duplicate spatial conflict family identity");
            }
        }
        for required in [
            SpatialConflictFamilyIdentity::EvidenceSelection,
            SpatialConflictFamilyIdentity::ReplayBoundarySelection,
        ] {
            if catalog.family(required).is_none() {
                return Err("missing required spatial conflict family");
            }
        }
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &catalog_digest_parts(&catalog),
        );
        Ok(Self {
            catalog,
            catalog_digest,
        })
    }

    pub const fn catalog(&self) -> &SpatialConflictFamilyCatalog {
        &self.catalog
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

pub fn current_spatial_conflict_family_catalog_closeout(
) -> Result<SpatialConflictFamilyCatalogCloseout, &'static str> {
    SpatialConflictFamilyCatalogCloseout::close(current_spatial_conflict_family_catalog())
}

fn catalog_digest_parts(catalog: &SpatialConflictFamilyCatalog) -> Vec<String> {
    let mut parts = vec!["worth-spatial:conflict-family-catalog-closeout:v1".to_string()];
    let mut declarations = catalog
        .declarations()
        .iter()
        .map(|declaration| format!("declaration:{}", declaration.declaration_digest()))
        .collect::<Vec<_>>();
    declarations.sort();
    parts.extend(declarations);
    parts
}
