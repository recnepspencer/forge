use crate::schema::data::{
    SchemaContinuationDescriptor, SchemaLineageArtifact, SchemaReconciliationDescriptor,
    SchemaTransitionArtifact,
};

use super::primitive_terms::ReplayDigestBuilder;

pub(crate) fn digest_schema_transition_descriptor(
    transition: &SchemaTransitionArtifact,
    semantics_version: crate::schema::data::DescriptorSemanticsVersion,
) -> [u8; 32] {
    ReplayDigestBuilder::new("forge.relational.replay.descriptor.schema_transition.v1")
        .descriptor_semantics_version(semantics_version)
        .schema_version_id(transition.source_schema_version_id)
        .schema_version_id(transition.target_schema_version_id)
        .usize(transition.diff_atoms.len())
        .label(transition.continuation_descriptor.bridge.continuation)
        .label(transition.continuation_descriptor.bridge.bridgeability)
        .label(transition.reconciliation_descriptor.classification)
        .label(transition.reconciliation_descriptor.policy)
        .finish()
}

pub(crate) fn digest_schema_transition_summary(transition: &SchemaTransitionArtifact) -> [u8; 32] {
    ReplayDigestBuilder::new("forge.relational.replay.summary.schema_transition.v1")
        .schema_version_id(transition.source_schema_version_id)
        .schema_version_id(transition.target_schema_version_id)
        .usize(transition.diff_atoms.len())
        .finish()
}

pub(crate) fn digest_schema_continuation_descriptor(
    descriptor: &SchemaContinuationDescriptor,
    semantics_version: crate::schema::data::DescriptorSemanticsVersion,
) -> [u8; 32] {
    ReplayDigestBuilder::new("forge.relational.replay.descriptor.schema_continuation.v1")
        .descriptor_semantics_version(semantics_version)
        .descriptor_canonicalization_version(descriptor.bridge.canonicalization_version)
        .boundary_fingerprint(descriptor.boundary_fingerprint)
        .label(descriptor.bridge.continuation)
        .label(descriptor.bridge.bridgeability)
        .label(descriptor.bridge.boundary_visibility)
        .usize(descriptor.normalized_boundary_count)
        .finish()
}

pub(crate) fn digest_schema_continuation_summary(
    descriptor: &SchemaContinuationDescriptor,
) -> [u8; 32] {
    ReplayDigestBuilder::new("forge.relational.replay.summary.schema_continuation.v1")
        .boundary_fingerprint(descriptor.boundary_fingerprint)
        .label(descriptor.bridge.continuation)
        .usize(descriptor.normalized_boundary_count)
        .finish()
}

pub(crate) fn digest_schema_reconciliation_descriptor(
    descriptor: &SchemaReconciliationDescriptor,
    semantics_version: crate::schema::data::DescriptorSemanticsVersion,
) -> [u8; 32] {
    ReplayDigestBuilder::new("forge.relational.replay.descriptor.schema_reconciliation.v1")
        .descriptor_semantics_version(semantics_version)
        .descriptor_canonicalization_version(descriptor.canonicalization_version)
        .label(descriptor.classification)
        .schema_version_id(descriptor.resulting_lineage.resulting_schema_version_id)
        .label(descriptor.resulting_lineage.ordering_mode)
        .finish()
}

pub(crate) fn digest_schema_reconciliation_summary(
    descriptor: &SchemaReconciliationDescriptor,
) -> [u8; 32] {
    ReplayDigestBuilder::new("forge.relational.replay.summary.schema_reconciliation.v1")
        .label(descriptor.classification)
        .schema_version_id(descriptor.resulting_lineage.resulting_schema_version_id)
        .label(descriptor.resulting_lineage.ordering_mode)
        .finish()
}

pub(crate) fn digest_schema_lineage_summary(lineage: &SchemaLineageArtifact) -> [u8; 32] {
    let mut builder = ReplayDigestBuilder::new("forge.relational.replay.summary.schema_lineage.v1")
        .schema_version_id(lineage.resulting_schema_version_id)
        .label(lineage.ordering_mode)
        .usize(lineage.parent_schema_version_ids.len());
    for parent in &lineage.parent_schema_version_ids {
        builder = builder.schema_version_id(*parent);
    }
    builder.finish()
}
