#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutResidueOwner {
    WorthSpatial,
    WorthTopo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutResidueDisposition {
    ExplicitResidue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutResidueRow {
    source_path: &'static str,
    current_surface: &'static str,
    owner: EvidenceLookupPublicCloseoutResidueOwner,
    disposition: EvidenceLookupPublicCloseoutResidueDisposition,
    blocker: &'static str,
    removal_trigger: &'static str,
}

impl EvidenceLookupPublicCloseoutResidueRow {
    pub const fn new(
        source_path: &'static str,
        current_surface: &'static str,
        owner: EvidenceLookupPublicCloseoutResidueOwner,
        disposition: EvidenceLookupPublicCloseoutResidueDisposition,
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

    pub const fn owner(&self) -> EvidenceLookupPublicCloseoutResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> EvidenceLookupPublicCloseoutResidueDisposition {
        self.disposition
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

const RESIDUE: [EvidenceLookupPublicCloseoutResidueRow; 0] = [];

pub fn current_evidence_lookup_public_closeout_residue_manifest(
) -> &'static [EvidenceLookupPublicCloseoutResidueRow] {
    &RESIDUE
}
