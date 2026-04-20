use serde::{Deserialize, Serialize};

use super::core::{AspectLayoutSliceId, EquivalenceContractVersion, StructuralBlockId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralBlockLookup {
    structural_block_id: StructuralBlockId,
}
impl StructuralBlockLookup {
    pub fn new(structural_block_id: StructuralBlockId) -> Self { Self { structural_block_id } }
    pub fn structural_block_id(&self) -> &StructuralBlockId { &self.structural_block_id }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StructuralBlockLookupResult {
    structural_block_id: StructuralBlockId,
    scope_class: String,
    equivalence_contract_version: EquivalenceContractVersion,
    slice_ids: Vec<AspectLayoutSliceId>,
    supporting_layout_materialization_artifact_ids: Vec<String>,
}
impl StructuralBlockLookupResult {
    pub(crate) fn new(
        structural_block_id: StructuralBlockId,
        scope_class: String,
        equivalence_contract_version: EquivalenceContractVersion,
        slice_ids: Vec<AspectLayoutSliceId>,
        supporting_layout_materialization_artifact_ids: Vec<String>,
    ) -> Self {
        Self { structural_block_id, scope_class, equivalence_contract_version, slice_ids, supporting_layout_materialization_artifact_ids }
    }
    pub fn structural_block_id(&self) -> &StructuralBlockId { &self.structural_block_id }
    pub fn scope_class(&self) -> &str { &self.scope_class }
    pub fn equivalence_contract_version(&self) -> EquivalenceContractVersion { self.equivalence_contract_version }
    pub fn slice_ids(&self) -> &[AspectLayoutSliceId] { &self.slice_ids }
    pub fn supporting_layout_materialization_artifact_ids(&self) -> &[String] {
        &self.supporting_layout_materialization_artifact_ids
    }
}
