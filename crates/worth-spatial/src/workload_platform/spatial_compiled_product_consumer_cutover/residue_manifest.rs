#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConsumerResidueOwner {
    WorthSpatial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConsumerResidueDisposition {
    ExplicitResidue,
    CertificationOnly,
    AuthoritativeOrdinaryConsumer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialConsumerResidueRow {
    source_path: &'static str,
    current_surface: &'static str,
    owner: SpatialConsumerResidueOwner,
    disposition: SpatialConsumerResidueDisposition,
    blocker: &'static str,
    removal_trigger: &'static str,
}

impl SpatialConsumerResidueRow {
    pub const fn new(
        source_path: &'static str,
        current_surface: &'static str,
        owner: SpatialConsumerResidueOwner,
        disposition: SpatialConsumerResidueDisposition,
        blocker: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            source_path,
            current_surface,
            owner,
            disposition,
            blocker,
            removal_trigger,
        }
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn current_surface(&self) -> &'static str {
        self.current_surface
    }

    pub const fn owner(&self) -> SpatialConsumerResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> SpatialConsumerResidueDisposition {
        self.disposition
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

const SPATIAL_RESIDUE: [SpatialConsumerResidueRow; 1] = [
    SpatialConsumerResidueRow::new(
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs",
        "current_evidence_lookup_public_closeout_assembly_input",
        SpatialConsumerResidueOwner::WorthSpatial,
        SpatialConsumerResidueDisposition::CertificationOnly,
        "assembly input remains an internal denial-proof seam and must not survive as ordinary consumer authority",
        "remove once public closeout denial coverage no longer needs a direct admitted-assembly proof surface",
    ),
];

pub fn current_spatial_consumer_residue_manifest() -> &'static [SpatialConsumerResidueRow] {
    &SPATIAL_RESIDUE
}
