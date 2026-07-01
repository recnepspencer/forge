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

const SPATIAL_RESIDUE: [SpatialConsumerResidueRow; 2] = [
    SpatialConsumerResidueRow::new(
        "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/current_source.rs",
        "current_evidence_lookup_public_closeout",
        SpatialConsumerResidueOwner::WorthSpatial,
        SpatialConsumerResidueDisposition::ExplicitResidue,
        "public closeout still belongs to phase 13 boundary-crossing consumer cutover",
        "replace once public closeout lowers selected equivalence and reuse products through the phase 13 boundary lane",
    ),
    SpatialConsumerResidueRow::new(
        "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/current_source.rs",
        "current_evidence_lookup_public_closeout_assembly_input",
        SpatialConsumerResidueOwner::WorthSpatial,
        SpatialConsumerResidueDisposition::CertificationOnly,
        "assembly input remains closeout-only support and must not survive as ordinary consumer authority",
        "remove once phase 13 public closeout lowerings no longer need a direct assembly proof surface",
    ),
];

pub fn current_spatial_consumer_residue_manifest() -> &'static [SpatialConsumerResidueRow] {
    &SPATIAL_RESIDUE
}
