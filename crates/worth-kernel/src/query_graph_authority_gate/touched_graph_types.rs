#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthTouchedGraphAuthorityInventoryCategory {
    StaticValidator,
    StaticInvariant,
    OperatorIntentLowering,
    QueryRuntimeSelection,
    DerivedInvalidation,
    DirtyPropagation,
    ReplayScope,
    EvidenceLookup,
    ConflictIndependence,
    CacheEquivalenceReuse,
    UndoTransactionScope,
    DiagnosticSurface,
    PublicFacade,
    DeletionLedger,
    CompositionLineCap,
}

impl WorthTouchedGraphAuthorityInventoryCategory {
    pub const ALL: &'static [Self] = &[
        Self::StaticValidator,
        Self::StaticInvariant,
        Self::OperatorIntentLowering,
        Self::QueryRuntimeSelection,
        Self::DerivedInvalidation,
        Self::DirtyPropagation,
        Self::ReplayScope,
        Self::EvidenceLookup,
        Self::ConflictIndependence,
        Self::CacheEquivalenceReuse,
        Self::UndoTransactionScope,
        Self::DiagnosticSurface,
        Self::PublicFacade,
        Self::DeletionLedger,
        Self::CompositionLineCap,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphAuthorityDisposition {
    Delete,
    Collapse,
    CertificationOnly,
    Residue,
    QueryGap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphAuthorityInventoryRow {
    source_id: &'static str,
    source_path: &'static str,
    category: WorthTouchedGraphAuthorityInventoryCategory,
    owner: &'static str,
    current_authority_source: &'static str,
    touched_graph_replacement: &'static str,
    disposition: WorthTouchedGraphAuthorityDisposition,
    residue_cap: &'static str,
    removal_trigger: &'static str,
    ordinary_public_facade: &'static str,
    qa_evidence: &'static str,
}

impl WorthTouchedGraphAuthorityInventoryRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        source_id: &'static str,
        source_path: &'static str,
        category: WorthTouchedGraphAuthorityInventoryCategory,
        owner: &'static str,
        current_authority_source: &'static str,
        touched_graph_replacement: &'static str,
        disposition: WorthTouchedGraphAuthorityDisposition,
        residue_cap: &'static str,
        removal_trigger: &'static str,
        ordinary_public_facade: &'static str,
        qa_evidence: &'static str,
    ) -> Self {
        Self {
            source_id,
            source_path,
            category,
            owner,
            current_authority_source,
            touched_graph_replacement,
            disposition,
            residue_cap,
            removal_trigger,
            ordinary_public_facade,
            qa_evidence,
        }
    }

    pub const fn source_id(&self) -> &'static str {
        self.source_id
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn category(&self) -> WorthTouchedGraphAuthorityInventoryCategory {
        self.category
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn current_authority_source(&self) -> &'static str {
        self.current_authority_source
    }

    pub const fn touched_graph_replacement(&self) -> &'static str {
        self.touched_graph_replacement
    }

    pub const fn disposition(&self) -> WorthTouchedGraphAuthorityDisposition {
        self.disposition
    }

    pub const fn residue_cap(&self) -> &'static str {
        self.residue_cap
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn ordinary_public_facade(&self) -> &'static str {
        self.ordinary_public_facade
    }

    pub const fn qa_evidence(&self) -> &'static str {
        self.qa_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphAuthorityDeletionLedgerRow {
    target_id: &'static str,
    source_id: &'static str,
    source_path: &'static str,
    owner: &'static str,
    disposition: WorthTouchedGraphAuthorityDisposition,
    former_public_surface: &'static str,
    enforced_outcome: &'static str,
    touched_graph_replacement: &'static str,
    removal_trigger: &'static str,
    ordinary_public_facade: &'static str,
    qa_evidence: &'static str,
}

impl WorthTouchedGraphAuthorityDeletionLedgerRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        target_id: &'static str,
        source_id: &'static str,
        source_path: &'static str,
        owner: &'static str,
        disposition: WorthTouchedGraphAuthorityDisposition,
        former_public_surface: &'static str,
        enforced_outcome: &'static str,
        touched_graph_replacement: &'static str,
        removal_trigger: &'static str,
        ordinary_public_facade: &'static str,
        qa_evidence: &'static str,
    ) -> Self {
        Self {
            target_id,
            source_id,
            source_path,
            owner,
            disposition,
            former_public_surface,
            enforced_outcome,
            touched_graph_replacement,
            removal_trigger,
            ordinary_public_facade,
            qa_evidence,
        }
    }

    pub const fn target_id(&self) -> &'static str {
        self.target_id
    }

    pub const fn source_id(&self) -> &'static str {
        self.source_id
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn disposition(&self) -> WorthTouchedGraphAuthorityDisposition {
        self.disposition
    }

    pub const fn former_public_surface(&self) -> &'static str {
        self.former_public_surface
    }

    pub const fn enforced_outcome(&self) -> &'static str {
        self.enforced_outcome
    }

    pub const fn touched_graph_replacement(&self) -> &'static str {
        self.touched_graph_replacement
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn ordinary_public_facade(&self) -> &'static str {
        self.ordinary_public_facade
    }

    pub const fn qa_evidence(&self) -> &'static str {
        self.qa_evidence
    }
}
