#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisInventoryOwner {
    Query,
    RuntimeBridge,
    Relational,
    Signal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisInventoryDisposition {
    ConsolidatedLifecycleHome,
    ReusedAuthority,
    CompatibilityAdapter,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasisInventoryRow {
    owner: BasisInventoryOwner,
    surface_label: &'static str,
    disposition: BasisInventoryDisposition,
    note: &'static str,
}

impl BasisInventoryRow {
    pub fn owner(&self) -> BasisInventoryOwner {
        self.owner
    }

    pub fn surface_label(&self) -> &'static str {
        self.surface_label
    }

    pub fn disposition(&self) -> BasisInventoryDisposition {
        self.disposition
    }

    pub fn note(&self) -> &'static str {
        self.note
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasisInventory {
    rows: &'static [BasisInventoryRow],
}

impl BasisInventory {
    pub fn rows(&self) -> &'static [BasisInventoryRow] {
        self.rows
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LowerRuntimeApiReuseClass {
    ReusedAuthority,
    QueryAdapter,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LowerRuntimeApiReuseRow {
    owner: BasisInventoryOwner,
    api_label: &'static str,
    reuse_class: LowerRuntimeApiReuseClass,
    note: &'static str,
}

impl LowerRuntimeApiReuseRow {
    pub fn owner(&self) -> BasisInventoryOwner {
        self.owner
    }

    pub fn api_label(&self) -> &'static str {
        self.api_label
    }

    pub fn reuse_class(&self) -> LowerRuntimeApiReuseClass {
        self.reuse_class
    }

    pub fn note(&self) -> &'static str {
        self.note
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LowerRuntimeApiReuseMatrix {
    rows: &'static [LowerRuntimeApiReuseRow],
}

impl LowerRuntimeApiReuseMatrix {
    pub fn rows(&self) -> &'static [LowerRuntimeApiReuseRow] {
        self.rows
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetDxTranscriptKind {
    CurrentHeadObservation,
    BranchHeadMutationPreparation,
    PreviewDenial,
    CausalInspection,
    LowerRuntimeEvidenceMaterialization,
    SupportDiscovery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetDxTranscriptRow {
    kind: TargetDxTranscriptKind,
    title: &'static str,
    note: &'static str,
}

impl TargetDxTranscriptRow {
    pub fn kind(&self) -> TargetDxTranscriptKind {
        self.kind
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn note(&self) -> &'static str {
        self.note
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetDxTranscriptInventory {
    rows: &'static [TargetDxTranscriptRow],
}

impl TargetDxTranscriptInventory {
    pub fn rows(&self) -> &'static [TargetDxTranscriptRow] {
        self.rows
    }
}

const BASIS_INVENTORY_ROWS: &[BasisInventoryRow] = &[
    BasisInventoryRow {
        owner: BasisInventoryOwner::Query,
        surface_label: "query_context::QueryBasisContextRequest",
        disposition: BasisInventoryDisposition::CompatibilityAdapter,
        note: "legacy raw request surface that must lower immediately into RawBasisIntent",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Query,
        surface_label: "query_context::{admit,bind,execute}_query_basis_context",
        disposition: BasisInventoryDisposition::CompatibilityAdapter,
        note: "existing branch and snapshot admission flow becomes a lifecycle adapter during migration",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Query,
        surface_label: "preview::*_preview_*",
        disposition: BasisInventoryDisposition::CompatibilityAdapter,
        note: "preview workflow helpers remain public but should route through lifecycle proof surfaces",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Query,
        surface_label: "query_basis_lifecycle::*",
        disposition: BasisInventoryDisposition::ConsolidatedLifecycleHome,
        note: "Query-owned home for basis intent, eligibility, capability, and readmission receipts",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        surface_label: "RuntimeBridge::{evaluate,evaluate_current,plan_truth_view_packet}",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "truth-view authority stays bridge-owned and Query only wraps it",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        surface_label: "RuntimeBridge::{plan_source_packet_set,materialize_source_packet}",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "materialization authority remains in runtime bridge",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        surface_label: "RuntimeBridge::{admit_subscription,admit_subscription_preview_basis}",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "subscription and preview basis admission stay bridge-mediated",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        surface_label: "RuntimeBridge::deliver_continuity",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "continuity authority remains bridge-owned and is only readmitted into Query",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        surface_label: "bridge writeback, replay, route, structural, and causal-envelope authorities",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "lower-runtime proof ownership remains outside Query even when Query exposes lifecycle wrappers",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Relational,
        surface_label: "worth_relational::facade::{history,runtime,snapshots,replay,bridge}",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "truth, branch, commit, snapshot, and lineage authority stays relational-owned",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Relational,
        surface_label: "RuntimeBridgeRelationalSource",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "bridge-bound relational flows must use the existing adapter instead of Query-side loaders",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Signal,
        surface_label: "worth_signal::facade::{history,diagnostics,specialist}",
        disposition: BasisInventoryDisposition::ReusedAuthority,
        note: "live observation, snapshot, replay, lineage, and forensic evidence stay signal-owned",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Signal,
        surface_label: "future async/store/durable basis neighbors",
        disposition: BasisInventoryDisposition::DeferredNeighbor,
        note: "unsupported temporal and durable neighbors deny during normalization until later milestones",
    },
    BasisInventoryRow {
        owner: BasisInventoryOwner::Query,
        surface_label: "fresh Query-side branch/snapshot/writeback/causal authority objects",
        disposition: BasisInventoryDisposition::ForbiddenDuplicate,
        note: "9.3.2 forbids duplicating authority proof already owned by bridge, relational, or signal",
    },
];

const LOWER_RUNTIME_API_REUSE_ROWS: &[LowerRuntimeApiReuseRow] = &[
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::evaluate",
        reuse_class: LowerRuntimeApiReuseClass::QueryAdapter,
        note: "Query lowers admitted observation basis into bridge truth-view evaluation without minting new authority",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::evaluate_current",
        reuse_class: LowerRuntimeApiReuseClass::QueryAdapter,
        note: "current-head common path should remain a Query adapter over bridge-owned current evaluation",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::plan_truth_view_packet",
        reuse_class: LowerRuntimeApiReuseClass::ReusedAuthority,
        note: "truth-view planning basis is bridge-owned authority reused directly",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::plan_source_packet_set",
        reuse_class: LowerRuntimeApiReuseClass::ReusedAuthority,
        note: "source-packet planning remains lower-runtime authority",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::materialize_source_packet",
        reuse_class: LowerRuntimeApiReuseClass::QueryAdapter,
        note: "Query materialization should attach proof around bridge-owned packet materialization",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::admit_subscription",
        reuse_class: LowerRuntimeApiReuseClass::QueryAdapter,
        note: "subscription declaration and activation wrap bridge admission instead of re-binding basis locally",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::admit_subscription_preview_basis",
        reuse_class: LowerRuntimeApiReuseClass::QueryAdapter,
        note: "preview-scoped subscription basis remains bridge-owned and should only be readmitted by Query",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "RuntimeBridge::deliver_continuity",
        reuse_class: LowerRuntimeApiReuseClass::QueryAdapter,
        note: "inspection continuity output should be readmitted through Query receipts",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "bridge writeback authority surfaces",
        reuse_class: LowerRuntimeApiReuseClass::ReusedAuthority,
        note: "writeback causality and idempotence basis stay bridge-owned",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::RuntimeBridge,
        api_label: "bridge causal-envelope authority surfaces",
        reuse_class: LowerRuntimeApiReuseClass::ReusedAuthority,
        note: "causal-envelope evidence remains bridge-owned and Query only includes it in outputs",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::Relational,
        api_label: "worth_relational::facade truth/history/snapshot/lineage APIs",
        reuse_class: LowerRuntimeApiReuseClass::ReusedAuthority,
        note: "relational branch, head, snapshot, and lineage evidence remains authority-owned",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::Signal,
        api_label: "worth_signal::facade live observation and invalidation APIs",
        reuse_class: LowerRuntimeApiReuseClass::ReusedAuthority,
        note: "signal observation and invalidation evidence must be facade-returned rather than Query-Worthd",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::Signal,
        api_label: "worth_signal::facade snapshot/replay/lineage/forensic APIs",
        reuse_class: LowerRuntimeApiReuseClass::DeferredNeighbor,
        note: "these are required inventory neighbors for 9.3.2 but some readmission paths remain deferred to later phase-4 batches",
    },
    LowerRuntimeApiReuseRow {
        owner: BasisInventoryOwner::Query,
        api_label: "fresh Query-owned lower-runtime authority clones",
        reuse_class: LowerRuntimeApiReuseClass::ForbiddenDuplicate,
        note: "Query may not invent branch, preview, continuity, writeback, or signal authority duplicates",
    },
];

const TARGET_DX_TRANSCRIPT_ROWS: &[TargetDxTranscriptRow] = &[
    TargetDxTranscriptRow {
        kind: TargetDxTranscriptKind::CurrentHeadObservation,
        title: "current_head_observation",
        note: "common path should read like intent and not require caller-side proof assembly",
    },
    TargetDxTranscriptRow {
        kind: TargetDxTranscriptKind::BranchHeadMutationPreparation,
        title: "branch_head_mutation_preparation",
        note: "branch-head mutation preparation stays basis-explicit and lane-typed",
    },
    TargetDxTranscriptRow {
        kind: TargetDxTranscriptKind::PreviewDenial,
        title: "preview_denial",
        note: "preview-backed authoritative requests deny early with typed posture rather than bridge leakage",
    },
    TargetDxTranscriptRow {
        kind: TargetDxTranscriptKind::CausalInspection,
        title: "causal_inspection",
        note: "inspection callers should consume scoped capability proof rather than raw basis digests",
    },
    TargetDxTranscriptRow {
        kind: TargetDxTranscriptKind::LowerRuntimeEvidenceMaterialization,
        title: "lower_runtime_evidence_materialization",
        note: "receipt/evidence materialization must attach authority facts without caller-side stitching",
    },
    TargetDxTranscriptRow {
        kind: TargetDxTranscriptKind::SupportDiscovery,
        title: "support_discovery",
        note: "support posture should be inspectable through a named API rather than implied by denial strings",
    },
];

pub fn basis_inventory() -> BasisInventory {
    BasisInventory {
        rows: BASIS_INVENTORY_ROWS,
    }
}

pub fn lower_runtime_api_reuse_matrix() -> LowerRuntimeApiReuseMatrix {
    LowerRuntimeApiReuseMatrix {
        rows: LOWER_RUNTIME_API_REUSE_ROWS,
    }
}

pub fn target_dx_transcript_inventory() -> TargetDxTranscriptInventory {
    TargetDxTranscriptInventory {
        rows: TARGET_DX_TRANSCRIPT_ROWS,
    }
}
