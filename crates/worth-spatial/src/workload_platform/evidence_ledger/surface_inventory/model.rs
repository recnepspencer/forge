#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceSurfaceAuthorityCategory {
    PublicFacadeExport,
    LedgerConstructor,
    BooleanReceiptImplementation,
    SpatialQueryAdoptionRow,
    KernelWorkloadEvidenceConsumption,
    TopologySubstitutionBoundary,
    DeletedLegacySurface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceSurfaceDeletionAction {
    Delete,
    CollapseToSpatialTouchAuthority,
    CollapseToQueryConsumerKitProof,
    CertificationOnly,
    CappedResidue,
    Deleted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceSurfaceOwner {
    WorthSpatial,
    WorthKernel,
    WorthTopo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceSurfaceCloseoutPosture {
    PlannedReplacement,
    CertificationOnly,
    CappedResidue,
    Deleted,
    ProductionReachableAfterReplacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceSurfaceDeletionLedgerRow {
    surface_name: &'static str,
    source_path: &'static str,
    exported_facade_path: &'static str,
    authority_category: SpatialEvidenceSurfaceAuthorityCategory,
    current_caller: &'static str,
    deletion_action: SpatialEvidenceSurfaceDeletionAction,
    owner: SpatialEvidenceSurfaceOwner,
    cap: &'static str,
    removal_trigger: &'static str,
    production_reachable: bool,
    replacement_exists: bool,
}

impl SpatialEvidenceSurfaceDeletionLedgerRow {
    pub const fn new(
        surface_name: &'static str,
        source_path: &'static str,
        exported_facade_path: &'static str,
        authority_category: SpatialEvidenceSurfaceAuthorityCategory,
        current_caller: &'static str,
        deletion_action: SpatialEvidenceSurfaceDeletionAction,
        owner: SpatialEvidenceSurfaceOwner,
        cap: &'static str,
        removal_trigger: &'static str,
        production_reachable: bool,
        replacement_exists: bool,
    ) -> Self {
        Self {
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
        }
    }

    pub const fn surface_name(&self) -> &'static str {
        self.surface_name
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn exported_facade_path(&self) -> &'static str {
        self.exported_facade_path
    }

    pub const fn authority_category(&self) -> SpatialEvidenceSurfaceAuthorityCategory {
        self.authority_category
    }

    pub const fn current_caller(&self) -> &'static str {
        self.current_caller
    }

    pub const fn deletion_action(&self) -> SpatialEvidenceSurfaceDeletionAction {
        self.deletion_action
    }

    pub const fn owner(&self) -> SpatialEvidenceSurfaceOwner {
        self.owner
    }

    pub const fn cap(&self) -> &'static str {
        self.cap
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn production_reachable(&self) -> bool {
        self.production_reachable
    }

    pub const fn replacement_exists(&self) -> bool {
        self.replacement_exists
    }

    pub const fn has_deletion_or_cap_plan(&self) -> bool {
        !self.cap.is_empty() && !self.removal_trigger.is_empty()
    }

    pub const fn closeout_posture(&self) -> SpatialEvidenceSurfaceCloseoutPosture {
        if self.production_reachable
            && self.replacement_exists
            && !matches!(
                self.deletion_action,
                SpatialEvidenceSurfaceDeletionAction::Deleted
            )
        {
            return SpatialEvidenceSurfaceCloseoutPosture::ProductionReachableAfterReplacement;
        }

        match self.deletion_action {
            SpatialEvidenceSurfaceDeletionAction::CertificationOnly => {
                SpatialEvidenceSurfaceCloseoutPosture::CertificationOnly
            }
            SpatialEvidenceSurfaceDeletionAction::CappedResidue => {
                SpatialEvidenceSurfaceCloseoutPosture::CappedResidue
            }
            SpatialEvidenceSurfaceDeletionAction::Deleted => {
                SpatialEvidenceSurfaceCloseoutPosture::Deleted
            }
            SpatialEvidenceSurfaceDeletionAction::Delete
            | SpatialEvidenceSurfaceDeletionAction::CollapseToSpatialTouchAuthority
            | SpatialEvidenceSurfaceDeletionAction::CollapseToQueryConsumerKitProof => {
                SpatialEvidenceSurfaceCloseoutPosture::PlannedReplacement
            }
        }
    }

    pub const fn violates_replaced_production_bypass(&self) -> bool {
        matches!(
            self.closeout_posture(),
            SpatialEvidenceSurfaceCloseoutPosture::ProductionReachableAfterReplacement
        )
    }
}
