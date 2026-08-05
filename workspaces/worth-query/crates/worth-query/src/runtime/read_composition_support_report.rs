use crate::identity::hash_parts;

use super::{
    WorthQueryReadCompositionExtensionHookBoundary, WorthQueryReadCompositionExtensionHookFamily,
    WorthQueryReadCompositionExtensionHookSupportRow, WorthQueryRuntime,
    WorthQueryRuntimeBackendPosture, WorthQueryRuntimeSupportProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryReadCompositionSupportClass {
    EntryPoint,
    GraphArtifact,
    ScopeClass,
    GraphFamily,
    ExecutionEngine,
    FallbackClass,
    BuiltInOperator,
    RelationshipProof,
    FamilyAdmission,
    ExtensionHook,
    BoundaryGuard,
    DenialLane,
}

impl WorthQueryReadCompositionSupportClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EntryPoint => "entry_point",
            Self::GraphArtifact => "graph_artifact",
            Self::ScopeClass => "scope_class",
            Self::GraphFamily => "graph_family",
            Self::ExecutionEngine => "execution_engine",
            Self::FallbackClass => "fallback_class",
            Self::BuiltInOperator => "built_in_operator",
            Self::RelationshipProof => "relationship_proof",
            Self::FamilyAdmission => "family_admission",
            Self::ExtensionHook => "extension_hook",
            Self::BoundaryGuard => "boundary_guard",
            Self::DenialLane => "denial_lane",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadCompositionSupportRow {
    capability_family: String,
    capability_class: WorthQueryReadCompositionSupportClass,
    row_digest: String,
}

impl WorthQueryReadCompositionSupportRow {
    fn new(
        capability_family: impl Into<String>,
        capability_class: WorthQueryReadCompositionSupportClass,
    ) -> Self {
        let capability_family = capability_family.into();
        let row_digest = hash_parts(&[
            format!("family:{capability_family}"),
            format!("class:{}", capability_class.as_str()),
        ]);
        Self {
            capability_family,
            capability_class,
            row_digest,
        }
    }

    pub fn capability_family(&self) -> &str {
        &self.capability_family
    }

    pub fn capability_class(&self) -> WorthQueryReadCompositionSupportClass {
        self.capability_class
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadCompositionSupportReport {
    backend_posture: WorthQueryRuntimeBackendPosture,
    extension_hooks: Vec<WorthQueryReadCompositionExtensionHookSupportRow>,
    rows: Vec<WorthQueryReadCompositionSupportRow>,
    support_digest: String,
}

impl WorthQueryReadCompositionSupportReport {
    pub fn derive(backend_posture: WorthQueryRuntimeBackendPosture) -> Self {
        let extension_hooks = default_read_composition_extension_hook_support_rows();
        let mut rows = Vec::new();
        rows.extend(build_rows(
            ENTRY_POINTS,
            WorthQueryReadCompositionSupportClass::EntryPoint,
        ));
        rows.extend(build_rows(
            GRAPH_ARTIFACTS,
            WorthQueryReadCompositionSupportClass::GraphArtifact,
        ));
        rows.extend(build_rows(
            SCOPE_CLASSES,
            WorthQueryReadCompositionSupportClass::ScopeClass,
        ));
        rows.extend(build_rows(
            GRAPH_FAMILIES,
            WorthQueryReadCompositionSupportClass::GraphFamily,
        ));
        rows.extend(build_rows(
            EXECUTION_ENGINES,
            WorthQueryReadCompositionSupportClass::ExecutionEngine,
        ));
        rows.extend(build_rows(
            FALLBACK_CLASSES,
            WorthQueryReadCompositionSupportClass::FallbackClass,
        ));
        rows.extend(build_rows(
            BUILT_IN_OPERATORS,
            WorthQueryReadCompositionSupportClass::BuiltInOperator,
        ));
        rows.extend(build_rows(
            RELATIONSHIP_PROOF_POSTURES,
            WorthQueryReadCompositionSupportClass::RelationshipProof,
        ));
        rows.extend(build_rows(
            FAMILY_ADMISSION_MODES,
            WorthQueryReadCompositionSupportClass::FamilyAdmission,
        ));
        rows.extend(build_rows(
            extension_hook_family_names(),
            WorthQueryReadCompositionSupportClass::ExtensionHook,
        ));
        rows.extend(build_rows(
            BOUNDARY_GUARDS,
            WorthQueryReadCompositionSupportClass::BoundaryGuard,
        ));
        rows.extend(build_rows(
            DENIAL_LANES,
            WorthQueryReadCompositionSupportClass::DenialLane,
        ));
        let mut parts = vec![
            "worth_query_read_composition_support_report_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
        ];
        parts.extend(
            extension_hooks
                .iter()
                .map(|row| format!("extension-hook:{}", row.row_digest())),
        );
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        let support_digest = hash_parts(&parts);
        Self {
            backend_posture,
            extension_hooks,
            rows,
            support_digest,
        }
    }

    pub fn backend_posture(&self) -> WorthQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn rows(&self) -> &[WorthQueryReadCompositionSupportRow] {
        &self.rows
    }

    pub fn extension_hooks(&self) -> &[WorthQueryReadCompositionExtensionHookSupportRow] {
        &self.extension_hooks
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn entry_points(&self) -> &'static [&'static str] {
        ENTRY_POINTS
    }

    pub fn graph_artifacts(&self) -> &'static [&'static str] {
        GRAPH_ARTIFACTS
    }

    pub fn scope_classes(&self) -> &'static [&'static str] {
        SCOPE_CLASSES
    }

    pub fn graph_families(&self) -> &'static [&'static str] {
        GRAPH_FAMILIES
    }

    pub fn built_in_operators(&self) -> &'static [&'static str] {
        BUILT_IN_OPERATORS
    }

    pub fn execution_engines(&self) -> &'static [&'static str] {
        EXECUTION_ENGINES
    }

    pub fn fallback_classes(&self) -> &'static [&'static str] {
        FALLBACK_CLASSES
    }

    pub fn relationship_proof_postures(&self) -> &'static [&'static str] {
        RELATIONSHIP_PROOF_POSTURES
    }

    pub fn family_admission_modes(&self) -> &'static [&'static str] {
        FAMILY_ADMISSION_MODES
    }

    pub fn extension_hook_families(&self) -> &'static [&'static str] {
        extension_hook_family_names()
    }

    pub fn boundary_guards(&self) -> &'static [&'static str] {
        BOUNDARY_GUARDS
    }

    pub fn denial_lanes(&self) -> &'static [&'static str] {
        DENIAL_LANES
    }
}

fn build_rows(
    families: &'static [&'static str],
    class: WorthQueryReadCompositionSupportClass,
) -> Vec<WorthQueryReadCompositionSupportRow> {
    families
        .iter()
        .map(|family| WorthQueryReadCompositionSupportRow::new(*family, class))
        .collect()
}

const ENTRY_POINTS: &[&str] = &[
    "compose_read",
    "define_read_family",
    "execute_read_family",
    "execute_read_family_in_basis_context",
];

const GRAPH_ARTIFACTS: &[&str] = &[
    "read_graph",
    "read_result",
    "read_receipt",
    "typed_read_denial",
];

const SCOPE_CLASSES: &[&str] = &[
    "local_neighborhood",
    "anchored_expansion",
    "explicit_broad_search",
];

const GRAPH_FAMILIES: &[&str] = &["detail", "collection"];

const EXECUTION_ENGINES: &[&str] = &[
    "query_runtime_current",
    "query_runtime_branch",
    "query_runtime_historical",
    "query_runtime_preview_derived",
];

const FALLBACK_CLASSES: &[&str] = &["none", "snapshot_indexed_debt", "whole_view_debt"];

const BUILT_IN_OPERATORS: &[&str] = &[
    "direct_edge",
    "successor_walk",
    "shared_endpoint",
    "shared_attachment",
    "bounded_ancestor",
    "bounded_descendant",
    "anchored_frontier",
    "frontier_search",
];

const RELATIONSHIP_PROOF_POSTURES: &[&str] =
    &["not_required", "descriptor_admitted_synthetic_runtime"];

const FAMILY_ADMISSION_MODES: &[&str] = &["kernel_only"];

const READ_COMPOSITION_EXTENSION_HOOK_FAMILIES: &[WorthQueryReadCompositionExtensionHookFamily] = &[
    WorthQueryReadCompositionExtensionHookFamily::DomainReadFamilyLowering,
    WorthQueryReadCompositionExtensionHookFamily::DomainDecoder,
    WorthQueryReadCompositionExtensionHookFamily::DomainResultCertification,
];

const READ_COMPOSITION_EXTENSION_HOOK_FAMILY_NAMES: &[&str] = &[
    "domain_read_family_lowering",
    "domain_decoder",
    "domain_result_certification",
];

const BOUNDARY_GUARDS: &[&str] = &[
    "operator_owned_builders_hide_traverse",
    "scope_class_relabeling_denies_typed",
    "built_in_operator_shape_denies_typed",
    "relationship_proof_admission_denies_typed",
];

const DENIAL_LANES: &[&str] = &[
    "invalid_root",
    "built_in_operator_denied",
    "relationship_proof_admission_denied",
    "scope_shape_denied",
    "authoring_denied",
    "canonicalization_denied",
    "validation_denied",
    "planning_denied",
    "basis_resolution_denied",
    "basis_preflight_denied",
    "execution_denied",
];

fn extension_hook_family_names() -> &'static [&'static str] {
    READ_COMPOSITION_EXTENSION_HOOK_FAMILY_NAMES
}

fn default_read_composition_extension_hook_support_rows(
) -> Vec<WorthQueryReadCompositionExtensionHookSupportRow> {
    debug_assert_eq!(READ_COMPOSITION_EXTENSION_HOOK_FAMILIES.len(), 3);
    vec![
        WorthQueryReadCompositionExtensionHookSupportRow::new(
            READ_COMPOSITION_EXTENSION_HOOK_FAMILIES[0],
            WorthQueryReadCompositionExtensionHookBoundary::Lowering,
            false,
        ),
        WorthQueryReadCompositionExtensionHookSupportRow::new(
            READ_COMPOSITION_EXTENSION_HOOK_FAMILIES[1],
            WorthQueryReadCompositionExtensionHookBoundary::Decoder,
            false,
        ),
        WorthQueryReadCompositionExtensionHookSupportRow::new(
            READ_COMPOSITION_EXTENSION_HOOK_FAMILIES[2],
            WorthQueryReadCompositionExtensionHookBoundary::Certification,
            false,
        ),
    ]
}

impl WorthQueryRuntime {
    pub fn public_read_composition_support_report_for_support_profile(
        support_profile: &WorthQueryRuntimeSupportProfile,
    ) -> WorthQueryReadCompositionSupportReport {
        WorthQueryReadCompositionSupportReport::derive(support_profile.posture())
    }

    pub fn public_read_composition_support_report(&self) -> WorthQueryReadCompositionSupportReport {
        Self::public_read_composition_support_report_for_support_profile(
            &self.backend.support_profile(),
        )
    }
}
