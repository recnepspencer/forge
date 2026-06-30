use super::model::{
    SpatialEvidenceSurfaceAuthorityCategory as Category,
    SpatialEvidenceSurfaceDeletionAction as Action, SpatialEvidenceSurfaceDeletionLedgerRow,
    SpatialEvidenceSurfaceOwner as Owner,
};

const FACADE: &str = "worth_spatial::facade::workload_vocabulary";
const QUERY_FACADE: &str = "worth_spatial::facade::query_adoption";
const SPATIAL_LEDGER_TRIGGER: &str =
    "Milestone 4 spatial touch authority proof replaces generic workload evidence inspection.";
const QUERY_TRIGGER: &str =
    "Phase 8 Query consumer-kit adoption replaces local query adoption inventory rows.";

macro_rules! workload_facade_exports {
    ($($name:ident),+ $(,)?) => {
        [
            $(
                (
                    stringify!($name),
                    concat!("worth_spatial::facade::workload_vocabulary::", stringify!($name)),
                ),
            )+
        ]
    };
}

const fn row(
    surface_name: &'static str,
    source_path: &'static str,
    exported_facade_path: &'static str,
    authority_category: Category,
    current_caller: &'static str,
    deletion_action: Action,
    owner: Owner,
    cap: &'static str,
    removal_trigger: &'static str,
    production_reachable: bool,
    replacement_exists: bool,
) -> SpatialEvidenceSurfaceDeletionLedgerRow {
    SpatialEvidenceSurfaceDeletionLedgerRow::new(
        surface_name,
        source_path,
        exported_facade_path,
        authority_category,
        current_caller,
        deletion_action,
        owner,
        cap,
        removal_trigger,
        production_reachable,
        replacement_exists,
    )
}

pub fn spatial_evidence_surface_deletion_ledger() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    let mut rows = Vec::new();
    rows.extend(public_facade_exports());
    rows.extend(ledger_constructors());
    rows.extend(boolean_receipt_implementations());
    rows.extend(spatial_query_adoption_rows());
    rows.extend(downstream_workload_consumption_paths());
    rows.extend(migrated_downstream_consumer_paths());
    rows.extend(topology_substitution_boundaries());
    rows.extend(deleted_legacy_surfaces());
    rows
}

fn public_facade_exports() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    workload_facade_exports![
        BooleanEvidenceReceipt,
        BooleanEvidenceRowAuthority,
        BooleanEvidenceStageKind,
        CompleteWorkloadEvidenceLedger,
        DiagnosticWorkload,
        DiagnosticWorkloadReceipt,
        GeometryBindingWorkload,
        GeometryBindingWorkloadReceipt,
        ProjectionWorkload,
        ProjectionWorkloadReceipt,
        ResponseWorkload,
        ResponseWorkloadReceipt,
        RetainedReplayWorkload,
        RetainedReplayWorkloadReceipt,
        SpatialEvidenceSubstitutionDenial,
        SpatialEvidenceSurfaceAuthorityCategory,
        SpatialEvidenceSurfaceCloseoutPosture,
        SpatialEvidenceSurfaceDeletionAction,
        SpatialEvidenceSurfaceDeletionLedgerRow,
        SpatialEvidenceSurfaceOwner,
        SpatialEvidenceTopologySubstitutionSurface,
        SpatialWorkloadStage,
        SurfaceSupportWorkload,
        SurfaceSupportWorkloadReceipt,
        TransformWorkload,
        TransformWorkloadReceipt,
        WorkloadEvidenceBacking,
        WorkloadEvidenceBooleanReceiptLookupProduct,
        WorkloadEvidenceCounters,
        WorkloadEvidenceGuard,
        WorkloadEvidenceGuardError,
        WorkloadEvidenceLedger,
        WorkloadEvidenceLedgerError,
        WorkloadEvidenceRow,
        WorkloadEvidenceStage,
        WorkloadEvidenceStageBinding,
        WorkloadEvidenceStageCounters,
        WorkloadEvidenceStageIndexCounters,
        WorkloadEvidenceStageIndexProduct,
        WorkloadEvidenceStageLink,
        WorkloadEvidenceStageLinkSet,
        WorkloadEvidenceSupport,
        WorkloadStageDenial,
        WorkloadStageEnvelope,
        WorkloadStageIdentity,
        WorkloadStagePosture,
        WorkloadStageSupport,
        deny_manual_evidence_row_as_spatial_touch_authority,
        deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority,
        deny_topology_laundering_as_spatial_touch_authority,
        deny_topology_touched_graph_basis_as_spatial_touch_authority,
        spatial_evidence_surface_deletion_ledger,
    ]
    .into_iter()
    .map(|(name, exported_path)| {
        row(
            name,
            "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
            exported_path,
            Category::PublicFacadeExport,
            "External callers through worth_spatial::facade::workload_vocabulary.",
            Action::CollapseToSpatialTouchAuthority,
            Owner::WorthSpatial,
            "Facade export is inventory-only for Phase 1 and cannot mint spatial touch authority.",
            SPATIAL_LEDGER_TRIGGER,
            true,
            false,
        )
    })
    .collect()
}

fn ledger_constructors() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    [
        ("WorkloadEvidenceRow::new", "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs"),
        ("WorkloadEvidenceRow::from_boolean_evidence_receipt", "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs"),
        ("WorkloadEvidenceLedger::from_rows", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("WorkloadEvidenceLedger::certify_complete", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("CompleteWorkloadEvidenceLedger::require_boolean_receipt", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("CompleteWorkloadEvidenceLedger::require_boolean_receipt_lookup", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("CompleteWorkloadEvidenceLedger::link_required_stages", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("CompleteWorkloadEvidenceLedger::with_boolean_evidence_receipt", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("CompleteWorkloadEvidenceLedger::into_ledger", "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
        ("WorkloadEvidenceStageIndexProduct::require_boolean_receipt", "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs"),
        ("WorkloadEvidenceStageIndexProduct::require_boolean_receipt_lookup", "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs"),
        ("WorkloadEvidenceStageIndexProduct::link_required_stages", "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs"),
    ]
    .into_iter()
    .map(|(name, source)| {
        let action = if name == "WorkloadEvidenceRow::new" {
            Action::CollapseToSpatialTouchAuthority
        } else {
            Action::CollapseToSpatialTouchAuthority
        };
        row(
            name,
            source,
            FACADE,
            Category::LedgerConstructor,
            "downstream workload composition and public facade certification contracts.",
            action,
            Owner::WorthSpatial,
            "Manual rows remain generic workload evidence only; boundary denial prevents spatial touch authority substitution.",
            SPATIAL_LEDGER_TRIGGER,
            true,
            false,
        )
    })
    .collect()
}

fn boolean_receipt_implementations() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    [
        ("PlanarBooleanEventLedgerReceipt", "crates/worth-spatial/src/workload_platform/planar_boolean_events/event_ledger/receipt.rs"),
        ("PlanarBooleanSegmentPairEnumerationReceipt", "crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/receipt.rs"),
        ("PlanarBooleanSplitEdgeChainLedgerReceipt", "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/split_edge_chain_ledger/receipt.rs"),
        ("PlanarBooleanLoopReconstructionLedgerReceipt", "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/loop_reconstruction_ledger/receipt.rs"),
    ]
    .into_iter()
    .map(|(name, source)| {
        row(
            name,
            source,
            FACADE,
            Category::BooleanReceiptImplementation,
            "WorkloadEvidenceRow::from_boolean_evidence_receipt and downstream boolean evidence gates.",
            Action::CollapseToSpatialTouchAuthority,
            Owner::WorthSpatial,
            "Receipts are sealed boolean evidence only until spatial touch authority admits them.",
            SPATIAL_LEDGER_TRIGGER,
            true,
            false,
        )
    })
    .collect()
}

fn spatial_query_adoption_rows() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    const DOWNSTREAM_CALLER: &str = "downstream workload query adoption inventory report";
    const INVENTORY_CAP: &str =
        "Query adoption inventory rows cannot construct or satisfy spatial evidence authority.";

    [
        query_row(
            "spatial_query_adoption_inventory::workload_platform",
            "crates/worth-spatial/src/workload_platform",
            Action::CollapseToQueryConsumerKitProof,
            INVENTORY_CAP,
            QUERY_TRIGGER,
        ),
        query_row(
            "spatial_query_adoption_inventory::witness_resolution",
            "crates/worth-spatial/src/witness_resolution",
            Action::CollapseToQueryConsumerKitProof,
            INVENTORY_CAP,
            QUERY_TRIGGER,
        ),
        query_row(
            "spatial_query_adoption_inventory::public_facade_contracts",
            "crates/worth-spatial/src/certification/public_facade_contracts",
            Action::CertificationOnly,
            "Certification contracts cannot construct ordinary spatial evidence authority.",
            "Certification-only rows are removed when public facade contracts migrate to Query proof fixtures.",
        ),
        query_row(
            "spatial_query_adoption_inventory::test_support",
            "crates/worth-spatial/src/test_support",
            Action::CappedResidue,
            "Test support is crate-local residue and cannot satisfy public authority APIs.",
            "Delete when spatial touch authority has first-party deterministic fixtures.",
        ),
        query_row(
            "spatial_query_adoption_inventory::workload_platform_vocabulary",
            "crates/worth-spatial/src/workload_platform/vocabulary",
            Action::CappedResidue,
            "Diagnostic vocabulary residue is capped to reports and cannot mint authority.",
            "Collapse when workload vocabulary diagnostics are Query-pinned.",
        ),
    ]
    .into_iter()
    .map(|(name, source, action, cap, removal_trigger)| {
        row(
            name,
            source,
            QUERY_FACADE,
            Category::SpatialQueryAdoptionRow,
            DOWNSTREAM_CALLER,
            action,
            Owner::WorthSpatial,
            cap,
            removal_trigger,
            true,
            false,
        )
    })
    .collect()
}

const fn query_row(
    name: &'static str,
    source: &'static str,
    action: Action,
    cap: &'static str,
    removal_trigger: &'static str,
) -> (
    &'static str,
    &'static str,
    Action,
    &'static str,
    &'static str,
) {
    (name, source, action, cap, removal_trigger)
}

fn downstream_workload_consumption_paths() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    [
        "WorthWorkload::require_boolean_declaration_entry",
        "WorthWorkload::require_boolean_route_plan",
        "WorthWorkload::require_boolean_operand_pair_construction",
        "WorthWorkload::require_boolean_blocker_provenance",
        "WorthWorkload::require_boolean_shared_plane_identity",
        "WorthWorkload::require_boolean_precision_agreement",
        "WorthWorkload::require_boolean_local_frame_selection",
        "WorthWorkload::require_boolean_operand_a_projection_consumption",
        "WorthWorkload::require_boolean_operand_b_projection_consumption",
        "WorthWorkload::require_boolean_reduced_operand_pair",
        "WorthWorkload::require_boolean_event_extraction_request",
        "WorthWorkload::require_boolean_segment_pair_enumeration",
        "WorthWorkload::require_boolean_event_ledger",
        "WorthWorkload::require_boolean_split",
        "WorthWorkload::require_boolean_loop_reconstruction",
        "WorthWorkload::with_completed_boolean_split_ledger",
        "WorthWorkload::complete_boolean_split_handoff",
        "WorthWorkload::with_completed_boolean_loop_reconstruction",
    ]
    .into_iter()
    .map(|name| {
        row(
            name,
            "downstream-workload-composition/boolean_stage_requirements.rs",
            "downstream_workload::workload_composition::WorthWorkload",
            Category::KernelWorkloadEvidenceConsumption,
            "Planar boolean workload composition.",
            Action::CollapseToSpatialTouchAuthority,
            Owner::WorthKernel,
            "Downstream workload consumption must continue through sealed receipt lookup products, not manual rows.",
            SPATIAL_LEDGER_TRIGGER,
            true,
            false,
        )
    })
    .collect()
}

fn migrated_downstream_consumer_paths() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    [(
        "PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(stage_index)",
        "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/downstream_split_consumption/input.rs",
        "worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanDownstreamSplitConsumptionInput",
        "CompletedBooleanSplitHandoff::admit_downstream_split_consumption",
    )]
    .into_iter()
    .map(|(name, source, exported_path, caller)| {
        row(
            name,
            source,
            exported_path,
            Category::KernelWorkloadEvidenceConsumption,
            caller,
            Action::Deleted,
            Owner::WorthSpatial,
            "Direct stage-index downstream split consumption input was removed; ordinary construction requires spatial touch authority and lookup product.",
            "Phase 6 migrated the first downstream consumer to the spatial facade proof product.",
            false,
            true,
        )
    })
    .collect()
}

fn topology_substitution_boundaries() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    [
        "TopologyTouchedGraphBasis",
        "TopologyDeclaredTouchedGraphBasisProof",
    ]
    .into_iter()
    .map(|name| {
        row(
            name,
            "crates/worth-topo/src/facade.rs",
            "topology::facade",
            Category::TopologySubstitutionBoundary,
            "External topology authority callers.",
            Action::CertificationOnly,
            Owner::WorthTopo,
            "Topology authority is a separate domain and has no constructor for spatial evidence authority.",
            "Spatial touch authority accepts only spatial evidence proof-bearing types.",
            false,
            false,
        )
    })
    .collect()
}

fn deleted_legacy_surfaces() -> Vec<SpatialEvidenceSurfaceDeletionLedgerRow> {
    [(
        "geometry_only_evidence_admission_from_boolean_evidence_receipt",
        "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
    )]
    .into_iter()
    .map(|(name, source)| {
        row(
            name,
            source,
            FACADE,
            Category::DeletedLegacySurface,
            "Former public facade compile-fail target.",
            Action::Deleted,
            Owner::WorthSpatial,
            "Compile-fail contract proves the legacy helper is not exported.",
            "No removal trigger; deleted surface must remain absent.",
            false,
            true,
        )
    })
    .collect()
}
