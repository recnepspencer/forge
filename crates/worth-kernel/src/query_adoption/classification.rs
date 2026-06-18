#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAdoptionInventoryOwner {
    Kernel,
    Spatial,
    Topology,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAdoptionClassification {
    Production,
    TestSupport,
    CertificationOnly,
    ExplicitResidue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAdoptionForbiddenPattern {
    SyntheticReceipt,
    ForgedEvidenceRow,
    DirectSupportPostureAssumption,
    LowerAuthorityIdentityReconstruction,
    TestFixtureTruthPromotion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAuthorityCategory {
    Authoritative,
    Derived,
    Diagnostic,
    CertificationOnly,
    TestSupportOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAuthorityDomain {
    TopologyTruth,
    SpatialWitnessTruth,
    SpatialEvidence,
    KernelOrchestration,
    QuerySupport,
    QueryEvidence,
    CertificationProof,
    TestSupport,
    DiagnosticResidue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAuthorityPromotionTarget {
    TopologyTruth,
    SpatialWitnessTruth,
    SupportPin,
    EvidenceReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryAdoptionInventoryRow {
    owner: WorthQueryAdoptionInventoryOwner,
    source_set: &'static str,
    responsibility: &'static str,
    classification: WorthQueryAdoptionClassification,
    authority_category: WorthQueryAuthorityCategory,
    authority_domain: WorthQueryAuthorityDomain,
    forbidden_pattern: Option<WorthQueryAdoptionForbiddenPattern>,
    replacement_surface: &'static str,
}

impl WorthQueryAdoptionInventoryRow {
    pub const fn new(
        owner: WorthQueryAdoptionInventoryOwner,
        source_set: &'static str,
        responsibility: &'static str,
        classification: WorthQueryAdoptionClassification,
        authority_category: WorthQueryAuthorityCategory,
        authority_domain: WorthQueryAuthorityDomain,
        forbidden_pattern: Option<WorthQueryAdoptionForbiddenPattern>,
        replacement_surface: &'static str,
    ) -> Self {
        Self {
            owner,
            source_set,
            responsibility,
            classification,
            authority_category,
            authority_domain,
            forbidden_pattern,
            replacement_surface,
        }
    }

    pub const fn owner(&self) -> WorthQueryAdoptionInventoryOwner {
        self.owner
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }

    pub const fn responsibility(&self) -> &'static str {
        self.responsibility
    }

    pub const fn classification(&self) -> WorthQueryAdoptionClassification {
        self.classification
    }

    pub const fn authority_category(&self) -> WorthQueryAuthorityCategory {
        self.authority_category
    }

    pub const fn authority_domain(&self) -> WorthQueryAuthorityDomain {
        self.authority_domain
    }

    pub const fn forbidden_pattern(&self) -> Option<WorthQueryAdoptionForbiddenPattern> {
        self.forbidden_pattern
    }

    pub const fn replacement_surface(&self) -> &'static str {
        self.replacement_surface
    }
}
