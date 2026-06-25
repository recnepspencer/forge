#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectionBoundaryInventory {
    rows: Vec<QuerySelectionBoundaryInventoryRow>,
}

impl QuerySelectionBoundaryInventory {
    pub fn new(rows: Vec<QuerySelectionBoundaryInventoryRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[QuerySelectionBoundaryInventoryRow] {
        &self.rows
    }

    pub fn row_named(&self, surface: &str) -> Option<&QuerySelectionBoundaryInventoryRow> {
        self.rows.iter().find(|row| row.surface() == surface)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectionBoundaryInventoryRow {
    source_path: &'static str,
    exported_facade_path: Option<&'static str>,
    surface: &'static str,
    classification: QuerySelectionSurfaceClassification,
    authority_posture: QuerySelectionAuthorityPosture,
    proof_strength: QuerySelectionProofStrength,
    current_caller: &'static str,
    deletion_action: QuerySelectionDeletionAction,
    owner: QuerySelectionSurfaceOwner,
    cap: Option<&'static str>,
    blocker: Option<&'static str>,
    removal_trigger: Option<&'static str>,
}

impl QuerySelectionBoundaryInventoryRow {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        source_path: &'static str,
        exported_facade_path: Option<&'static str>,
        surface: &'static str,
        classification: QuerySelectionSurfaceClassification,
        authority_posture: QuerySelectionAuthorityPosture,
        proof_strength: QuerySelectionProofStrength,
        current_caller: &'static str,
        deletion_action: QuerySelectionDeletionAction,
        owner: QuerySelectionSurfaceOwner,
        cap: Option<&'static str>,
        blocker: Option<&'static str>,
        removal_trigger: Option<&'static str>,
    ) -> Self {
        Self {
            source_path,
            exported_facade_path,
            surface,
            classification,
            authority_posture,
            proof_strength,
            current_caller,
            deletion_action,
            owner,
            cap,
            blocker,
            removal_trigger,
        }
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn exported_facade_path(&self) -> Option<&'static str> {
        self.exported_facade_path
    }

    pub fn surface(&self) -> &'static str {
        self.surface
    }

    pub fn classification(&self) -> QuerySelectionSurfaceClassification {
        self.classification
    }

    pub fn authority_posture(&self) -> QuerySelectionAuthorityPosture {
        self.authority_posture
    }

    pub fn proof_strength(&self) -> QuerySelectionProofStrength {
        self.proof_strength
    }

    pub fn current_caller(&self) -> &'static str {
        self.current_caller
    }

    pub fn deletion_action(&self) -> QuerySelectionDeletionAction {
        self.deletion_action
    }

    pub fn owner(&self) -> QuerySelectionSurfaceOwner {
        self.owner
    }

    pub fn cap(&self) -> Option<&'static str> {
        self.cap
    }

    pub fn blocker(&self) -> Option<&'static str> {
        self.blocker
    }

    pub fn removal_trigger(&self) -> Option<&'static str> {
        self.removal_trigger
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectionSurfaceClassification {
    SourceDescriptor,
    QueryOwnedSelection,
    MigrationProjection,
    CertificationOnlySupport,
    DeletionTarget,
    CappedResidue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectionAuthorityPosture {
    DescriptorInput,
    RegistrationDeclaration,
    SelectorCoverageDeclaration,
    SupportPin,
    SupportMatrix,
    LocalCeremonyAudit,
    ResidueManifest,
    InMemorySelectionAdoption,
    ExecutionBackedSelectionAdoption,
    SelectorPrecisionCounters,
    SelectedObligationExecutionEvidence,
    PublicFacadeStatus,
}

impl QuerySelectionAuthorityPosture {
    pub const fn is_selected_obligation_proof(self) -> bool {
        matches!(
            self,
            Self::ExecutionBackedSelectionAdoption | Self::SelectedObligationExecutionEvidence
        )
    }

    pub const fn is_support_only(self) -> bool {
        matches!(self, Self::SupportPin | Self::SupportMatrix)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectionProofStrength {
    SourceDescriptorOnly,
    RegistrationOnly,
    SupportOnly,
    LocalCeremonyOnly,
    ResidueOnly,
    InMemorySelection,
    ExecutionBackedAdoption,
    ExecutionEnvelope,
    CounterOnly,
    PublicStatusOnly,
}

impl QuerySelectionProofStrength {
    pub const fn claims_execution(self) -> bool {
        matches!(
            self,
            Self::ExecutionBackedAdoption | Self::ExecutionEnvelope
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectionDeletionAction {
    KeepAsSourceDescriptor,
    KeepAsQueryOwnedSelection,
    MigrateToParallelSelectionSubstrate,
    CollapseToQueryOwnedSelection,
    CertificationOnly,
    DeleteAfterVerticalLane,
    CappedResidue,
    QueryGapBlocksMigration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectionSurfaceOwner {
    ForgeQuery,
    WorthKernel,
    WorthSpatial,
    WorthTopo,
}
