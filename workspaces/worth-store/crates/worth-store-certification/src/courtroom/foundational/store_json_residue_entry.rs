#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoreJsonResidueTokenKind {
    SerdeJson,
    JsonMacro,
    Serialize,
    Deserialize,
    DeserializeOwned,
    RawJsonHelper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoreJsonResidueZone {
    LegacyCompatibilityResidue,
    LegacyHostileDenialTest,
    DedicatedWorkspaceCertificationEnforcement,
    DedicatedWorkspaceDurableSerdeContract,
    DedicatedWorkspaceTerminalBoundary,
    DedicatedWorkspaceHostileReadmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoreJsonAuthorityRisk {
    LegacySerdeAuthorityResidue,
    LegacyJsonPersistenceResidue,
    LegacyDigestBasisResidue,
    HostileDenialOnly,
    CertificationScannerVocabulary,
    CertificationToolProtocolOnly,
    DurableSerdeContractOnly,
    TerminalProjectionOnly,
    HostileReadmissionOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreJsonResidueOccurrence {
    path: String,
    line: u32,
    token: StoreJsonResidueTokenKind,
    excerpt: String,
}

impl StoreJsonResidueOccurrence {
    pub fn new(
        path: impl Into<String>,
        line: u32,
        token: StoreJsonResidueTokenKind,
        excerpt: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            line,
            token,
            excerpt: excerpt.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn line(&self) -> u32 {
        self.line
    }

    pub const fn token(&self) -> StoreJsonResidueTokenKind {
        self.token
    }

    pub fn excerpt(&self) -> &str {
        &self.excerpt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreJsonResidueClassification {
    occurrence: StoreJsonResidueOccurrence,
    zone: StoreJsonResidueZone,
    owner: &'static str,
    authority_risk: StoreJsonAuthorityRisk,
    quarantine_or_removal_condition: &'static str,
}

impl StoreJsonResidueClassification {
    pub(crate) fn checked(
        occurrence: StoreJsonResidueOccurrence,
        zone: StoreJsonResidueZone,
        owner: &'static str,
        authority_risk: StoreJsonAuthorityRisk,
        quarantine_or_removal_condition: &'static str,
    ) -> Option<Self> {
        if owner.is_empty() || quarantine_or_removal_condition.is_empty() {
            return None;
        }
        Some(Self {
            occurrence,
            zone,
            owner,
            authority_risk,
            quarantine_or_removal_condition,
        })
    }

    pub fn occurrence(&self) -> &StoreJsonResidueOccurrence {
        &self.occurrence
    }

    pub const fn zone(&self) -> StoreJsonResidueZone {
        self.zone
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn authority_risk(&self) -> StoreJsonAuthorityRisk {
        self.authority_risk
    }

    pub const fn quarantine_or_removal_condition(&self) -> &'static str {
        self.quarantine_or_removal_condition
    }

    pub const fn is_quarantined_terminal_or_hostile_boundary(&self) -> bool {
        matches!(
            self.zone,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement
                | StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary
                | StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission
        )
    }

    pub fn is_durable_serde_contract(&self) -> bool {
        self.zone == StoreJsonResidueZone::DedicatedWorkspaceDurableSerdeContract
            && self.authority_risk == StoreJsonAuthorityRisk::DurableSerdeContractOnly
    }
}
