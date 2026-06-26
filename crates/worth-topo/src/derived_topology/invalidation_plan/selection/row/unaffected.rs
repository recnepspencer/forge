use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyProductFamilyIdentity, DerivedTopologyProductFamilyRecord,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationUnaffectedRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    family_digest: String,
    execution_work_count: usize,
    row_digest: String,
}

impl DerivedInvalidationUnaffectedRow {
    pub(crate) fn from_family(family: &DerivedTopologyProductFamilyRecord) -> Self {
        let execution_work_count = 0;
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-unaffected-row:v1".to_string(),
            format!("family:{}", family.identity().as_str()),
            format!("family-digest:{}", family.family_digest()),
            format!("execution-work:{execution_work_count}"),
        ]);
        Self {
            family_identity: family.identity(),
            family_digest: family.family_digest().to_string(),
            execution_work_count,
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
