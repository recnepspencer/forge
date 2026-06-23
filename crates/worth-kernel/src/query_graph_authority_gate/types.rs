#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthGraphAuthorityOwner {
    Topology,
    Spatial,
    Kernel,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthGraphAuthorityRowClass {
    RootAuthority,
    DuplicateSupport,
    Residue,
    QueryCapabilityGap,
    CertificationOnly,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthGraphAuthorityDeletionTarget {
    None,
    DuplicateSupportReport,
    LocalSupportPinWrapper,
    BlueprintProofObligation,
    CeremonyAudit,
    HandoffOnlyHelper,
    RawEvidenceScan,
    CopiedEvidenceRows,
    StringStageLink,
    SyntheticFixture,
    CompatibilityReport,
    ResidueManifest,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthGraphAuthorityDiscoverySource {
    Compiler,
    CompileFail,
    PublicFacade,
    CertificationOnly,
    SearchSeed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthGraphAuthorityAction {
    Keep,
    Collapse,
    Delete,
    Residue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthGraphAuthorityRootFamily {
    TopologyOperatorCatalog,
    TopologyQueryNativeBoundary,
    SpatialQueryAdoption,
    SpatialEvidenceStageIndex,
    KernelQueryAuthority,
    KernelGraphObligationAdoption,
    KernelPhaseChain,
    WorkloadCompositionHandoffs,
    ForgeQueryConsumerKit,
}

impl WorthGraphAuthorityRootFamily {
    pub const ALL: [Self; 9] = [
        Self::TopologyOperatorCatalog,
        Self::TopologyQueryNativeBoundary,
        Self::SpatialQueryAdoption,
        Self::SpatialEvidenceStageIndex,
        Self::KernelQueryAuthority,
        Self::KernelGraphObligationAdoption,
        Self::KernelPhaseChain,
        Self::WorkloadCompositionHandoffs,
        Self::ForgeQueryConsumerKit,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthLowerAuthorityPromotionCase {
    SupportReportNotGraphAuthority,
    CrateAdoptionNotGraphObligationProof,
    HandoffNotExecutedBirth,
    RawEvidenceNotStageIndex,
    StringPrefixNotTypedStageLink,
    SplitDigestNotLoopDigest,
    SyntheticFixtureNotProductionProof,
    ResidueLiteralNotCertifiedManifest,
}

impl WorthLowerAuthorityPromotionCase {
    pub const ALL: [Self; 8] = [
        Self::SupportReportNotGraphAuthority,
        Self::CrateAdoptionNotGraphObligationProof,
        Self::HandoffNotExecutedBirth,
        Self::RawEvidenceNotStageIndex,
        Self::StringPrefixNotTypedStageLink,
        Self::SplitDigestNotLoopDigest,
        Self::SyntheticFixtureNotProductionProof,
        Self::ResidueLiteralNotCertifiedManifest,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityInventoryRow {
    pub(crate) source_id: &'static str,
    pub(crate) source_path: &'static str,
    pub(crate) source_scope: WorthGraphAuthoritySourceScope,
    pub(crate) owner: WorthGraphAuthorityOwner,
    pub(crate) row_class: WorthGraphAuthorityRowClass,
    pub(crate) deletion_target: WorthGraphAuthorityDeletionTarget,
    pub(crate) discovery_source: WorthGraphAuthorityDiscoverySource,
    pub(crate) action: WorthGraphAuthorityAction,
    pub(crate) authority_claim: &'static str,
    pub(crate) replacement_or_blocker: &'static str,
    pub(crate) qa_evidence: &'static str,
}

impl WorthGraphAuthorityInventoryRow {
    pub const fn source_id(&self) -> &'static str {
        self.source_id
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn source_scope(&self) -> WorthGraphAuthoritySourceScope {
        self.source_scope
    }

    pub const fn owner(&self) -> WorthGraphAuthorityOwner {
        self.owner
    }

    pub const fn row_class(&self) -> WorthGraphAuthorityRowClass {
        self.row_class
    }

    pub const fn deletion_target(&self) -> WorthGraphAuthorityDeletionTarget {
        self.deletion_target
    }

    pub const fn discovery_source(&self) -> WorthGraphAuthorityDiscoverySource {
        self.discovery_source
    }

    pub const fn action(&self) -> WorthGraphAuthorityAction {
        self.action
    }

    pub const fn authority_claim(&self) -> &'static str {
        self.authority_claim
    }

    pub const fn replacement_or_blocker(&self) -> &'static str {
        self.replacement_or_blocker
    }

    pub const fn qa_evidence(&self) -> &'static str {
        self.qa_evidence
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphAuthoritySourceScope {
    ExactSource,
    AuditedSourceSet {
        expected_sources: usize,
        manifest_digest: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityDeletionLedgerRow {
    pub(crate) target_id: &'static str,
    pub(crate) source_path: &'static str,
    pub(crate) owner: WorthGraphAuthorityOwner,
    pub(crate) deletion_target: WorthGraphAuthorityDeletionTarget,
    pub(crate) action: WorthGraphAuthorityAction,
    pub(crate) replacement_or_blocker: &'static str,
    pub(crate) distinct_authority_proof: &'static str,
    pub(crate) residue_owner: &'static str,
    pub(crate) residue_cap: &'static str,
    pub(crate) introduced_phase: &'static str,
    pub(crate) removal_trigger: &'static str,
    pub(crate) qa_evidence: &'static str,
}

impl WorthGraphAuthorityDeletionLedgerRow {
    pub const fn target_id(&self) -> &'static str {
        self.target_id
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn owner(&self) -> WorthGraphAuthorityOwner {
        self.owner
    }

    pub const fn deletion_target(&self) -> WorthGraphAuthorityDeletionTarget {
        self.deletion_target
    }

    pub const fn action(&self) -> WorthGraphAuthorityAction {
        self.action
    }

    pub const fn replacement_or_blocker(&self) -> &'static str {
        self.replacement_or_blocker
    }

    pub const fn distinct_authority_proof(&self) -> &'static str {
        self.distinct_authority_proof
    }

    pub const fn residue_owner(&self) -> &'static str {
        self.residue_owner
    }

    pub const fn residue_cap(&self) -> &'static str {
        self.residue_cap
    }

    pub const fn introduced_phase(&self) -> &'static str {
        self.introduced_phase
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn qa_evidence(&self) -> &'static str {
        self.qa_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityDiscoveryRecord {
    pub(crate) root_family: WorthGraphAuthorityRootFamily,
    pub(crate) owner: WorthGraphAuthorityOwner,
    pub(crate) root_surface: &'static str,
    pub(crate) intentional_break: &'static str,
    pub(crate) downstream_compile_failures: &'static str,
    pub(crate) final_enforced_api: &'static str,
    pub(crate) qa_evidence: &'static str,
}

impl WorthGraphAuthorityDiscoveryRecord {
    pub const fn root_family(&self) -> WorthGraphAuthorityRootFamily {
        self.root_family
    }

    pub const fn owner(&self) -> WorthGraphAuthorityOwner {
        self.owner
    }

    pub const fn root_surface(&self) -> &'static str {
        self.root_surface
    }

    pub const fn intentional_break(&self) -> &'static str {
        self.intentional_break
    }

    pub const fn downstream_compile_failures(&self) -> &'static str {
        self.downstream_compile_failures
    }

    pub const fn final_enforced_api(&self) -> &'static str {
        self.final_enforced_api
    }

    pub const fn qa_evidence(&self) -> &'static str {
        self.qa_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthLowerAuthorityPromotionGuardPlan {
    pub(crate) promotion_case: WorthLowerAuthorityPromotionCase,
    pub(crate) lower_authority_surface: &'static str,
    pub(crate) required_authority_type: &'static str,
    pub(crate) planned_compile_fail_path: &'static str,
    pub(crate) enforcement_stage: &'static str,
    pub(crate) qa_evidence: &'static str,
}

impl WorthLowerAuthorityPromotionGuardPlan {
    pub const fn promotion_case(&self) -> WorthLowerAuthorityPromotionCase {
        self.promotion_case
    }

    pub const fn lower_authority_surface(&self) -> &'static str {
        self.lower_authority_surface
    }

    pub const fn required_authority_type(&self) -> &'static str {
        self.required_authority_type
    }

    pub const fn planned_compile_fail_path(&self) -> &'static str {
        self.planned_compile_fail_path
    }

    pub const fn enforcement_stage(&self) -> &'static str {
        self.enforcement_stage
    }

    pub const fn qa_evidence(&self) -> &'static str {
        self.qa_evidence
    }
}
