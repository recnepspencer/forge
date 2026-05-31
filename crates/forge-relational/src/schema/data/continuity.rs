use std::collections::BTreeSet;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use forge_foundational::FieldKey;

use crate::history::data::BranchId;
use crate::identity::data::KindId;

use super::{SchemaId, SchemaVersionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DescriptorSemanticsVersion(pub u32);

impl Default for DescriptorSemanticsVersion {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorSemanticsSupportPolicy {
    current_write_version: DescriptorSemanticsVersion,
    supported_historical_versions: BTreeSet<DescriptorSemanticsVersion>,
}

impl DescriptorSemanticsSupportPolicy {
    pub fn new(
        current_write_version: DescriptorSemanticsVersion,
        supported_historical_versions: impl IntoIterator<Item = DescriptorSemanticsVersion>,
    ) -> Self {
        let mut supported_historical_versions = supported_historical_versions
            .into_iter()
            .collect::<BTreeSet<_>>();
        supported_historical_versions.insert(current_write_version);
        Self {
            current_write_version,
            supported_historical_versions,
        }
    }

    pub fn current_write_version(&self) -> DescriptorSemanticsVersion {
        self.current_write_version
    }

    pub fn supports(&self, version: DescriptorSemanticsVersion) -> bool {
        self.supported_historical_versions.contains(&version)
    }
}

impl Default for DescriptorSemanticsSupportPolicy {
    fn default() -> Self {
        Self::new(
            DescriptorSemanticsVersion::default(),
            [DescriptorSemanticsVersion::default()],
        )
    }
}

pub fn runtime_descriptor_semantics_policy() -> DescriptorSemanticsSupportPolicy {
    DescriptorSemanticsSupportPolicy::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DescriptorCanonicalBasisVersion(pub u32);

impl Default for DescriptorCanonicalBasisVersion {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorCanonicalBasisSupportPolicy {
    current_write_version: DescriptorCanonicalBasisVersion,
    supported_historical_versions: BTreeSet<DescriptorCanonicalBasisVersion>,
}

impl DescriptorCanonicalBasisSupportPolicy {
    pub fn new(
        current_write_version: DescriptorCanonicalBasisVersion,
        supported_historical_versions: impl IntoIterator<Item = DescriptorCanonicalBasisVersion>,
    ) -> Self {
        let mut supported_historical_versions = supported_historical_versions
            .into_iter()
            .collect::<BTreeSet<_>>();
        supported_historical_versions.insert(current_write_version);
        Self {
            current_write_version,
            supported_historical_versions,
        }
    }

    pub fn current_write_version(&self) -> DescriptorCanonicalBasisVersion {
        self.current_write_version
    }

    pub fn supports(&self, version: DescriptorCanonicalBasisVersion) -> bool {
        self.supported_historical_versions.contains(&version)
    }
}

impl Default for DescriptorCanonicalBasisSupportPolicy {
    fn default() -> Self {
        Self::new(
            DescriptorCanonicalBasisVersion::default(),
            [DescriptorCanonicalBasisVersion::default()],
        )
    }
}

pub fn runtime_descriptor_canonical_basis_policy() -> DescriptorCanonicalBasisSupportPolicy {
    DescriptorCanonicalBasisSupportPolicy::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaBoundaryFingerprint(pub [u8; 32]);

impl SchemaBoundaryFingerprint {
    pub const ZERO: Self = Self([0; 32]);

    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaStratum {
    StructuralShape,
    ValueDomain,
    EntityIdentitySemantics,
    CorrespondenceSemantics,
    LineageSemantics,
    BehavioralSemantics,
    PublicationContract,
    SubscriberContract,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HistoricalInterpretationSensitivity {
    NotSensitive = 0,
    SensitiveToValueMeaning = 1,
    SensitiveToLegalityMeaning = 2,
    SensitiveToIdentityMeaning = 3,
    SensitiveToPublicationMeaning = 4,
    SensitiveToDerivedMeaning = 5,
}

impl HistoricalInterpretationSensitivity {
    pub const fn sensitivity_rank(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaElementKind {
    Schema,
    EntityKind,
    RelationKind,
    Field,
    RelationEndpoint,
    EnumDomain,
    PrecisionContract,
    InvariantContract,
    ProjectionContract,
    SubscriberContract,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaElementRef {
    pub kind: SchemaElementKind,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub kind_id: Option<KindId>,
    pub element_name: Arc<str>,
}

impl SchemaElementRef {
    pub fn new(
        kind: SchemaElementKind,
        schema_id: SchemaId,
        schema_version_id: SchemaVersionId,
        kind_id: Option<KindId>,
        element_name: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            schema_id,
            schema_version_id,
            kind_id,
            element_name: element_name.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaPublicationImpact {
    None,
    ObservableSurfaceChanged,
    PatchEncodingChanged,
    ProjectionContractChanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaSubscriberImpact {
    None,
    ConsumableSurfaceChanged,
    ContractUpgradeRequired,
    RenegotiationRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SubscriberBoundaryVisibility {
    NotVisible,
    VisibleSemanticallyIgnorable,
    VisibleRequiresContractUptake,
}

pub const fn default_boundary_visibility_for_subscriber_impact(
    subscriber_impact: SchemaSubscriberImpact,
) -> SubscriberBoundaryVisibility {
    match subscriber_impact {
        SchemaSubscriberImpact::ContractUpgradeRequired => {
            SubscriberBoundaryVisibility::VisibleRequiresContractUptake
        }
        _ => SubscriberBoundaryVisibility::NotVisible,
    }
}

pub const fn default_boundary_visibility_for_continuation(
    continuation: SchemaContinuationClassification,
) -> SubscriberBoundaryVisibility {
    match continuation {
        SchemaContinuationClassification::ContinueWithContractUpgrade => {
            SubscriberBoundaryVisibility::VisibleRequiresContractUptake
        }
        _ => SubscriberBoundaryVisibility::NotVisible,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FreeFormSchemaDiffIntent {
    Additive,
    StructuralContinuityDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaDiffDetail {
    AddedField {
        field: FieldKey,
        required: bool,
        default_expression: Option<Arc<str>>,
    },
    RemovedField {
        field: FieldKey,
    },
    TypeChanged {
        field: FieldKey,
        from_type: Arc<str>,
        to_type: Arc<str>,
    },
    EnumDomainExpanded {
        field: FieldKey,
        added_variants: Vec<Arc<str>>,
    },
    InvariantContractChanged {
        contract_name: Arc<str>,
    },
    ProjectionContractChanged {
        projection_name: Arc<str>,
    },
    SubscriberContractChanged {
        contract_name: Arc<str>,
    },
    FreeText {
        detail: Arc<str>,
        declared_intent: FreeFormSchemaDiffIntent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaDiffAtom {
    pub element: SchemaElementRef,
    pub strata: Vec<SchemaStratum>,
    pub publication_impact: SchemaPublicationImpact,
    pub subscriber_impact: SchemaSubscriberImpact,
    pub boundary_visibility: SubscriberBoundaryVisibility,
    pub historical_interpretation: HistoricalInterpretationSensitivity,
    pub detail: SchemaDiffDetail,
}

impl SchemaDiffAtom {
    pub fn new(
        element: SchemaElementRef,
        strata: Vec<SchemaStratum>,
        publication_impact: SchemaPublicationImpact,
        subscriber_impact: SchemaSubscriberImpact,
        historical_interpretation: HistoricalInterpretationSensitivity,
        detail: SchemaDiffDetail,
    ) -> Self {
        Self {
            element,
            strata,
            publication_impact,
            subscriber_impact,
            boundary_visibility: default_boundary_visibility_for_subscriber_impact(
                subscriber_impact,
            ),
            historical_interpretation,
            detail,
        }
    }

    pub fn with_boundary_visibility_proof(
        mut self,
        boundary_visibility: SubscriberBoundaryVisibility,
    ) -> Self {
        self.boundary_visibility = boundary_visibility;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaTransitionBarrier {
    ConstructionBarrier,
    ValidationBarrier,
    LoweringBarrier,
    ExecutionBarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaReconciliationClassification {
    Additive,
    Narrowing,
    TypeContinuityDenied,
    StructuralContinuityDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaBridgeabilityClassification {
    Transparent,
    SubscriberVisible,
    ContractUpgradeOnly,
    RenegotiationOnly,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaContinuationClassification {
    ContinueUnchanged,
    ContinueWithTransparentBridge,
    ContinueWithVisibleBridge,
    ContinueWithContractUpgrade,
    RequireRenegotiation,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaContinuationAdmissionObservation {
    RejectedInAllLayers,
    NonRejectedInAtLeastOneLayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaReconciliationPolicy {
    RejectLossyNarrowing,
    PreserveInformation,
    PreserveTargetContract,
    PreserveSourceContract,
    PermitLossyNarrowingWithAnnotation,
    RequireExplicitProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaReconciliationOrderingMode {
    CanonicalizedPair,
    ExplicitDirectional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaLineageOrderingSemantics {
    SymmetricResult,
    DirectionSensitiveResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposedSchemaTransition {
    pub source_schema_id: SchemaId,
    pub source_schema_version_id: SchemaVersionId,
    pub target_schema_id: SchemaId,
    pub target_schema_version_id: SchemaVersionId,
    pub diff_atoms: Vec<SchemaDiffAtom>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedSchemaTransition {
    pub proposed: ProposedSchemaTransition,
    pub continuation_admission_observation: SchemaContinuationAdmissionObservation,
    pub reconciliation: SchemaReconciliationClassification,
    pub continuation: SchemaContinuationClassification,
    pub bridgeability: SchemaBridgeabilityClassification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaBridgeDescriptor {
    pub boundary_fingerprint: SchemaBoundaryFingerprint,
    pub semantics_version: DescriptorSemanticsVersion,
    pub canonical_basis_version: DescriptorCanonicalBasisVersion,
    pub continuation: SchemaContinuationClassification,
    pub bridgeability: SchemaBridgeabilityClassification,
    pub boundary_visibility: SubscriberBoundaryVisibility,
    pub historical_interpretation: HistoricalInterpretationSensitivity,
    pub changed_strata: Vec<SchemaStratum>,
}

impl SchemaBridgeDescriptor {
    pub fn new(
        boundary_fingerprint: SchemaBoundaryFingerprint,
        semantics_version: DescriptorSemanticsVersion,
        canonical_basis_version: DescriptorCanonicalBasisVersion,
        continuation: SchemaContinuationClassification,
        bridgeability: SchemaBridgeabilityClassification,
        historical_interpretation: HistoricalInterpretationSensitivity,
        changed_strata: Vec<SchemaStratum>,
    ) -> Self {
        Self::new_with_visibility(
            boundary_fingerprint,
            semantics_version,
            canonical_basis_version,
            continuation,
            bridgeability,
            default_boundary_visibility_for_continuation(continuation),
            historical_interpretation,
            changed_strata,
        )
    }

    pub fn new_with_visibility(
        boundary_fingerprint: SchemaBoundaryFingerprint,
        semantics_version: DescriptorSemanticsVersion,
        canonical_basis_version: DescriptorCanonicalBasisVersion,
        continuation: SchemaContinuationClassification,
        bridgeability: SchemaBridgeabilityClassification,
        boundary_visibility: SubscriberBoundaryVisibility,
        historical_interpretation: HistoricalInterpretationSensitivity,
        changed_strata: Vec<SchemaStratum>,
    ) -> Self {
        Self {
            boundary_fingerprint,
            semantics_version,
            canonical_basis_version,
            continuation,
            bridgeability,
            boundary_visibility,
            historical_interpretation,
            changed_strata,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaContinuationDescriptor {
    pub boundary_fingerprint: SchemaBoundaryFingerprint,
    pub bridge: SchemaBridgeDescriptor,
    pub normalized_boundary_count: usize,
}

impl SchemaContinuationDescriptor {
    pub fn new(
        boundary_fingerprint: SchemaBoundaryFingerprint,
        bridge: SchemaBridgeDescriptor,
        normalized_boundary_count: usize,
    ) -> Self {
        Self {
            boundary_fingerprint,
            bridge,
            normalized_boundary_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaLineageArtifact {
    pub resulting_schema_id: SchemaId,
    pub resulting_schema_version_id: SchemaVersionId,
    pub parent_schema_ids: Vec<SchemaId>,
    pub parent_schema_version_ids: Vec<SchemaVersionId>,
    pub branch_context: Option<BranchId>,
    pub ordering_mode: SchemaReconciliationOrderingMode,
    pub ordering_semantics: SchemaLineageOrderingSemantics,
}

impl SchemaLineageArtifact {
    pub fn new(
        resulting_schema_id: SchemaId,
        resulting_schema_version_id: SchemaVersionId,
        parent_schema_ids: Vec<SchemaId>,
        parent_schema_version_ids: Vec<SchemaVersionId>,
        branch_context: Option<BranchId>,
        ordering_mode: SchemaReconciliationOrderingMode,
        ordering_semantics: SchemaLineageOrderingSemantics,
    ) -> Self {
        Self {
            resulting_schema_id,
            resulting_schema_version_id,
            parent_schema_ids,
            parent_schema_version_ids,
            branch_context,
            ordering_mode,
            ordering_semantics,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaReconciliationDescriptor {
    pub semantics_version: DescriptorSemanticsVersion,
    pub canonical_basis_version: DescriptorCanonicalBasisVersion,
    pub classification: SchemaReconciliationClassification,
    pub policy: SchemaReconciliationPolicy,
    pub resulting_lineage: SchemaLineageArtifact,
}

impl SchemaReconciliationDescriptor {
    pub fn new(
        semantics_version: DescriptorSemanticsVersion,
        canonical_basis_version: DescriptorCanonicalBasisVersion,
        classification: SchemaReconciliationClassification,
        policy: SchemaReconciliationPolicy,
        resulting_lineage: SchemaLineageArtifact,
    ) -> Self {
        Self {
            semantics_version,
            canonical_basis_version,
            classification,
            policy,
            resulting_lineage,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredSchemaTransitionPlan {
    pub validated: ValidatedSchemaTransition,
    pub continuation_descriptor: SchemaContinuationDescriptor,
    pub reconciliation_descriptor: SchemaReconciliationDescriptor,
}

impl LoweredSchemaTransitionPlan {
    pub fn new(
        validated: ValidatedSchemaTransition,
        continuation_descriptor: SchemaContinuationDescriptor,
        reconciliation_descriptor: SchemaReconciliationDescriptor,
    ) -> Self {
        Self {
            validated,
            continuation_descriptor,
            reconciliation_descriptor,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaTransitionArtifact {
    pub source_schema_id: SchemaId,
    pub source_schema_version_id: SchemaVersionId,
    pub target_schema_id: SchemaId,
    pub target_schema_version_id: SchemaVersionId,
    pub diff_atoms: Vec<SchemaDiffAtom>,
    pub continuation_descriptor: SchemaContinuationDescriptor,
    pub reconciliation_descriptor: SchemaReconciliationDescriptor,
}

impl SchemaTransitionArtifact {
    pub fn new(
        source_schema_id: SchemaId,
        source_schema_version_id: SchemaVersionId,
        target_schema_id: SchemaId,
        target_schema_version_id: SchemaVersionId,
        diff_atoms: Vec<SchemaDiffAtom>,
        continuation_descriptor: SchemaContinuationDescriptor,
        reconciliation_descriptor: SchemaReconciliationDescriptor,
    ) -> Self {
        Self {
            source_schema_id,
            source_schema_version_id,
            target_schema_id,
            target_schema_version_id,
            diff_atoms,
            continuation_descriptor,
            reconciliation_descriptor,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaTransitionSummary {
    pub changed_atom_count: usize,
    pub changed_strata: Vec<SchemaStratum>,
    pub continuation: SchemaContinuationClassification,
    pub bridgeability: SchemaBridgeabilityClassification,
    pub reconciliation: SchemaReconciliationClassification,
    pub historical_interpretation: HistoricalInterpretationSensitivity,
}

impl SchemaTransitionSummary {
    pub fn from_artifact(artifact: &SchemaTransitionArtifact) -> Self {
        let changed_strata = artifact
            .diff_atoms
            .iter()
            .flat_map(|atom| atom.strata.iter().copied())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Self {
            changed_atom_count: artifact.diff_atoms.len(),
            changed_strata,
            continuation: artifact.continuation_descriptor.bridge.continuation,
            bridgeability: artifact.continuation_descriptor.bridge.bridgeability,
            reconciliation: artifact.reconciliation_descriptor.classification,
            historical_interpretation: artifact
                .continuation_descriptor
                .bridge
                .historical_interpretation,
        }
    }
}
