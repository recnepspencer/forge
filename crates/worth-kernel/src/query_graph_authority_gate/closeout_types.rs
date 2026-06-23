use super::types::{
    WorthGraphAuthorityAction, WorthGraphAuthorityDeletionTarget, WorthGraphAuthorityInventoryRow,
    WorthGraphAuthorityOwner, WorthGraphAuthorityRootFamily, WorthGraphAuthoritySourceScope,
    WorthLowerAuthorityPromotionCase,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthGraphAuthorityCloseoutDisposition {
    PublicFacadeStatusOnly,
    CollapsedCanonicalQueryProof,
    CollapsedSplitLedgerReceipt,
    CollapsedLoopLedgerReceipt,
    CertificationOnlyBoundary,
    DeletedSurface,
    ExplicitResidue,
    QueryCapabilityGap,
}

impl WorthGraphAuthorityCloseoutDisposition {
    pub fn label(self) -> &'static str {
        match self {
            Self::PublicFacadeStatusOnly => "public-facade-status-only",
            Self::CollapsedCanonicalQueryProof => "collapsed-canonical-query-proof",
            Self::CollapsedSplitLedgerReceipt => "collapsed-split-ledger-receipt",
            Self::CollapsedLoopLedgerReceipt => "collapsed-loop-ledger-receipt",
            Self::CertificationOnlyBoundary => "certification-only-boundary",
            Self::DeletedSurface => "deleted-surface",
            Self::ExplicitResidue => "explicit-residue",
            Self::QueryCapabilityGap => "query-capability-gap",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthGraphAuthorityCloseoutBypassClass {
    SyntheticProof,
    LocalSupportPin,
    CopiedRows,
    HandoffOnlyReceipt,
    RawEvidenceVector,
    StringStageLink,
}

impl WorthGraphAuthorityCloseoutBypassClass {
    pub const ALL: [Self; 6] = [
        Self::SyntheticProof,
        Self::LocalSupportPin,
        Self::CopiedRows,
        Self::HandoffOnlyReceipt,
        Self::RawEvidenceVector,
        Self::StringStageLink,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::SyntheticProof => "synthetic-proof",
            Self::LocalSupportPin => "local-support-pin",
            Self::CopiedRows => "copied-rows",
            Self::HandoffOnlyReceipt => "handoff-only-receipt",
            Self::RawEvidenceVector => "raw-evidence-vector",
            Self::StringStageLink => "string-stage-link",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthGraphAuthorityPublicFacadeProof {
    TopologyOperatorQuerySurface,
    SpatialEvidenceLedgerSurface,
    KernelCloseoutReportSurface,
    ForgeQueryConsumerKitSurface,
}

impl WorthGraphAuthorityPublicFacadeProof {
    pub const ALL: [Self; 4] = [
        Self::TopologyOperatorQuerySurface,
        Self::SpatialEvidenceLedgerSurface,
        Self::KernelCloseoutReportSurface,
        Self::ForgeQueryConsumerKitSurface,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::TopologyOperatorQuerySurface => "topology-operator-query-surface",
            Self::SpatialEvidenceLedgerSurface => "spatial-evidence-ledger-surface",
            Self::KernelCloseoutReportSurface => "kernel-closeout-report-surface",
            Self::ForgeQueryConsumerKitSurface => "forge-query-consumer-kit-surface",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityPublicFacadeEvidence {
    proof: WorthGraphAuthorityPublicFacadeProof,
    ordinary_api: &'static str,
    posture_accessor: &'static str,
    contract_test_path: &'static str,
    contract_symbols: &'static [&'static str],
}

impl WorthGraphAuthorityPublicFacadeEvidence {
    pub(crate) const fn new(
        proof: WorthGraphAuthorityPublicFacadeProof,
        ordinary_api: &'static str,
        posture_accessor: &'static str,
        contract_test_path: &'static str,
        contract_symbols: &'static [&'static str],
    ) -> Self {
        Self {
            proof,
            ordinary_api,
            posture_accessor,
            contract_test_path,
            contract_symbols,
        }
    }

    pub fn proof(&self) -> WorthGraphAuthorityPublicFacadeProof {
        self.proof
    }

    pub fn ordinary_api(&self) -> &'static str {
        self.ordinary_api
    }

    pub fn posture_accessor(&self) -> &'static str {
        self.posture_accessor
    }

    pub fn contract_test_path(&self) -> &'static str {
        self.contract_test_path
    }

    pub fn contract_symbols(&self) -> &'static [&'static str] {
        self.contract_symbols
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityCloseoutBypassEvidence {
    bypass_class: WorthGraphAuthorityCloseoutBypassClass,
    promotion_case: WorthLowerAuthorityPromotionCase,
    compile_fail_path: &'static str,
    rejected_artifact: &'static str,
    required_authority_type: &'static str,
}

impl WorthGraphAuthorityCloseoutBypassEvidence {
    pub(crate) fn new(
        bypass_class: WorthGraphAuthorityCloseoutBypassClass,
        promotion_case: WorthLowerAuthorityPromotionCase,
        compile_fail_path: &'static str,
        rejected_artifact: &'static str,
        required_authority_type: &'static str,
    ) -> Self {
        Self {
            bypass_class,
            promotion_case,
            compile_fail_path,
            rejected_artifact,
            required_authority_type,
        }
    }

    pub fn bypass_class(&self) -> WorthGraphAuthorityCloseoutBypassClass {
        self.bypass_class
    }

    pub fn promotion_case(&self) -> WorthLowerAuthorityPromotionCase {
        self.promotion_case
    }

    pub fn compile_fail_path(&self) -> &'static str {
        self.compile_fail_path
    }

    pub fn rejected_artifact(&self) -> &'static str {
        self.rejected_artifact
    }

    pub fn required_authority_type(&self) -> &'static str {
        self.required_authority_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityDeletionClassCloseoutEvidence {
    deletion_target: WorthGraphAuthorityDeletionTarget,
    removal_ledger_rows: usize,
    affected_source_files: usize,
    affected_source_lines: usize,
}

impl WorthGraphAuthorityDeletionClassCloseoutEvidence {
    pub(crate) fn new(
        deletion_target: WorthGraphAuthorityDeletionTarget,
        removal_ledger_rows: usize,
        affected_source_files: usize,
        affected_source_lines: usize,
    ) -> Self {
        Self {
            deletion_target,
            removal_ledger_rows,
            affected_source_files,
            affected_source_lines,
        }
    }

    pub fn deletion_target(&self) -> WorthGraphAuthorityDeletionTarget {
        self.deletion_target
    }

    pub fn removal_ledger_rows(&self) -> usize {
        self.removal_ledger_rows
    }

    pub fn affected_source_files(&self) -> usize {
        self.affected_source_files
    }

    pub fn affected_source_lines(&self) -> usize {
        self.affected_source_lines
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityCloseoutMatrixRow {
    pub(crate) source_id: &'static str,
    pub(crate) source_scope: &'static str,
    pub(crate) owner: WorthGraphAuthorityOwner,
    pub(crate) root_family: Option<WorthGraphAuthorityRootFamily>,
    pub(crate) deletion_target: WorthGraphAuthorityDeletionTarget,
    pub(crate) disposition: WorthGraphAuthorityCloseoutDisposition,
    pub(crate) public_facade_evidence: WorthGraphAuthorityPublicFacadeEvidence,
    pub(crate) proof_boundary: &'static str,
}

impl WorthGraphAuthorityCloseoutMatrixRow {
    pub fn source_id(&self) -> &'static str {
        self.source_id
    }

    pub fn source_scope(&self) -> &'static str {
        self.source_scope
    }

    pub fn owner(&self) -> WorthGraphAuthorityOwner {
        self.owner
    }

    pub fn root_family(&self) -> Option<WorthGraphAuthorityRootFamily> {
        self.root_family
    }

    pub fn deletion_target(&self) -> WorthGraphAuthorityDeletionTarget {
        self.deletion_target
    }

    pub fn disposition(&self) -> WorthGraphAuthorityCloseoutDisposition {
        self.disposition
    }

    pub fn ordinary_public_facade(&self) -> &'static str {
        self.public_facade_evidence.ordinary_api()
    }

    pub fn public_facade_evidence(&self) -> WorthGraphAuthorityPublicFacadeEvidence {
        self.public_facade_evidence
    }

    pub fn proof_boundary(&self) -> &'static str {
        self.proof_boundary
    }
}

pub(crate) fn closeout_row_for_inventory(
    row: &WorthGraphAuthorityInventoryRow,
) -> WorthGraphAuthorityCloseoutMatrixRow {
    WorthGraphAuthorityCloseoutMatrixRow {
        source_id: row.source_id(),
        source_scope: source_scope_label(row.source_scope()),
        owner: row.owner(),
        root_family: None,
        deletion_target: row.deletion_target(),
        disposition: disposition_for(row.action(), row.deletion_target()),
        public_facade_evidence: facade_evidence_for(row.owner()),
        proof_boundary: proof_boundary_for(row.deletion_target()),
    }
}

fn source_scope_label(source_scope: WorthGraphAuthoritySourceScope) -> &'static str {
    match source_scope {
        WorthGraphAuthoritySourceScope::ExactSource => "exact-source",
        WorthGraphAuthoritySourceScope::AuditedSourceSet { .. } => "audited-source-set",
    }
}

pub(crate) fn disposition_for(
    action: WorthGraphAuthorityAction,
    target: WorthGraphAuthorityDeletionTarget,
) -> WorthGraphAuthorityCloseoutDisposition {
    match action {
        WorthGraphAuthorityAction::Delete => WorthGraphAuthorityCloseoutDisposition::DeletedSurface,
        WorthGraphAuthorityAction::Residue => {
            WorthGraphAuthorityCloseoutDisposition::ExplicitResidue
        }
        WorthGraphAuthorityAction::QueryGap => {
            WorthGraphAuthorityCloseoutDisposition::QueryCapabilityGap
        }
        WorthGraphAuthorityAction::Keep => {
            WorthGraphAuthorityCloseoutDisposition::PublicFacadeStatusOnly
        }
        WorthGraphAuthorityAction::Collapse => match target {
            WorthGraphAuthorityDeletionTarget::CopiedEvidenceRows => {
                WorthGraphAuthorityCloseoutDisposition::CollapsedSplitLedgerReceipt
            }
            WorthGraphAuthorityDeletionTarget::StringStageLink => {
                WorthGraphAuthorityCloseoutDisposition::CollapsedLoopLedgerReceipt
            }
            WorthGraphAuthorityDeletionTarget::SyntheticFixture => {
                WorthGraphAuthorityCloseoutDisposition::CertificationOnlyBoundary
            }
            _ => WorthGraphAuthorityCloseoutDisposition::CollapsedCanonicalQueryProof,
        },
    }
}

fn facade_evidence_for(owner: WorthGraphAuthorityOwner) -> WorthGraphAuthorityPublicFacadeEvidence {
    match owner {
        WorthGraphAuthorityOwner::Topology => WorthGraphAuthorityPublicFacadeEvidence::new(
            WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface,
            "worth_topo::facade::topology_operator_graph_obligation_catalog",
            "TopologyOperatorGraphObligationCatalog::rows",
            "crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api_topology_operator_surface.rs",
            &[
                "topology_operator_graph_obligation_catalog",
                "TopologyOperatorGraphObligationCatalog",
                "rows",
            ],
        ),
        WorthGraphAuthorityOwner::Spatial => WorthGraphAuthorityPublicFacadeEvidence::new(
            WorthGraphAuthorityPublicFacadeProof::SpatialEvidenceLedgerSurface,
            "worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStageIndexProduct",
            "WorkloadEvidenceStageIndexProduct::counters",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/workload_vocabulary/boolean_evidence_ledger.rs",
            &["WorkloadEvidenceStageIndexProduct", "counters"],
        ),
        WorthGraphAuthorityOwner::Kernel => WorthGraphAuthorityPublicFacadeEvidence::new(
            WorthGraphAuthorityPublicFacadeProof::KernelCloseoutReportSurface,
            "worth_kernel::query_graph_authority_gate::current_worth_graph_authority_closeout_report",
            "WorthGraphAuthorityCloseoutReport::counters",
            "crates/worth-kernel/src/query_graph_authority_gate/closeout_tests.rs",
            &[
                "current_worth_graph_authority_closeout_report",
                "WorthGraphAuthorityCloseoutReport",
                "counters",
            ],
        ),
        WorthGraphAuthorityOwner::ForgeQuery => WorthGraphAuthorityPublicFacadeEvidence::new(
            WorthGraphAuthorityPublicFacadeProof::ForgeQueryConsumerKitSurface,
            "forge_query::graph_obligation_consumer_kit",
            "ForgeQueryGraphObligationAdoptionProof::manifest",
            "crates/forge-query/src/consumer_kit/graph_obligation_adoption/tests/adoption.rs",
            &[
                "graph_obligation_consumer_kit",
                "ForgeQueryGraphObligationAdoptionProof",
                "manifest",
            ],
        ),
    }
}

fn proof_boundary_for(target: WorthGraphAuthorityDeletionTarget) -> &'static str {
    match target {
        WorthGraphAuthorityDeletionTarget::None => "canonical Query graph authority receipt",
        WorthGraphAuthorityDeletionTarget::DuplicateSupportReport => "Query support manifest",
        WorthGraphAuthorityDeletionTarget::LocalSupportPinWrapper => "sealed support receipt",
        WorthGraphAuthorityDeletionTarget::BlueprintProofObligation => {
            "Query graph obligation receipt"
        }
        WorthGraphAuthorityDeletionTarget::CeremonyAudit => "deleted local ceremony",
        WorthGraphAuthorityDeletionTarget::HandoffOnlyHelper => {
            "executed graph-authority result type"
        }
        WorthGraphAuthorityDeletionTarget::RawEvidenceScan => {
            "indexed evidence-stage lookup product"
        }
        WorthGraphAuthorityDeletionTarget::CopiedEvidenceRows => "split ledger receipt identity",
        WorthGraphAuthorityDeletionTarget::StringStageLink => {
            "loop ledger stage-link receipt identity"
        }
        WorthGraphAuthorityDeletionTarget::SyntheticFixture => {
            "compile-fail certification boundary"
        }
        WorthGraphAuthorityDeletionTarget::CompatibilityReport => "deleted compatibility report",
        WorthGraphAuthorityDeletionTarget::ResidueManifest => "capped residue manifest",
    }
}
