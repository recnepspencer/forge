use forge_query::facade::ForgeQueryReadFamilyAdmission;

use super::catalog_key::{stable_digest, WorthGraphReadDeclarationCatalogKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadQueryFamilyAnchor {
    family_name: String,
    family_digest_seed: String,
    admission_boundary: ForgeQueryReadFamilyAdmission,
}

impl WorthGraphReadQueryFamilyAnchor {
    pub(crate) fn from_catalog_key(key: &WorthGraphReadDeclarationCatalogKey) -> Self {
        let family_name = format!("worth_graph_read_family_{}", key.key_digest());
        let family_digest_seed = stable_digest(&[
            "worth_graph_read_query_family_anchor_v1".to_string(),
            format!("family_name:{family_name}"),
            format!("catalog_key:{}", key.key_digest()),
            "query_admission_boundary:KernelOnly".to_string(),
        ]);
        Self {
            family_name,
            family_digest_seed,
            admission_boundary: ForgeQueryReadFamilyAdmission::KernelOnly,
        }
    }

    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    pub fn family_digest_seed(&self) -> &str {
        &self.family_digest_seed
    }

    pub fn admission_boundary(&self) -> &ForgeQueryReadFamilyAdmission {
        &self.admission_boundary
    }

    pub const fn claims_query_read_family_constructed(&self) -> bool {
        false
    }
}
