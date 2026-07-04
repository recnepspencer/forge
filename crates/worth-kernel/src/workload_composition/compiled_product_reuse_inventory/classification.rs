#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CompiledProductReuseSemanticCategory {
    OrdinaryReuse,
    PseudoReuse,
    RebuildSuppression,
    ParityShortcut,
    RetainedProductCarryForward,
    LocalEquivalenceHelper,
}

impl CompiledProductReuseSemanticCategory {
    pub const REQUIRED_COVERED: [Self; 6] = [
        Self::OrdinaryReuse,
        Self::PseudoReuse,
        Self::RebuildSuppression,
        Self::ParityShortcut,
        Self::RetainedProductCarryForward,
        Self::LocalEquivalenceHelper,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryReuse => "ordinary-reuse",
            Self::PseudoReuse => "pseudo-reuse",
            Self::RebuildSuppression => "rebuild-suppression",
            Self::ParityShortcut => "parity-shortcut",
            Self::RetainedProductCarryForward => "retained-product-carry-forward",
            Self::LocalEquivalenceHelper => "local-equivalence-helper",
        }
    }

    pub const fn is_named(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompiledProductReuseSemanticDistinction {
    CompiledProductIdentity,
    Equivalence,
    Compatibility,
    AuthorityTruth,
}

impl CompiledProductReuseSemanticDistinction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompiledProductIdentity => "compiled-product-identity",
            Self::Equivalence => "equivalence",
            Self::Compatibility => "compatibility",
            Self::AuthorityTruth => "authority-truth",
        }
    }

    pub const fn is_named(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompiledProductReuseAuthorityKind {
    LocalDigestReuseKey,
    LocalEquivalenceDigestComparison,
    RowCountShortcut,
    RenderedShapeEquality,
    BoundedRebuildPosture,
    RetainedParityHelper,
    RetainedCarryForwardHelper,
    PublicReadModelReuseDescriptor,
    CloseoutConsumerReusePressure,
}

impl CompiledProductReuseAuthorityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDigestReuseKey => "local-digest-reuse-key",
            Self::LocalEquivalenceDigestComparison => "local-equivalence-digest-comparison",
            Self::RowCountShortcut => "row-count-shortcut",
            Self::RenderedShapeEquality => "rendered-shape-equality",
            Self::BoundedRebuildPosture => "bounded-rebuild-posture",
            Self::RetainedParityHelper => "retained-parity-helper",
            Self::RetainedCarryForwardHelper => "retained-carry-forward-helper",
            Self::PublicReadModelReuseDescriptor => "public-read-model-reuse-descriptor",
            Self::CloseoutConsumerReusePressure => "closeout-consumer-reuse-pressure",
        }
    }

    pub const fn is_named(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompiledProductReuseDisposition {
    Migrate,
    Delete,
    Cap,
    CertificationOnly,
    QueryGap,
}

impl CompiledProductReuseDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Migrate => "migrate",
            Self::Delete => "delete",
            Self::Cap => "cap",
            Self::CertificationOnly => "certification-only",
            Self::QueryGap => "query-gap",
        }
    }

    pub const fn is_classified(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompiledProductReuseOwner {
    WorthKernel,
    WorthTopo,
    WorthSpatial,
    ForgeQuery,
}

impl CompiledProductReuseOwner {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorthKernel => "worth-kernel",
            Self::WorthTopo => "worth-topo",
            Self::WorthSpatial => "worth-spatial",
            Self::ForgeQuery => "forge-query",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompiledProductReuseReplacementPhase {
    PhaseTwoSharedVocabulary,
    PhaseThreeTopologyFamilyCatalog,
    PhaseFourSpatialFamilyCatalog,
    PhaseFiveKernelDependencyMatrix,
    PhaseEightSelectedEquivalenceFamily,
    PhaseNineReuseDecision,
    PhaseTwelveSpatialConsumerCutover,
    PhaseThirteenPublicReadModelCutover,
    PhaseFifteenFirewallDeletion,
    NotReplacedCertificationOnly,
    BlockedOnQueryCapability,
}

impl CompiledProductReuseReplacementPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhaseTwoSharedVocabulary => "phase-two-shared-vocabulary",
            Self::PhaseThreeTopologyFamilyCatalog => "phase-three-topology-family-catalog",
            Self::PhaseFourSpatialFamilyCatalog => "phase-four-spatial-family-catalog",
            Self::PhaseFiveKernelDependencyMatrix => "phase-five-kernel-dependency-matrix",
            Self::PhaseEightSelectedEquivalenceFamily => "phase-eight-selected-equivalence-family",
            Self::PhaseNineReuseDecision => "phase-nine-reuse-decision",
            Self::PhaseTwelveSpatialConsumerCutover => "phase-twelve-spatial-consumer-cutover",
            Self::PhaseThirteenPublicReadModelCutover => "phase-thirteen-public-read-model-cutover",
            Self::PhaseFifteenFirewallDeletion => "phase-fifteen-firewall-deletion",
            Self::NotReplacedCertificationOnly => "not-replaced-certification-only",
            Self::BlockedOnQueryCapability => "blocked-on-query-capability",
        }
    }
}
