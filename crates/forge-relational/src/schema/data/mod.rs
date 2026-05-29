mod aspect_semantics;
mod aspect_traces;
mod continuity;
mod registry_errors;
mod relation_integrity;
mod structural_invariants;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
use crate::identity::data::KindId;
use crate::merge::data::{AspectMergePolicyDeclaration, IdentityBasisDeclaration};

pub use aspect_semantics::{
    AspectBinding, AspectPlanCatalog, AspectPlanRevision, DeclaredAspect, KindAspectDeclarations,
    LoweredAspectBinding, LoweredAspectPlan,
};
pub use aspect_traces::{
    AspectDeclarationTrace, AspectDeclarationTraceRow, AspectLoweringTrace, AspectLoweringTraceRow,
};
pub use continuity::{
    runtime_descriptor_canonical_basis_policy, runtime_descriptor_semantics_policy,
    CompatibilityObservation, DescriptorCanonicalBasisCompatibilityPolicy,
    DescriptorCanonicalBasisVersion, DescriptorSemanticsCompatibilityPolicy,
    DescriptorSemanticsVersion, FreeFormSchemaDiffIntent, HistoricalInterpretationSensitivity,
    LoweredSchemaTransitionPlan, ProposedSchemaTransition, SchemaBoundaryFingerprint,
    SchemaBridgeDescriptor, SchemaBridgeabilityClassification, SchemaContinuationClassification,
    SchemaContinuationDescriptor, SchemaDiffAtom, SchemaDiffDetail, SchemaElementKind,
    SchemaElementRef, SchemaLineageArtifact, SchemaLineageOrderingSemantics,
    SchemaPublicationImpact, SchemaReconciliationClassification, SchemaReconciliationDescriptor,
    SchemaReconciliationOrderingMode, SchemaReconciliationPolicy, SchemaStratum,
    SchemaSubscriberImpact, SchemaTransitionArtifact, SchemaTransitionBarrier,
    SchemaTransitionSummary, SubscriberBoundaryVisibility, ValidatedSchemaTransition,
};
pub use registry_errors::{SchemaRegistryError, SchemaRegistryErrorClass};
pub(crate) use relation_integrity::derive_relation_integrity_plan_revision;
pub use relation_integrity::{
    CardinalityContractDeclaration, ContractId, EndpointDeletionIntegrityDeclaration,
    EndpointDeletionIntegrityMode, EndpointKindContractDeclaration,
    LoweredCardinalityMaximumContract, LoweredCardinalityMinimumContract,
    LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
    LoweredRelationIntegrityPlan, LoweredSymmetryContract, LoweredUniquenessContract,
    MinimumCardinalityEnforcement, PairMinimumSemantics, RelationIntegrityDeclarations,
    RelationIntegrityPlanCatalog, RelationIntegrityPlanRevision, SymmetryContractDeclaration,
    SymmetryMode, UniquenessContractDeclaration, UniquenessScope,
};
pub use structural_invariants::{
    AcyclicityContractDeclaration, AllowedCycleClass, ConnectivityMinimumContractDeclaration,
    ConnectivityMinimumEnforcement, DirectedTraversalKind, LoweredAcyclicityContract,
    LoweredConnectivityMinimumContract, LoweredPartitionIsolationContract,
    PartitionIsolationContractDeclaration, PartitionIsolationMode,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaVersionId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityKindRegistration {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub aspect_declarations: KindAspectDeclarations,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationKindRegistration {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub cross_context_policy: CrossContextPolicy,
    pub cascade_delete_policy: CascadeDeletePolicy,
    pub aspect_declarations: KindAspectDeclarations,
    pub relation_integrity: RelationIntegrityDeclarations,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindResolution {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
}

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
    pub aspect_plan_revision: AspectPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaAuthorityRelationSnapshot {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub aspect_plan_revision: AspectPlanRevision,
    pub relation_integrity_plan_revision: RelationIntegrityPlanRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RelationalSchemaRegistry {
    pub entity_kinds: BTreeMap<KindId, EntityKindRegistration>,
    pub relation_kinds: BTreeMap<KindId, RelationKindRegistration>,
}

impl RelationalSchemaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity_kind(
        mut self,
        registration: EntityKindRegistration,
    ) -> Result<Self, SchemaRegistryError> {
        if self.relation_kinds.contains_key(&registration.kind_id) {
            return Err(SchemaRegistryError::entity_relation_kind_collision(
                registration.kind_id,
            ));
        }
        let registration = crate::schema::logic::canonicalize_entity_registration(registration)?;
        self.entity_kinds.insert(registration.kind_id, registration);
        Ok(self)
    }

    pub fn register_relation_kind(
        mut self,
        registration: RelationKindRegistration,
    ) -> Result<Self, SchemaRegistryError> {
        if self.entity_kinds.contains_key(&registration.kind_id) {
            return Err(SchemaRegistryError::entity_relation_kind_collision(
                registration.kind_id,
            ));
        }
        let registration = crate::schema::logic::canonicalize_relation_registration(registration)?;
        self.relation_kinds
            .insert(registration.kind_id, registration);
        Ok(self)
    }

    pub fn resolve_entity(&self, kind_id: KindId) -> Result<KindResolution, SchemaRegistryError> {
        self.entity_kinds
            .get(&kind_id)
            .map(|registration| KindResolution {
                kind_id: registration.kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
            })
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))
    }

    pub fn resolve_relation(&self, kind_id: KindId) -> Result<KindResolution, SchemaRegistryError> {
        self.relation_kinds
            .get(&kind_id)
            .map(|registration| KindResolution {
                kind_id: registration.kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
            })
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))
    }

    pub fn relation_registration(
        &self,
        kind_id: KindId,
    ) -> Result<&RelationKindRegistration, SchemaRegistryError> {
        self.relation_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))
    }

    pub fn entity_registration(
        &self,
        kind_id: KindId,
    ) -> Result<&EntityKindRegistration, SchemaRegistryError> {
        self.entity_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))
    }

    pub fn entity_aspect_declaration_trace(
        &self,
        kind_id: KindId,
    ) -> Result<AspectDeclarationTrace, SchemaRegistryError> {
        let registration = self
            .entity_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))?;
        Ok(registration.aspect_declarations.declaration_trace(kind_id))
    }

    pub fn relation_aspect_declaration_trace(
        &self,
        kind_id: KindId,
    ) -> Result<AspectDeclarationTrace, SchemaRegistryError> {
        let registration = self
            .relation_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))?;
        Ok(registration.aspect_declarations.declaration_trace(kind_id))
    }

    pub fn entity_identity_declarations(
        &self,
        kind_id: KindId,
    ) -> Result<&[IdentityBasisDeclaration], SchemaRegistryError> {
        let registration = self
            .entity_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))?;
        Ok(&registration.aspect_declarations.identity_declarations)
    }

    pub fn relation_identity_declarations(
        &self,
        kind_id: KindId,
    ) -> Result<&[IdentityBasisDeclaration], SchemaRegistryError> {
        let registration = self
            .relation_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))?;
        Ok(&registration.aspect_declarations.identity_declarations)
    }

    pub fn entity_merge_policy_declarations(
        &self,
        kind_id: KindId,
    ) -> Result<&[AspectMergePolicyDeclaration], SchemaRegistryError> {
        let registration = self
            .entity_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))?;
        Ok(&registration.aspect_declarations.merge_policy_declarations)
    }

    pub fn relation_merge_policy_declarations(
        &self,
        kind_id: KindId,
    ) -> Result<&[AspectMergePolicyDeclaration], SchemaRegistryError> {
        let registration = self
            .relation_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))?;
        Ok(&registration.aspect_declarations.merge_policy_declarations)
    }

    pub fn authoritative_schema_basis(
        &self,
    ) -> Result<Option<(SchemaId, SchemaVersionId)>, SchemaRegistryError> {
        let mut registrations = self
            .entity_kinds
            .values()
            .map(|registration| (&registration.schema_id, registration.schema_version_id))
            .chain(
                self.relation_kinds
                    .values()
                    .map(|registration| (&registration.schema_id, registration.schema_version_id)),
            );
        let Some((schema_id, schema_version_id)) = registrations.next() else {
            return Ok(None);
        };
        for (next_schema_id, next_schema_version_id) in registrations {
            if next_schema_id != schema_id || next_schema_version_id != schema_version_id {
                return Err(SchemaRegistryError::inconsistent_schema_basis(format!(
                    "found mixed schema basis {:?}/{:?} and {:?}/{:?}",
                    schema_id, schema_version_id, next_schema_id, next_schema_version_id
                )));
            }
        }
        Ok(Some((schema_id.clone(), schema_version_id)))
    }

    pub fn authority_snapshot(&self) -> SchemaAuthoritySnapshot {
        let (primary_schema_id, primary_schema_version_id) = self
            .authoritative_schema_basis()
            .ok()
            .flatten()
            .map(|(schema_id, schema_version_id)| (Some(schema_id), Some(schema_version_id)))
            .unwrap_or((None, None));

        let entity_kinds = self
            .entity_kinds
            .values()
            .map(|registration| SchemaAuthorityKindSnapshot {
                kind_id: registration.kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
                aspect_plan_revision: registration.aspect_declarations.plan_revision,
            })
            .collect();
        let relation_kinds = self
            .relation_kinds
            .values()
            .map(|registration| SchemaAuthorityRelationSnapshot {
                kind_id: registration.kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
                aspect_plan_revision: registration.aspect_declarations.plan_revision,
                relation_integrity_plan_revision: registration.relation_integrity.plan_revision,
            })
            .collect();

        SchemaAuthoritySnapshot {
            primary_schema_id,
            primary_schema_version_id,
            entity_kinds,
            relation_kinds,
        }
    }

    pub fn authority_digest_bytes(&self) -> [u8; 32] {
        schema_authority_snapshot_digest_bytes(&self.authority_snapshot())
    }
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
