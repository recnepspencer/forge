use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::identity::data::KindId;

use super::{AspectContractPlanRevision, RelationIntegrityPlanRevision, SchemaId, SchemaVersionId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaAuthoritySnapshot {
    pub primary_schema_id: Option<SchemaId>,
    pub primary_schema_version_id: Option<SchemaVersionId>,
    pub entity_kinds: Vec<SchemaAuthorityKindSnapshot>,
    pub relation_kinds: Vec<SchemaAuthorityRelationSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaAuthorityKindSnapshot {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub aspect_plan_revision: AspectContractPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaAuthorityRelationSnapshot {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub aspect_plan_revision: AspectContractPlanRevision,
    pub relation_integrity_plan_revision: RelationIntegrityPlanRevision,
}

pub fn schema_authority_snapshot_digest_bytes(snapshot: &SchemaAuthoritySnapshot) -> [u8; 32] {
    let mut hasher = Sha256::new();
    if let Some(schema_id) = &snapshot.primary_schema_id {
        hasher.update(b"primary_schema_id:");
        hasher.update(schema_id.0.as_bytes());
    } else {
        hasher.update(b"primary_schema_id:none");
    }
    if let Some(schema_version_id) = snapshot.primary_schema_version_id {
        hasher.update(b"primary_schema_version:");
        hasher.update(schema_version_id.0.to_le_bytes());
    } else {
        hasher.update(b"primary_schema_version:none");
    }
    for entity_kind in &snapshot.entity_kinds {
        hasher.update(b"entity_kind");
        hasher.update(entity_kind.kind_id.0.to_le_bytes());
        hasher.update(entity_kind.kind_name.as_bytes());
        hasher.update(entity_kind.schema_id.0.as_bytes());
        hasher.update(entity_kind.schema_version_id.0.to_le_bytes());
        hasher.update(entity_kind.aspect_plan_revision.0.to_le_bytes());
    }
    for relation_kind in &snapshot.relation_kinds {
        hasher.update(b"relation_kind");
        hasher.update(relation_kind.kind_id.0.to_le_bytes());
        hasher.update(relation_kind.kind_name.as_bytes());
        hasher.update(relation_kind.schema_id.0.as_bytes());
        hasher.update(relation_kind.schema_version_id.0.to_le_bytes());
        hasher.update(relation_kind.aspect_plan_revision.0.to_le_bytes());
        hasher.update(
            relation_kind
                .relation_integrity_plan_revision
                .0
                .to_le_bytes(),
        );
    }
    let digest = hasher.finalize();
    let mut out = [0_u8; 32];
    out.copy_from_slice(&digest);
    out
}
