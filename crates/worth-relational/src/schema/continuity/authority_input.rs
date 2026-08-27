use crate::schema::data::{
    DescriptorCanonicalBasisVersion, DescriptorSemanticsVersion, SchemaAuthoritySnapshot, SchemaId,
    SchemaVersionId,
};

/// Schema authority selected by the owner for one continuity derivation.
///
/// Ordinary commits derive this from their admitted root. Declared schema
/// transitions use the live target, while recovery may carry an admitted
/// historical snapshot so replay does not reinterpret an older commit through
/// the runtime's final schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SchemaContinuityAuthorityInput {
    target_schema_version: SchemaVersionId,
    target_schema_authority: SchemaAuthoritySnapshot,
    target_schema_basis: Option<(SchemaId, SchemaVersionId)>,
    descriptor_semantics_version: DescriptorSemanticsVersion,
    descriptor_canonical_basis_version: DescriptorCanonicalBasisVersion,
}

impl SchemaContinuityAuthorityInput {
    pub(crate) fn from_runtime(runtime: &crate::runtime::RelationalRuntime) -> Self {
        use crate::capabilities::{SchemaSource, SchemaVersionSource};

        Self::new(
            runtime.primary_schema_version_id(),
            runtime.schema_registry().authority_snapshot(),
            runtime
                .config
                .schema
                .descriptor_semantics_policy
                .current_write_version(),
            runtime
                .config
                .schema
                .descriptor_canonical_basis_policy
                .current_write_version(),
        )
    }

    pub(crate) fn new(
        target_schema_version: SchemaVersionId,
        target_schema_authority: SchemaAuthoritySnapshot,
        descriptor_semantics_version: DescriptorSemanticsVersion,
        descriptor_canonical_basis_version: DescriptorCanonicalBasisVersion,
    ) -> Self {
        let target_schema_basis = target_schema_authority
            .primary_schema_id
            .clone()
            .zip(target_schema_authority.primary_schema_version_id);
        Self {
            target_schema_version,
            target_schema_authority,
            target_schema_basis,
            descriptor_semantics_version,
            descriptor_canonical_basis_version,
        }
    }

    pub(crate) const fn target_schema_version(&self) -> SchemaVersionId {
        self.target_schema_version
    }

    pub(crate) fn target_schema_authority(&self) -> &SchemaAuthoritySnapshot {
        &self.target_schema_authority
    }

    pub(crate) fn target_schema_basis(&self) -> Option<(SchemaId, SchemaVersionId)> {
        self.target_schema_basis.clone()
    }

    pub(crate) const fn descriptor_semantics_version(&self) -> DescriptorSemanticsVersion {
        self.descriptor_semantics_version
    }

    pub(crate) const fn descriptor_canonical_basis_version(
        &self,
    ) -> DescriptorCanonicalBasisVersion {
        self.descriptor_canonical_basis_version
    }
}
