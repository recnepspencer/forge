use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::catalog::TopologyConflictFamilyCatalog;
use super::current_catalog::current_topology_conflict_family_catalog;
use super::family_identity::TopologyConflictFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConflictFamilyCatalogCloseout {
    catalog: TopologyConflictFamilyCatalog,
    catalog_digest: String,
}

impl TopologyConflictFamilyCatalogCloseout {
    pub fn close(catalog: TopologyConflictFamilyCatalog) -> Result<Self, &'static str> {
        let mut seen = BTreeSet::new();
        for declaration in catalog.declarations() {
            if !seen.insert(declaration.identity().as_str()) {
                return Err("duplicate topology conflict family identity");
            }
        }
        for required in [
            TopologyConflictFamilyIdentity::AspectSelection,
            TopologyConflictFamilyIdentity::ValidatorSelection,
            TopologyConflictFamilyIdentity::ReplayBoundarySelection,
        ] {
            if catalog.family(required).is_none() {
                return Err("missing required topology conflict family");
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

    pub const fn catalog(&self) -> &TopologyConflictFamilyCatalog {
        &self.catalog
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

pub fn current_topology_conflict_family_catalog_closeout(
) -> Result<TopologyConflictFamilyCatalogCloseout, &'static str> {
    TopologyConflictFamilyCatalogCloseout::close(current_topology_conflict_family_catalog())
}

fn catalog_digest_parts(catalog: &TopologyConflictFamilyCatalog) -> Vec<String> {
    let mut parts = vec!["worth-topo:conflict-family-catalog-closeout:v1".to_string()];
    let mut declarations = catalog
        .declarations()
        .iter()
        .map(|declaration| format!("declaration:{}", declaration.declaration_digest()))
        .collect::<Vec<_>>();
    declarations.sort();
    parts.extend(declarations);
    parts
}
