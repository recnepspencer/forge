use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::inventory_rows::{effect_lifecycle_family_rows, effect_lifecycle_public_surface_rows};
use super::planning::EffectAuthorityOwner;
use super::support_matrix::{EffectLifecycleSupportRow, EffectSupportPosture};
use super::taxonomy::EffectFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectLifecycleFamilyKey {
    Mutation,
    Merge,
    Writeback,
    OrderedBatch,
}

impl EffectLifecycleFamilyKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Merge => "merge",
            Self::Writeback => "writeback",
            Self::OrderedBatch => "ordered_batch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectLoweredArtifactKind {
    LoweredMutationIntentDeclaration,
    LoweredMergeWorkflowDeclaration,
    QueryWritebackDeclaration,
    LoweredEffectBatchExecutionPlan,
}

impl EffectLoweredArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoweredMutationIntentDeclaration => "lowered_mutation_intent_declaration",
            Self::LoweredMergeWorkflowDeclaration => "lowered_merge_workflow_declaration",
            Self::QueryWritebackDeclaration => "query_writeback_declaration",
            Self::LoweredEffectBatchExecutionPlan => "lowered_effect_batch_execution_plan",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectReceiptArtifactKind {
    ForgeQueryIntentExecution,
    ForgeQueryWriteReceipt,
    ForgeQueryBatchWriteReceipt,
    SelfDescribingEffectEnvelope,
}

impl EffectReceiptArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ForgeQueryIntentExecution => "forge_query_intent_execution",
            Self::ForgeQueryWriteReceipt => "forge_query_write_receipt",
            Self::ForgeQueryBatchWriteReceipt => "forge_query_batch_write_receipt",
            Self::SelfDescribingEffectEnvelope => "self_describing_effect_envelope",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleFamilyInventoryRow {
    family_key: EffectLifecycleFamilyKey,
    authority_owner: EffectAuthorityOwner,
    admitted_basis_families: Vec<BasisFamily>,
    lowered_artifact_kind: EffectLoweredArtifactKind,
    receipt_artifact_kind: EffectReceiptArtifactKind,
    denial_posture: EffectSupportPosture,
    deferred_posture: EffectSupportPosture,
    row_digest: String,
}

impl EffectLifecycleFamilyInventoryRow {
    pub(super) fn new(
        family_key: EffectLifecycleFamilyKey,
        authority_owner: EffectAuthorityOwner,
        admitted_basis_families: Vec<BasisFamily>,
        lowered_artifact_kind: EffectLoweredArtifactKind,
        receipt_artifact_kind: EffectReceiptArtifactKind,
        denial_posture: EffectSupportPosture,
        deferred_posture: EffectSupportPosture,
    ) -> Self {
        let row_digest = hash_parts(
            &std::iter::once(format!("family:{}", family_key.as_str()))
                .chain(std::iter::once(format!(
                    "authority_owner:{}",
                    authority_owner.as_str()
                )))
                .chain(
                    admitted_basis_families
                        .iter()
                        .map(|basis| format!("basis:{}", basis.as_str())),
                )
                .chain(std::iter::once(format!(
                    "lowered_artifact:{}",
                    lowered_artifact_kind.as_str()
                )))
                .chain(std::iter::once(format!(
                    "receipt_artifact:{}",
                    receipt_artifact_kind.as_str()
                )))
                .chain(std::iter::once(format!(
                    "denial_posture:{}",
                    denial_posture.as_str()
                )))
                .chain(std::iter::once(format!(
                    "deferred_posture:{}",
                    deferred_posture.as_str()
                )))
                .collect::<Vec<_>>(),
        );
        Self {
            family_key,
            authority_owner,
            admitted_basis_families,
            lowered_artifact_kind,
            receipt_artifact_kind,
            denial_posture,
            deferred_posture,
            row_digest,
        }
    }

    pub fn family_key(&self) -> EffectLifecycleFamilyKey {
        self.family_key
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn admitted_basis_families(&self) -> &[BasisFamily] {
        &self.admitted_basis_families
    }

    pub fn lowered_artifact_kind(&self) -> EffectLoweredArtifactKind {
        self.lowered_artifact_kind
    }

    pub fn receipt_artifact_kind(&self) -> EffectReceiptArtifactKind {
        self.receipt_artifact_kind
    }

    pub fn denial_posture(&self) -> EffectSupportPosture {
        self.denial_posture
    }

    pub fn deferred_posture(&self) -> EffectSupportPosture {
        self.deferred_posture
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleFamilyInventory {
    rows: Vec<EffectLifecycleFamilyInventoryRow>,
    inventory_digest: String,
}

impl EffectLifecycleFamilyInventory {
    pub fn rows(&self) -> &[EffectLifecycleFamilyInventoryRow] {
        &self.rows
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectPublicSurfaceKind {
    CommonPathIntentAuthoring,
    InspectableLoweredPlan,
    SupportDiscovery,
    DenialOrRebind,
    BatchExecution,
    DiagnosticsEnvelope,
    HiddenLowerRuntimeTypes,
}

impl EffectPublicSurfaceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CommonPathIntentAuthoring => "common_path_intent_authoring",
            Self::InspectableLoweredPlan => "inspectable_lowered_plan",
            Self::SupportDiscovery => "support_discovery",
            Self::DenialOrRebind => "denial_or_rebind",
            Self::BatchExecution => "batch_execution",
            Self::DiagnosticsEnvelope => "diagnostics_envelope",
            Self::HiddenLowerRuntimeTypes => "hidden_lower_runtime_types",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectPublicSurfaceAvailability {
    Implemented,
    DeferredToPhase5,
}

impl EffectPublicSurfaceAvailability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::DeferredToPhase5 => "deferred_to_phase5",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecyclePublicSurfaceRow {
    surface_kind: EffectPublicSurfaceKind,
    entrypoint: Option<&'static str>,
    primary_artifact_kind: Option<EffectReceiptArtifactKind>,
    availability: EffectPublicSurfaceAvailability,
    lower_runtime_visibility_hidden: bool,
    row_digest: String,
}

impl EffectLifecyclePublicSurfaceRow {
    pub(super) fn new(
        surface_kind: EffectPublicSurfaceKind,
        entrypoint: Option<&'static str>,
        primary_artifact_kind: Option<EffectReceiptArtifactKind>,
        availability: EffectPublicSurfaceAvailability,
        lower_runtime_visibility_hidden: bool,
    ) -> Self {
        let row_digest = hash_parts(&[
            format!("surface_kind:{}", surface_kind.as_str()),
            format!("entrypoint:{}", entrypoint.unwrap_or("none")),
            format!(
                "artifact:{}",
                primary_artifact_kind
                    .map(|kind| kind.as_str())
                    .unwrap_or("none")
            ),
            format!("availability:{}", availability.as_str()),
            format!("hidden:{lower_runtime_visibility_hidden}"),
        ]);
        Self {
            surface_kind,
            entrypoint,
            primary_artifact_kind,
            availability,
            lower_runtime_visibility_hidden,
            row_digest,
        }
    }

    pub fn surface_kind(&self) -> EffectPublicSurfaceKind {
        self.surface_kind
    }

    pub fn entrypoint(&self) -> Option<&'static str> {
        self.entrypoint
    }

    pub fn primary_artifact_kind(&self) -> Option<EffectReceiptArtifactKind> {
        self.primary_artifact_kind
    }

    pub fn availability(&self) -> EffectPublicSurfaceAvailability {
        self.availability
    }

    pub fn lower_runtime_visibility_hidden(&self) -> bool {
        self.lower_runtime_visibility_hidden
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecyclePublicSurfaceInventory {
    rows: Vec<EffectLifecyclePublicSurfaceRow>,
    inventory_digest: String,
}

impl EffectLifecyclePublicSurfaceInventory {
    pub fn rows(&self) -> &[EffectLifecyclePublicSurfaceRow] {
        &self.rows
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}

pub fn effect_lifecycle_family_inventory() -> EffectLifecycleFamilyInventory {
    let rows = effect_lifecycle_family_rows();
    let inventory_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    EffectLifecycleFamilyInventory {
        rows,
        inventory_digest,
    }
}

pub fn effect_lifecycle_public_surface_inventory() -> EffectLifecyclePublicSurfaceInventory {
    let rows = effect_lifecycle_public_surface_rows();
    let inventory_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    EffectLifecyclePublicSurfaceInventory {
        rows,
        inventory_digest,
    }
}

pub fn effect_lifecycle_family_row_for_key(
    family_key: EffectLifecycleFamilyKey,
) -> Option<EffectLifecycleFamilyInventoryRow> {
    effect_lifecycle_family_inventory()
        .rows
        .into_iter()
        .find(|row| row.family_key() == family_key)
}

pub fn effect_lifecycle_family_row_for(
    effect_family: EffectFamily,
) -> Option<EffectLifecycleFamilyInventoryRow> {
    let family_key = match effect_family {
        EffectFamily::Mutation => EffectLifecycleFamilyKey::Mutation,
        EffectFamily::Merge => EffectLifecycleFamilyKey::Merge,
        EffectFamily::Writeback => EffectLifecycleFamilyKey::Writeback,
    };
    effect_lifecycle_family_row_for_key(family_key)
}

pub fn effect_lifecycle_supported_basis_families(effect_family: EffectFamily) -> Vec<BasisFamily> {
    effect_lifecycle_family_row_for(effect_family)
        .map(|row| row.admitted_basis_families().to_vec())
        .unwrap_or_default()
}

pub fn effect_lifecycle_support_row_matches_inventory(row: &EffectLifecycleSupportRow) -> bool {
    let Some(family_row) = effect_lifecycle_family_row_for(row.effect_family()) else {
        return false;
    };
    row.authority_owner() == family_row.authority_owner()
        && row.lowered_artifact_kind() == family_row.lowered_artifact_kind()
        && row.receipt_artifact_kind() == family_row.receipt_artifact_kind()
}
