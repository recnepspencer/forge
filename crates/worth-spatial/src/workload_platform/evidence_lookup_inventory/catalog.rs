use super::row::{
    EvidenceLookupAuthorityKind as Kind, EvidenceLookupCertificationPosture as Cert,
    EvidenceLookupCostPosture as Cost, EvidenceLookupDisposition as Disposition,
    EvidenceLookupInventoryRow, EvidenceLookupInventoryRowBuilder,
    EvidenceLookupInventoryRowScope as Scope, EvidenceLookupOwner as Owner,
    EvidenceLookupQuerySurface as Query, EvidenceLookupReplacementPhase as Phase,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EvidenceLookupCatalogDiscoveryExpectation {
    DiscoveryRequired,
    CatalogOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredEvidenceLookupSurface {
    pub(super) source_path: &'static str,
    pub(super) surface_name: &'static str,
    pub(super) owner: Owner,
    pub(super) current_caller: &'static str,
    pub(super) authority_kind: Kind,
    pub(super) disposition: Disposition,
    pub(super) replacement_phase: Phase,
    pub(super) blocker: &'static str,
    pub(super) removal_trigger: &'static str,
    pub(super) certification_posture: Cert,
    pub(super) cost_posture: Cost,
    pub(super) query_surface: Query,
    pub(super) row_scope: Scope,
    pub(super) discovery_expectation: EvidenceLookupCatalogDiscoveryExpectation,
}

impl CoveredEvidenceLookupSurface {
    pub(crate) fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub(crate) const fn row_scope(&self) -> Scope {
        self.row_scope
    }

    pub(crate) const fn discovery_expectation(&self) -> EvidenceLookupCatalogDiscoveryExpectation {
        self.discovery_expectation
    }

    pub(crate) fn into_row_builder(&self) -> EvidenceLookupInventoryRowBuilder {
        EvidenceLookupInventoryRow::builder()
            .source_path(self.source_path)
            .surface_name(self.surface_name)
            .owner(self.owner)
            .current_caller(self.current_caller)
            .authority_kind(self.authority_kind)
            .disposition(self.disposition)
            .replacement_phase(self.replacement_phase)
            .blocker(self.blocker)
            .removal_trigger(self.removal_trigger)
            .certification_posture(self.certification_posture)
            .cost_posture(self.cost_posture)
            .query_surface(self.query_surface)
            .row_scope(self.row_scope)
    }
}

pub(crate) fn covered_evidence_lookup_surfaces() -> Vec<CoveredEvidenceLookupSurface> {
    vec![
        surface(
            "workload vocabulary public evidence exports",
            "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
            Owner::WorthSpatial,
            "external callers through worth_spatial::facade::workload_vocabulary",
            Kind::PublicEvidenceRowExposure,
            Disposition::Migrate,
            Phase::PhaseEightSweep,
            "public facade exports old evidence rows and ledgers as ordinary vocabulary",
            "replace public evidence exposure with read-only lookup closeout and receipt products",
            Cert::OrdinaryProductionReachable,
            Cost::PublicFacadeExposure,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
        surface(
            "CompleteWorkloadEvidenceLedger::require_boolean_receipt_lookup",
            "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs",
            Owner::WorthSpatial,
            "boolean stage receipt gates and workload composition",
            Kind::BroadReceiptScan,
            Disposition::Migrate,
            Phase::PhaseTwoFamilyCatalog,
            "ledger receipt lookup is old authority until family catalog selection exists",
            "replace with selected lookup family and execution receipt",
            Cert::OrdinaryProductionReachable,
            Cost::BroadReceiptLedgerScan,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
        ),
        surface(
            "WorkloadEvidenceStageIndexProduct::require_boolean_receipt_lookup",
            "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs",
            Owner::WorthSpatial,
            "stage index receipt lookup products",
            Kind::BooleanStageLookupHelper,
            Disposition::Migrate,
            Phase::PhaseTwoFamilyCatalog,
            "stage index lookup still names evidence by stage-local helper",
            "replace with lookup family declaration over spatial touch authority",
            Cert::OrdinaryProductionReachable,
            Cost::LocalTypedLookup,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
        surface(
            "WorkloadEvidenceRow::new",
            "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs",
            Owner::WorthSpatial,
            "manual workload evidence row construction",
            Kind::RawEvidenceVectorAccess,
            Disposition::Delete,
            Phase::PhaseFourteenDeletion,
            "manual evidence rows cannot satisfy lookup product proof",
            "delete public ordinary use after lookup products own stage evidence",
            Cert::OrdinaryProductionReachable,
            Cost::PublicFacadeExposure,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
        ),
        surface(
            "evidence ledger guard and stage link lookup",
            "crates/worth-spatial/src/workload_platform/evidence_ledger",
            Owner::WorthSpatial,
            "workload evidence guard and required-stage link validation",
            Kind::StageLocalNearbyLookup,
            Disposition::Migrate,
            Phase::PhaseTwoFamilyCatalog,
            "evidence ledger guard paths still look up rows and links by stage identity",
            "replace guard lookup with selected lookup plans and execution receipt products",
            Cert::OrdinaryProductionReachable,
            Cost::LocalTypedLookup,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
        surface(
            "certification workload evidence snapshots",
            "crates/worth-spatial/src/certification/workload_evidence.rs",
            Owner::WorthSpatial,
            "public contract and malformed-ledger certification tests",
            Kind::PublicEvidenceRowExposure,
            Disposition::CertificationOnly,
            Phase::NotReplacedCertificationOnly,
            "certification snapshots inspect rows but cannot mint ordinary lookup proof",
            "keep only while public contracts need malformed evidence fixtures",
            Cert::CertificationOnlyDeniedAsOrdinaryProof,
            Cost::SourceInventoryOnly,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
        ),
        surface(
            "planar boolean edge splitting stage index consumers",
            "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting",
            Owner::WorthSpatial,
            "planar boolean edge splitting request and candidate gates",
            Kind::BooleanStageLookupHelper,
            Disposition::Migrate,
            Phase::PhaseEightSweep,
            "edge splitting uses stage index and evidence identity checks as old lookup authority",
            "replace edge-splitting lookup with family-specific lookup products during sweep",
            Cert::OrdinaryProductionReachable,
            Cost::LocalTypedLookup,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
        ),
        surface(
            "planar boolean loop reconstruction test continuation index",
            "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support",
            Owner::WorthSpatial,
            "loop reconstruction adversarial runtime tests",
            Kind::StageLocalNearbyLookup,
            Disposition::Cap,
            Phase::PhaseFourteenDeletion,
            "test continuation index is non-ordinary residue and cannot seed lookup planning",
            "delete when Phase 8 sweep has migrated loop reconstruction lookup",
            Cert::NonOrdinaryResidueDeniedAsOrdinaryProof,
            Cost::SourceInventoryOnly,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
        ),
        surface(
            "spatial query adoption inventory rows",
            "crates/worth-spatial/src/query_adoption.rs",
            Owner::WorthSpatial,
            "downstream query adoption inventory report",
            Kind::QueryLookingLocalProof,
            Disposition::QueryGap,
            Phase::PhaseTenConsumerKit,
            "local query-looking rows cannot satisfy spatial lookup authority",
            "replace with Consumer Kit proof and exact Query surface matrix rows",
            Cert::OrdinaryProductionReachable,
            Cost::SourceInventoryOnly,
            Query::ConsumerKitProof,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
        surface(
            "spatial touch admission query lowering denials",
            "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission",
            Owner::WorthSpatial,
            "spatial touch query descriptor boundary tests",
            Kind::QueryLookingLocalProof,
            Disposition::QueryGap,
            Phase::PhaseThreeAdmission,
            "query descriptors are adjacent inputs and cannot become lookup products",
            "bind Query surfaces explicitly during lookup input admission",
            Cert::OrdinaryProductionReachable,
            Cost::SourceInventoryOnly,
            Query::TypedArtifactIdentity,
            EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
        ),
        surface(
            "kernel boolean chain legacy stage index lookup accounting",
            "crates/worth-kernel/src/workload_composition/worth_workload/boolean_chain_handoff.rs",
            Owner::WorthKernel,
            "kernel boolean chain integration handoff",
            Kind::BooleanStageLookupHelper,
            Disposition::Migrate,
            Phase::PhaseEightSweep,
            "boolean chain integration still counts old split and loop receipt lookup products as ordinary stage-index lookup proof",
            "replace boolean chain lookup accounting with family-specific receipt-backed lookup proof counters",
            Cert::OrdinaryProductionReachable,
            Cost::LocalTypedLookup,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
        surface(
            "kernel workload composition lookup-shaped residue",
            "crates/worth-kernel/src/workload_composition",
            Owner::WorthKernel,
            "kernel workload composition family summary",
            Kind::BooleanStageLookupHelper,
            Disposition::Migrate,
            Phase::PhaseEightSweep,
            "kernel workload composition still contains ordinary lookup-shaped split or loop receipt helper residue",
            "finish deleting generic kernel receipt lookup authority from workload composition after the migrated lookup products fully own the lane",
            Cert::OrdinaryProductionReachable,
            Cost::LocalTypedLookup,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
        surface(
            "kernel boolean split and loop legacy receipt lookup helpers",
            "crates/worth-kernel/src/workload_composition/worth_workload/boolean_stage_requirements.rs",
            Owner::WorthKernel,
            "kernel workload composition handoffs and boolean chain integration",
            Kind::BooleanStageLookupHelper,
            Disposition::Migrate,
            Phase::PhaseEightSweep,
            "kernel still exposes generic split and loop receipt lookup helpers as ordinary authority",
            "replace kernel legacy receipt lookup helpers with family-specific receipt-backed lookup products or remove them from ordinary execution",
            Cert::OrdinaryProductionReachable,
            Cost::LocalTypedLookup,
            Query::NotQuery,
            EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        ),
    ]
}

fn surface(
    surface_name: &'static str,
    source_path: &'static str,
    owner: Owner,
    current_caller: &'static str,
    authority_kind: Kind,
    disposition: Disposition,
    replacement_phase: Phase,
    blocker: &'static str,
    removal_trigger: &'static str,
    certification_posture: Cert,
    cost_posture: Cost,
    query_surface: Query,
    discovery_expectation: EvidenceLookupCatalogDiscoveryExpectation,
) -> CoveredEvidenceLookupSurface {
    CoveredEvidenceLookupSurface {
        source_path,
        surface_name,
        owner,
        current_caller,
        authority_kind,
        disposition,
        replacement_phase,
        blocker,
        removal_trigger,
        certification_posture,
        cost_posture,
        query_surface,
        row_scope: row_scope_for_source_path(source_path),
        discovery_expectation,
    }
}

fn row_scope_for_source_path(source_path: &str) -> Scope {
    if source_path.as_bytes().ends_with(b".rs") {
        Scope::ConcreteSource
    } else {
        Scope::FamilySummary
    }
}
