use crate::identity::data::{EntityId, KindId, LineageId, StructuralFingerprint, VersionId};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LifecycleCounts {
    pub(crate) live: usize,
    pub(crate) deleted: usize,
    pub(crate) reusable: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct VersionedEntityMetadata {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) generation: u32,
    pub(crate) kind_id: KindId,
    pub(crate) lineage_id: Option<LineageId>,
    pub(crate) authoritative_aspect_state:
        Option<forge_foundational::facade::AuthoritativeRecordAspectState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelationEndpoints {
    pub(crate) source: EntityId,
    pub(crate) target: EntityId,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RelationExtra {
    pub(crate) endpoints: Option<RelationEndpoints>,
    pub(crate) authoritative_aspect_state:
        Option<forge_foundational::facade::AuthoritativeRecordAspectState>,
}

#[derive(Debug, Clone)]
pub(crate) struct VersionedRelationMetadata {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) generation: u32,
    pub(crate) kind_id: KindId,
    pub(crate) endpoints: RelationEndpoints,
    pub(crate) authoritative_aspect_state:
        Option<forge_foundational::facade::AuthoritativeRecordAspectState>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct EntityExtra {
    pub(crate) structural_fingerprint: Option<StructuralFingerprint>,
    pub(crate) lineage_id: Option<LineageId>,
    pub(crate) authoritative_aspect_state:
        Option<forge_foundational::facade::AuthoritativeRecordAspectState>,
}
